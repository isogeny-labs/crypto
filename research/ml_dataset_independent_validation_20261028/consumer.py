#!/usr/bin/env python3
"""Independent dataset binding audit and allowlisted static routing export.

This analysis tool does not implement cryptography, run experiments, or certify
mathematical labels. Native replay, if requested, is separately identified.
"""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path, PurePosixPath
import re
import subprocess
import tempfile
from urllib.parse import urlsplit

PLAN_FIELDS = (
    "schema field_sizes arities families solvers curve_seeds generators keys seed "
    "factor_base_size degree_bound work_limit twists protocol_sha256 split_rule"
).split()
REQUIRED = {"config.json", "run.json", "plan.json", "protocol.json", "attempt.json",
            "worker-process.json", "stdout.log", "stderr.log"}
PROVENANCE = ("seed", "curve_seed", "generator_index", "key_index")
SUCCESS_ROLES = {"restricted-independent-scalar-label", "independently-verified-pdp"}
SHA = re.compile(r"[0-9a-f]{64}\Z")
MAX_JSON = 64 * 1024 * 1024


def require(test, message):
    if not test:
        raise ValueError(message)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def compact(value, sort=False):
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"),
                      sort_keys=sort, allow_nan=False).encode()


def unique_pairs(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, f"duplicate JSON key: {key}")
        result[key] = value
    return result


def load(path):
    require(path.stat().st_size <= MAX_JSON, f"JSON size limit: {path}")
    value = json.loads(path.read_bytes(), object_pairs_hook=unique_pairs,
                       parse_constant=lambda v: (_ for _ in ()).throw(ValueError(v)))
    require(isinstance(value, dict), f"expected JSON object: {path}")
    return value


def contained(root, relative):
    require(isinstance(relative, str) and relative, "empty artifact path")
    parts = PurePosixPath(relative)
    require(not parts.is_absolute() and all(p not in ("..", ".") for p in parts.parts)
            and str(parts) == relative and "\\" not in relative, "noncanonical artifact path")
    require(root.is_dir() and not root.is_symlink(), "root must be a real directory")
    path = root
    for part in parts.parts:
        path = path / part
        require(not path.is_symlink(), f"symlink artifact: {relative}")
    return path


def check_artifact(root, artifact):
    require(SHA.fullmatch(artifact["sha256"]) is not None, "invalid artifact digest")
    size = artifact["bytes"]
    require(type(size) is int and size >= 0, "invalid artifact size")
    path = contained(root, artifact["path"])
    require(path.is_file() and path.stat().st_size == size, f"artifact size: {path}")
    h = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            h.update(block)
    require(h.hexdigest() == artifact["sha256"], f"artifact hash: {path}")
    return path


def plan_info(root):
    plan = load(contained(root, "plan.json"))
    require(set(plan) == set(PLAN_FIELDS), "unknown/missing plan fields")
    require(plan["schema"] == "crypto.ml-dataset.plan/v1" and plan["twists"] is True,
            "unsupported plan schema/twist policy")
    require(plan["split_rule"] == "curve-twist-component-sha256-v1", "unknown split rule")
    axes = [plan[k] for k in ("families", "field_sizes", "arities", "solvers")]
    require(all(isinstance(a, list) and a and len(set(a)) == len(a) for a in axes), "invalid axes")
    require(all(type(n) is int and 3 <= n <= 61 for n in plan["field_sizes"]), "invalid field sizes")
    require(all(type(m) is int and 2 <= m <= 15 for m in plan["arities"]), "invalid arities")
    require(set(plan["families"]) <= {"koblitz", "binary", "prime"}, "unknown family")
    for key in ("curve_seeds", "generators", "keys", "factor_base_size", "degree_bound", "work_limit"):
        require(type(plan[key]) is int and plan[key] > 0, f"invalid {key}")
    require(type(plan["seed"]) is int and 0 <= plan["seed"] < 2**64, "invalid seed")
    protocol = contained(root, "protocol.json").read_bytes()
    require(digest(protocol) == plan["protocol_sha256"], "protocol hash mismatch")
    require(load(contained(root, "protocol.json"))["schema"] == "experiment.protocol/v1", "protocol schema")
    plan_sha = digest(compact({key: plan[key] for key in PLAN_FIELDS}))
    binding = load(contained(root, "run.json"))
    require(binding["schema"] == "crypto.ml-dataset.run/v1" and binding["plan_sha256"] == plan_sha,
            "run/plan binding mismatch")
    require(SHA.fullmatch(binding["binary_sha256"]) is not None, "invalid binary identity")
    require(re.fullmatch(r"[0-9a-f]{40}", binding["source_commit"]) is not None, "invalid source identity")
    require(type(binding["shard_count"]) is int and type(binding["shard_index"]) is int
            and 0 <= binding["shard_index"] < binding["shard_count"], "invalid shard")
    total = math.prod(map(len, axes)) * plan["curve_seeds"] * plan["generators"] * plan["keys"] * 2
    require(total < 2**64, "plan count overflow")
    return plan, plan_sha, binding, total


def config_at(plan, index):
    config = {k: plan[k] for k in ("seed", "factor_base_size", "degree_bound", "work_limit")}
    for key, values in (
        ("solver", plan["solvers"]), ("twist", [False, True]), ("m", plan["arities"]),
        ("key_index", range(plan["keys"])), ("generator_index", range(plan["generators"])),
        ("curve_seed", range(plan["curve_seeds"])), ("n", plan["field_sizes"]),
        ("family", plan["families"]),
    ):
        index, remainder = divmod(index, len(values))
        config[key] = values[remainder]
    require(index == 0, "cell index outside plan")
    return config


def phase_measurements(root, members):
    """Validate reported PMU label consistency, not PMU calibration or full cost."""
    starts, ends = {}, {}
    for artifact in members.values():
        role = artifact["role"]
        if role not in ("phase-start", "hardware-and-resource-telemetry"):
            continue
        record = load(root / artifact["path"])
        phase = record.get("phase")
        require(isinstance(phase, str) and re.fullmatch(r"[a-z][a-z0-9-]{0,63}", phase), "invalid telemetry phase")
        start = role == "phase-start"
        expected = "crypto.ml-dataset.phase-start/v1" if start else "crypto.ml-dataset.phase-metrics/v1"
        suffix = "-start" if start else ""
        require(record.get("schema") == expected and artifact["path"] == f"telemetry/{phase}{suffix}.json",
                "telemetry schema/path mismatch")
        target = starts if start else ends
        require(phase not in target, "duplicate telemetry phase")
        target[phase] = (record, artifact["sha256"])
    require(ends.keys() <= starts.keys(), "phase metrics lack durable start boundary")
    result = []
    for phase, (start, start_sha) in sorted(starts.items()):
        entry = {"phase": phase, "start_sha256": start_sha, "start": start,
                 "metrics_sha256": None, "producer_reported": None,
                 "measurement_state": "interrupted_or_unfinished", "independently_verified": False}
        if phase not in ends:
            result.append(entry)
            continue
        metrics, sha = ends[phase]
        hardware = metrics.get("hardware")
        require(isinstance(hardware, dict), "missing hardware record")
        for key in ("instructions", "instructions_scaled", "instructions_time_enabled_ns", "instructions_time_running_ns",
                    "cycles", "cycles_scaled", "cycles_time_enabled_ns", "cycles_time_running_ns"):
            value = hardware.get(key)
            require(value is None or type(value) is int and 0 <= value < 2**64, "invalid hardware counter")
        any_counter, expected_mux = False, False
        for event in ("instructions", "cycles"):
            raw, enabled, running = (hardware.get(event), hardware.get(event + "_time_enabled_ns"),
                                     hardware.get(event + "_time_running_ns"))
            scaled = None
            if raw is not None and enabled is not None and running is not None:
                any_counter = True
                require(running <= enabled, "invalid PMU running/enabled time")
                expected_mux |= running < enabled
                if running == enabled:  # Also matches the producer's (0, 0) boundary.
                    scaled = raw
                elif running > 0:
                    scaled = min(raw * enabled // running, 2**64 - 1)
            require(hardware.get(event + "_scaled") == scaled, f"inconsistent {event} scaling")
        multiplexed = hardware.get("multiplexed")
        require(multiplexed is None or type(multiplexed) is bool, "invalid multiplexing flag")
        require(multiplexed == (expected_mux if any_counter else None), "inconsistent aggregate multiplexing flag")
        raw, running = hardware.get("instructions"), hardware.get("instructions_time_running_ns")
        usable = raw is not None and running is not None and running > 0
        status = metrics.get("instructions_status")
        expected = "unavailable" if not usable else "measured_unmultiplexed" if multiplexed is False else "measured_multiplexed_estimate"
        require(status == expected, "instruction status contradicts hardware")
        for key in ("retired_instructions_raw", "retired_instructions_exact_in_scope"):
            value = metrics.get(key)
            require(value is None or type(value) is int and 0 <= value < 2**64, "invalid instruction label type")
        require(metrics.get("retired_instructions_raw") == (raw if usable else None), "raw instruction label mismatch")
        exact = raw if usable and multiplexed is False else None
        require(metrics.get("retired_instructions_exact_in_scope") == exact, "exact instruction label mismatch")
        require(metrics.get("clock_valid_for_comparison") is False, "unsupported comparative clock claim")
        require(isinstance(metrics.get("counter_scope"), str) and metrics["counter_scope"]
                and isinstance(metrics.get("excludes"), list) and metrics["excludes"], "missing counter scope/exclusions")
        require(type(metrics.get("returned_ok")) is bool, "missing phase completion status")
        require((metrics.get("returned_error") is None) == metrics["returned_ok"], "phase error/status mismatch")
        entry.update(metrics_sha256=sha, producer_reported=metrics, measurement_state="reported_phase_readback")
        result.append(entry)
    return result


def audit_cell(root, plan, plan_sha, binding):
    manifest_path = contained(root, "manifest.json")
    manifest = load(manifest_path)
    require(manifest["schema"] == "crypto.ml-dataset.cell/v1", "cell schema")
    index = manifest["cell_index"]
    require(type(index) is int and index >= 0 and root.name == f"{index:012}", "cell directory/index")
    require(index % binding["shard_count"] == binding["shard_index"], "cell outside shard")
    config = config_at(plan, index)
    require(manifest["plan_sha256"] == plan_sha and manifest["config"] == config, "cell/plan config")
    members = {}
    for artifact in manifest["artifacts"]:
        require(artifact["path"] not in members, "duplicate artifact path")
        check_artifact(root, artifact)
        members[artifact["path"]] = artifact
    require(REQUIRED <= members.keys(), "missing mandatory provenance artifact")
    require(load(root / "config.json") == config, "config artifact mismatch")
    require(load(root / "run.json") == binding and load(root / "plan.json") == plan, "transplanted provenance")
    require(digest((root / "protocol.json").read_bytes()) == plan["protocol_sha256"], "cell protocol mismatch")
    outcome = manifest["outcome"]
    require(isinstance(outcome.get("status"), str), "missing outcome status")
    roles = {a["role"] for a in members.values()}
    public = load(root / "public.json") if "public.json" in members else None
    private = load(root / "witness.json") if "witness.json" in members else None
    base = load(root / "factor-base.json") if "factor-base.json" in members else None
    scalar = outcome.get("scalar_recovered_without_witness") is True
    pdp = outcome.get("pdp_witness_verified") is True
    success = scalar or pdp or bool(roles & SUCCESS_ROLES)
    if success:
        require(public is not None and private is not None and base is not None,
                "success certificate lacks public/witness/factor-base inputs")
        require(outcome.get("solver_completed") is True, "success with incomplete solver")
        require(outcome.get("exhaustive_no_decomposition") is not True, "contradictory SAT/UNSAT labels")
        require(outcome["status"] in ("verified", "solved"), "success with failure status")
    if scalar:
        require("restricted-independent-scalar-label" in roles and outcome.get("scalar_replay_verified") is True,
                "scalar label lacks certificate/replay")
    if pdp:
        require("independently-verified-pdp" in roles, "PDP label lacks certificate")
    if outcome.get("known_answer_verified") is True:
        require(public is not None and private is not None, "known answer lacks inputs")
    if outcome.get("factor_base_verified") is True:
        require(public is not None and base is not None, "factor base lacks inputs")
    if public is not None:
        require(public.get("schema") == "ml-ecdlp-instance/v1", "public instance schema")
        for key in ("family", "n", "m", "twist"):
            require(public.get(key) == config[key], f"public/config {key} mismatch")
        require(public.get("split_component") == f"field-size-n{config['n']}", "unbound split component")
        if private is not None:
            require(private.get("schema") == "ml-ecdlp-private-witness/v1", "private witness schema")
            require(private.get("sampling_provenance") == {k: config[k] for k in PROVENANCE},
                    "transplanted sampling provenance")
        if base is not None:
            require(base.get("requested_size") == config["factor_base_size"], "factor-base size provenance")
            require(base.get("schema") == "ml-ecdlp-factor-base/v1", "factor-base schema")
            points = base.get("points")
            require(isinstance(points, list), "missing factor-base points")
            if base.get("construction_status") == "verified":
                r = int(public["curve"]["subgroup_order"])
                require(len(points) >= config["factor_base_size"] or len(points) == r - 1,
                        "factor-base completion falsely asserted")
    for artifact in members.values():
        if artifact["role"] not in SUCCESS_ROLES:
            continue
        certificate = load(root / artifact["path"])
        require(certificate.get("target") == public["target"], "certificate target mismatch")
        if artifact["role"] == "independently-verified-pdp":
            require(len(certificate.get("summands", [])) == config["m"], "PDP arity mismatch")
            if "factor_base_definition" in certificate:
                require(config["solver"] == "pkm", "alternate factor base outside PKM")
    return {"id": digest(manifest_path.read_bytes()), "index": index, "config": config,
            "public": public, "factor_base": base, "outcome": outcome,
            "phase_measurements": phase_measurements(root, members),
            "artifact_roles": sorted(roles), "manifest_sha256": digest(manifest_path.read_bytes())}


def audit(root, expected_run_sha=None):
    root = root.absolute()
    plan, plan_sha, binding, total = plan_info(root)
    run_sha = digest((root / "run.json").read_bytes())
    if expected_run_sha is not None:
        require(run_sha == expected_run_sha, "trusted run digest mismatch")
    rows, incomplete = [], []
    cells = contained(root, "cells")
    if cells.exists():
        for cell in sorted(cells.iterdir()):
            require(not cell.is_symlink() and cell.is_dir() and re.fullmatch(r"[0-9]{12}", cell.name),
                    "invalid cell directory")
            if (cell / "manifest.json").exists():
                rows.append(audit_cell(cell, plan, plan_sha, binding))
            else:
                incomplete.append(cell.name)
    counts = {}
    for row in rows:
        status = row["outcome"]["status"]
        counts[status] = counts.get(status, 0) + 1
    return {"schema": "ml-dataset.consumer-audit/v1", "run_sha256": run_sha,
            "trusted_run_anchor_checked": expected_run_sha is not None,
            "plan_sha256": plan_sha, "planned_cells": total, "checked_cells": len(rows),
            "incomplete_attempts": incomplete, "outcomes": counts,
            "hashes_and_cross_artifact_bindings_verified": True,
            "mathematics_independently_verified": False, "producer_native_replay_passed": False,
            "remote_rechecked": False, "full_scope_complete": False}, rows


def static_features(row):
    """Allowlist only. Manifests, seeds, certificates and solver traces stay out."""
    public, base, config = row["public"], row["factor_base"], row["config"]
    if public is None or base is None:
        return None
    curve_keys = ("model", "a", "b", "base_a", "base_b", "twist_parameter",
                  "group_order", "subgroup_order", "cofactor")
    field_keys = ("characteristic", "degree", "modulus", "irreducible_low_terms", "basis")
    def decimal(value):
        require(isinstance(value, str) and re.fullmatch(r"0|[1-9][0-9]*", value), "invalid public decimal")
        return value
    def point(p):
        return None if p is None else {k: decimal(p[k]) for k in ("x", "y")}
    require(base["rule"] in ("small-x-cofactor-projection", "small-x-prime-primary-projection-with-cyclic-membership"),
            "unsupported factor-base rule")
    base_view = {"rule": base["rule"]}
    for key in ("negation_action", "frobenius_action"):
        if key in base:
            require(isinstance(base[key], list) and all(x is None or type(x) is int and 0 <= x < len(base["points"])
                                                       for x in base[key]), "invalid factor-base action")
            base_view[key] = base[key]
    if "frobenius_power" in base:
        require(type(base["frobenius_power"]) is int and base["frobenius_power"] > 0, "invalid Frobenius power")
        base_view["frobenius_power"] = base["frobenius_power"]
    base_view["points"] = [point(p) for p in base["points"]]
    if "orbits" in base:
        require(all(type(o["stabilizer_size"]) is int and o["stabilizer_size"] > 0 for o in base["orbits"]),
                "invalid orbit stabilizer")
        base_view["orbits"] = [{"points": [point(p) for p in o["points"]],
                                "stabilizer_size": o["stabilizer_size"]} for o in base["orbits"]]
    field = {k: public["field"][k] for k in field_keys if k in public["field"]}
    for key in ("characteristic", "modulus"):
        if key in field:
            decimal(field[key])
    require(type(field["degree"]) is int and 1 <= field["degree"] <= 61, "invalid field degree")
    require(field["basis"] in ("canonical-residue", "polynomial-lsb-first"), "unknown field basis")
    if "irreducible_low_terms" in field:
        require(isinstance(field["irreducible_low_terms"], list) and all(type(x) is int and 0 <= x < field["degree"]
                for x in field["irreducible_low_terms"]), "invalid polynomial terms")
    curve = {k: public["curve"][k] for k in curve_keys if k in public["curve"]}
    require(curve["model"] in ("short-weierstrass", "binary-ordinary"), "unknown curve model")
    for key, value in curve.items():
        if key != "model":
            decimal(value)
    return {"family": public["family"], "n": public["n"], "m": public["m"], "twist": public["twist"],
            "field": field, "curve": curve,
            "generator": point(public["generator"]), "target": point(public["target"]),
            "factor_base": base_view,
            "solver": config["solver"], "degree_bound": config["degree_bound"],
            "work_limit": config["work_limit"]}


def export_rows(rows, path):
    """Static cost/routing view. Preserve all outcomes; missing inputs get null features."""
    fd, temporary = tempfile.mkstemp(prefix=".consumer-export-", dir=path.absolute().parent)
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as out:
            for row in rows:
                component = f"field-size-n{row['config']['n']}"
                split_number = int(digest(("ml-ecdlp-static-split/v1:" + component).encode()), 16) % 100
                split = "train" if split_number < 80 else "validation" if split_number < 90 else "test"
                record = {"schema": "ml-dataset.static-routing-view/v1", "record_id": row["id"],
                          "split": split, "independent_unit": component, "features": static_features(row),
                          "labels": {"producer_reported": row["outcome"], "independently_verified": False,
                                     "phase_measurements": row["phase_measurements"],
                                     "whole_worker_instruction_count": None},
                          "feature_policy": "allowlisted public static inputs only; no trace/manifest/witness features"}
                out.write(compact(record).decode() + "\n")
            out.flush()
            os.fsync(out.fileno())
        os.link(temporary, path)
        directory = os.open(path.absolute().parent, os.O_RDONLY | os.O_DIRECTORY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)
    finally:
        os.unlink(temporary)


def fetch_cell(receipt, expected_sha, destination, maximum_bytes=20 * 1024**3):
    """Read S3 only, restoring one cell into a new isolated run directory."""
    require(digest(receipt.read_bytes()) == expected_sha, "completion receipt digest mismatch")
    record = load(receipt)
    require(record["schema"] == "ml-dataset.publication/v1", "completion schema")
    remote = urlsplit(record["remote"])
    require(remote.scheme == "s3" and remote.netloc == "setec-astronomy-artifacts"
            and not remote.query and not remote.fragment, "canonical S3 remote required")
    objects = record["objects"]
    require(len(objects) <= 100000, "object count limit")
    require(all(type(o["artifact"]["bytes"]) is int and o["artifact"]["bytes"] >= 0 for o in objects), "invalid object size")
    require(sum(o["artifact"]["bytes"] for o in objects) <= maximum_bytes, "download budget exceeded")
    require(not destination.exists(), "restore destination must not exist")
    destination.mkdir(parents=True)
    scratch = destination / "incomplete-download"
    scratch.mkdir()
    seen = set()
    for obj in objects:
        artifact = obj["artifact"]
        sha = artifact["sha256"]
        require(SHA.fullmatch(sha) is not None, "invalid object digest")
        require(artifact["path"] not in seen, "duplicate downloaded path")
        seen.add(artifact["path"])
        key = f"sha256/{sha[:2]}/{sha}"
        require(obj["object_key"] == key, "noncanonical content-addressed key")
        target = contained(scratch, artifact["path"])
        target.parent.mkdir(parents=True, exist_ok=True)
        prefix = remote.path.strip("/")
        address = ["--bucket", remote.netloc, "--key", f"{prefix}/{key}" if prefix else key]
        head = subprocess.run(["aws", "--region", "us-east-2", "s3api", "head-object", *address],
                              capture_output=True, text=True, timeout=60)
        require(head.returncode == 0, f"S3 HEAD failed: {head.stderr.strip()}")
        metadata = json.loads(head.stdout, object_pairs_hook=unique_pairs)
        require(metadata["ContentLength"] == artifact["bytes"], "remote size differs from declared budget")
        require(isinstance(metadata["ETag"], str) and metadata["ETag"], "missing S3 ETag")
        result = subprocess.run(["aws", "--region", "us-east-2", "s3api", "get-object", *address,
                                 "--if-match", metadata["ETag"],
                                 str(target)], capture_output=True, text=True, timeout=300)
        require(result.returncode == 0, f"S3 GET failed: {result.stderr.strip()}")
        check_artifact(scratch, artifact)
    require("manifest.json" in seen, "publication lacks manifest")
    manifest = load(scratch / "manifest.json")
    require(digest((scratch / "manifest.json").read_bytes()) == record["manifest_sha256"], "remote manifest digest")
    published_members = [o["artifact"] for o in objects if o["artifact"]["path"] != "manifest.json"]
    require(sorted(published_members, key=lambda a: a["path"]) == sorted(manifest["artifacts"], key=lambda a: a["path"]),
            "completion member set differs from manifest")
    for name in ("plan.json", "run.json", "protocol.json"):
        (destination / name).write_bytes((scratch / name).read_bytes())
    index = manifest["cell_index"]
    require(type(index) is int and index >= 0, "invalid restored index")
    (destination / "cells").mkdir()
    scratch.rename(destination / "cells" / f"{index:012}")
    report, _ = audit(destination)
    report["remote_rechecked"] = True
    report["trusted_completion_receipt_sha256"] = expected_sha
    return report


def replay_native(root, binary, report, rows):
    """Detect changes across replay; callers should use an isolated restored tree."""
    binary_sha = digest(binary.read_bytes())
    replay = subprocess.run([str(binary.absolute()), "dataset", "verify", "--out", str(root.absolute())],
                            capture_output=True, text=True, timeout=600)
    require(replay.returncode == 0, f"native replay failed: {replay.stderr.strip()}")
    require(digest(binary.read_bytes()) == binary_sha, "native verifier changed during replay")
    native = json.loads(replay.stdout, object_pairs_hook=unique_pairs)
    require(isinstance(native, dict) and native.get("schema") == "crypto.ml-dataset.status/v1"
            and native.get("known_answer_replay_performed") is True
            and native.get("artifact_hashes_verified") is True
            and type(native.get("completed_receipts")) is int
            and native["completed_receipts"] == len(rows), "native replay coverage mismatch")
    _, after = audit(root, report["run_sha256"])
    require([(r["index"], r["id"]) for r in rows] == [(r["index"], r["id"]) for r in after],
            "cell contents changed during replay")
    report["producer_native_replay_passed"] = True
    report["native_verifier_sha256"] = binary_sha
    report["native_replay_stdout_sha256"] = digest(replay.stdout.encode())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    for name in ("audit", "export"):
        sub = commands.add_parser(name)
        sub.add_argument("--run", required=True, type=Path)
        sub.add_argument("--expected-run-sha256")
        sub.add_argument("--native-verifier", type=Path)
        if name == "export":
            sub.add_argument("--out", required=True, type=Path)
            sub.add_argument("--allow-unreplayed-preview", action="store_true")
    fetch = commands.add_parser("fetch")
    fetch.add_argument("--receipt", type=Path, required=True)
    fetch.add_argument("--expected-sha256", required=True)
    fetch.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    try:
        if args.command == "fetch":
            report = fetch_cell(args.receipt, args.expected_sha256, args.out)
        else:
            report, rows = audit(args.run, args.expected_run_sha256)
            if args.native_verifier:
                replay_native(args.run, args.native_verifier, report, rows)
            if args.command == "export":
                require(report["producer_native_replay_passed"] or args.allow_unreplayed_preview,
                        "export requires native replay or explicit unreplayed preview")
                export_rows(rows, args.out)
                report["export_sha256"] = digest(args.out.read_bytes())
                report["export_rows"] = len(rows)
        print(json.dumps(report, indent=2))
    except (ValueError, KeyError, TypeError, OSError, subprocess.TimeoutExpired) as error:
        parser.exit(1, f"consumer rejected dataset: {error}\n")


if __name__ == "__main__":
    main()
