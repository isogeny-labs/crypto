#!/usr/bin/env python3
"""Replay one pinned N83 provider archive on an independent runner."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import stat
import subprocess
import sys
import zipfile

HERE = Path(__file__).resolve().parent


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def canonical_sha256(value: object) -> str:
    blob = json.dumps(value, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(blob.encode()).hexdigest()


def check_archive(archive: Path, row: dict) -> list[zipfile.ZipInfo]:
    require(archive.stat().st_size == row["zip_bytes"]
            and sha256_file(archive) == row["zip_sha256"],
            "provider artifact size or SHA-256 differs")
    with zipfile.ZipFile(archive) as provider:
        members = provider.infolist()
        names = [member.filename for member in members]
        require(len(names) == len(set(names)), "duplicate provider member")
        require(all(not Path(name).is_absolute()
                    and ".." not in Path(name).parts
                    and not stat.S_ISLNK(member.external_attr >> 16)
                    for name, member in zip(names, members)),
                "unsafe provider member")
        results = [member for member in members
                   if member.filename.startswith("results/")]
        require(len(results) == row["result_members"]
                and sum(member.file_size for member in results) ==
                row["result_uncompressed_bytes"]
                and any(member.filename == "results/analysis.json"
                        for member in results),
                "provider results coverage differs")
        return results


def source_commit(source_root: Path) -> str:
    return subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=source_root, text=True).strip()


def replay(stratum: int, archive: Path, source_root: Path,
           work_root: Path, receipt_dir: Path) -> dict:
    dispatch_bytes = (HERE / "dispatch.json").read_bytes()
    dispatch = json.loads(dispatch_bytes)
    require(dispatch["schema"] ==
            "n83.crossfamily-independent-hosted-replay-dispatch/v1"
            and dispatch["selected_best_total_runtime"] is None
            and [row["stratum"] for row in dispatch["strata"]] ==
            [0, 1, 2, 3], "frozen dispatch differs")
    row = dispatch["strata"][stratum]
    require(source_commit(source_root) == dispatch["source_commit"],
            "source checkout differs")
    results = check_archive(archive, row)
    extracted = work_root / "extracted"
    extracted.mkdir(parents=True, exist_ok=False)
    with zipfile.ZipFile(archive) as provider:
        provider.extractall(extracted, members=[member.filename
                                                for member in results])
    result_root = extracted / "results"
    analysis_file = result_root / "analysis.json"
    require(sha256_file(analysis_file) == row["provider_analysis_sha256"],
            "provider analysis bytes differ")
    analysis = json.loads(analysis_file.read_text())
    command = [sys.executable,
               str(source_root / "research" /
                   "n83_crossfamily_cancel_20261017" / "verify.py"),
               str(result_root), "--stratum", str(stratum)]
    process = subprocess.run(command, cwd=source_root, capture_output=True,
                             text=True, check=False)
    (receipt_dir / "replay.stdout.json").write_text(process.stdout)
    (receipt_dir / "replay.stderr.txt").write_text(process.stderr)
    (receipt_dir / "replay.exit_status.txt").write_text(
        str(process.returncode) + "\n")
    require(process.returncode == 0 and not process.stderr,
            f"source verifier failed: {process.returncode}")
    checked = json.loads(process.stdout)
    require(checked == analysis
            and analysis["status"] == "STRATUM_ADMITTED"
            and analysis["evidence_status"] == "PASS_INDEPENDENT_REPLAY"
            and analysis["tier_status"] == "PASS_TIER1"
            and analysis["stratum"] == stratum,
            "independent replay differs from provider analysis")
    return {
        "schema": "n83.crossfamily-independent-hosted-replay/v1",
        "status": "PASS_INDEPENDENT_HOSTED_REPLAY",
        "stratum": stratum,
        "source_commit": dispatch["source_commit"],
        "provider_run_id": dispatch["provider_run_id"],
        "provider_run_attempt": dispatch["provider_run_attempt"],
        "provider_artifact_id": row["artifact_id"],
        "provider_zip_sha256": row["zip_sha256"],
        "provider_analysis_sha256": row["provider_analysis_sha256"],
        "canonical_analysis_sha256": canonical_sha256(checked),
        "dispatch_sha256": hashlib.sha256(dispatch_bytes).hexdigest(),
        "bound_result_members": len(results),
        "runner_name": os.environ.get("RUNNER_NAME"),
        "runner_os": os.environ.get("RUNNER_OS"),
        "independent_run_id": os.environ.get("GITHUB_RUN_ID"),
        "independent_run_attempt": os.environ.get("GITHUB_RUN_ATTEMPT"),
        "independent_job": os.environ.get("GITHUB_JOB"),
        "selected_best_total_runtime": None,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--stratum", type=int, choices=range(4), required=True)
    parser.add_argument("--archive", type=Path, required=True)
    parser.add_argument("--source-root", type=Path, required=True)
    parser.add_argument("--work-root", type=Path, required=True)
    parser.add_argument("--receipt-dir", type=Path, required=True)
    args = parser.parse_args()
    args.receipt_dir.mkdir(parents=True, exist_ok=True)
    try:
        result = replay(args.stratum, args.archive, args.source_root,
                        args.work_root, args.receipt_dir)
    except Exception as error:
        failure = {"schema": "n83.crossfamily-independent-hosted-replay/v1",
                   "status": "REPLAY_FAILED", "stratum": args.stratum,
                   "error": f"{type(error).__name__}: {error}",
                   "selected_best_total_runtime": None}
        (args.receipt_dir / "replay.json").write_text(
            json.dumps(failure, sort_keys=True, indent=2) + "\n")
        raise
    (args.receipt_dir / "replay.json").write_text(
        json.dumps(result, sort_keys=True, indent=2) + "\n")
    print(json.dumps({"status": result["status"],
                      "stratum": args.stratum}, sort_keys=True))


if __name__ == "__main__":
    main()
