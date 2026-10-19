#!/usr/bin/env python3
"""Independent Tier 0 audit of strict-service rho worker A/B and A/A arms."""

import argparse
import base64
import binascii
from hashlib import sha256
import json
from math import ceil, exp, isfinite, log
from pathlib import Path
import random
import statistics

Q_COUNT = 32
EXPECTED_Q_SHA256 = 'f46ad43788b7f9241927904541cbeb03ea70a96ca95aa84d527a9b22ba8d12b8'
REPEATS = 4
BOOTSTRAP_SEED = 0x71CC20261017
PAIR_FIELDS = {
    'job_id', 'checkin_blake3_time_normalized', 'walkers_done',
    'walk_steps', 'distinguished_points', 'dead_trails',
    'units_completed', 'checkins', 'q_index', 'q_hex',
    'dp_bits', 'step_cap', 'checkin_every',
}
NOISE_POLICY = {
    'allowed_local_interrupt_sources': ['LOC', 'IWI', 'RES', 'CAL', 'TLB'],
    'max_local_interrupts_per_second': 5,
    'max_local_interrupts_fixed': 10,
    'max_cpu_pressure_usec': 1000,
    'max_involuntary_switches': 2,
    'max_major_faults': 0,
    'max_external_interrupts': 0,
}


def digest(path):
    return sha256(Path(path).read_bytes()).hexdigest()


def lines(path):
    return [json.loads(x) for x in path.read_text().splitlines() if x]


def interval(q_logs):
    randomizer = random.Random(BOOTSTRAP_SEED)
    draws = sorted(exp(statistics.median(q_logs[randomizer.randrange(Q_COUNT)]
                                          for _ in range(Q_COUNT)))
                   for _ in range(10000))
    return [draws[250], draws[9749]]


def check_noise(row):
    before, after = row['before'], row['after']
    elapsed = (after['monotonic_ns'] - before['monotonic_ns']) / 1_000_000_000
    problems = []
    if elapsed <= 0:
        problems.append('nonpositive counter interval')
    local_limit = NOISE_POLICY['max_local_interrupts_fixed'] + ceil(
        NOISE_POLICY['max_local_interrupts_per_second'] * max(elapsed, 0))
    local = external = 0
    for cpu, old_sources in before['interrupts_by_source'].items():
        new_sources = after['interrupts_by_source'][cpu]
        source_total = 0
        for source in set(old_sources) | set(new_sources):
            delta = new_sources.get(source, 0) - old_sources.get(source, 0)
            if delta < 0:
                problems.append('interrupt counter reset')
            source_total += delta
            if source in NOISE_POLICY['allowed_local_interrupt_sources']:
                local += delta
            else:
                external += delta
        if source_total != after['interrupts'][cpu] - before['interrupts'][cpu]:
            problems.append('interrupt source sum mismatch')
    if external > NOISE_POLICY['max_external_interrupts'] or local > local_limit:
        problems.append('interrupt envelope exceeded')
    for kind in ('some', 'full'):
        delta = after['cpu_pressure_total'].get(kind, 0) - before['cpu_pressure_total'].get(kind, 0)
        if delta < 0 or delta > NOISE_POLICY['max_cpu_pressure_usec']:
            problems.append('CPU pressure envelope exceeded')
    if (row['child_involuntary_switches'] > NOISE_POLICY['max_involuntary_switches'] or
            row['child_major_faults'] > NOISE_POLICY['max_major_faults']):
        problems.append('child scheduling or fault envelope exceeded')
    return problems


def audit(folder, strict_runner):
    manifest_path = folder / 'manifest.json'
    manifest = json.loads(manifest_path.read_text())
    plan = json.loads(Path(manifest['worker_plan']).read_text())
    preflight = json.loads((folder / 'preflight.json').read_text())
    summary = json.loads((folder / 'summary.json').read_text())
    runs = lines(folder / 'runs.jsonl')
    pairs = lines(folder / 'pairs.jsonl')
    issues = []
    bad_arms = []
    bad_pairs = []
    aa_failures = []
    memory_failures = []
    comparison = {i: [] for i in range(Q_COUNT)}
    phases = {'serial': [], 'batch': []}
    if (manifest.get('measurement_tier') != 'tier0_physical_host' or
            manifest.get('repetitions') != REPEATS or len(manifest.get('cases', [])) != 96 or
            set(manifest.get('pair_fields', [])) != PAIR_FIELDS or
            plan['workload']['walkers_requested'] != 128 or
            plan['workload']['checkin_every'] != 64 or
            plan['workload']['batch_width'] != 32 or
            plan['workload'].get('direct_strict_output') is not True or
            manifest.get('noise_gate') != NOISE_POLICY):
        issues.append('frozen plan shape mismatch')
    host = preflight.get('evidence', {})
    if not preflight.get('ok') or host.get('virtualization') != 'none':
        issues.append('physical-host preflight did not pass')
    if (host.get('cpuset.cpus.partition') != 'isolated' or
            host.get('irq_affinity_overlap') or
            host.get('execution_cpu') != manifest['isolation']['execution_cpu']):
        issues.append('exclusive CPU, IRQ, or execution-CPU receipt mismatch')
    if (plan.get('fixture_manifest_sha256') != EXPECTED_Q_SHA256 or
            digest(plan['artifacts'][1]) != EXPECTED_Q_SHA256):
        issues.append('frozen Q manifest mismatch')
    if (summary.get('status') != 'completed' or not summary.get('artifacts_stable') or
            summary.get('manifest_sha256') != digest(manifest_path) or
            summary.get('runner_sha256') != digest(strict_runner)):
        issues.append('strict runner summary mismatch')
    for path in manifest.get('artifacts', []):
        try:
            stable = summary.get('artifact_sha256', {}).get(path) == digest(path)
        except OSError:
            stable = False
        if not stable:
            issues.append('artifact changed or missing: ' + path)
    if len(runs) != 768 or len(pairs) != 384:
        issues.append('incomplete arm or pair count')
    if [case['id'] for case in manifest['cases']] != [case['id'] for case in plan['cases']]:
        issues.append('case order changed')
    by_key = {}
    for serial, row in enumerate(runs, 1):
        arm_issues = []
        key = (row.get('repeat'), row.get('case'), row.get('variant'))
        if key in by_key:
            arm_issues.append('duplicate arm')
        by_key[key] = row
        if row.get('serial') != serial or row.get('status') != 'valid' or row.get('issues'):
            arm_issues.append('strict runner marked arm invalid')
        try:
            arm_issues.extend(check_noise(row))
        except (KeyError, TypeError, ValueError):
            arm_issues.append('noise counters incomplete')
        if not row.get('postflight', {}).get('ok'):
            arm_issues.append('postflight failed')
        try:
            case = next(x for x in plan['cases'] if x['id'] == row['case'])
            variant = row['variant']
            manifest_case = next(x for x in manifest['cases'] if x['id'] == row['case'])
            if row['command'] != manifest_case[variant] or row['command'] != case[variant]:
                arm_issues.append('command differs from frozen plan')
            raw_line = (folder / row['stdout_file']).read_text().strip()
            parsed = dict(word.split('=', 1) for word in raw_line.split())
            if parsed != row['fields'] or (folder / row['stderr_file']).read_text():
                arm_issues.append('raw stdout or stderr mismatch')
            fields = row['fields']
            raw = json.loads(base64.urlsafe_b64decode(fields['raw_b64']))
            if (raw['schema'] != 'rho-worker-batch-benchmark-v1' or
                    raw['status'] != 'complete_frozen_work' or
                    raw['verified_distinguished_points'] != raw['distinguished_points'] or
                    raw['q_index'] != case['q_index'] or raw['q_hex'] != case['q_hex'] or
                    raw['mode'] != case[variant + '_mode'] or
                    raw['batch_width'] != case[variant + '_width'] or
                    raw['walkers_requested'] != 128 or raw['dp_bits'] != 12 or
                    raw['step_cap'] != 8192 or raw['checkin_every'] != 64 or
                    raw['total_charged_ns'] != raw['setup_ns'] + raw['walk_ns']):
                arm_issues.append('worker correctness or phase receipt mismatch')
            if any(str(raw[name]) != fields[name] for name in PAIR_FIELDS):
                arm_issues.append('wrapper field mismatch')
            if fields['verified'] != '1' or abs(float(fields['online_ms']) - raw['total_charged_ns']/1e6) > 1e-6:
                arm_issues.append('charged online metric mismatch')
            if int(fields['peak_rss_kib']) != raw['peak_rss_kib']:
                arm_issues.append('peak RSS mismatch')
            if not arm_issues:
                phases[raw['mode']].append(raw)
        except (OSError, KeyError, ValueError, TypeError, StopIteration, json.JSONDecodeError, binascii.Error):
            arm_issues.append('incomplete arm evidence')
        if arm_issues:
            bad_arms.append({'serial': serial, 'issues': arm_issues})
    for position, case in enumerate(case for _ in range(REPEATS) for case in plan['cases']):
        repeat = position // 96
        pair = pairs[position] if position < len(pairs) else None
        a = by_key.get((repeat, case['id'], 'reference'))
        b = by_key.get((repeat, case['id'], 'candidate'))
        pair_issues = []
        if a is None or b is None or pair is None:
            pair_issues.append('missing pair or arm')
        else:
            for name in PAIR_FIELDS:
                if a['fields'].get(name) != b['fields'].get(name):
                    pair_issues.append('exact field mismatch: ' + name)
            ratio = float(a['fields']['online_ms']) / float(b['fields']['online_ms'])
            recorded_ratio = pair.get('speedup')
            if (not isfinite(ratio) or ratio <= 0 or
                    not isinstance(recorded_ratio, (int, float)) or
                    abs(ratio - recorded_ratio) > 1e-6):
                pair_issues.append('ratio mismatch')
            if (pair['case'] != case['id'] or pair['repeat'] != repeat or
                    pair['status'] != 'valid' or pair['issues'] or
                    pair['run_serials'] != {'reference': a['serial'], 'candidate': b['serial']}):
                pair_issues.append('pair receipt mismatch')
            if not pair_issues and case['kind'] == 'comparison':
                baseline = int(a['fields']['peak_rss_kib'])
                candidate = int(b['fields']['peak_rss_kib'])
                gate = plan['claim_gate']
                if (candidate > baseline * gate['max_relative_peak_rss_ratio'] or
                        candidate - baseline > gate['max_absolute_peak_rss_delta_kib']):
                    memory_failures.append({'case': case['id'], 'repeat': repeat,
                                            'serial_kib': baseline, 'batch_kib': candidate})
                comparison[case['q_index']].append(ratio)
            elif not pair_issues and case['kind'].startswith('aa_'):
                if not plan['claim_gate']['aa_ratio_min'] <= ratio <= plan['claim_gate']['aa_ratio_max']:
                    aa_failures.append({'case': case['id'], 'repeat': repeat, 'ratio': ratio})
        if pair_issues:
            bad_pairs.append({'case': case['id'], 'repeat': repeat, 'issues': pair_issues})
    q_logs = [statistics.median(log(x) for x in comparison[i])
              for i in range(Q_COUNT) if len(comparison[i]) == REPEATS]
    gate = not (issues or bad_arms or bad_pairs or aa_failures or memory_failures) and len(q_logs) == Q_COUNT
    bounds = interval(q_logs) if gate else None
    return {'schema': 'rho-worker-tier0-audit-v1',
            'gate_passed': gate,
            'decision': 'qualified_5pct_improvement' if gate and bounds[0] > 1.05 else
                        'qualified_below_5pct_gate' if gate else 'unknown_gate_failed',
            'q_cluster_bootstrap95': bounds,
            'paired_ratio_descriptive': exp(statistics.median(q_logs)) if gate else None,
            'completed_arms': len(runs), 'completed_pairs': len(pairs),
            'structural_issues': issues, 'invalid_arms': bad_arms,
            'invalid_pairs': bad_pairs, 'aa_failures': aa_failures,
            'memory_failures': memory_failures,
            'median_phase_and_rss': {
                mode: {name: statistics.median(row[name] for row in values)
                       for name in ('setup_ns', 'walk_ns', 'total_charged_ns', 'peak_rss_kib')}
                for mode, values in phases.items() if values},
            'raw_manifest_sha256': digest(manifest_path),
            'raw_summary_sha256': digest(folder / 'summary.json')}


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--result-dir', type=Path, required=True)
    p.add_argument('--strict-runner', type=Path, required=True)
    p.add_argument('--output', type=Path, required=True)
    a = p.parse_args()
    result = audit(a.result_dir, a.strict_runner)
    a.output.write_text(json.dumps(result, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'decision': result['decision'],
                      'completed_pairs': result['completed_pairs']}))


if __name__ == '__main__':
    main()
