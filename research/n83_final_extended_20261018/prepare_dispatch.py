#!/usr/bin/env python3
"""Freeze the extended holdout after native replay of the six-arm panel."""

import argparse
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
if sys.path and Path(sys.path[0]).resolve() == HERE:
    sys.path.pop(0)
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from research.n83_final_extended_20261018 import arms, check_dispatch, run  # noqa: E402


def copy_once(source, destination):
    data = source.read_bytes()
    if destination.exists() and destination.read_bytes() != data:
        raise ValueError(f"sealed upstream evidence differs: {destination}")
    destination.write_bytes(data)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--upstream-validated', type=Path, required=True)
    parser.add_argument('--upstream-selection', type=Path, required=True)
    args = parser.parse_args()
    copy_once(args.upstream_validated.resolve(), HERE / 'upstream_validated.json')
    copy_once(args.upstream_selection.resolve(), HERE / 'upstream_selection.json')
    chosen, choice = arms.choose(HERE / 'upstream_validated.json',
                                 HERE / 'upstream_selection.json')
    paths = check_dispatch.required_paths(chosen)
    hashes = {path: check_dispatch.digest(ROOT / path) for path in sorted(paths)}
    dispatch = {
        'schema': 'n83.final-extended-dispatch/v1',
        'status': 'SEALED_FOR_DISPATCH',
        'point_count': 16,
        'host_strata': 2,
        'points_per_host': 8,
        'producer_cap_seconds': run.GLOBAL_PRODUCER_CAP_SECONDS,
        'job_cap_minutes': 270,
        'source_sha256': run.SOURCE_SHA256,
        'targets_sha256': run.TARGETS_SHA256,
        'answers_sha256': run.ANSWERS_SHA256,
        'ordered_candidate_ids': [item[3] for item in chosen],
        'choice': choice,
        'sha256': hashes,
    }
    path = HERE / 'dispatch.json'
    blob = json.dumps(dispatch, sort_keys=True, indent=2) + '\n'
    if path.exists() and path.read_text() != blob:
        raise ValueError('sealed dispatch differs')
    path.write_text(blob)
    check_dispatch.main()


if __name__ == '__main__':
    main()
