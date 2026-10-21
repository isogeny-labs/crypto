#!/usr/bin/env python3
"""Fail closed on the adaptive N83 extended finalist dispatch."""

import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
if sys.path and Path(sys.path[0]).resolve() == HERE:
    sys.path.pop(0)
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from research.n83_fresh_cold_finalists_20261019 import arms, run, verify  # noqa: E402


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def required_paths(chosen):
    return {
        '.github/workflows/n83-fresh-cold-finalists.yml', 'Cargo.lock',
        'examples/koblitz_orbit_dlp_fast.rs',
        'examples/n83_rho_optimization_eval.rs',
        'research/n83_final_extended_20261018/pilots/k3300.tar.gz',
        *[f'research/n83_fresh_cold_finalists_20261019/{name}' for name in (
            'PROTOCOL.md', 'arms.py', 'run.py', 'verify.py', 'select.py',
            'aggregate.py', 'provider_jobs.py', 'materialize.py',
            'check_dispatch.py',
            'prepare_dispatch.py', 'verify_mid.py',
            'custody_missing.py', 'base_custody_missing_receipt.json',
            'base_custody_prior_three_receipt.json',
            'test_arms.py', 'test_preflight.py',
            'test_select.py', 'test_provider_jobs.py',
            'historical_finalist_targets.jsonl',
            'fresh_manifest.json', 'fresh_targets.jsonl', 'fresh_answers.jsonl',
            'fresh_exclusion_audit.json', 'generate_fresh.py', 'audit_fresh.py',
            'upstream_validated.json', 'upstream_selection.json')],
        *[str(arms.manifest(arm[3])[0].relative_to(ROOT)) for arm in chosen],
    }


def main():
    dispatch = json.loads((HERE / 'dispatch.json').read_text())
    paths = (HERE / 'upstream_validated.json', HERE / 'upstream_selection.json')
    chosen, choice = arms.choose(*paths)
    if (dispatch.get('schema') != 'n83.fresh-cold-finalists-dispatch/v1' or
            dispatch.get('point_count') != 16 or
            dispatch.get('host_strata') != 2 or
            dispatch.get('points_per_host') != 8 or
            dispatch.get('producer_cap_seconds') != run.GLOBAL_PRODUCER_CAP_SECONDS or
            dispatch.get('job_cap_minutes') != 360 or
            dispatch.get('status') != 'SEALED_FOR_DISPATCH' or
            dispatch.get('source_sha256') != run.SOURCE_SHA256 or
            dispatch.get('targets_sha256') != run.TARGETS_SHA256 or
            dispatch.get('answers_sha256') != run.ANSWERS_SHA256 or
            dispatch.get('ordered_candidate_ids') != [arm[3] for arm in chosen] or
            dispatch.get('choice') != choice or
            any(digest(ROOT / path) != expected
                for path, expected in dispatch.get('sha256', {}).items())):
        raise ValueError('dispatch source, fixture, choice or code differs')
    required = required_paths(chosen)
    if not required <= set(dispatch['sha256']):
        raise ValueError('dispatch omits a required sealed component')
    exclusion = json.loads((HERE / 'fresh_exclusion_audit.json').read_text())
    if (exclusion.get('schema') != 'n83.fresh-cold-finalists-exclusion/v1' or
            exclusion.get('fresh_targets_sha256') != run.TARGETS_SHA256 or
            exclusion.get('fresh_count') != 16 or
            exclusion.get('prior_unique_point_count', 0) < 31_488 or
            exclusion.get('prior_overlap_count') != 0 or
            exclusion.get('finalists_overlap_count') != 0 or
            exclusion.get('status') != 'PASS_ZERO_OVERLAP'):
        raise ValueError('fresh-point exclusion audit differs')
    _, points, scalars = verify.fixture_points()
    if (len(points) != 16 or len(set(points)) != 16 or
            any(not 0 < value < verify.group.R for value in scalars)):
        raise ValueError('sealed subgroup fixture differs')
    for name, k, layout, candidate_id in chosen:
        _, manifest = arms.manifest(candidate_id)
        flags = run.ic_environment(HERE / 'preflight', layout, name)
        if (manifest['point_decomposition']['m'] != 4 or
                manifest['factor_base']['actual_usable_points'] != 166 * k or
                manifest['factor_base']['effective_columns'] != k or
                flags['KIC_TARGET_THREADS'] != '2' or
                flags['KIC_TARGET_CANCEL'] != '1' or
                flags['KIC_INDEX_ROOT_BATCH'] != '1' or
                flags['KIC_INDEXED5_ID30'] != '0' or
                run.IC_CAP_SECONDS[name] !=
                (600 if k < 2000 else 1200 if k in (3300, 3450, 3500)
                 else 900)):
            raise ValueError(f'{name}: exact candidate flags or cap differ')
    for stratum in range(2):
        events = run.schedule(stratum, chosen)
        if (len(events) != 8 * len(chosen) * 8 or
                {item['point_index'] for item in events} !=
                set(range(stratum * 8, stratum * 8 + 8)) or
                {item['kind'] for item in events} !=
                {'ic', 'rho', 'calibration'}):
            raise ValueError(f'stratum {stratum}: event schedule differs')
    print(json.dumps({'status': 'PASS_SEALED_DISPATCH',
                      'candidate_ids': dispatch['ordered_candidate_ids'],
                      'targets_sha256': run.TARGETS_SHA256}, sort_keys=True))


if __name__ == '__main__':
    main()
