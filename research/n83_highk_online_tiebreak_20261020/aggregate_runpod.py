#!/usr/bin/env python3
"""Bind the tie-break archive, trigger, provider, and independent replay."""

import argparse
import json
import subprocess
import sys
from pathlib import Path

from research.n83_highk_online_distribution_20261020 import aggregate_runpod as prior_aggregate
from research.n83_highk_online_tiebreak_20261020 import check_trigger
from research.n83_highk_online_tiebreak_20261020 import runpod_provider
from research.n83_highk_online_tiebreak_20261020 import select

ROOT = Path(__file__).resolve().parents[2]


def aggregate(snapshot, results, archive, head_sha, prior_aggregate_path,
              prior_archive_path, output):
    output.mkdir(parents=True, exist_ok=False)
    bound = prior_aggregate.archive_files(archive, results)
    trigger = check_trigger.check(prior_aggregate_path, prior_archive_path)
    if json.loads((results / "trigger.json").read_text()) != trigger:
        raise ValueError("tie-break trigger differs from prior replay bytes")
    provider = runpod_provider.verify(snapshot, results, head_sha)
    command = [sys.executable, "-m",
               "research.n83_highk_online_tiebreak_20261020.verify_screen",
               str(results)]
    process = subprocess.run(command, cwd=ROOT, text=True,
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    (output / "verify_screen.stdout.txt").write_text(process.stdout)
    (output / "verify_screen.stderr.txt").write_text(process.stderr)
    (output / "verify_screen.exit.txt").write_text(f"{process.returncode}\n")
    if process.returncode not in (0, 2) or process.stderr:
        raise ValueError("tie-break local independent replay failed")
    reported = json.loads(process.stdout)
    validated = json.loads((results / "screen_validated.json").read_text())
    selection = json.loads((results / "screen_selection.json").read_text())
    if (reported != {"admission_status": selection["admission_status"],
                     "status": selection["status"],
                     "selected_candidate_id": selection["selected_candidate_id"]} or
            selection != select.select(validated)):
        raise ValueError("tie-break selector differs from replay")
    if prior_aggregate.archive_files(archive, results) != bound:
        raise ValueError("local replay changed archived tie-break bytes")
    result = {"schema": "n83.online-tiebreak-local-aggregate/v1",
              "provider": provider, "archive": bound,
              "trigger": trigger, "screen_selection": selection}
    (output / "aggregate.json").write_text(
        json.dumps(result, indent=2, sort_keys=True) + "\n")
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--snapshot", type=Path, required=True)
    parser.add_argument("--results", type=Path, required=True)
    parser.add_argument("--archive", type=Path, required=True)
    parser.add_argument("--head-sha", required=True)
    parser.add_argument("--prior-aggregate", type=Path, required=True)
    parser.add_argument("--prior-archive", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = aggregate(args.snapshot.resolve(), args.results.resolve(),
                       args.archive.resolve(), args.head_sha,
                       args.prior_aggregate.resolve(),
                       args.prior_archive.resolve(), args.output.resolve())
    print(json.dumps({"screen_status": result["screen_selection"]["status"],
                      "selected_candidate_id":
                      result["screen_selection"]["selected_candidate_id"]},
                     sort_keys=True))


if __name__ == "__main__":
    main()
