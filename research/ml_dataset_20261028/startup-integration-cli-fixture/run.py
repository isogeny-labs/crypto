#!/usr/bin/env python3
"""Offline, bounded release-CLI correctness fixture; never a corpus producer."""

import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import signal
import stat
import subprocess
import sys
import time


STRATEGY = "prime-affine-s3-chain-f4-visitor/v1"
INPUT_SCHEMA = "ml-dataset.prime-f4-chain-input/v1"
LIMITS = {"seconds_per_case": 20, "memory_mib": 512, "trace_mib": 16}
COMMAND_TIMEOUT = 45
TOTAL_TIMEOUT = 240


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def utc_now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def terminate_fixture(signum, _frame):
    # Reach invoke()'s child cleanup and main()'s failure receipt on ordinary
    # supervisor cancellation. A second SIGTERM must not interrupt cleanup.
    signal.signal(signum, signal.SIG_IGN)
    raise KeyboardInterrupt("received SIGTERM; stopping software fixture")


def file_hash(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_json(path):
    with path.open(encoding="utf-8") as stream:
        return json.load(stream)


def write_exclusive(path, value):
    with path.open("x", encoding="utf-8") as stream:
        json.dump(value, stream, sort_keys=True, indent=2, ensure_ascii=False)
        stream.write("\n")
        stream.flush()
        os.fsync(stream.fileno())


def checked_file(root, relative):
    path = Path(relative)
    require(not path.is_absolute() and ".." not in path.parts, "unsafe artifact path")
    current = root
    for part in path.parts:
        current = current / part
        require(not current.is_symlink(), "symlink in retained artifacts")
    require(stat.S_ISREG(current.stat().st_mode), "artifact is not an ordinary file")
    return current


def validate_fixture(directory):
    manifest = read_json(directory / "fixture-manifest.json")
    require(manifest["schema"] == "ml-dataset.cli-correctness-fixture/v1", "unknown fixture schema")
    expected = {"README.md", "protocol.json", "run.py"}
    require(set(manifest["files"]) == expected, "unexpected fixture file set")
    for name, record in manifest["files"].items():
        path = checked_file(directory, name)
        require(file_hash(path) == record["sha256"], "fixture digest differs: " + name)
        require(path.stat().st_size == record["bytes"], "fixture size differs: " + name)
    protocol = read_json(directory / "protocol.json")
    require(protocol["schema"] == "experiment.protocol/v1", "unknown protocol schema")
    freeze = protocol.pop("freeze")
    canonical = json.dumps(protocol, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
    require(freeze["status"] == "frozen" and freeze["filed_before_affected_data"] is True,
            "protocol is not frozen before execution")
    require(hashlib.sha256(canonical).hexdigest() == freeze["protocol_sha256"],
            "protocol body differs from freeze digest")
    require(protocol["analysis_plan"]["script_sha256"] == file_hash(directory / "run.py"),
            "protocol does not bind this runner")
    return manifest


class Runner:
    def __init__(self, args, root):
        self.args = args
        self.root = root
        self.deadline = time.monotonic() + TOTAL_TIMEOUT
        self.commands = []
        self.cells = []
        self.tamper = None
        # No inherited AWS/IRSA, budget, store, proxy or Iso Lab routing. No
        # command below supplies --publish, --remote or any --budget-* option.
        self.env = {key: os.environ[key] for key in ("PATH", "HOME", "USER", "LOGNAME")
                    if key in os.environ}
        self.env.update({"RAYON_NUM_THREADS": "1", "LANG": "C.UTF-8", "LC_ALL": "C.UTF-8",
                         "TMPDIR": str(root / "tmp")})
        (root / "tmp").mkdir()
        (root / "commands").mkdir()

    def invoke(self, label, arguments, expected_success=True):
        require(file_hash(self.args.binary) == self.args.binary_sha256, "binary changed before command")
        remaining = self.deadline - time.monotonic()
        require(remaining > 0, "overall fixture deadline exhausted")
        timeout = min(COMMAND_TIMEOUT, remaining)
        command = [str(self.args.binary), "dataset", *arguments]
        require(not any(arg == "--publish" or arg == "--remote" or arg.startswith("--budget-")
                        for arg in arguments), "network/publication option forbidden")
        prefix = self.root / "commands" / label
        record = {"label": label, "argv": command, "started_utc": utc_now(),
                  "timeout_seconds": timeout, "exit_status": None, "timed_out": False,
                  "clock_valid_for_comparison": False}
        self.commands.append(record)
        write_exclusive(prefix.with_suffix(".started.json"), record)
        process = None
        try:
            with prefix.with_suffix(".stdout").open("xb") as stdout, prefix.with_suffix(".stderr").open("xb") as stderr:
                cancellation = {signal.SIGINT, signal.SIGTERM}
                previous_mask = signal.pthread_sigmask(signal.SIG_BLOCK, cancellation)

                def restore_child_signal_mask():
                    # This runner is single-threaded. Do not leave the native
                    # CLI with the parent's temporary launch mask after exec.
                    signal.pthread_sigmask(signal.SIG_SETMASK, previous_mask)

                try:
                    # Signals cannot interrupt the handoff between a running
                    # child and the Python handle protected by this finally.
                    process = subprocess.Popen(command, cwd=self.root, env=self.env, stdin=subprocess.DEVNULL,
                                               stdout=stdout, stderr=stderr, start_new_session=True,
                                               preexec_fn=restore_child_signal_mask)
                    record["pid"] = process.pid
                    signal.pthread_sigmask(signal.SIG_SETMASK, previous_mask)
                    try:
                        record["exit_status"] = process.wait(timeout=timeout)
                    except subprocess.TimeoutExpired:
                        record["timed_out"] = True
                        raise RuntimeError("command deadline exhausted: " + label) from None
                finally:
                    # Also defer a first cancellation arriving during timeout
                    # cleanup until the child is terminated and reaped.
                    signal.pthread_sigmask(signal.SIG_BLOCK, cancellation)
                    try:
                        if process is not None:
                            if process.poll() is None:
                                try:
                                    os.killpg(process.pid, signal.SIGTERM)
                                except ProcessLookupError:
                                    pass
                                try:
                                    process.wait(timeout=5)
                                except subprocess.TimeoutExpired:
                                    try:
                                        os.killpg(process.pid, signal.SIGKILL)
                                    except ProcessLookupError:
                                        pass
                                    process.wait()
                            record["exit_status"] = process.returncode
                        stdout.flush()
                        stderr.flush()
                        os.fsync(stdout.fileno())
                        os.fsync(stderr.fileno())
                    finally:
                        signal.pthread_sigmask(signal.SIG_SETMASK, previous_mask)
        finally:
            record["finished_utc"] = utc_now()
            record["binary_sha256_after"] = file_hash(self.args.binary)
            for stream in ("stdout", "stderr"):
                path = prefix.with_suffix("." + stream)
                if path.exists():
                    record[stream] = {"path": str(path.relative_to(self.root)),
                                      "sha256": file_hash(path), "bytes": path.stat().st_size}
            write_exclusive(prefix.with_suffix(".receipt.json"), record)
        require(record["binary_sha256_after"] == self.args.binary_sha256, "binary changed during command")
        require((record["exit_status"] == 0) == expected_success,
                "unexpected command exit status: " + label)
        return prefix

    def json_command(self, label, arguments):
        prefix = self.invoke(label, arguments)
        return read_json(prefix.with_suffix(".stdout"))

    def check_status(self, status, replay):
        for key, expected in {"planned_cells": 64, "assigned_shard_cells": 1,
                              "completed_shard_receipts": 1, "artifact_hashes_verified": True,
                              "known_answer_replay_performed": replay, "shard_attempts_complete": True,
                              "campaign_attempts_complete": False, "full_scope_complete": False,
                              "published_completion_receipts": 0, "published_cells": 0,
                              "discoverable_catalog_cells": 0, "independently_solved_ecdlp_cells": 0}.items():
            require(status.get(key) == expected, "unexpected status field: " + key)
        require(status["incomplete_attempts"] == [], "unfinished software cell")

    def execute(self):
        protocol = self.root / "fixture" / "protocol.json"
        protocol_sha = file_hash(protocol)
        plan = self.root / "plan.json"
        planned = self.json_command("01-plan", ["plan", "--out", str(plan), "--n", "7", "--m", "3",
            "--families", "prime", "--solvers", "f4", "--curve-seeds", "8", "--generators", "2",
            "--keys", "2", "--seed", "42", "--factor-base-size", "2", "--degree-bound", "4",
            "--work-limit", "512", "--protocol-sha256", protocol_sha])
        require(planned["planned_cells"] == 64 and planned["generated_cells"] == 0, "plan counts differ")
        require(planned["sha256"] == file_hash(plan), "plan file digest differs")
        preflight = self.json_command("02-preflight", ["preflight", "--plan", str(plan), "--protocol", str(protocol)])
        for key, expected in {"source_commit": self.args.source_commit, "source_dirty": False,
                              "binary_sha256": self.args.binary_sha256, "plan_sha256": planned["plan_sha256"],
                              "protocol_file_sha256": protocol_sha, "lab_receipt_uri": None,
                              "generated_cells": 0, "cloud_permissions_checked": False}.items():
            require(preflight.get(key) == expected, "preflight binding differs: " + key)
        for ordinal, index in enumerate((62, 63)):
            out = self.root / ("shard-" + str(index))
            arguments = ["generate", "--plan", str(plan), "--protocol", str(protocol), "--out", str(out),
                         "--max-cases", "1", "--shard-index", str(index), "--shard-count", "64"]
            for name, value in LIMITS.items():
                arguments.extend(["--" + name.replace("_", "-"), str(value)])
            self.invoke(str(3 + ordinal * 3).zfill(2) + "-generate-" + str(index), arguments)
            binding = read_json(out / "run.json")
            for key, expected in {"source_commit": self.args.source_commit, "binary_sha256": self.args.binary_sha256,
                                  "plan_sha256": planned["plan_sha256"], "shard_index": index,
                                  "shard_count": 64, **LIMITS}.items():
                require(binding.get(key) == expected, "generated run binding differs: " + key)
            cell = out / "cells" / f"{index:012d}"
            config = read_json(cell / "config.json")
            for key, expected in {"family": "prime", "n": 7, "m": 3, "solver": "f4", "seed": 42,
                                  "curve_seed": 7, "generator_index": 1, "key_index": 1,
                                  "twist": bool(ordinal)}.items():
                require(config.get(key) == expected, "selected cell differs: " + key)
            manifest = read_json(cell / "manifest.json")
            chain = read_json(cell / "prime-f4-input.json")
            require(chain["schema"] == INPUT_SCHEMA and chain["strategy"] == STRATEGY, "missing exact chain encoding")
            require(chain["m"] == 3 and chain["n_vars"] == 4 and chain["domains"][3] is None,
                    "unexpected chain variable/domain map")
            require(manifest["outcome"]["strategy"] == STRATEGY, "wrong solver strategy")
            require(manifest["outcome"]["exhaustive_no_decomposition"] is False, "invalid PDP impossibility label")
            verified = self.json_command(str(4 + ordinal * 3).zfill(2) + "-verify-" + str(index),
                                         ["verify", "--out", str(out)])
            self.check_status(verified, True)
            status = self.json_command(str(5 + ordinal * 3).zfill(2) + "-status-" + str(index),
                                       ["status", "--out", str(out)])
            self.check_status(status, False)
            self.cells.append({"cell_index": index, "twist": bool(ordinal), "manifest_sha256": file_hash(cell / "manifest.json"),
                               "outcome": manifest["outcome"], "included_in_research_corpus": False})

        source = self.root / "shard-62"
        damaged = self.root / "tampered-copy"
        # Only copy ordinary files. The pristine run is retained unchanged.
        for path in source.rglob("*"):
            require(not path.is_symlink(), "cannot copy symlink into tamper fixture")
            require(path.is_file() or path.is_dir(), "cannot copy special file into tamper fixture")
        shutil.copytree(source, damaged)
        cell = damaged / "cells" / "000000000062"
        input_path = cell / "prime-f4-input.json"
        original_input_sha = file_hash(input_path)
        chain = read_json(input_path)
        require(chain["domains"][0] and chain["domains"][3] is None, "invalid tamper precondition")
        chain["domains"][3] = list(chain["domains"][0])
        coefficient = chain["polynomials"][0][0]["coefficient"]
        chain["polynomials"][0][0]["coefficient"] = str((int(coefficient) + 1) % int(chain["modulus"]))
        input_path.write_text(json.dumps(chain, indent=2) + "\n", encoding="utf-8")
        manifest_path = cell / "manifest.json"
        manifest = read_json(manifest_path)
        matched = 0
        for artifact in manifest["artifacts"]:
            path = checked_file(cell, artifact["path"])
            artifact["sha256"], artifact["bytes"] = file_hash(path), path.stat().st_size
            matched += artifact["path"] == "prime-f4-input.json"
        require(matched == 1, "input must have exactly one manifest entry")
        manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
        hashed = self.json_command("09-tampered-status", ["status", "--out", str(damaged)])
        self.check_status(hashed, False)
        rejected = self.invoke("10-tampered-verify", ["verify", "--out", str(damaged)], expected_success=False)
        error = rejected.with_suffix(".stderr").read_text(encoding="utf-8")
        require("prime F4 chain input differs at" in error, "rejection did not come from semantic reconstruction")
        self.tamper = {"source_shard": 62, "artifact": "prime-f4-input.json", "original_sha256": original_input_sha,
                       "tampered_sha256": file_hash(input_path), "manifest_sha256": file_hash(manifest_path),
                       "changes": ["auxiliary domain restricted to leaf abscissae", "one S3 coefficient changed"],
                       "hash_only_status_passed": True, "semantic_verify_rejected": True,
                       "included_in_research_corpus": False}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--binary-sha256", required=True)
    parser.add_argument("--source-commit", required=True)
    parser.add_argument("--fixture-commit", required=True)
    parser.add_argument("--scratch", required=True, type=Path, help="fresh nonexistent output directory; parent must exist")
    args = parser.parse_args()
    require(sys.platform.startswith("linux"), "fixture requires the native Linux supervisor contract")
    for name, width in (("binary_sha256", 64), ("source_commit", 40), ("fixture_commit", 40)):
        require(re.fullmatch("[0-9a-f]{" + str(width) + "}", getattr(args, name)) is not None,
                "expected exact lowercase hexadecimal " + name)
    args.binary = args.binary.resolve(strict=True)
    require(args.binary.is_file() and os.access(args.binary, os.X_OK), "binary must be executable")
    require(file_hash(args.binary) == args.binary_sha256, "binary does not match expected digest")
    fixture = Path(__file__).resolve().parent
    validate_fixture(fixture)
    root = args.scratch.parent.resolve(strict=True) / args.scratch.name
    require(not root.exists() and not root.is_symlink(), "scratch exists; automatic retry/resume is forbidden")
    root.mkdir(mode=0o700)
    copied = root / "fixture"
    copied.mkdir()
    for name in ("README.md", "protocol.json", "run.py", "fixture-manifest.json"):
        destination = copied / name
        with destination.open("xb") as stream:
            stream.write((fixture / name).read_bytes())
            stream.flush()
            os.fsync(stream.fileno())
        destination.chmod(0o444)
    validate_fixture(copied)
    inputs = {"schema": "ml-dataset.cli-correctness-inputs/v1", "classification": "software_correctness_fixture",
              "source_commit": args.source_commit, "binary_path": str(args.binary), "binary_sha256": args.binary_sha256,
              "fixture_commit": args.fixture_commit, "fixture_commit_is_supplied_not_git_verified": True,
              "fixture_manifest_sha256": file_hash(copied / "fixture-manifest.json"),
              "protocol_file_sha256": file_hash(copied / "protocol.json"), "started_utc": utc_now(),
              "limits": LIMITS, "overall_deadline_seconds": TOTAL_TIMEOUT, "per_command_timeout_seconds": COMMAND_TIMEOUT,
              "corpus_rows_added": 0, "publication_requested": False, "cloud_permissions_checked": False}
    write_exclusive(root / "execution-inputs.json", inputs)
    runner = Runner(args, root)
    failure = None
    previous_sigterm = signal.signal(signal.SIGTERM, terminate_fixture)
    try:
        runner.execute()
    except BaseException as error:
        failure = {"type": type(error).__name__, "message": str(error)}
    finally:
        inventory = []
        for path in sorted(root.rglob("*")):
            if path.is_file() and not path.is_symlink():
                inventory.append({"path": str(path.relative_to(root)), "sha256": file_hash(path), "bytes": path.stat().st_size})
        final_binary = file_hash(args.binary)
        if final_binary != args.binary_sha256 and failure is None:
            failure = {"type": "RuntimeError", "message": "binary changed before final receipt"}
        write_exclusive(root / "receipt.json", {**inputs, "schema": "ml-dataset.cli-correctness-receipt/v1",
            "finished_utc": utc_now(), "verdict": "passed_software_checks" if failure is None else "failed_software_checks",
            "failure": failure, "commands": runner.commands, "verified_local_cells": runner.cells,
            "semantic_tamper": runner.tamper, "binary_sha256_after": final_binary, "inventory": inventory,
            "clock_valid_for_comparison": False, "verified_cloud_dataset_cells": 0,
            "startup_gate_exercised": False, "runtime_source_must_match_preflight": True})
        signal.signal(signal.SIGTERM, previous_sigterm)
    print(json.dumps({"receipt": str(root / "receipt.json"), "passed": failure is None, "failure": failure}))
    return 0 if failure is None else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception as error:
        print(type(error).__name__ + ": " + str(error), file=sys.stderr)
        sys.exit(1)
