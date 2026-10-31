#!/usr/bin/env python3
"""Project every retained native method cell into experiment.export/v1.

This reads custody and native artifacts only. No solver or S3 write is invoked.
"""

import argparse
import hashlib
import importlib.util
import json
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

HERE = Path(__file__).resolve().parent
METHODS = ("sat", "f4", "f5", "weil", "double-large-prime", "bsgs")


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path):
    return json.loads(path.read_text())


def encoded(value):
    return (json.dumps(value, sort_keys=True, indent=2, allow_nan=False) + "\n").encode()


def write_equal(path, value):
    data = encoded(value)
    if path.exists():
        if path.read_bytes() != data:
            raise RuntimeError(f"existing output differs: {path}")
    else:
        path.write_bytes(data)


def file_entry(path, kind):
    return {"path": str(path.relative_to(HERE)), "sha256": digest(path),
            "bytes": path.stat().st_size, "kind": kind}


def iso_ns(value):
    return datetime.fromtimestamp(int(value) / 1_000_000_000, timezone.utc).isoformat()


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--native-root", type=Path, default=Path("/mnt/spot-codex-state/tmp/ml-dataset-binary-methods-20261031"))
    ap.add_argument("--tooling", type=Path, default=Path("/home/ec2-user/crypto-autoresearcher-fall-degree-submission-20261029/tools/experiment_protocol.py"))
    args = ap.parse_args()
    protocol, plan = read(HERE / "protocol.json"), read(HERE / "plan.json")
    assert protocol["freeze"]["status"] == "frozen" and len(protocol["arms"]) == 12
    assert plan["protocol_sha256"] == digest(HERE / "protocol.json")
    loader = importlib.util.spec_from_file_location("experiment_protocol_pinned", args.tooling)
    tooling = importlib.util.module_from_spec(loader)
    loader.loader.exec_module(tooling)
    arms = {a["id"]: a for a in protocol["arms"]}
    rows, source, finished_ns = [], [], []
    for i in range(48):
        shard = args.native_root / f"shard-{i:02d}"
        status_path = shard / "local-status.json"
        pointer_path = HERE / "checkpoints" / f"shard-{i:02d}.json"
        status, pointer = read(status_path), read(pointer_path)
        spec = status["cell_spec"]
        assert spec == {"solver": METHODS[i % 6], "twist": bool((i // 6) % 2),
                        "key_index": (i // 12) % 2, "generator_index": (i // 24) % 2}
        assert pointer["index"] == i and pointer["custody"]["index"] == i
        assert pointer["custody"]["protocol_file_sha256"] == digest(HERE / "protocol.json")
        assert pointer["custody"]["plan_file_sha256"] == digest(HERE / "plan.json")
        assert pointer["published"]["archive_fresh_get"] is True
        assert pointer["published"]["pointer_fresh_get"] is True
        assert pointer["published"]["archive_sha256"] == pointer["custody"]["archive_sha256"]
        arm = f"binary23-{spec['solver']}-{'twisted' if spec['twist'] else 'untwisted'}"
        assert arm in arms
        receipt_path = shard / "runner" / "receipt.json"
        manifest_path = shard / "runner" / "attempt" / "cells" / f"{i:012d}" / "manifest.json"
        receipt = read(receipt_path) if receipt_path.is_file() else None
        manifest = read(manifest_path) if manifest_path.is_file() else None
        if receipt:
            assert receipt["binary_sha256_before"] == receipt["binary_sha256_after"] == arms[arm]["instrument"]["binary_sha256"]
            assert receipt["source_commit"] == arms[arm]["instrument"]["commit"]
        if manifest:
            assert manifest["cell_index"] == i and manifest["config"]["solver"] == spec["solver"]
            assert manifest["config"]["twist"] == spec["twist"]
        if status["ecdlp_solve_promoted"]:
            assert status["sealed_cell_verified"] and status["verified_and_exported"]
            assert status["independently_solved_ecdlp_cells"] == 1
            assert manifest["outcome"]["scalar_recovered_without_witness"] is True
            assert manifest["outcome"]["scalar_replay_verified"] is True
        if status["verified_and_exported"]:
            export_status, verification = "succeeded", "verified"
        elif status["launcher"].get("timed_out") or (receipt and receipt.get("timed_out")):
            export_status, verification = "timeout", "no_claim"
        elif status["launcher"].get("interrupted_before_launcher_receipt"):
            export_status, verification = "interrupted", "no_claim"
        elif status["launcher"].get("exit_code") not in (0, None):
            export_status, verification = "failed", "no_claim"
        else:
            export_status, verification = "infra_error", "no_claim"
        row = {"run_id": f"shard-{i:02d}", "arm": arm,
               "instance_id": str(2 * spec["generator_index"] + spec["key_index"]),
               "seed": 2026103102, "replicate": 0, "warmup": False,
               "status": export_status, "verification": verification,
               "sealed-cell-verified": int(bool(status["sealed_cell_verified"])),
               "independent-ecdlp-solve": int(bool(status["ecdlp_solve_promoted"])),
               "durable-cell": 1,
               "native_status": status["native_status"], "method_artifact_verified": status["method_artifact_verified"],
               "restricted_archive_sha256": pointer["custody"]["archive_sha256"],
               "restricted_archive_s3_uri": pointer["custody"]["s3_uri"]}
        if receipt and receipt.get("observed_wall_nanoseconds"):
            row["wall_seconds"] = int(receipt["observed_wall_nanoseconds"]) / 1_000_000_000
        rows.append(row)
        source.append((receipt, manifest, pointer_path))
        if status["launcher"].get("finished_unix_ns"):
            finished_ns.append(int(status["launcher"]["finished_unix_ns"]))
    rows_path = HERE / "experiment-rows.jsonl"
    row_bytes = b"".join((json.dumps(r, sort_keys=True, allow_nan=False) + "\n").encode() for r in rows)
    if rows_path.exists() and rows_path.read_bytes() != row_bytes:
        raise RuntimeError("retained experiment rows differ")
    if not rows_path.exists():
        rows_path.write_bytes(row_bytes)
    export = tooling.from_rows(rows_path, protocol, "protocol.json", "binary23-methods-48-20261031", {})
    export["created"] = datetime.fromtimestamp(max(finished_ns) / 1_000_000_000, timezone.utc).isoformat()
    export["producer"] = {"runtime": "codex_cli", "model": None, "session": "T-1079", "role": "executor"}
    export["source"] = {"repo": protocol["repository"], "commit": protocol["arms"][0]["instrument"]["commit"], "dirty": False}
    export["backend"] = "local"
    export["arms"] = [{"id": a["id"], "role": a["role"],
                       "binary_sha256": a["instrument"]["binary_sha256"],
                       "commit": a["instrument"]["commit"]} for a in protocol["arms"]]
    export["files"].extend(file_entry(HERE / name, kind) for name, kind in (
        ("protocol.json", "protocol"), ("plan.json", "raw"),
        ("make_experiment_export.py", "other")))
    for row, (receipt, manifest, pointer_path) in zip(export["runs"], source):
        row["verification"]["verifier"] = "crypto dataset verify; exact native method scope and scalar replay only where promoted"
        row["artifacts"] = [file_entry(pointer_path, "checkpoint_manifest")]
        row["backend_ref"] = f"native-cell-{int(row['run_id'].split('-')[1]):012d}"
        row["isolation"]["tier"] = "none"
        row["measurements"]["wall_valid"] = False
        if receipt:
            row["started_at"] = iso_ns(receipt["started_unix_nanoseconds"])
            row["finished_at"] = iso_ns(receipt["finished_unix_nanoseconds"])
            row["measurements"]["counters"]["observed-wall-ns"] = receipt.get("observed_wall_nanoseconds")
        if manifest:
            for key, value in manifest["outcome"].get("counters", {}).items():
                if isinstance(value, (int, float, str)):
                    row["measurements"]["counters"]["native_" + key] = value
    export["flow"] = tooling.compute_flow(export["runs"])
    export["notes"] = [{"kind": "observation", "text": "All 48 fixed cells remain in the export, including native partial, budget_exhausted, unsupported, failed and timeout outcomes. No shared-host timing comparison is made.", "by": "deterministic receipt projection"}]
    output = HERE / "experiment-export.json"
    write_equal(output, export)
    checked = subprocess.run([sys.executable, str(args.tooling), "check-export", str(output),
                              "--protocol", str(HERE / "protocol.json"), "--verify-files"],
                             capture_output=True, text=True, check=False)
    if checked.returncode:
        raise RuntimeError(f"official check-export failed: {checked.stdout}{checked.stderr}")
    write_equal(HERE / "experiment-export-producer-receipt.json",
                {"schema": "ml-dataset.binary-methods-export-receipt/v1", "export_sha256": digest(output),
                 "rows_sha256": digest(rows_path), "tooling_sha256": digest(args.tooling),
                 "protocol_body_sha256": protocol["freeze"]["protocol_sha256"],
                 "flow": export["flow"], "official_checker_exit_code": checked.returncode,
                 "official_checker_stdout": checked.stdout})
    print(json.dumps({"export_sha256": digest(output), "flow": export["flow"]}, sort_keys=True))


if __name__ == "__main__":
    main()
