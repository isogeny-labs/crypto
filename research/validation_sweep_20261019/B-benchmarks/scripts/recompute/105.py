#!/usr/bin/env python3
# SWB-RECOMPUTE-14 row 105: 20261013-gaudry-quartic-d4. Reads committed files only; appends one JSON object.
import json, os, re, statistics, sys

ROOT = '/home/user/crypto'
OUT = os.path.join(ROOT, 'research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-14.jsonl')
D = 'research/gaudry_quartic_d4_20261013'
R = 'research/gaudry_quartic_replay_20261013'

def rd(p):
    with open(os.path.join(ROOT, p)) as f:
        return [json.loads(l) for l in f if l.strip()]

planted = rd(D + '/raw/solves_planted.jsonl')
uniform = rd(D + '/raw/solves_uniform.jsonl')
deg15 = rd(D + '/raw/solves_planted_deg15.jsonl')
hit = rd(D + '/raw/hit_rate.jsonl')
s5 = rd(R + '/raw/s5_p269.jsonl')

def fmt(x):
    return repr(x)

base_muls = [r['baseline']['total_muls'] for r in s5]
base_dim = [r['baseline']['dim'] for r in s5]
base_ms = [r['baseline']['total_ms'] for r in s5]

d4_all = planted + uniform
d4_14 = [r for r in d4_all if r.get('macaulay_degree') == 14 and 'solve' in r]
d4_muls = [r['solve']['total_muls'] for r in d4_14]
d4_dim = [r['solve']['dim'] for r in d4_14]
d4_ms = [r['solve']['total_ms'] for r in d4_14]

# degenerate / missing targets: count "target N:" lines in the two logs
log_lines = []
for p in (D + '/raw/solves_planted.log', D + '/raw/solves_uniform.log'):
    with open(os.path.join(ROOT, p)) as f:
        log_lines += [l for l in f if re.match(r'target \d+:', l)]
not_closed = []
for p in (D + '/raw/solves_planted.log', D + '/raw/solves_uniform.log'):
    with open(os.path.join(ROOT, p)) as f:
        not_closed += [l.strip() for l in f if 'did not close' in l]

# decompositions agree flag
agree_flags = [r['decompositions_agree'] for r in d4_all if 'decompositions_agree' in r]

# decomposition rate raw counts
x_hit = sum(r['x_hit_targets'] for r in hit)
y_hit = sum(r['y_hit_targets'] for r in hit)
n_targets = sum(r['targets'] for r in hit)

lo_ratio = min(base_muls) / max(d4_muls)
hi_ratio = max(base_muls) / min(d4_muls)

computed = {
    'baseline_total_muls_min_max_median': [min(base_muls), max(base_muls), statistics.median(base_muls)],
    'd4_deg14_total_muls_min_max_median': [min(d4_muls), max(d4_muls), statistics.median(d4_muls)],
    'ratio_min_base_over_max_d4': lo_ratio,
    'ratio_max_base_over_min_d4': hi_ratio,
    'baseline_dim': base_dim,
    'd4_deg14_dim': sorted(set(d4_dim)),
    'baseline_total_ms_min_max_median': [min(base_ms), max(base_ms), statistics.median(base_ms)],
    'd4_deg14_total_ms_min_max_median': [min(d4_ms), max(d4_ms), statistics.median(d4_ms)],
    'n_d4_deg14_records': len(d4_14),
    'target_lines_in_logs_planted_uniform': len(log_lines),
    'not_closed_lines': not_closed,
    'planted_record_targets': sorted(r['target'] for r in planted),
    'uniform_record_targets': sorted(r['target'] for r in uniform),
    'deg15_records': len(deg15),
    'decompositions_agree_true': sum(1 for a in agree_flags if a),
    'decompositions_agree_records': len(agree_flags),
    'hit_rate_targets_total': n_targets,
    'x_hit_targets_sum': x_hit,
    'y_hit_targets_sum': y_hit,
    'hit_rate_rows': [{k: r[k] for k in ('seed', 'targets', 'x_base_values', 'x_decompositions', 'x_hit_targets', 'y_base_values', 'y_decompositions', 'y_hit_targets')} for r in hit],
}
for k, v in computed.items():
    print(k, '=', json.dumps(v))

def fm(x):
    return '%.4e' % x

claims = [
    {'claimed': '`C₄` `1.21·10¹²`',
     'computed': 'baseline total_muls min %s, max %s, median %s (3 systems)' % (fm(min(base_muls)), fm(max(base_muls)), fm(statistics.median(base_muls))),
     'definition': 'total_muls of the x-line S5 baseline, s5_p269.jsonl; study gives no single-value aggregation rule for the headline 1.21',
     'files': [R + '/raw/s5_p269.jsonl'],
     'status': 'cannot_define'},
    {'claimed': '`2.54·10¹⁰` `F_p` multiplications',
     'computed': 'D4 degree-14 solve total_muls min %s, max %s, median %s (n=%d)' % (fm(min(d4_muls)), fm(max(d4_muls)), fm(statistics.median(d4_muls)), len(d4_muls)),
     'definition': 'solve.total_muls over planted+uniform records with macaulay_degree 14; study gives no single-value aggregation rule for the headline 2.54',
     'files': [D + '/raw/solves_planted.jsonl', D + '/raw/solves_uniform.jsonl'],
     'status': 'cannot_define'},
    {'claimed': '47.5–48.4×',
     'computed': 'min-base/max-D4 = %.2f ; max-base/min-D4 = %.2f' % (lo_ratio, hi_ratio),
     'definition': 'endpoints of the README range of baseline C4 over D4 C4 (min base / max D4 and max base / min D4), using the raw ranges above',
     'files': [R + '/raw/s5_p269.jsonl', D + '/raw/solves_planted.jsonl', D + '/raw/solves_uniform.jsonl'],
     'status': 'match' if (abs(lo_ratio - 47.5) / 47.5 <= 0.01 and abs(hi_ratio - 48.4) / 48.4 <= 0.01) else 'mismatch'},
    {'claimed': 'quotient dimension 4,096 → 512',
     'computed': 'baseline dim %s ; D4 deg-14 dim %s' % (sorted(set(base_dim)), sorted(set(d4_dim))),
     'definition': 'solve.dim (D4) and baseline.dim (S4 x-line, s5 file)',
     'files': [R + '/raw/s5_p269.jsonl', D + '/raw/solves_planted.jsonl', D + '/raw/solves_uniform.jsonl'],
     'status': 'match' if (sorted(set(base_dim)) == [4096] and sorted(set(d4_dim)) == [512]) else 'mismatch'},
    {'claimed': '3 min → 5 s a solve',
     'computed': 'baseline total_ms min %.1f s, max %.1f s, median %.1f s ; D4 deg-14 total_ms min %.2f s, max %.2f s, median %.2f s' % (min(base_ms) / 1000, max(base_ms) / 1000, statistics.median(base_ms) / 1000, min(d4_ms) / 1000, max(d4_ms) / 1000, statistics.median(d4_ms) / 1000),
     'definition': 'total_ms of baseline (s5 file) and D4 deg-14 solves; the headline "3 min" and "5 s" are single values with no stated aggregation rule',
     'files': [R + '/raw/s5_p269.jsonl', D + '/raw/solves_planted.jsonl', D + '/raw/solves_uniform.jsonl'],
     'status': 'cannot_define'},
    {'claimed': 'crossover with rho moves from `2^{150}` to `2^{135}` (`h = 2`)',
     'computed': 'not computed from raw',
     'definition': 'depends on the §11.16 model with r∞ (phi 0.73 to 1) and the n^{1/4} and n^{1/2} extrapolation, which are not in the raw records',
     'files': [D + '/README.md'],
     'status': 'cannot_define'},
    {'claimed': '1 of 20 targets degenerate',
     'computed': 'target lines in planted+uniform logs = %d ; not-closed lines = %d (%s) ; planted record targets = %s' % (len(log_lines), len(not_closed), '; '.join(not_closed), sorted(r['target'] for r in planted)),
     'definition': 'count of "target N:" lines in solves_planted.log and solves_uniform.log, and of lines reporting that the Macaulay matrix did not close',
     'files': [D + '/raw/solves_planted.log', D + '/raw/solves_uniform.log'],
     'status': 'match' if (len(log_lines) == 20 and len(not_closed) == 1) else 'mismatch'},
    {'claimed': 'decomposes ... 12 and 24 targets against ... 197 and 138 (4,000 uniform targets per curve)',
     'computed': 'y_hit_targets per seed %s ; x_hit_targets per seed %s ; total targets %d' % ([r['y_hit_targets'] for r in hit], [r['x_hit_targets'] for r in hit], n_targets),
     'definition': 'raw hit counts from hit_rate.jsonl; the README rate ratios (0.092, 0.174, 0.133 pooled) use an "equal base size" normalisation whose exact formula is not stated, so they are not recomputed',
     'files': [D + '/raw/hit_rate.jsonl', D + '/README.md'],
     'status': 'partial'},
    {'claimed': '0.133 pooled decomposition-rate ratio (equal base size)',
     'computed': 'raw counts only: y_hit sum %d, x_hit sum %d' % (y_hit, x_hit),
     'definition': 'the equal-base-size normalisation is not written out in the study files; choice missing',
     'files': [D + '/raw/hit_rate.jsonl', D + '/README.md'],
     'status': 'cannot_define'},
    {'claimed': 'decomposition rates on 8,000 targets',
     'computed': 'sum of targets over hit_rate.jsonl rows = %d' % n_targets,
     'definition': 'sum of the "targets" field over the two seeds of hit_rate.jsonl',
     'files': [D + '/raw/hit_rate.jsonl'],
     'status': 'match' if n_targets == 8000 else 'mismatch'},
]

identical = {
    'claimed': True,
    'n_compared': len(agree_flags),
    'n_equal': sum(1 for a in agree_flags if a),
    'n_differ': sum(1 for a in agree_flags if not a),
    'files': [D + '/raw/solves_planted.jsonl', D + '/raw/solves_uniform.jsonl'],
    'status': 'match' if all(agree_flags) else 'mismatch',
}

def grep_lines(path, pattern):
    out = []
    with open(os.path.join(ROOT, path)) as f:
        for i, line in enumerate(f, 1):
            if re.search(pattern, line):
                out.append('%s:%d: %s' % (path, i, line.rstrip('\n')))
    return out

offrepo_refs = []
aa_lines = []
isolation_lines = []
for p in (D + '/README.md', D + '/PROTOCOL.md', D + '/REPORT.typ'):
    offrepo_refs += grep_lines(p, r'drive\.google|/Volumes|SSD|/Users|s3://|gs://|store:|Modal|not committed|scratch')
    aa_lines += grep_lines(p, r'A/A|null-control|null control|negative control|positive control')
    isolation_lines += grep_lines(p, r'isolat|shared|idle|reserved|Xeon|vCPU')

row = {
    'row_no': 105,
    'slug': '20261013-gaudry-quartic-d4',
    'claims': claims,
    'identical': identical,
    'offrepo_refs': offrepo_refs,
    'aa_lines': aa_lines,
    'isolation_lines': isolation_lines,
    'protocol_git_date': 'ca00c151764f7c3221b22a43a138887f171e051d 2026-10-17T10:19:32-07:00 (git log --diff-filter=A -- ' + D + '/PROTOCOL.md)',
    'first_raw_git_date': 'ca00c151764f7c3221b22a43a138887f171e051d 2026-10-17T10:19:32-07:00 (git log --diff-filter=A -- ' + D + '/raw/solves_planted.jsonl); same commit as PROTOCOL.md',
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/105.py',
    'notes': 'Headline C4 baseline 1.21e12 is in research/gaudry_quartic_replay_20261013/raw/s5_p269.jsonl (outside this study dir). Planted target 8 has no record in solves_planted.jsonl (9 records, targets %s); the log line for target 8 reports the Macaulay matrix at degree 14 did not close. D4 degree-14 records counted: %d. Hit-rate raw file has 2 seeds, 4000 targets each.' % (sorted(r['target'] for r in planted), len(d4_14)),
}

os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, 'a') as f:
    f.write(json.dumps(row, ensure_ascii=False) + '\n')
print('row 105 written; claims:', len(claims))
for c in claims:
    print(c['status'], '|', c['claimed'], '|', c['computed'])
print('identical:', json.dumps(identical))
print('offrepo_refs:', len(offrepo_refs), 'aa_lines:', len(aa_lines), 'isolation_lines:', len(isolation_lines))
