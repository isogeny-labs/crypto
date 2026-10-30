#!/usr/bin/env python3
"""Bounded native CLI fixture. This file is inert until explicitly invoked."""
import argparse
import datetime
import os
from pathlib import Path
import platform
import resource
import shutil
import signal
import subprocess
import sys
import time
import traceback

from common import (HERE, LIMITS, canonical, config, decode, digest_bytes, file_bindings,
                    inventory, jsonl, load, require, sha, write)
from verify import cell_directory, inspect, read_events


class Stop(Exception):
    pass


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def directory_bytes(root):
    total = 0
    for path in Path(root).rglob("*"):
        require(not path.is_symlink(), "symlink in attempt directory")
        if path.is_file():
            total += path.stat().st_size
    return total


def metadata_command(args):
    # Git identity only, never a solver command. Bound output/time.
    result = subprocess.run(args, capture_output=True, timeout=20, check=False)
    require(result.returncode == 0, "Git identity command failed: " + repr(args))
    require(len(result.stdout) <= 32 * 1024**2, "Git metadata output too large")
    return result.stdout


def admit_protocol(path, binary):
    protocol = load(path)
    freeze = protocol.get("freeze", {})
    require(protocol.get("schema") == "experiment.protocol/v1" and freeze.get("status") == "frozen"
            and freeze.get("filed_before_affected_data") is True and freeze.get("frozen_at"),
            "requires preexisting frozen protocol; driver never prepares or freezes it")
    body = dict(protocol)
    del body["freeze"]
    require(digest_bytes(canonical(body)) == freeze.get("protocol_sha256"), "protocol canonical body hash")
    fixture = protocol["fixture"]
    require(fixture["limits"] == LIMITS and fixture["driver_files"] == file_bindings(),
            "protocol differs from frozen driver/plan/limits")
    require(fixture["candidates"] == [{"cell_index": i, "config": config(i)} for i in range(16)],
            "protocol candidate table mismatch")
    require(sha(binary) == fixture["binary_sha256"], "binary differs from frozen protocol")
    receipt_path = path.with_name(path.stem + ".validation.json")
    require(receipt_path.exists(), "official protocol schema/checker gate unavailable: missing validation receipt")
    receipt = load(receipt_path)
    require(receipt.get("schema") == "ml-dataset.fixture-protocol-validation-receipt/v1" and
            receipt.get("official_schema_validation") == "passed" and receipt.get("exit_code") == 0 and
            receipt.get("protocol_file_sha256") == sha(path) and
            receipt.get("protocol_body_sha256") == freeze["protocol_sha256"],
            "missing/mismatched authoritative checker result (native preflight is not a substitute)")
    checker = receipt["checker"]
    require(checker.get("authority") and checker.get("version") and checker.get("command"),
            "checker identity/provenance missing")
    for label in ("checker", "stdout", "stderr", "schema_file"):
        item = receipt[label]
        checked = Path(item["path"])
        if not checked.is_absolute():
            checked = receipt_path.parent / checked
        require(checked.is_file() and not checked.is_symlink() and sha(checked) == item["sha256"],
                "checker evidence hash mismatch: " + label)
    repo = Path(metadata_command(["git", "-C", str(path.parent), "rev-parse", "--show-toplevel"]).decode().strip())
    relative = path.relative_to(repo).as_posix()
    commit = receipt["protocol_commit"]
    require(len(commit) == 40 and all(c in "0123456789abcdef" for c in commit), "full protocol commit required")
    committed = metadata_command(["git", "-C", str(repo), "show", f"{commit}:{relative}"])
    require(committed == path.read_bytes(), "frozen protocol bytes not present in specified Git commit")
    metadata_command(["git", "-C", str(repo), "merge-base", "--is-ancestor", commit, "HEAD"])
    return protocol, {"path": str(path), "file_sha256": sha(path), "body_sha256": freeze["protocol_sha256"],
                      "protocol_commit": commit, "validation_receipt": str(receipt_path),
                      "validation_receipt_sha256": sha(receipt_path), "checker_identity": checker}


class Driver:
    def __init__(self, output, binary, protocol):
        self.out = output
        self.original_binary = binary
        self.protocol_path = protocol
        self.binary = output / "frozen-crypto"
        self.started = time.monotonic()
        self.child = None
        self.frozen_sources = {}
        self.receipt = {"schema": "ml-dataset.sat-cli-fixture-result/v1", "started_utc": utc(),
                        "status": "initializing", "commands": [], "candidates": [], "controls": [],
                        "selected_pair": None, "transition_witness": None, "unrun_cells": list(range(16)),
                        "scientific_samples_added": 0, "verified_cloud_dataset_cells": 0,
                        "native_execution_trace_verified": False, "native_instruction_count_verified": False,
                        "unsat_proof_checked": False, "clock_valid_for_comparison": False,
                        "unified_export_gate": {"status": "missing", "reason": "authoritative experiment.export/v1 adapter/checker not available at authoring; rows/flow below are explicitly software-validation schemas"}}

    def checkpoint(self):
        write(self.out / "receipt.json", self.receipt)

    def remaining(self):
        return LIMITS["driver_seconds"] - (time.monotonic() - self.started)

    def stop_child(self):
        if self.child is None or self.child.poll() is not None:
            return
        try:
            os.killpg(self.child.pid, signal.SIGTERM)
        except ProcessLookupError:
            return
        try:
            self.child.wait(timeout=2)
        except subprocess.TimeoutExpired:
            try:
                os.killpg(self.child.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            self.child.wait(timeout=5)

    def command(self, name, args, timeout, expected_failure=False):
        require(all(row["name"] != name for row in self.receipt["commands"]), "duplicate command name")
        used = directory_bytes(self.out)
        if used >= LIMITS["attempt_bytes"]:
            raise Stop("attempt storage admission ceiling reached")
        # Leave up to120 seconds inside the overall wall envelope for retention.
        available = self.remaining() - 120
        if available <= 0:
            raise Stop("global wall admission ceiling reached (retention reserve)")
        timeout = min(timeout, available)
        log = self.out / "logs"
        stdout, stderr = log / (name + ".stdout"), log / (name + ".stderr")
        row = {"name": name, "argv": [str(self.binary), *map(str, args)], "started_utc": utc(),
               "timeout_seconds": timeout, "expected_failure": expected_failure,
               "stdout": str(stdout.relative_to(self.out)), "stderr": str(stderr.relative_to(self.out)),
               "status": "running", "attempt_bytes_before": used}
        self.receipt["commands"].append(row)
        self.checkpoint()
        before = resource.getrusage(resource.RUSAGE_CHILDREN)
        started = time.monotonic()
        try:
            with stdout.open("xb") as out, stderr.open("xb") as err:
                self.child = subprocess.Popen(row["argv"], stdout=out, stderr=err,
                                              start_new_session=True, env=dict(os.environ, RAYON_NUM_THREADS="8"))
                try:
                    row["exit_code"] = self.child.wait(timeout=timeout)
                    row["status"] = "completed" if row["exit_code"] == 0 else "nonzero_exit"
                except subprocess.TimeoutExpired:
                    row["status"] = "timeout"
                    self.stop_child()
                    row["exit_code"] = self.child.returncode
        except BaseException:
            row["status"] = "interrupted"
            self.stop_child()
            row["exit_code"] = self.child.returncode if self.child else None
            raise
        finally:
            self.child = None
            after = resource.getrusage(resource.RUSAGE_CHILDREN)
            row.update(finished_utc=utc(), elapsed_seconds_diagnostic=time.monotonic() - started,
                       clock_valid_for_comparison=False,
                       child_user_cpu_seconds=after.ru_utime - before.ru_utime,
                       child_system_cpu_seconds=after.ru_stime - before.ru_stime,
                       cumulative_child_peak_rss_native=after.ru_maxrss,
                       rss_unit="KiB on this Linux runner; high-water mark, not additive",
                       attempt_bytes_after=directory_bytes(self.out))
            for stream in (stdout, stderr):
                if stream.exists():
                    row[stream.suffix[1:] + "_sha256"] = sha(stream)
                    row[stream.suffix[1:] + "_bytes"] = stream.stat().st_size
            self.checkpoint()
        if row["status"] in ("timeout", "interrupted"):
            raise Stop("command did not complete: " + name)
        if row["attempt_bytes_after"] >= LIMITS["attempt_bytes"]:
            raise Stop("attempt storage ceiling exceeded; overshoot retained")
        if row["exit_code"] and not expected_failure:
            raise Stop("unexpected nonzero native CLI exit: " + name)
        return row

    def candidate(self, index):
        source = self.out / f"candidate-{index:02d}"
        destination = self.out / f"export-{index:02d}"
        row = {"cell_index": index, "config": config(index), "status": "started"}
        self.receipt["candidates"].append(row)
        self.receipt["unrun_cells"].remove(index)
        self.checkpoint()
        self.command(f"generate-{index:02d}", ["dataset", "generate", "--plan", self.out / "plan.json",
                     "--protocol", self.protocol_path, "--out", source, "--max-cases", "1",
                     "--shard-index", str(index), "--shard-count", "16", "--seconds-per-case", "30",
                     "--memory-mib", "2048", "--trace-mib", "64"], LIMITS["generate_seconds"])
        self.frozen_sources[index] = inventory(source)
        write(self.out / f"source-{index:02d}-before.json", self.frozen_sources[index])
        self.command(f"verify-{index:02d}", ["dataset", "verify", "--out", source], LIMITS["verify_seconds"])
        self.command(f"status-{index:02d}", ["dataset", "status", "--out", source], LIMITS["status_seconds"])
        self.command(f"export-{index:02d}", ["dataset", "export", "--out", source,
                     "--destination", destination], LIMITS["export_seconds"])
        audit = inspect(source, destination, index)
        write(self.out / f"audit-{index:02d}.json", audit, exclusive=True)
        row.update(status="checked", verified_decisions=audit["verified_decisions"],
                   transition_witness=audit["transition_witness"],
                   audit_path=f"audit-{index:02d}.json", audit_sha256=sha(self.out / f"audit-{index:02d}.json"))
        require(inventory(source) == self.frozen_sources[index], "source changed during verification/export")
        self.checkpoint()
        return audit

    def retry(self, index):
        source, destination = self.out / f"candidate-{index:02d}", self.out / f"export-{index:02d}"
        before = inventory(destination)
        self.command(f"retry-{index:02d}", ["dataset", "export", "--out", source,
                     "--destination", destination], LIMITS["export_seconds"])
        require(inventory(destination) == before, "immutable retry changed export inventory")
        require((self.out / "logs" / f"retry-{index:02d}.stdout").read_bytes() ==
                (self.out / "logs" / f"export-{index:02d}.stdout").read_bytes(), "immutable retry receipt changed")
        require(inventory(source) == self.frozen_sources[index], "retry changed source")

    def copied_mutation(self, index, name, witness):
        root = self.out / "controls" / name / "source"
        shutil.copytree(self.out / f"candidate-{index:02d}", root)
        cell = cell_directory(root, index)
        before = inventory(root)
        manifest = load(cell / "manifest.json")
        recipe = {"control": name, "cell_index": index, "source_inventory_before": before}
        if name == "map-substitution":
            relative = f"sat-pattern-{witness['pattern']}.gate-map.json"
            data = load(cell / relative)
            recipe["old_b"], data["b"] = data["b"], "0"
            write(cell / relative, data)
        else:
            relative = "sat-native-events.jsonl"
            events = read_events(cell)
            if name == "forged-reason":
                choices = [e for e in events if e["sequence"] <= witness["decision_after"] and
                           e["native"]["kind"] == "assignment" and not e["native"]["initial"] and
                           e["native"]["reason"]["kind"] in ("clause", "xor")]
                require(choices, "qualified transition lacks a propagation to corrupt")
                event = choices[0]
                recipe["event_sequence"], recipe["original_reason"] = event["sequence"], event["native"]["reason"]
                event["native"]["reason"] = {"kind": "clause", "clause_id": 4294967295}
            elif name == "unpaired-decision":
                recipe["cut_sequence"] = witness["decision_after"]
                events = events[:witness["decision_after"]]
            elif name == "dynamic-clause":
                cut = witness["assignment_after"]
                events = events[:cut]
                events.append({"schema": "ml-dataset.sat-event/v1", "pattern": witness["pattern"],
                               "sequence": cut + 1, "native": {"kind": "clause", "clause_id": 4294967295,
                               "literals": [1], "existing": False, "detached": False}})
                recipe["cut_sequence"] = cut
            else:
                raise ValueError("unknown corruption control")
            with (cell / relative).open("wb") as handle:
                for event in events:
                    handle.write(canonical(event) + b"\n")
        matched = [item for item in manifest["artifacts"] if item["path"] == relative]
        require(len(matched) == 1, "mutation artifact missing from manifest")
        recipe["original_artifact"] = dict(matched[0])
        matched[0].update(sha256=sha(cell / relative), bytes=(cell / relative).stat().st_size)
        recipe["mutated_artifact"] = dict(matched[0])
        write(cell / "manifest.json", manifest)
        recipe["source_inventory_after"] = inventory(root)
        write(root.parent / "mutation.json", recipe, exclusive=True)
        return root, recipe

    def controls(self, index, audit):
        witness = audit["transition_witness"]
        for name in ("map-substitution", "forged-reason", "unpaired-decision", "dynamic-clause"):
            root, recipe = self.copied_mutation(index, name, witness)
            destination = root.parent / "export"
            command = self.command("control-" + name, ["dataset", "export", "--out", root,
                                   "--destination", destination], LIMITS["export_seconds"], expected_failure=True)
            text = (self.out / command["stderr"]).read_text(errors="replace")
            control = {"name": name, "command": command["name"], "exit_code": command["exit_code"],
                       "mutation_sha256": sha(root.parent / "mutation.json"), "passed": False}
            if name in ("map-substitution", "forged-reason"):
                semantic = "gate-map source substitution" in text if name == "map-substitution" else "SAT pattern " in text
                control.update(passed=bool(command["exit_code"] and semantic),
                               stage="semantic_replay_rejection" if command["exit_code"] and semantic else
                               "earlier_or_unclassified_rejection" if command["exit_code"] else "unexpected_acceptance")
            elif command["exit_code"]:
                control.update(stage="earlier_or_unclassified_rejection", semantic_cli_gate="missing")
            else:
                changed = inspect(root, destination, index)
                write(root.parent / "audit.json", changed, exclusive=True)
                cut = recipe["cut_sequence"]
                baseline = {(d["pattern"], d["decision_sequence"]): d["feature_id"] for d in audit["decision_index"]
                            if d["assignment_sequence"] <= cut}
                actual = {(d["pattern"], d["decision_sequence"]): d["feature_id"] for d in changed["decision_index"]}
                require(actual == baseline, "corruption changed admitted prefix identities or admitted a forbidden suffix")
                reason = "truncated" if name == "unpaired-decision" else "unsupported"
                matching = [u for u in changed["unknown_suffixes"] if u["reason"]["kind"] == reason and
                            (name != "dynamic-clause" or u["reason"].get("transition") == "dynamic_clause")]
                control.update(passed=bool(matching), stage="semantic_prefix_censoring" if matching else "missing_expected_censor",
                               preserved_decision_occurrences=len(actual), unknown_suffix_records=matching)
            self.receipt["controls"].append(control)
            self.checkpoint()
        require(all(c["passed"] for c in self.receipt["controls"]),
                "semantic CLI controls incomplete; earlier custody rejections are not substituted")

    def run(self):
        protocol, binding = admit_protocol(self.protocol_path, self.original_binary)
        self.receipt["protocol"] = binding
        self.receipt["fixture_bindings"] = protocol["fixture"]
        shutil.copyfile(self.original_binary, self.binary)
        self.binary.chmod(0o700)
        require(sha(self.binary) == protocol["fixture"]["binary_sha256"], "copied binary hash mismatch")
        shutil.copyfile(self.protocol_path, self.out / "protocol.json")
        write(self.out / "host.json", {"platform": platform.platform(), "uname": list(platform.uname()),
              "cpu_count": os.cpu_count(), "python": sys.version, "worker_threads": 1,
              "driver_rayon_environment": os.environ.get("RAYON_NUM_THREADS"),
              "proc_cpuinfo": Path("/proc/cpuinfo").read_text() if Path("/proc/cpuinfo").exists() else None,
              "timing_isolation_attested": False, "clock_valid_for_comparison": False})
        self.receipt["status"] = "running"
        self.checkpoint()
        self.command("plan", ["dataset", "plan", "--out", self.out / "plan.json", "--n", "7", "--m", "2,3",
                     "--families", "koblitz,binary", "--solvers", "sat", "--curve-seeds", "1", "--generators", "1",
                     "--keys", "2", "--seed", "20261030", "--factor-base-size", "8", "--degree-bound", "3",
                     "--work-limit", "10000", "--protocol-sha256", binding["file_sha256"]], 30)
        self.command("preflight", ["dataset", "preflight", "--plan", self.out / "plan.json",
                     "--protocol", self.protocol_path], 30)
        preflight = load(self.out / "logs/preflight.stdout")
        require(preflight["source_commit"] == protocol["fixture"]["source_commit"] and
                preflight["binary_sha256"] == protocol["fixture"]["binary_sha256"] and
                preflight["protocol_file_sha256"] == binding["file_sha256"] and
                preflight["planned_cells"] == 16 and preflight["generated_cells"] == 0 and
                preflight["source_dirty"] is False, "native preflight identity mismatch")
        self.receipt["native_preflight"] = preflight
        for first in range(0, 16, 2):
            curve, twist = self.candidate(first), self.candidate(first + 1)
            rich = curve if curve["transition_witness"] else twist if twist["transition_witness"] else None
            if curve["verified_decisions"] and twist["verified_decisions"] and rich:
                self.receipt["selected_pair"] = [first, first + 1]
                self.receipt["transition_witness"] = dict(rich["transition_witness"], cell_index=rich["cell_index"])
                self.receipt["unrun_reason"] = "not_run_after_first_qualified_pair"
                self.checkpoint()
                self.retry(first)
                self.retry(first + 1)
                self.controls(rich["cell_index"], rich)
                self.receipt["status"] = "native_fixture_passed_unified_export_pending"
                return
        self.receipt["status"] = "missing_fixture"
        self.receipt["unrun_reason"] = "table_exhausted"

    def finish(self):
        self.stop_child()
        inventories = {}
        for index, expected in self.frozen_sources.items():
            actual = inventory(self.out / f"candidate-{index:02d}")
            write(self.out / f"source-{index:02d}-after.json", actual)
            inventories[str(index)] = actual == expected
        self.receipt["source_inventories_unchanged"] = inventories
        if not all(inventories.values()):
            self.receipt["status"] = "failed_source_changed"
        self.receipt["finished_utc"] = utc()
        self.receipt["driver_elapsed_seconds_diagnostic"] = time.monotonic() - self.started
        self.receipt["attempt_bytes_before_final_receipts"] = directory_bytes(self.out)
        flow = {"schema": "software-validation-flow/v1", "planned_cells": 16,
                "attempted_cells": len(self.receipt["candidates"]),
                "checked_cells": sum(c["status"] == "checked" for c in self.receipt["candidates"]),
                "incomplete_cells": sum(c["status"] != "checked" for c in self.receipt["candidates"]),
                "unrun_cells": self.receipt["unrun_cells"],
                "unrun_reason": self.receipt.get("unrun_reason", "stopped_on_error_or_resource_cap"),
                "resumed_cells": 0, "implicit_generation_retries": 0,
                "command_count": len(self.receipt["commands"]),
                "command_failures": sum(c.get("exit_code") != 0 for c in self.receipt["commands"]),
                "official_experiment_export_validation": "missing; this is not experiment.export/v1"}
        self.receipt["flow"] = flow
        write(self.out / "flow.json", flow)
        with (self.out / "software-execution-rows.jsonl").open("wb") as handle:
            for row in self.receipt["candidates"]:
                handle.write(canonical({"schema": "software-validation-row/v1", "arm": "native-sat",
                                        "instance": str(row["cell_index"]), "seed": 20261030,
                                        "replicate": 0, "warmup": False, "attempts": 1, "resumes": 0,
                                        "clock_valid_for_comparison": False, "candidate": row}) + b"\n")
        self.checkpoint()
        files = inventory(self.out)
        write(self.out / "inventory.json", {"schema": "software-fixture-inventory/v1",
                                           "files": files, "excludes": ["inventory.json (self)"]})
        archive = self.out.with_name(self.out.name + ".tar")
        require(not archive.exists(), "archive collision; original output preserved")
        remaining = self.remaining()
        archive_receipt = {"schema": "software-fixture-local-archive/v1", "attempt": str(self.out),
                           "archive": str(archive), "remote_upload_performed": False,
                           "parent_s3_archive_and_fresh_readback_required": True}
        try:
            require(remaining > 0, "no global wall budget remains for archive; parent must archive raw tree")
            result = subprocess.run(["tar", "--sort=name", "--mtime=@0", "--owner=0", "--group=0",
                                     "--numeric-owner", "--format=gnu", "-cf", str(archive), "-C",
                                     str(self.out.parent), self.out.name], capture_output=True,
                                    timeout=min(120, remaining), check=False)
            archive_receipt.update(exit_code=result.returncode, stderr=result.stderr.decode(errors="replace"))
            require(result.returncode == 0, "archive failed; partial tar and raw files retained")
            archive_receipt.update(status="complete", sha256=sha(archive), bytes=archive.stat().st_size)
        except BaseException as error:
            archive_receipt.update(status="failed_or_interrupted", error=repr(error),
                                   partial_archive_present=archive.exists())
        write(self.out.with_name(self.out.name + ".archive.json"), archive_receipt, exclusive=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True)
    parser.add_argument("--out", required=True)
    parser.add_argument("--protocol", required=True)
    args = parser.parse_args()
    output = Path(args.out).absolute()
    require(not output.exists(), "attempt directory already exists; never overwrite or resume it")
    output.mkdir(mode=0o700)
    (output / "logs").mkdir()
    (output / "controls").mkdir()
    driver = Driver(output, Path(args.binary).resolve(), Path(args.protocol).resolve())
    driver.checkpoint()
    def interrupted(signum, _frame):
        raise Stop("signal " + str(signum))
    for number in (signal.SIGTERM, signal.SIGINT, signal.SIGALRM):
        signal.signal(number, interrupted)
    signal.alarm(LIMITS["driver_seconds"])
    try:
        driver.run()
    except BaseException as error:
        driver.receipt.update(status="failed_or_blocked", error=repr(error), traceback=traceback.format_exc())
        if driver.receipt["selected_pair"] is None:
            driver.receipt["unrun_reason"] = "not_run_due_to_error_or_resource_cap"
    finally:
        try:
            driver.finish()
        except BaseException as error:
            driver.receipt.update(retention_error=repr(error))
            driver.checkpoint()
        signal.alarm(0)
    print(canonical(driver.receipt).decode())
    return 0 if driver.receipt["status"] == "native_fixture_passed_unified_export_pending" and not driver.receipt.get("retention_error") else 2


if __name__ == "__main__":
    raise SystemExit(main())
