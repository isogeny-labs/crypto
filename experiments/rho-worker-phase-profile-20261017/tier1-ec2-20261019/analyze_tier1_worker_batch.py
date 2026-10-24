#!/usr/bin/env python3
"""Independent exact-output and Tier 1 audit of the full rho worker panel."""

import argparse
from hashlib import sha256
import json
from math import exp, isfinite, log
from pathlib import Path
import random
import statistics


EXPECTED_Q_SHA256 = '546fe0b842a2f559395c9ac5d74ced12b96ce366e41322387eb0baec5dcb3adc'
REPETITIONS = 4
Q_COUNT = 32
BOOTSTRAP_SEED = 0x71CC20261017


def digest(path):
    try:
        return sha256(Path(path).read_bytes()).hexdigest()
    except OSError:
        return None


def rows(path):
    try:
        return [json.loads(line) for line in path.read_text().splitlines() if line]
    except OSError:
        return []


def interval(by_q):
    if len(by_q) != Q_COUNT:
        return None
    rng = random.Random(BOOTSTRAP_SEED)
    draws = sorted(exp(statistics.median(by_q[rng.randrange(Q_COUNT)]
                                          for _ in range(Q_COUNT)))
                   for _ in range(10000))
    return [draws[250], draws[9749]]


def audit(folder):
    manifest_path = folder / 'manifest.json'
    manifest = json.loads(manifest_path.read_text())
    preflight = json.loads((folder / 'preflight.json').read_text())
    summary = json.loads((folder / 'summary.json').read_text())
    runs = rows(folder / 'runs.jsonl')
    pairs = rows(folder / 'pairs.jsonl')
    cases = {case['id']: case for case in manifest['cases']}
    structural = []
    invalid_arms = []
    invalid_pairs = []
    aa_failures = []
    memory_failures = []
    comparison = {i: [] for i in range(Q_COUNT)}
    phases = {'serial': [], 'batch': []}

    if manifest.get('fixture_manifest_sha256') != EXPECTED_Q_SHA256:
        structural.append('fixture identity mismatch')
    q_manifest_path = Path(manifest['artifacts'][1])
    if digest(q_manifest_path) != EXPECTED_Q_SHA256:
        structural.append('Q fixture artifact mismatch')
    else:
        fixtures = json.loads(q_manifest_path.read_text())['fixtures']
        if len(fixtures) != Q_COUNT:
            structural.append('Q fixture count mismatch')
        else:
            for case in cases.values():
                if case['q_hex'] != fixtures[case['q_index']]['q_hex']:
                    structural.append('case Q mismatch: ' + case['id'])
    if summary.get('manifest_sha256') != digest(manifest_path):
        structural.append('manifest receipt mismatch')
    queue_root = summary.get('queue_root')
    lock_path = summary.get('lock_path')
    if (summary.get('lock_scope') != 'shared_queue_root_mount' or
            not isinstance(queue_root, str) or not isinstance(lock_path, str) or
            lock_path != str(Path(queue_root) / '.rho-worker-panel.lock')):
        structural.append('shared queue lock receipt mismatch')
    runner = Path(__file__).with_name('run_tier1_worker_batch.py')
    if summary.get('runner_sha256') != digest(runner):
        structural.append('runner source mismatch')
    if not summary.get('artifacts_stable') or not all(
            summary.get('artifact_sha256', {}).get(path) == digest(path)
            for path in manifest['artifacts']):
        structural.append('artifact hashes changed or missing')
    if len(cases) != 96 or manifest.get('repetitions') != REPETITIONS:
        structural.append('frozen panel size mismatch')
    if set(manifest.get('pair_fields', [])) != {
            'job_id', 'checkin_blake3_time_normalized', 'walkers_done',
            'walk_steps', 'distinguished_points', 'dead_trails',
            'units_completed', 'checkins'}:
        structural.append('exact-output field set changed')
    if not preflight.get('ok'):
        structural.append('preflight failed')
    if len(runs) != 768 or len(pairs) != 384:
        structural.append('arm or pair count incomplete')

    lookup = {}
    binary_hash = summary.get('artifact_sha256', {}).get(manifest['artifacts'][0])
    for serial, row in enumerate(runs, 1):
        issues = []
        key = (row.get('repeat'), row.get('case'), row.get('variant'))
        if key in lookup:
            issues.append('duplicate arm')
        lookup[key] = row
        case = cases.get(row.get('case'))
        variant = row.get('variant')
        if row.get('serial') != serial or case is None or variant not in ('reference', 'candidate'):
            issues.append('arm identity mismatch')
        elif row.get('command') != case[variant]:
            issues.append('arm command mismatch')
        if row.get('binary_sha256') != binary_hash:
            issues.append('binary hash mismatch')
        if row.get('status') != 'valid' or row.get('issues') or row.get('returncode') != 0:
            issues.append('runner marked arm invalid')
        if not row.get('postflight', {}).get('ok') or 'counter_deltas' not in row.get(
                'tier1_observations', {}):
            issues.append('host receipt missing')
        delta = row.get('tier1_observations', {}).get('counter_deltas', {})
        if (delta.get('cgroup_v1_cpu_stat', {}).get('nr_throttled', 0) or
                delta.get('cgroup_v2_cpu_stat', {}).get('nr_throttled', 0) or
                delta.get('cgroup_v2_memory_events', {}).get('oom', 0) or
                delta.get('cgroup_v2_memory_events', {}).get('oom_kill', 0) or
                (len(delta.get('cpu_times', [])) <= 7 or delta['cpu_times'][7]) or
                (delta.get('cgroup_v1_memory_failcnt') or 0) > 0 or
                (delta.get('thermal_throttle') or 0) > 0 or
                (row.get('child_major_faults') or 0) > 0):
            issues.append('host noise or memory event')
        try:
            raw = (folder / row['stdout_file']).read_text()
            fields = json.loads(raw)
            stderr = (folder / row['stderr_file']).read_text()
            if fields != row.get('fields') or stderr:
                issues.append('stdout or stderr receipt mismatch')
            if fields['schema'] != 'rho-worker-batch-benchmark-v1' or fields[
                    'status'] != 'complete_frozen_work':
                issues.append('worker status mismatch')
            if fields['verified_distinguished_points'] != fields['distinguished_points']:
                issues.append('unverified DP')
            if case is not None and (fields['q_index'] != case['q_index'] or
                                     fields['q_hex'] != case['q_hex'] or
                                     fields['mode'] != case[variant + '_mode'] or
                                     fields['batch_width'] != case[variant + '_width']):
                issues.append('wrong frozen input or mode')
            total = int(fields['total_charged_ns'])
            setup = int(fields['setup_ns'])
            walk = int(fields['walk_ns'])
            memory = int(fields['peak_rss_kib'])
            if total <= 0 or setup <= 0 or walk <= 0 or total != setup + walk or memory <= 0:
                issues.append('phase or memory accounting invalid')
            if row.get('total_charged_ns') != total:
                issues.append('runner total mismatch')
            if not issues:
                phases[fields['mode']].append({
                    'setup_ns': setup, 'walk_ns': walk,
                    'total_charged_ns': total, 'peak_rss_kib': memory,
                })
        except (OSError, KeyError, TypeError, ValueError, json.JSONDecodeError):
            issues.append('arm evidence incomplete')
        if issues:
            invalid_arms.append({'serial': serial, 'issues': issues})

    ordered = []
    for position, case in enumerate(
            case for _ in range(REPETITIONS) for case in manifest['cases']):
        repeat = position // len(cases)
        pair = pairs[position] if position < len(pairs) else None
        arms = {variant: lookup.get((repeat, case['id'], variant))
                for variant in ('reference', 'candidate')}
        first = 'reference' if (repeat + position % len(cases)) % 2 == 0 else 'candidate'
        ordered.extend(arms[v]['serial'] for v in (first,
                       'candidate' if first == 'reference' else 'reference')
                       if arms[v] is not None)
        issues = []
        if any(arm is None or arm['status'] != 'valid' for arm in arms.values()):
            issues.append('invalid or absent arm')
        ratio = None
        if not issues:
            try:
                reference = arms['reference']['fields']
                candidate = arms['candidate']['fields']
                for field in manifest['pair_fields']:
                    if reference[field] != candidate[field]:
                        issues.append('exact-output mismatch: ' + field)
                ratio = (arms['reference']['total_charged_ns'] /
                         arms['candidate']['total_charged_ns'])
                if not isfinite(ratio) or ratio <= 0:
                    issues.append('invalid ratio')
                if case['kind'] == 'comparison':
                    rss_base = reference['peak_rss_kib']
                    rss_candidate = candidate['peak_rss_kib']
                    gate = manifest['claim_gate']
                    if (rss_candidate > rss_base * gate['max_relative_peak_rss_ratio'] or
                            rss_candidate - rss_base > gate['max_absolute_peak_rss_delta_kib']):
                        memory_failures.append({'case': case['id'], 'repeat': repeat,
                                                'serial_kib': rss_base,
                                                'batch_kib': rss_candidate})
            except (KeyError, TypeError, ValueError, ZeroDivisionError):
                issues.append('pair accounting incomplete')
        if pair is None or pair.get('case') != case['id'] or pair.get('repeat') != repeat or pair.get(
                'kind') != case['kind'] or pair.get('status') != 'valid' or pair.get('issues'):
            issues.append('pair receipt mismatch')
        if not issues:
            if pair.get('run_serials') != {v: arm['serial'] for v, arm in arms.items()}:
                issues.append('pair arm references mismatch')
            if not isinstance(pair.get('speedup'), (int, float)) or abs(
                    pair['speedup'] - ratio) > 1e-9:
                issues.append('pair ratio mismatch')
        if issues:
            invalid_pairs.append({'case': case['id'], 'repeat': repeat, 'issues': issues})
        elif case['kind'].startswith('aa_'):
            gate = manifest['claim_gate']
            if not gate['aa_ratio_min'] <= ratio <= gate['aa_ratio_max']:
                aa_failures.append({'case': case['id'], 'repeat': repeat, 'ratio': ratio})
        else:
            comparison[case['q_index']].append(ratio)
    if ordered != list(range(1, len(runs) + 1)):
        structural.append('arm execution order mismatch')

    q_logs = [statistics.median(log(ratio) for ratio in comparison[i])
              for i in range(Q_COUNT) if len(comparison[i]) == REPETITIONS]
    gate_passed = bool(preflight.get('ok') and summary.get('status') == 'completed'
                       and not structural and not invalid_arms and not invalid_pairs
                       and not aa_failures and not memory_failures
                       and len(q_logs) == Q_COUNT)
    bounds = interval(q_logs) if gate_passed else None
    result = {
        'schema': 'rho-worker-batch-audit-v1',
        'measurement_tier': 'tier1_instance',
        'gate_passed': gate_passed,
        'decision': ('qualified_5pct_improvement'
                     if gate_passed and bounds and
                     bounds[0] > manifest['claim_gate']['comparison_lower_95_gt']
                     else 'qualified_below_5pct_gate' if gate_passed
                     else 'unknown_gate_failed'),
        'paired_ratio_descriptive': (exp(statistics.median(q_logs))
                                     if gate_passed else None),
        'q_cluster_bootstrap95': bounds,
        'valid_q_clusters': len(q_logs),
        'structural_issues': structural,
        'invalid_arms': invalid_arms,
        'invalid_pairs': invalid_pairs,
        'aa_failures': aa_failures,
        'memory_failures': memory_failures,
        'median_phase_and_rss': {
            mode: {key: statistics.median(row[key] for row in values)
                   for key in ('setup_ns', 'walk_ns', 'total_charged_ns', 'peak_rss_kib')}
            for mode, values in phases.items() if values
        },
        'raw_manifest_sha256': digest(manifest_path),
        'raw_summary_sha256': digest(folder / 'summary.json'),
    }
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--result-dir', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.result_dir)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'decision': result['decision'],
                      'valid_q_clusters': result['valid_q_clusters'],
                      'output': str(args.output)}))


if __name__ == '__main__':
    main()
