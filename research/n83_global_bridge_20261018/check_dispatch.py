#!/usr/bin/env python3
"""Fail closed on the selected-finalist N83 bridge dispatch."""

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

from research.n83_global_bridge_20261018 import arms, run, verify  # noqa: E402


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def required_paths(chosen):
    own = ('PROTOCOL.md', 'arms.py', 'run.py', 'verify.py', 'select.py',
           'aggregate.py', 'provider_jobs.py', 'materialize.py',
           'check_dispatch.py', 'prepare_dispatch.py', 'test_arms.py',
           'test_select.py', 'test_provider_jobs.py', 'fresh_manifest.json',
           'fresh_targets.jsonl', 'fresh_answers.jsonl',
           'fresh_exclusion_audit.json', 'generate_fresh.py', 'audit_fresh.py',
           'extended_validated.json', 'extended_selection.json',
           'root4_validated.json', 'root4_selection.json')
    return {'.github/workflows/n83-global-bridge.yml', 'Cargo.lock',
            'examples/koblitz_orbit_dlp_fast.rs',
            'examples/n83_rho_optimization_eval.rs',
            'research/n83_final_extended_20261018/dispatch.json',
            'research/n83_root4_frontier_current_20261018/dispatch.json',
            *[f'research/n83_global_bridge_20261018/{name}' for name in own],
            *[str(arms.manifest(arm)[0].relative_to(ROOT)) for arm in chosen]}


def main():
    dispatch = json.loads((HERE / 'dispatch.json').read_text())
    chosen, choice = arms.choose()
    if (choice['status'] != 'TWO_DISTINCT_ADMITTED' or
            dispatch.get('schema') != 'n83.global-bridge-dispatch/v1' or
            dispatch.get('status') != 'SEALED_FOR_DISPATCH' or
            dispatch.get('point_count') != 16 or
            dispatch.get('host_strata') != 2 or
            dispatch.get('points_per_host') != 8 or
            dispatch.get('producer_cap_seconds') != run.GLOBAL_PRODUCER_CAP_SECONDS or
            dispatch.get('job_cap_minutes') != 330 or
            dispatch.get('source_sha256') != run.SOURCE_SHA256 or
            dispatch.get('targets_sha256') != run.TARGETS_SHA256 or
            dispatch.get('answers_sha256') != run.ANSWERS_SHA256 or
            dispatch.get('ordered_candidate_ids') != [arm[3] for arm in chosen] or
            dispatch.get('choice') != choice or
            not isinstance(dispatch.get('sha256'), dict) or
            any(digest(ROOT / path) != expected
                for path, expected in dispatch['sha256'].items())):
        raise ValueError('bridge dispatch source, choice or code differs')
    if not required_paths(chosen) <= set(dispatch['sha256']):
        raise ValueError('bridge dispatch omits a required sealed component')
    exclusion = json.loads((HERE / 'fresh_exclusion_audit.json').read_text())
    if (exclusion.get('schema') != 'n83.global-bridge-exclusion/v1' or
            exclusion.get('fresh_targets_sha256') != run.TARGETS_SHA256 or
            exclusion.get('fresh_count') != 16 or
            exclusion.get('prior_unique_point_count', 0) < 31520 or
            exclusion.get('prior_overlap_count') != 0 or
            exclusion.get('finalists_overlap_count') != 0 or
            exclusion.get('status') != 'PASS_ZERO_OVERLAP'):
        raise ValueError('bridge point exclusion audit differs')
    _, points, scalars = verify.fixture_points()
    if (len(points) != 16 or len(set(points)) != 16 or
            any(not 0 < value < verify.group.R for value in scalars)):
        raise ValueError('bridge subgroup fixture differs')
    for name, k, layout, candidate_id in chosen:
        _, manifest = arms.manifest((name, k, layout, candidate_id))
        flags = run.ic_environment(HERE / 'preflight', layout, name)
        if (manifest['point_decomposition']['m'] != 4 or
                manifest['factor_base']['actual_usable_points'] != 166 * k or
                manifest['factor_base']['effective_columns'] != k or
                flags['KIC_TARGET_THREADS'] != '2' or
                flags['KIC_TARGET_CANCEL'] != '1' or
                flags['KIC_INDEX_ROOT_BATCH'] != '1' or
                flags['KIC_INDEXED5_ID30'] != '0' or
                run.IC_CAP_SECONDS[name] != 1200):
            raise ValueError(f'{name}: exact bridge candidate flags differ')
    for stratum in range(2):
        events = run.schedule(stratum, chosen)
        if (len(events) != 128 or
                {item['point_index'] for item in events} !=
                set(range(stratum * 8, stratum * 8 + 8)) or
                {item['kind'] for item in events} !=
                {'ic', 'rho', 'calibration'}):
            raise ValueError(f'stratum {stratum}: bridge schedule differs')
    print(json.dumps({'status': 'PASS_SEALED_DISPATCH',
                      'candidate_ids': dispatch['ordered_candidate_ids'],
                      'targets_sha256': run.TARGETS_SHA256}, sort_keys=True))


if __name__ == '__main__':
    main()
