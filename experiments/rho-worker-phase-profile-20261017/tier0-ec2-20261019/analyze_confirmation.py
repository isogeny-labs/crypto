#!/usr/bin/env python3
"""Independently audit the fresh-target Tier 0 worker confirmation."""

import argparse
import json
from math import exp, log
from pathlib import Path
import statistics

import analyze as base

Q_SHA256 = '546fe0b842a2f559395c9ac5d74ced12b96ce366e41322387eb0baec5dcb3adc'
base.EXPECTED_Q_SHA256 = Q_SHA256


def audit(folder, strict_runner):
    structural = base.audit(folder, strict_runner)
    plan = json.loads(Path(json.loads((folder / 'manifest.json').read_text())['worker_plan']).read_text())
    pairs = base.lines(folder / 'pairs.jsonl')
    cases = {case['id']: case for case in plan['cases']}
    gate = plan['claim_gate']
    expected_gate = {
        'aa_ratio_min': 0.90, 'aa_ratio_max': 1.11,
        'aa_extreme_min': 0.80, 'aa_extreme_max': 1.25,
        'max_aa_outliers': 2,
        'aa_mode_median_min': 0.98, 'aa_mode_median_max': 1.02,
        'aa_q_mode_median_min': 0.95, 'aa_q_mode_median_max': 1.05,
        'comparison_lower_95_gt': 1.05,
        'max_relative_peak_rss_ratio': 1.10,
        'max_absolute_peak_rss_delta_kib': 65536,
    }
    issues = list(structural['structural_issues'])
    if any(gate.get(key) != value for key, value in expected_gate.items()):
        issues.append('confirmation claim gate differs from frozen protocol')
    if plan.get('fixture_manifest_sha256') != Q_SHA256:
        issues.append('confirmation Q digest mismatch')
    aa_by_mode = {mode: [] for mode in ('aa_serial', 'aa_batch')}
    aa_by_q_mode = {(q, mode): [] for q in range(32)
                    for mode in ('aa_serial', 'aa_batch')}
    comparisons = {q: [] for q in range(32)}
    outliers = []
    extreme = []
    for pair in pairs:
        case = cases.get(pair.get('case'))
        if case is None or pair.get('status') != 'valid':
            issues.append('missing or invalid pair in confirmation controls')
            continue
        ratio = pair['speedup']
        mode = case['kind']
        q = case['q_index']
        if mode == 'comparison':
            comparisons[q].append(ratio)
        elif mode in aa_by_mode:
            aa_by_mode[mode].append(ratio)
            aa_by_q_mode[(q, mode)].append(ratio)
            if not gate['aa_ratio_min'] <= ratio <= gate['aa_ratio_max']:
                outliers.append({'case': pair['case'], 'repeat': pair['repeat'],
                                 'ratio': ratio})
            if not gate['aa_extreme_min'] <= ratio <= gate['aa_extreme_max']:
                extreme.append({'case': pair['case'], 'repeat': pair['repeat'],
                                'ratio': ratio})
        else:
            issues.append('unexpected case kind')
    if (any(len(values) != 128 for values in aa_by_mode.values()) or
            any(len(values) != 4 for values in aa_by_q_mode.values()) or
            any(len(values) != 4 for values in comparisons.values())):
        issues.append('incomplete Q repetitions')
    if outliers != structural['aa_failures']:
        issues.append('A/A control list differs from arm-level independent audit')
    aa_mode_medians = {
        mode: exp(statistics.median(log(ratio) for ratio in values))
        for mode, values in aa_by_mode.items() if values
    }
    aa_q_mode_failures = [
        {'q_index': q, 'mode': mode, 'median_ratio': median}
        for (q, mode), values in aa_by_q_mode.items() if values
        for median in [exp(statistics.median(log(ratio) for ratio in values))]
        if not gate['aa_q_mode_median_min'] <= median <= gate['aa_q_mode_median_max']
    ]
    aa_gate = (
        not extreme and len(outliers) <= gate['max_aa_outliers'] and
        len(aa_mode_medians) == 2 and not aa_q_mode_failures and
        all(gate['aa_mode_median_min'] <= median <= gate['aa_mode_median_max']
            for median in aa_mode_medians.values())
    )
    exact_gate = not (issues or structural['invalid_arms'] or structural['invalid_pairs']
                      or structural['memory_failures'])
    q_logs = [statistics.median(log(ratio) for ratio in comparisons[q])
              for q in range(32) if len(comparisons[q]) == 4]
    bounds = base.interval(q_logs) if len(q_logs) == 32 else None
    qualified = exact_gate and aa_gate and bounds is not None
    return {
        'schema': 'rho-worker-tier0-confirmation-audit-v1',
        'gate_passed': qualified,
        'decision': ('qualified_5pct_improvement' if qualified and bounds[0] > 1.05
                     else 'qualified_below_5pct_gate' if qualified
                     else 'unknown_gate_failed'),
        'qualified_q_cluster_bootstrap95': bounds if qualified else None,
        'comparison_q_cluster_bootstrap95_diagnostic': bounds,
        'comparison_ratio_descriptive': exp(statistics.median(q_logs)) if q_logs else None,
        'bootstrap_seed': base.BOOTSTRAP_SEED,
        'completed_arms': structural['completed_arms'],
        'completed_pairs': structural['completed_pairs'],
        'structural_issues': issues,
        'invalid_arms': structural['invalid_arms'],
        'invalid_pairs': structural['invalid_pairs'],
        'memory_failures': structural['memory_failures'],
        'aa_outliers': outliers, 'aa_extreme_failures': extreme,
        'aa_mode_median_ratios': aa_mode_medians,
        'aa_q_mode_median_failures': aa_q_mode_failures,
        'raw_manifest_sha256': structural['raw_manifest_sha256'],
        'raw_summary_sha256': structural['raw_summary_sha256'],
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--result-dir', type=Path, required=True)
    parser.add_argument('--strict-runner', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.result_dir, args.strict_runner)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'decision': result['decision'],
                      'completed_pairs': result['completed_pairs']}))


if __name__ == '__main__':
    main()
