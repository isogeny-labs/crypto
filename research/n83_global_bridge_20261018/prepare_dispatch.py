#!/usr/bin/env python3
"""Bind upstream selections and seal the two-arm bridge if distinct."""

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

from research.n83_global_bridge_20261018 import arms, check_dispatch, run  # noqa: E402


def copy_once(source, destination):
    blob = source.read_bytes()
    if destination.exists() and destination.read_bytes() != blob:
        raise ValueError(f'upstream sealed evidence differs: {destination}')
    destination.write_bytes(blob)


def write_once(path, value):
    blob = json.dumps(value, indent=2, sort_keys=True) + '\n'
    if path.exists() and path.read_text() != blob:
        raise ValueError(f'sealed bridge record differs: {path}')
    path.write_text(blob)


def main():
    parser = argparse.ArgumentParser()
    for name in ('extended_validated', 'extended_selection',
                 'root4_validated', 'root4_selection'):
        parser.add_argument('--' + name.replace('_', '-'), type=Path,
                            required=True)
    args = parser.parse_args()
    for name in ('extended_validated', 'extended_selection',
                 'root4_validated', 'root4_selection'):
        copy_once(getattr(args, name).resolve(), HERE / f'{name}.json')
    chosen, choice = arms.choose()
    if choice['status'] != 'TWO_DISTINCT_ADMITTED':
        write_once(HERE / 'outcome.json', {
            'schema': 'n83.global-bridge-outcome/v1',
            'status': choice['status'], 'selected_candidate_id':
            (choice['ordered_candidate_ids'][0] if
             choice['status'] == 'CONVERGED_IDENTICAL' else None),
            'choice': choice})
        print(json.dumps({'status': choice['status']}, sort_keys=True))
        return
    paths = check_dispatch.required_paths(chosen)
    hashes = {path: check_dispatch.digest(ROOT / path) for path in sorted(paths)}
    dispatch = {
        'schema': 'n83.global-bridge-dispatch/v1',
        'status': 'SEALED_FOR_DISPATCH',
        'point_count': 16, 'host_strata': 2, 'points_per_host': 8,
        'producer_cap_seconds': run.GLOBAL_PRODUCER_CAP_SECONDS,
        'job_cap_minutes': 330, 'source_sha256': run.SOURCE_SHA256,
        'targets_sha256': run.TARGETS_SHA256,
        'answers_sha256': run.ANSWERS_SHA256,
        'ordered_candidate_ids': [item[3] for item in chosen],
        'choice': choice, 'sha256': hashes,
    }
    write_once(HERE / 'dispatch.json', dispatch)
    check_dispatch.main()


if __name__ == '__main__':
    main()
