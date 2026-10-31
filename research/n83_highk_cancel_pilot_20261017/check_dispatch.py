#!/usr/bin/env python3
"""Fail closed on the sealed high-K factorial dispatch and its exact inputs."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    import run_fresh as run
    import verify_fresh as verify

    dispatch = json.loads((HERE / 'dispatch.json').read_text())
    arms, _ = run.choose()
    if (dispatch['schema'] != 'n83.highk-cancel-factorial-dispatch/v1' or
            dispatch['point_count'] != 24 or
            dispatch['host_strata'] != 4 or
            dispatch['points_per_host'] != 6 or
            dispatch['producer_cap_seconds'] != run.GLOBAL_PRODUCER_CAP_SECONDS or
            dispatch['job_cap_minutes'] != 350 or
            dispatch['status'] != 'SEALED_FOR_DISPATCH' or
            dispatch['source_sha256'] != run.SOURCE_SHA256 or
            dispatch['targets_sha256'] != run.TARGETS_SHA256 or
            dispatch['answers_sha256'] != run.ANSWERS_SHA256 or
            dispatch['ordered_candidate_ids'] != [arm[3] for arm in arms] or
            any(digest(ROOT / path) != expected
                for path, expected in dispatch['sha256'].items())):
        raise ValueError('dispatch source, fixture, candidate or code differs')
    _, points, scalars = verify.fixture_points()
    if (len(points) != 24 or len(set(points)) != 24 or
            any(not 0 < value < verify.group.R for value in scalars)):
        raise ValueError('sealed subgroup fixture differs')
    for name, k, layout, candidate_id in arms:
        manifest = json.loads((HERE / f'candidate_{candidate_id}.json').read_text())
        stripped = {key: value for key, value in manifest.items()
                    if key != 'candidate_id'}
        calculated = hashlib.sha256(json.dumps(
            stripped, sort_keys=True, separators=(',', ':'),
            ensure_ascii=False).encode()).hexdigest()[:12]
        if (not candidate_id.endswith('h' + calculated) or
                manifest['candidate_id'] != candidate_id or
                manifest['point_decomposition']['m'] != 4 or
                manifest['factor_base']['actual_usable_points'] != 166 * k or
                manifest['factor_base']['effective_columns'] != k or
                manifest['implementation']['source_sha256']['ic'] !=
                run.SOURCE_SHA256 or
                manifest['implementation']['algorithm_flags']['target_threads'] != 2 or
                manifest['implementation']['algorithm_flags']['target_cancel']
                is not name.endswith('_cancel') or layout != 'root11_dynamic'):
            raise ValueError(f'{name}: canonical candidate differs')
    for stratum in range(4):
        events = run.schedule(stratum, arms)
        if (len(events) != 192 or
                {item['point_index'] for item in events} !=
                set(range(stratum * 6, stratum * 6 + 6)) or
                {item['kind'] for item in events} !=
                {'ic', 'rho', 'calibration'}):
            raise ValueError(f'stratum {stratum}: event schedule differs')
    print(json.dumps({'status': 'PASS_SEALED_DISPATCH',
                      'candidate_ids': dispatch['ordered_candidate_ids'],
                      'targets_sha256': run.TARGETS_SHA256}, sort_keys=True))


if __name__ == '__main__':
    main()
