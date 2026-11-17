import sys, os, json, statistics
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _common import *

BASE = 'research/20261016-n83-precomputed-rho-control/'
R1 = BASE + 'records/run-38855595549/'
R2 = BASE + 'records/run-38878565324/'
OUT = p('research/validation_sweep_20261019/B-benchmarks/recompute/rows/row24.json')
med = statistics.median

files = {'r1k3300': R1 + 'k3300/analysis.json', 'r1k600': R1 + 'k600/analysis.json',
         'r2k3300': R2 + 'k3300/analysis.json', 'r2k600': R2 + 'k600/analysis.json'}
A = {k: json.load(open(p(v))) for k, v in files.items()}
ref = [l.strip() for l in open(p(BASE + 'reference_scalars.txt')) if l.strip()]

# per-target recomputation of median paired ratio: ic online_ms / arm online_ms
def recompute(an, num_arm, den_arm):
    ic = an['arms']['ic']['online_ms']
    den = an['arms'][den_arm]['online_ms']
    if num_arm != 'ic':
        num = an['arms'][num_arm]['online_ms']
    else:
        num = ic
    ratios = [a / b for a, b in zip(num, den)]
    return ratios

def check_pair(key_file, pkey, num_arm, den_arm, claimed, ci_claim, label):
    an = A[key_file]
    pr = an['paired'][pkey]
    assert pr['numerator'] == num_arm and pr['denominator'] == den_arm, (pr['numerator'], pr['denominator'])
    ratios = recompute(an, num_arm, den_arm)
    m = med(ratios)
    ok_med = round(m, 2) == claimed
    stored_ok = abs(m - pr['median_of_paired_ratios']) / pr['median_of_paired_ratios'] < 1e-9
    stored_ci = pr['median_of_paired_ratios_ci95']
    return {
        'claimed': f"{label} {claimed:.2f}× {ci_claim}",
        'computed': f"median of 16 per-target online ratios = {m:.4f} (stored {pr['median_of_paired_ratios']:.4f}, equal={stored_ok}); stored CI [{stored_ci[0]:.2f}, {stored_ci[1]:.2f}] not recomputed (bootstrap seed not in repo)",
        'definition': f"median over targets of {num_arm}.online_ms / {den_arm}.online_ms (paired key {pkey})",
        'files': [files[key_file]],
        'status': 'partial' if ok_med and stored_ok else 'mismatch',
    }, stored_ci

claims = []
bounds = []
c, ci = check_pair('r2k3300', 'ic_over_bl_a2', 'ic', 'bl_a2_wall_equal', 3.74, '(2.65–11.05), time-matched (a′)', 'K3300 (a′)'); claims.append(c); bounds.append(ci)
c, ci = check_pair('r2k3300', 'ic_over_bl_b', 'ic', 'bl_b_memory_matched', 40.61, '(24.27–71.72), memory-matched (b)', 'K3300 (b)'); claims.append(c); bounds.append(ci)
c, ci = check_pair('r2k600', 'ic_over_bl_a2', 'ic', 'bl_a2_wall_equal', 81.52, '(46.35–236.59), time-matched (a′)', 'K600 (a′)'); claims.append(c); bounds.append(ci)
c, ci = check_pair('r1k600', 'ic_over_bl_a', 'ic', 'bl_a_time_matched', 214.73, '(47.94–483.84), time-matched (a), run 1', 'K600 (a) run 1'); claims.append(c); bounds.append(ci)
c, ci = check_pair('r2k600', 'ic_over_bl_b', 'ic', 'bl_b_memory_matched', 395.53, '(177.93–1134.45), memory-matched (b)', 'K600 (b)'); claims.append(c); bounds.append(ci)

# positive controls: rho_plain_fold / ic
c, ci = check_pair('r2k3300', 'rho_plain_over_ic', 'rho_plain_fold', 'ic', 70.9, '(19.2–92.4)', 'positive control K3300 (empty-table rho / IC)')
c['status'] = c['status']
claims.append(c)
c, ci = check_pair('r2k600', 'rho_plain_over_ic', 'rho_plain_fold', 'ic', 3.62, '(0.93–4.48)', 'positive control K600 (empty-table rho / IC)')
claims.append(c)

allpos = all(lo > 1 for lo, hi in bounds)
claims.append({'claimed': 'all matched 95% intervals of IC/precomputed-rho lie above 1', 'computed': f"lower bounds (stored) = {[round(b[0],2) for b in bounds]}",
               'definition': 'min of stored lower CI bounds of the five matched paired ratios > 1', 'files': [files[k] for k in files],
               'status': 'partial' if allpos else 'mismatch'})

# identical: recovered scalars per arm vs reference, all four analyses, all targets
n_cmp = n_eq = n_diff = 0
replay_ok = 0
agree_ok = 0
for k, an in A.items():
    for i, v in enumerate(an['verification']):
        arms = {a: d for a, d in v.items() if isinstance(d, dict) and 'recovered_scalar' in d}
        for a, d in arms.items():
            n_cmp += 1
            if d['recovered_scalar'] == ref[i]:
                n_eq += 1
            else:
                n_diff += 1
            if d.get('generic_replay') is True:
                replay_ok += 1
        if v.get('arms_agree') is True:
            agree_ok += 1
ident = {'claimed': True, 'n_compared': n_cmp, 'n_equal': n_eq, 'n_differ': n_diff,
         'files': [files[k] for k in files] + [BASE + 'reference_scalars.txt'],
         'status': 'match' if n_diff == 0 and n_cmp > 0 else 'mismatch'}

study = [BASE + 'README.md', BASE + 'PROTOCOL.md', BASE + 'REPORT.typ', BASE + 'tables.typ', BASE + 'verdict.typ', BASE + 'notes.typ']
study_off = grep_lines(study, OFFREPO_RX)
study_aa = grep_lines(study, AA_RX)
study_iso = grep_lines(study, ISO_RX)
proto = BASE + 'PROTOCOL.md'
first_raw = files['r1k3300']

row = {
    'row_no': 24, 'slug': '20261016-n83-precomputed-rho-control',
    'claims': claims, 'identical': ident,
    'offrepo_refs': study_off, 'aa_lines': study_aa, 'isolation_lines': study_iso,
    'protocol_git_date': git_added(proto), 'first_raw_git_date': git_added(first_raw),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/24.py',
    'notes': (f"Claims map to run/key as in README table (run 2 for K3300 a', b, positive control and K600 a', b, positive control; run 1 for K600 (a)). "
              f"Medians recomputed from online_ms arrays per target; CIs are stored bootstrap percentile intervals (10000 resamples, fixed seeds not in the repo), so they are not recomputed. "
              f"Replay: generic_replay True in {replay_ok} of {n_cmp} arm verifications; arms_agree True in {agree_ok} of {sum(len(a['verification']) for a in A.values())} targets across the four analysis files. "
              "Protocol file = PROTOCOL.md; first raw file = records/run-38855595549/k3300/analysis.json (first listed in card)."),
}
write_row(24, row, OUT)
