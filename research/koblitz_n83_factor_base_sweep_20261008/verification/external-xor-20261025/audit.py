#!/usr/bin/env python3
"""Rebuild retained N83 XCNF and audit the bounded external-solver receipts."""

import argparse
import gzip
import hashlib
import json
import pathlib
import re
import subprocess
import tempfile


HERE = pathlib.Path(__file__).resolve().parent
STUDY = HERE.parent.parent
MANIFEST = STUDY / "pilot-01/manifest.json"
CORPUS = STUDY / "pilot-01/probe-corpus.json"


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def audit(worker: pathlib.Path, archive_dir: pathlib.Path) -> dict:
    receipt = json.loads((HERE / "receipt.json").read_bytes())
    require(receipt["schema"] == "n83.external-xor-local-pilot/v1", "receipt schema")
    results = []
    for arm in receipt["arms"]:
        name = arm["name"]
        folder = HERE / name
        export_expected = json.loads((folder / "export.json").read_bytes())
        base_hash = export_expected["base_object_blake3"]
        pool_hash = export_expected["pool_object_blake3"]
        def archive(digest: str) -> pathlib.Path:
            path = archive_dir / f"s3-{digest}.jsonl.gz"
            require(path.is_file(), f"missing archive {digest}")
            return path

        with tempfile.TemporaryDirectory(prefix="n83-xcnf-audit-") as temporary:
            out = pathlib.Path(temporary) / "export"
            command = [
                str(worker), "export-cross-xcnf", base_hash, str(MANIFEST),
                str(archive(base_hash)), pool_hash, str(MANIFEST),
                str(archive(pool_hash)), str(CORPUS),
                str(export_expected["base_summands"]),
                str(export_expected["residual_summands"]), str(out),
            ]
            replay = subprocess.run(command, capture_output=True, timeout=120)
            require(replay.returncode == 0, f"{name}: export replay failed: {replay.stderr[-400:]!r}")
            actual = json.loads((out / "export.json").read_bytes())
            require(actual == export_expected, f"{name}: export metadata changed")
            xcnf = (out / "instance.xcnf").read_bytes()
            require(sha256(xcnf) == arm["xcnf_sha256"], f"{name}: XCNF changed")
            require(len(xcnf) == export_expected["xcnf_bytes"], f"{name}: XCNF size")

        raw = gzip.decompress((folder / "solver.stdout.gz").read_bytes())
        stderr = gzip.decompress((folder / "solver.stderr.gz").read_bytes())
        require(sha256(raw) == arm["solver_stdout_sha256"], f"{name}: stdout digest")
        require(sha256(stderr) == arm["solver_stderr_sha256"], f"{name}: stderr digest")
        require(stderr == b"", f"{name}: solver wrote stderr")
        lines = raw.decode("utf-8").splitlines()
        statuses = [line for line in lines if line.startswith("s ")]
        require(statuses == ["s INDETERMINATE"], f"{name}: solver status")
        require(not any(line.startswith("v ") for line in lines), f"{name}: unexpected model")
        conflicts = [int(m.group(1)) for line in lines
                     if (m := re.fullmatch(r"c conflicts\s+:\s+(\d+)\s*", line))]
        require(len(conflicts) == 1, f"{name}: conflict counter missing or repeated")
        require(arm["conflict_cap"] <= conflicts[0] <= arm["conflict_cap"] + 100,
                f"{name}: conflict counter outside stop tolerance")
        process = json.loads((folder / "solver-process.json").read_bytes())
        require(process["status"] == "completed" and process["exit_code"] == 15,
                f"{name}: process exit")
        require(0 < process["elapsed_seconds"] <= arm["watchdog_seconds"],
                f"{name}: process elapsed time")
        require(process["stdout_bytes"] == len(raw) and process["stderr_bytes"] == len(stderr),
                f"{name}: output lengths")
        results.append({"name": name, "status": "PASS_REPLAY_BOUNDED_INDETERMINATE",
                        "conflicts": conflicts[0], "xcnf_sha256": arm["xcnf_sha256"],
                        "solver_wall_seconds_L0": process["elapsed_seconds"]})
    return {"schema": "n83.external-xor-local-pilot-replay/v1", "status": "PASS",
            "arms": results, "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--worker", required=True, type=pathlib.Path)
    parser.add_argument("--archive-dir", required=True, type=pathlib.Path)
    args = parser.parse_args()
    print(json.dumps(audit(args.worker, args.archive_dir), indent=2, sort_keys=True))
