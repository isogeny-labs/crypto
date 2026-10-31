#!/usr/bin/env python3
"""Deterministically bind the retained 16 native cells to experiment.export/v1.

This reads native evidence only. It neither runs a solver nor contacts S3.
The official checker verifies local row, protocol, plan and pointer bytes;
the pointers contain the separately obtained S3 fresh-GET receipts.
"""

import argparse
import hashlib
import importlib.util
import json
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path):
    return json.loads(path.read_text())


def encoded(obj):
    return (json.dumps(obj, indent=2, sort_keys=True, ensure_ascii=False, allow_nan=False) + "\n").encode()


def write_once_or_equal(path, data):
    if path.exists():
        if path.read_bytes() != data:
            raise RuntimeError(f"existing output differs: {path}")
    else:
        path.write_bytes(data)


def file_entry(base, path, kind):
    return {"path": str(path.relative_to(base)), "sha256": digest(path), "bytes": path.stat().st_size, "kind": kind}


def iso_ns(value):
    return datetime.fromtimestamp(int(value) / 1_000_000_000, timezone.utc).isoformat()


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--native-root", type=Path, required=True)
    ap.add_argument("--tooling", type=Path, required=True, help="Pinned official experiment_protocol.py")
    args = ap.parse_args()
    base = Path(__file__).resolve().parent
    protocol = read(base / "protocol.json")
    plan = read(base / "plan.json")
    if protocol["freeze"]["status"] != "frozen" or len(protocol["arms"]) != 4:
        raise RuntimeError("unexpected frozen protocol")
    if plan.get("schema") is None:
        raise RuntimeError("plan missing schema")
    spec = importlib.util.spec_from_file_location("pinned_experiment_protocol", args.tooling)
    tooling = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(tooling)
    arms = {a["id"]: a for a in protocol["arms"]}
    rows = []
    source = []
    for i in range(16):
        shard = args.native_root / f"shard-{i:02d}"
        status_path = shard / "local-status.json"
        receipt_path = shard / "runner/receipt.json"
        manifest_path = shard / f"runner/attempt/cells/{i:012d}/manifest.json"
        pointer_path = base / f"checkpoints/shard-{i:02d}.json"
        status, receipt, manifest, pointer = map(read, (status_path, receipt_path, manifest_path, pointer_path))
        spec_cell = status["cell_spec"]
        outcome = manifest["outcome"]
        solver, twist = spec_cell["solver"], spec_cell["twist"]
        arm = f"binary23-{solver}-{'twisted' if twist else 'untwisted'}"
        assert arm in arms and status["index"] == i and manifest["cell_index"] == i
        assert pointer["index"] == i and pointer["custody"]["index"] == i
        assert pointer["published"]["archive_fresh_get"] and pointer["published"]["pointer_fresh_get"]
        assert pointer["custody"]["archive_sha256"] == pointer["published"]["archive_sha256"]
        assert receipt["binary_sha256_before"] == receipt["binary_sha256_after"] == arms[arm]["instrument"]["binary_sha256"]
        assert receipt["source_commit"] == arms[arm]["instrument"]["commit"]
        assert receipt["exit_code"] == 0 and not receipt["timed_out"]
        assert status["verify"] == status["report"] == status["export"] == 0
        assert status["sealed_cell_verified"] is True and status["artifact_hashes_verified"] is True
        native_status = outcome["status"]
        if solver == "root":
            assert native_status == "partial" and status["root_certificate_promoted"] is True
            assert status["root_queries_completed"] is True and status["root_censored"] is False
            assert status["ecdlp_solve_promoted"] is False and status["scalar_recovery_label"] is None
            export_status, verification = "succeeded", "verified"
            counted = {"sealed-cell-verified": 1, "verified-root-certificate": 1}
        elif native_status == "verified":
            assert status["ecdlp_solve_promoted"] is True and status["independently_solved_ecdlp_cells"] == 1
            assert outcome["scalar_recovered_without_witness"] and outcome["scalar_replay_verified"]
            export_status, verification = "succeeded", "verified"
            counted = {"sealed-cell-verified": 1, "independent-ecdlp-solve": 1}
        else:
            assert solver == "bsgs" and native_status == "timeout" and i in (1, 11)
            assert status["ecdlp_solve_promoted"] is False and status["independently_solved_ecdlp_cells"] == 0
            export_status, verification = "timeout", "no_claim"
            counted = {"sealed-cell-verified": 1, "independent-ecdlp-solve": 0}
        row = {
            "run_id": f"shard-{i:02d}", "arm": arm,
            "instance_id": str(2 * spec_cell["generator_index"] + spec_cell["key_index"]),
            "native_instance_id": f"g{spec_cell['generator_index']}k{spec_cell['key_index']}",
            "seed": 20261031, "replicate": 0, "warmup": False,
            "status": export_status, "verification": verification,
            **counted,
            "wall_seconds": int(receipt["observed_wall_nanoseconds"]) / 1_000_000_000,
            "native_status": native_status, "native_manifest_sha256": digest(manifest_path),
            "native_receipt_sha256": digest(receipt_path), "local_status_sha256": digest(status_path),
            "checkpoint_sha256": digest(pointer_path),
            "restricted_archive_sha256": pointer["custody"]["archive_sha256"],
            "restricted_archive_s3_uri": pointer["custody"]["s3_uri"],
        }
        rows.append(row)
        source.append((receipt, outcome, pointer_path))
    rows_path = base / "experiment-rows.jsonl"
    rows_bytes = b"".join((json.dumps(r, sort_keys=True, ensure_ascii=False, allow_nan=False) + "\n").encode() for r in rows)
    write_once_or_equal(rows_path, rows_bytes)
    export = tooling.from_rows(rows_path, protocol, "protocol.json", "binary23-root-bsgs-16-20261031", {})
    export["created"] = read(base / "TERMINAL-01.json")["observed_at_utc"]
    export["producer"] = {"runtime": "codex_cli", "model": None, "session": "T-1066", "role": "executor"}
    export["source"] = {"repo": protocol["repository"], "commit": protocol["arms"][0]["instrument"]["commit"], "dirty": False}
    export["backend"] = "local"
    export["arms"] = [{"id": a["id"], "role": a["role"], "binary_sha256": a["instrument"]["binary_sha256"], "commit": a["instrument"]["commit"]} for a in protocol["arms"]]
    export["files"].extend(file_entry(base, base / name, kind) for name, kind in (
        ("protocol.json", "protocol"), ("plan.json", "raw"), ("TERMINAL-01.json", "receipt"),
        ("AUDIT-01.json", "summary"), ("make_experiment_export.py", "other")))
    for row, (receipt, outcome, pointer_path) in zip(export["runs"], source):
        row["verification"]["verifier"] = "crypto dataset verify; root certificates or scalar replay only where promoted"
        row["artifacts"] = [file_entry(base, pointer_path, "checkpoint_manifest")]
        row["backend_ref"] = f"native-cell-{int(row['run_id'].split('-')[1]):012d}"
        row["started_at"] = iso_ns(receipt["started_unix_nanoseconds"])
        row["finished_at"] = iso_ns(receipt["finished_unix_nanoseconds"])
        row["measurements"]["counters"] = {"observed-wall-ns": receipt["observed_wall_nanoseconds"]}
        for key, value in outcome.get("counters", {}).items():
            if isinstance(value, (int, float, str)):
                row["measurements"]["counters"]["native_" + key] = value
        row["measurements"]["wall_valid"] = False
        row["isolation"]["tier"] = "none"
    export["flow"] = tooling.compute_flow(export["runs"])
    export["notes"] = [
        {"kind": "observation", "text": "All 16 native generate commands exited zero, retained sealed manifests and passed dataset verify; two inner BSGS workers timed out at indices 1 and 11.", "by": "deterministic receipt projection"},
        {"kind": "observation", "text": "Eight root outcomes are native partial with completed distinct-root certificates, no positive PDP witness and no scalar-recovery label. Six BSGS outcomes have independently replayed scalars.", "by": "deterministic receipt projection"},
        {"kind": "observation", "text": "Observed wrapper nanoseconds are retained as diagnostic counters; native receipts mark their clocks invalid for comparison. Worker retired instruction counts are unavailable.", "by": "deterministic receipt projection"},
    ]
    export_path = base / "experiment-export.json"
    write_once_or_equal(export_path, encoded(export))
    command = [sys.executable, str(args.tooling), "check-export", str(export_path), "--protocol", str(base / "protocol.json"), "--verify-files"]
    checked = subprocess.run(command, text=True, capture_output=True, check=False)
    if checked.returncode != 0:
        raise RuntimeError(f"official check-export failed ({checked.returncode}): {checked.stdout}{checked.stderr}")
    report = {"schema": "ml-dataset.binary-experiment-export-receipt/v1", "export_sha256": digest(export_path),
              "rows_sha256": digest(rows_path), "tooling_sha256": digest(args.tooling),
              "protocol_sha256": protocol["freeze"]["protocol_sha256"], "flow": export["flow"],
              "native_status_counts": {s: sum(r["native_status"] == s for r in rows) for s in ("partial", "verified", "timeout")},
              "s3_archives_fresh_get_receipted": 16,
              "official_checker": {"argv": command, "exit_code": checked.returncode, "stdout": checked.stdout, "stderr": checked.stderr}}
    write_once_or_equal(base / "experiment-export-producer-receipt.json", encoded(report))
    print(json.dumps({"export": str(export_path), "sha256": digest(export_path), "flow": export["flow"]}, sort_keys=True))


if __name__ == "__main__":
    main()
