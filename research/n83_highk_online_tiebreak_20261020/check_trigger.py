#!/usr/bin/env python3
"""Bind the tie-break trigger to the independently replayed prior archive."""

import argparse
import hashlib
import json
from pathlib import Path

from research.n83_highk_online_tiebreak_20261020 import select

PRIOR_SCREEN_TARGETS_SHA256 = (
    "0617ee4c55b08e3b4379d99490cad9c249c50ef2b0cc6ab321fe63a0bb1ba0b3")


def digest(path):
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(chunk)
    return value.hexdigest()


def check(aggregate_path, archive_path):
    aggregate = json.loads(aggregate_path.read_text())
    screen = aggregate.get("screen_selection", {})
    holdout = aggregate.get("holdout", {})
    provider = aggregate.get("provider", {})
    archive = aggregate.get("archive", {})
    archive_sha256 = digest(archive_path)
    if (aggregate.get("schema") !=
            "n83.online-distribution-local-aggregate/v1" or
            provider.get("schema") !=
            "n83.online-distribution-runpod-provider-proof/v1" or
            provider.get("status") != "PASS_SINGLE_RECORDED_RUNPOD_ALLOCATION" or
            screen.get("schema") !=
            "n83.online-distribution-screen-selection/v1" or
            screen.get("admission_status") != "PASS" or
            screen.get("status") != "BOUNDED_UNKNOWN" or
            screen.get("selected_candidate_id") is not None or
            screen.get("candidate_ids") != list(select.IDS) or
            screen.get("point_count") != 192 or
            screen.get("screen_mode") !=
            "secondary_shared_setup_per_point_intervals" or
            archive.get("archive_sha256") != archive_sha256 or
            not isinstance(archive.get("file_count"), int) or
            archive["file_count"] <= 0 or
            holdout.get("schema") !=
            "n83.online-distribution-not-dispatched/v1" or
            holdout.get("reason") != "SCREEN_BOUNDED_UNKNOWN"):
        raise ValueError("prior screen is not an admitted, unresolved archive")
    return {
        "schema": "n83.online-tiebreak-trigger/v1",
        "status": "PASS_ADMITTED_BOUNDED_UNKNOWN",
        "trigger_aggregate_sha256": digest(aggregate_path),
        "trigger_archive_sha256": archive_sha256,
        "prior_screen_targets_sha256": PRIOR_SCREEN_TARGETS_SHA256,
        "prior_source_sha256": select.SOURCE_SHA256,
        "candidate_ids": list(select.IDS),
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--aggregate", type=Path, required=True)
    parser.add_argument("--archive", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    receipt = check(args.aggregate, args.archive)
    blob = (json.dumps(receipt, indent=2, sort_keys=True) + "\n").encode()
    if args.output.exists() and args.output.read_bytes() != blob:
        raise ValueError("sealed trigger receipt differs")
    args.output.write_bytes(blob)
    print(json.dumps({"status": receipt["status"],
                      "trigger_aggregate_sha256":
                      receipt["trigger_aggregate_sha256"]}, sort_keys=True))


if __name__ == "__main__":
    main()
