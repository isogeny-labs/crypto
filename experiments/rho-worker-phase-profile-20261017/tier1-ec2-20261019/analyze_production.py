#!/usr/bin/env python3
"""Audit the prospective 32-lane Tier 1 fresh-Q worker confirmation."""

import argparse
from hashlib import sha256
import json
from math import exp, log
from pathlib import Path
import statistics

import analyze_tier1_worker_batch as base


EXPECTED_PLAN_SHA256 = '3caf3cd9c5b4aab90a53bdbdc44c9fe7f9f72920c28f336004cce135c36fbcfc'
EXPECTED_Q_SHA256 = '546fe0b842a2f559395c9ac5d74ced12b96ce366e41322387eb0baec5dcb3adc'
base.EXPECTED_Q_SHA256 = EXPECTED_Q_SHA256


def audit(folder):
    manifest_path = folder / 'manifest.json'
    if sha256(manifest_path.read_bytes()).hexdigest() != EXPECTED_PLAN_SHA256:
        raise ValueError('production plan hash mismatch')
    plan = json.loads(manifest_path.read_text())
    core = base.audit(folder)
    pairs = base.rows(folder / 'pairs.jsonl')
    summary = json.loads((folder / 'summary.json').read_text())
    gate = plan['claim_gate']
    required = {
        'comparison_lower_95_gt': 1.05,
        'aa_ratio_min': 0.90, 'aa_ratio_max': 1.11,
        'aa_extreme_min': 0.80, 'aa_extreme_max': 1.25,
        'max_aa_outliers': 2,
        'aa_mode_median_min': 0.98, 'aa_mode_median_max': 1.02,
        'aa_q_mode_median_min': 0.95, 'aa_q_mode_median_max': 1.05,
        'max_relative_peak_rss_ratio': 1.10,
        'max_absolute_peak_rss_delta_kib': 65536,
    }
    issues = list(core['structural_issues'])
    if any(gate.get(key) != value for key, value in required.items()):
        issues.append('claim gate changed')
    if plan.get('measurement_tier') != 'tier1_instance':
        issues.append('wrong measurement tier')
    if plan.get('workload', {}).get('walkers_requested') != 128 or plan[
            'workload'].get('checkin_every') != 64 or plan['workload'].get('batch_width') != 32:
        issues.append('production workload changed')
    if summary.get('status') != 'completed' or summary.get('completed_arms') != 768 or summary[
            'completed_pairs'] != 384 or summary.get('valid_pairs') != 384:
        issues.append('incomplete run summary')
    cases = {case['id']: case for case in plan['cases']}
    aa = {'aa_serial': [], 'aa_batch': []}
    aa_q = {(q, mode): [] for q in range(32) for mode in aa}
    comparisons = {q: [] for q in range(32)}
    outliers = []
    extreme = []
    for pair in pairs:
        case = cases.get(pair.get('case'))
        if case is None or pair.get('status') != 'valid':
            issues.append('missing or invalid pair')
            continue
        ratio = pair.get('speedup')
        if not isinstance(ratio, (int, float)) or ratio <= 0:
            issues.append('invalid pair ratio')
            continue
        mode, q = case['kind'], case['q_index']
        if mode == 'comparison':
            comparisons[q].append(ratio)
        elif mode in aa:
            aa[mode].append(ratio)
            aa_q[(q, mode)].append(ratio)
            if not gate['aa_ratio_min'] <= ratio <= gate['aa_ratio_max']:
                outliers.append({'case': pair['case'], 'repeat': pair['repeat'], 'ratio': ratio})
            if not gate['aa_extreme_min'] <= ratio <= gate['aa_extreme_max']:
                extreme.append({'case': pair['case'], 'repeat': pair['repeat'], 'ratio': ratio})
        else:
            issues.append('unknown case kind')
    if (any(len(values) != 128 for values in aa.values()) or
            any(len(values) != 4 for values in aa_q.values()) or
            any(len(values) != 4 for values in comparisons.values())):
        issues.append('Q repetition count mismatch')
    if outliers != core['aa_failures']:
        issues.append('A/A outlier list differs from arm audit')
    mode_medians = {
        mode: exp(statistics.median(log(value) for value in values))
        for mode, values in aa.items() if values
    }
    q_mode_failures = [
        {'q_index': q, 'mode': mode, 'median_ratio': median}
        for (q, mode), values in aa_q.items() if values
        for median in [exp(statistics.median(log(value) for value in values))]
        if not gate['aa_q_mode_median_min'] <= median <= gate['aa_q_mode_median_max']
    ]
    aa_passed = (not extreme and len(outliers) <= gate['max_aa_outliers'] and
                 len(mode_medians) == 2 and not q_mode_failures and
                 all(gate['aa_mode_median_min'] <= value <= gate['aa_mode_median_max']
                     for value in mode_medians.values()))
    q_logs = [statistics.median(log(value) for value in comparisons[q])
              for q in range(32) if len(comparisons[q]) == 4]
    bounds = base.interval(q_logs) if len(q_logs) == 32 else None
    qualified = (not issues and not core['invalid_arms'] and not core['invalid_pairs'] and
                 not core['memory_failures'] and aa_passed and bounds is not None)
    return {
        'schema': 'rho-worker-tier1-production-audit-v1',
        'measurement_tier': 'tier1_instance',
        'gate_passed': qualified,
        'decision': ('qualified_5pct_improvement' if qualified and bounds[0] > 1.05
                     else 'qualified_below_5pct_gate' if qualified
                     else 'unknown_gate_failed'),
        'qualified_q_cluster_bootstrap95': bounds if qualified else None,
        'comparison_q_cluster_bootstrap95_diagnostic': bounds,
        'comparison_ratio_descriptive': exp(statistics.median(q_logs)) if q_logs else None,
        'completed_arms': summary.get('completed_arms'),
        'completed_pairs': summary.get('completed_pairs'),
        'structural_issues': issues,
        'invalid_arms': core['invalid_arms'],
        'invalid_pairs': core['invalid_pairs'],
        'memory_failures': core['memory_failures'],
        'aa_outliers': outliers,
        'aa_extreme_failures': extreme,
        'aa_mode_median_ratios': mode_medians,
        'aa_q_mode_median_failures': q_mode_failures,
        'median_phase_and_rss': core['median_phase_and_rss'],
        'raw_manifest_sha256': core['raw_manifest_sha256'],
        'raw_summary_sha256': core['raw_summary_sha256'],
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--result-dir', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.result_dir)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'decision': result['decision'], 'completed_pairs': result['completed_pairs']}))


if __name__ == '__main__':
    main()
