#!/usr/bin/env python3
"""Exercise the full Tier 1 auditor with synthetic timing and host receipts.

This checks policy behavior only. Generated rows are never measurements.
"""

import argparse
import copy
from hashlib import sha256
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

from analyze_tier1_worker_batch import audit


HERE = Path(__file__).resolve().parent


def digest(path):
    return sha256(Path(path).read_bytes()).hexdigest()


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')


def write_rows(path, values):
    path.write_text(''.join(json.dumps(value, sort_keys=True) + '\n'
                            for value in values))


def fixture_outputs():
    source = HERE / 'local-all32-long-worker-replay/arms.jsonl'
    outputs = {}
    for line in source.read_text().splitlines():
        arm = json.loads(line)
        outputs[(arm['q_index'], arm['mode'])] = arm['output']
    if len(outputs) != 64:
        raise ValueError('long correctness fixture is incomplete')
    return outputs


def make_complete(folder, plan, outputs):
    folder.mkdir()
    write_json(folder / 'manifest.json', plan)
    write_json(folder / 'preflight.json', {'ok': True, 'synthetic': True})
    runs = []
    pairs = []
    for repeat in range(plan['repetitions']):
        for index, case in enumerate(plan['cases']):
            first = 'reference' if (repeat + index) % 2 == 0 else 'candidate'
            order = (first, 'candidate' if first == 'reference' else 'reference')
            outcomes = {}
            for variant in order:
                serial = len(runs) + 1
                mode = case[variant + '_mode']
                fields = copy.deepcopy(outputs[(case['q_index'], mode)])
                total = (110_000_000 if variant == 'reference' else 100_000_000)
                if case['kind'] != 'comparison':
                    total = 100_000_000
                fields.update(setup_ns=1_000_000, walk_ns=total - 1_000_000,
                              total_charged_ns=total, peak_rss_kib=100_000)
                stem = f'{serial:05d}'
                stdout_file = f'{stem}.stdout.txt'
                stderr_file = f'{stem}.stderr.txt'
                write_json(folder / stdout_file, fields)
                (folder / stderr_file).write_text('')
                row = {
                    'serial': serial, 'case': case['id'], 'repeat': repeat,
                    'variant': variant, 'status': 'valid', 'issues': [],
                    'returncode': 0, 'command': case[variant],
                    'binary_sha256': digest(case[variant][0]),
                    'postflight': {'ok': True},
                    'tier1_observations': {'counter_deltas': {
                        'cgroup_v1_cpu_stat': {'nr_throttled': 0},
                        'cgroup_v2_cpu_stat': {'nr_throttled': 0},
                        'cgroup_v2_memory_events': {'oom': 0, 'oom_kill': 0},
                        'cpu_times': [0] * 8,
                        'cgroup_v1_memory_failcnt': 0,
                        'thermal_throttle': 0,
                    }},
                    'child_major_faults': 0, 'fields': fields,
                    'total_charged_ns': total,
                    'stdout_file': stdout_file, 'stderr_file': stderr_file,
                }
                runs.append(row)
                outcomes[variant] = row
            pairs.append({
                'case': case['id'], 'repeat': repeat, 'kind': case['kind'],
                'status': 'valid', 'issues': [],
                'run_serials': {variant: outcomes[variant]['serial']
                                for variant in ('reference', 'candidate')},
                'speedup': outcomes['reference']['total_charged_ns'] /
                           outcomes['candidate']['total_charged_ns'],
            })
    write_rows(folder / 'runs.jsonl', runs)
    write_rows(folder / 'pairs.jsonl', pairs)
    write_json(folder / 'summary.json', {
        'status': 'completed',
        'queue_root': str(folder.parent),
        'lock_path': str(folder.parent / '.rho-worker-panel.lock'),
        'lock_scope': 'shared_queue_root_mount',
        'manifest_sha256': digest(folder / 'manifest.json'),
        'runner_sha256': digest(HERE / 'run_tier1_worker_batch.py'),
        'artifacts_stable': True,
        'artifact_sha256': {path: digest(path) for path in plan['artifacts']},
    })
    return runs, pairs


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--source-bundle', type=Path, required=True)
    parser.add_argument('--q-manifest', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    with tempfile.TemporaryDirectory(prefix='rho-worker-audit-policy-') as temp:
        root = Path(temp)
        allocation = root / 'allocation.json'
        write_json(allocation, {'synthetic': True})
        plan_path = root / 'plan.json'
        subprocess.run([
            sys.executable, str(HERE / 'build_worker_batch_plan.py'),
            '--binary', str(args.binary), '--source-bundle', str(args.source_bundle),
            '--q-manifest', str(args.q_manifest),
            '--allocation-receipt', str(allocation),
            '--allocation-id', 'synthetic-auditor-policy',
            '--execution-cpu', '0', '--out', str(plan_path),
        ], check=True, capture_output=True, text=True)
        plan = json.loads(plan_path.read_text())
        good = root / 'pass'
        runs, pairs = make_complete(good, plan, fixture_outputs())
        passed = audit(good)
        if (passed['decision'] != 'qualified_5pct_improvement' or
                passed['q_cluster_bootstrap95'] != [1.1, 1.1] or
                len(runs) != 768 or len(pairs) != 384):
            raise AssertionError('synthetic complete panel did not pass')

        bad_aa = root / 'bad_aa'
        shutil.copytree(good, bad_aa)
        arm = next(row for row in runs if row['case'] == 'q00-aa_serial'
                   and row['repeat'] == 0 and row['variant'] == 'candidate')
        arm['total_charged_ns'] = 125_000_000
        arm['fields']['total_charged_ns'] = 125_000_000
        arm['fields']['walk_ns'] = 124_000_000
        write_json(bad_aa / arm['stdout_file'], arm['fields'])
        pair = next(row for row in pairs if row['case'] == 'q00-aa_serial'
                    and row['repeat'] == 0)
        pair['speedup'] = 0.8
        write_rows(bad_aa / 'runs.jsonl', runs)
        write_rows(bad_aa / 'pairs.jsonl', pairs)
        aa_result = audit(bad_aa)
        if (aa_result['decision'] != 'unknown_gate_failed' or
                len(aa_result['aa_failures']) != 1 or
                aa_result['q_cluster_bootstrap95'] is not None or
                aa_result['paired_ratio_descriptive'] is not None):
            raise AssertionError('A/A failure did not suppress aggregate')

        missing = root / 'missing_stdout'
        shutil.copytree(good, missing)
        (missing / '00001.stdout.txt').unlink()
        missing_result = audit(missing)
        if (missing_result['decision'] != 'unknown_gate_failed' or
                not missing_result['invalid_arms'] or
                missing_result['q_cluster_bootstrap95'] is not None or
                missing_result['paired_ratio_descriptive'] is not None):
            raise AssertionError('missing raw output did not suppress aggregate')

        bad_lock = root / 'bad_lock'
        shutil.copytree(good, bad_lock)
        lock_summary = json.loads((bad_lock / 'summary.json').read_text())
        lock_summary['lock_path'] = '/run/lock/container-local.lock'
        write_json(bad_lock / 'summary.json', lock_summary)
        lock_result = audit(bad_lock)
        if (lock_result['decision'] != 'unknown_gate_failed' or
                'shared queue lock receipt mismatch' not in
                lock_result['structural_issues'] or
                lock_result['q_cluster_bootstrap95'] is not None):
            raise AssertionError('container-local lock receipt did not suppress aggregate')

    result = {
        'schema': 'rho-worker-auditor-policy-selfcheck-v1',
        'data_kind': 'synthetic_policy_validation_only',
        'fixture_arms_sha256': digest(HERE / 'local-all32-long-worker-replay/arms.jsonl'),
        'auditor_sha256': digest(HERE / 'analyze_tier1_worker_batch.py'),
        'runner_sha256': digest(HERE / 'run_tier1_worker_batch.py'),
        'planned_arms': 768,
        'planned_pairs': 384,
        'complete_panel': passed['decision'],
        'complete_panel_bootstrap95': passed['q_cluster_bootstrap95'],
        'one_aa_failure': aa_result['decision'],
        'missing_raw_stdout': missing_result['decision'],
        'container_local_lock': lock_result['decision'],
        'synthetic_performance_result': None,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    write_json(args.output, result)
    print(json.dumps({'status': 'pass', 'output': str(args.output),
                      'arms': result['planned_arms']}))


if __name__ == '__main__':
    main()
