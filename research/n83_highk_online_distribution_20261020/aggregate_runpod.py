#!/usr/bin/env python3
"""Bind the distribution archive, provider allocation, and local replay."""

import argparse
import hashlib
import json
import subprocess
import sys
import tarfile
from pathlib import Path

from research.n83_highk_online_distribution_20261020 import runpod_provider
from research.n83_highk_online_distribution_20261020 import select as screen_selector
from research.n83_highk_online_distribution_20261020 import holdout_select

HERE = Path(__file__).resolve().parent


def digest(path):
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(chunk)
    return value.hexdigest()


def archive_files(archive, results):
    expected = {}
    with tarfile.open(archive, "r:gz") as bundle:
        for member in bundle:
            path = Path(member.name)
            if (path.is_absolute() or ".." in path.parts or
                    not path.parts or path.parts[0] != "results" or
                    member.issym() or member.islnk() or
                    not (member.isfile() or member.isdir())):
                raise ValueError("unsafe distribution archive member")
            if member.isdir():
                continue
            relative = Path(*path.parts[1:])
            if relative in expected:
                raise ValueError("duplicate distribution archive member")
            handle = bundle.extractfile(member)
            if handle is None:
                raise ValueError("unreadable distribution archive member")
            expected[relative] = hashlib.sha256(handle.read()).hexdigest()
    actual = {path.relative_to(results): digest(path)
              for path in results.rglob("*") if path.is_file()}
    if not expected or expected != actual:
        raise ValueError("archive and extracted distribution bytes differ")
    return {"archive_sha256": digest(archive), "file_count": len(expected)}


def replay(module, results, output):
    command = [sys.executable, str(HERE / module), str(results)]
    process = subprocess.run(command, cwd=HERE.parents[1], text=True,
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    (output / f"{module}.stdout.txt").write_text(process.stdout)
    (output / f"{module}.stderr.txt").write_text(process.stderr)
    (output / f"{module}.exit.txt").write_text(f"{process.returncode}\n")
    if process.returncode not in (0, 2) or process.stderr:
        raise ValueError(f"{module}: local independent replay failed")
    return json.loads(process.stdout)


def aggregate(snapshot, results, archive, head_sha, output):
    output.mkdir(parents=True, exist_ok=False)
    bound = archive_files(archive, results)
    provider = runpod_provider.verify(snapshot, results, head_sha)
    screen = replay("verify_screen.py", results, output)
    screen_analysis = json.loads((results / "screen_analysis.json").read_text())
    screen_validated = json.loads((results / "screen_validated.json").read_text())
    screen_selection = json.loads((results / "screen_selection.json").read_text())
    if (screen != {"admission_status": screen_selection["admission_status"],
                   "status": screen_selection["status"],
                   "selected_candidate_id": screen_selection["selected_candidate_id"]} or
            screen_selection != screen_selector.select(screen_validated) or
            screen_analysis["evidence_status"] != screen_validated["evidence_status"]):
        raise ValueError("screen replay and sealed selection differ")
    holdout_path = results / "holdout_selection.json"
    if screen_selection["status"] == "SELECTED" and screen_selection["admission_status"] == "PASS":
        holdout = replay("verify_holdout.py", results, output)
        holdout_validated = json.loads((results / "holdout_validated.json").read_text())
        holdout_selection = json.loads(holdout_path.read_text())
        if (holdout != {"admission_status": holdout_selection["admission_status"],
                        "status": holdout_selection["status"],
                        "verified_speedup_candidate_id":
                        holdout_selection["verified_speedup_candidate_id"]} or
                holdout_selection != holdout_select.select(screen_selection, holdout_validated)):
            raise ValueError("holdout replay and sealed selection differ")
        holdout_summary = holdout_selection
    else:
        if holdout_path.exists() or (results / "holdout").exists():
            raise ValueError("holdout ran without admitted screen selection")
        receipt = json.loads((results / "holdout_not_dispatched.json").read_text())
        if (receipt.get("schema") != "n83.online-distribution-not-dispatched/v1" or
                receipt.get("reason") not in
                ("SCREEN_BOUNDED_UNKNOWN", "SCREEN_NOT_ADMITTED")):
            raise ValueError("missing conditional holdout decision")
        holdout_summary = receipt
    if archive_files(archive, results) != bound:
        raise ValueError("local replay changed archived result bytes")
    result = {"schema": "n83.online-distribution-local-aggregate/v1",
              "provider": provider, "archive": bound,
              "screen_selection": screen_selection,
              "holdout": holdout_summary}
    (output / "aggregate.json").write_text(
        json.dumps(result, indent=2, sort_keys=True) + "\n")
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--snapshot", type=Path, required=True)
    parser.add_argument("--results", type=Path, required=True)
    parser.add_argument("--archive", type=Path, required=True)
    parser.add_argument("--head-sha", required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = aggregate(args.snapshot.resolve(), args.results.resolve(),
                       args.archive.resolve(), args.head_sha,
                       args.output.resolve())
    print(json.dumps({"screen_status": result["screen_selection"]["status"],
                      "holdout_status": result["holdout"]["status"]
                      if "status" in result["holdout"] else
                      result["holdout"]["reason"]}, sort_keys=True))


if __name__ == "__main__":
    main()
