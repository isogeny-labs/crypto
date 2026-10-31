import sys, os, json, statistics, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _common import *

BASE = 'research/sea_point_count_20261016/'
EV = BASE + 'evidence/run-20261016T0205Z/'
OUT = p('research/validation_sweep_20261019/B-benchmarks/recompute/rows/row23.json')
med = statistics.median

recs = [json.loads(l) for l in open(p(EV + 'runs.jsonl'))]
summ = {r['case']: r for r in json.load(open(p(EV + 'summary.json')))}


def per_call_us(r):
    res = r['result']
    return int(res['elapsed_ns']) / int(res['calls']) / 1000.0


def per_pair(kind, case):
    ab = collections.defaultdict(dict)
    for r in recs:
        if r['kind'] == kind and r['case'] == case:
            ab[r['round']][r['slot']] = r
    return ab


def stats_case(case):
    ab = collections.defaultdict(dict)
    for r in recs:
        if r['kind'] == 'ab' and r['case'] == case:
            ab[r['round']][r['arm']] = r
    A = [per_call_us(ab[k]['baseline']) for k in sorted(ab)]
    B = [per_call_us(ab[k]['candidate']) for k in sorted(ab)]
    ratios = [a / b for a, b in zip(A, B)]
    return {'A_med': med(A), 'B_med': med(B), 'ratio_med': med(ratios), 'ratio_min': min(ratios), 'ratio_max': max(ratios), 'n_pairs': len(ratios), 'A': A, 'B': B}


def aa_dev(case):
    aa = collections.defaultdict(dict)
    for r in recs:
        if r['kind'] == 'aa' and r['case'] == case:
            aa[r['round']][r['slot']] = r
    devs = [abs(per_call_us(aa[k][0]) / per_call_us(aa[k][1]) - 1) for k in sorted(aa)]
    return max(devs)


cases = ['bsgs24', 'bsgs32', 'bsgs40', 'sea40_cached', 'sea192_cached', 'count192_cold', 'cm256_control']
S = {c: stats_case(c) for c in cases}
cross = []
for c in cases:
    sc = summ[c]
    cross.append((c, round(S[c]['ratio_med'] - sc['ratio_median'], 9), round(S[c]['A_med'] - sc['a_median_us'], 6), round(S[c]['B_med'] - sc['b_median_us'], 6), round(aa_dev(c) - sc['aa_max_deviation'], 9)))
print('cross-check vs summary.json (ratio, A, B, aa_dev diffs):', cross, file=sys.stderr)

verified = sum(1 for r in recs if r.get('result', {}).get('verified') is True)
claims = []


def mk(claimed, computed, definition, status):
    return {'claimed': claimed, 'computed': computed, 'definition': definition,
            'files': [EV + 'runs.jsonl'], 'status': status}


for c, a_claim, b_claim, r_claim, label in [
        ('bsgs24', 61.747, 31.495, 1.98652, 'BSGS24'),
        ('bsgs32', 327.251, 117.994, 2.79019, 'BSGS32'),
        ('bsgs40', 1936.899, 591.519, 3.24310, 'BSGS40')]:
    s = S[c]
    claims.append(mk(f"{label} {a_claim}→{b_claim} µs/call", f"A={s['A_med']:.3f} B={s['B_med']:.3f} µs/call (median over {s['n_pairs']} ab pairs of elapsed_ns/calls)",
                     'median over the 5 round A (baseline) and B (candidate) ab records of elapsed_ns/calls/1000',
                     'match' if round(s['A_med'], 3) == a_claim and round(s['B_med'], 3) == b_claim else 'mismatch'))
    claims.append(mk(f"{label} paired ratio {r_claim}×", f"median of per-pair A/B ratios = {s['ratio_med']:.5f} (paired range {s['ratio_min']:.5f}..{s['ratio_max']:.5f})",
                     'median over 5 ab pairs of (baseline per-call)/(candidate per-call) in the same round',
                     'match' if round(s['ratio_med'], 5) == r_claim else 'mismatch'))
for c, r_claim in [('sea40_cached', 0.81228), ('sea192_cached', 1.02393), ('count192_cold', 1.00469)]:
    s = S[c]
    claims.append(mk(f"{c} ratio {r_claim}×", f"median of per-pair A/B ratios = {s['ratio_med']:.5f}; aa_max_deviation={aa_dev(c):.5f}",
                     'same paired-ratio rule as above; A/A deviation = max over rounds of |slot0/slot1 - 1| of aa records',
                     'match' if round(s['ratio_med'], 5) == r_claim else 'mismatch'))
claims.append(mk('140/140 verified records', f"verified=True in {verified} of {len(recs)} records",
                 'count of records with result.verified == True over runs.jsonl',
                 'match' if verified == 140 and len(recs) == 140 else 'mismatch'))

study = [BASE + 'README.md', BASE + 'REPORT.md', BASE + 'MEASUREMENT.md', BASE + 'VALIDATION.md', BASE + 'VERIFICATION.md']
study_off = grep_lines(study, OFFREPO_RX)
study_aa = grep_lines(study, AA_RX)
study_iso = grep_lines(study, ISO_RX)
proto = BASE + 'PROTOCOL.md'
first_raw = EV + 'host.json'

row = {
    'row_no': 23, 'slug': '20261016-sea-point-count',
    'claims': claims,
    'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None, 'files': [], 'status': 'not_claimed'},
    'offrepo_refs': study_off, 'aa_lines': study_aa, 'isolation_lines': study_iso,
    'protocol_git_date': git_added(proto), 'first_raw_git_date': git_added(first_raw),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/23.py',
    'notes': ("protocol file = PROTOCOL.md; first raw file = evidence/run-20261016T0205Z/host.json (first listed in card). "
              "Per-call time = elapsed_ns/calls (result field); paired ratio per round = baseline/candidate. "
              "Cross-check vs summary.json values printed to stderr (ratio, A, B and A/A deviation diffs). "
              "5 A/A pairs and 5 A/B pairs per case are the 10 aa and 10 ab records per case in runs.jsonl. "
              "The 'within A/A envelope' statement is not a claim row here; the aa_max_deviation values are in the cross-check."),
}
write_row(23, row, OUT)
