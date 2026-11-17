#!/usr/bin/env python3
"""Fail closed on the sealed cross-family cancellation dispatch."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    import run
    import verify

    dispatch = json.loads((HERE / 'dispatch.json').read_text())
    arms, _ = run.choose()
    if (dispatch['schema'] != 'n83.crossfamily-cancel-dispatch/v1' or
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
    exclusion = json.loads((HERE / 'fresh_exclusion_audit.json').read_text())
    if (exclusion['schema'] != 'n83.crossfamily-cancel-exclusion/v1' or
            exclusion['fresh_targets_sha256'] != run.TARGETS_SHA256 or
            exclusion['fresh_count'] != 24 or
            exclusion['prior_unique_point_count'] < 31_400 or
            exclusion['prior_overlap_count'] != 0 or
            exclusion['finalists_point_count'] != 32 or
            exclusion['finalists_overlap_count'] != 0 or
            exclusion['status'] != 'PASS_ZERO_OVERLAP'):
        raise ValueError('fresh-point exclusion audit differs')
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
                not manifest['factor_base']['ordered_base_dump_sha256'] or
                not manifest['factor_base']['source_base_hash_blake3'] or
                manifest['implementation']['source_sha256']['ic'] !=
                run.SOURCE_SHA256 or
                manifest['implementation']['algorithm_flags']['target_threads'] != 2 or
                manifest['implementation']['algorithm_flags']['target_cancel']
                is not True or
                manifest['implementation']['algorithm_flags']['indexed5_id30_request']
                is not False or
                run.ic_environment(HERE / 'preflight', layout, name)['KIC_TARGET_CANCEL'] != '1'):
            raise ValueError(f'{name}: canonical candidate differs')
        flags = run.ic_environment(HERE / 'preflight', layout, name)
        if (flags['KIC_TARGET_THREADS'] != '2' or
                flags['KIC_INDEXED5_ID30'] != '0' or
                run.IC_CAP_SECONDS[name] !=
                (900 if k in (2816, 3072) else 1200) or
                (layout == 'root11_wide3') !=
                (flags['KIC_ROOT_BLOOM_PROFILE'] == 'wide3'
                 if 'KIC_ROOT_BLOOM_PROFILE' in flags else False) or
                (layout == 'root4_standard') !=
                (flags['KIC_INDEXED_ROOT4'] == '1')):
            raise ValueError(f'{name}: layout flags or caps differ')
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
