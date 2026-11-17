import sys, os, csv, statistics, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _common import *

BASE = 'research/two_sylow_fp64_20261017/'
RUNS = ['base-1', 'cand-1', 'base-2', 'cand-2']
OUT = p('research/validation_sweep_20261019/B-benchmarks/recompute/rows/row22.json')
med = statistics.median


def samples(run):
    out = {}
    with open(p(BASE + 'runs/' + run + '/samples.csv')) as fh:
        for r in csv.DictReader(fh):
            out.setdefault(r['arm'], []).append(float(r['value']))
    return out


def medfile(run):
    with open(p(BASE + 'runs/' + run + '/medians.csv')) as fh:
        rows = list(csv.reader(fh))
    out = {}
    for r in rows:
        if len(r) == 2 and r[0] != 'arm':
            out[r[0]] = float(r[1])
    return out


S = {r: samples(r) for r in RUNS}
M = {r: medfile(r) for r in RUNS}
# consistency: samples median vs medians.csv (aggregation rule check)
mism = []
for r in RUNS:
    for arm, v in M[r].items():
        if arm in S[r]:
            if abs(med(S[r][arm]) - v) > 1e-6 * max(1, abs(v)):
                mism.append((r, arm, med(S[r][arm]), v))
print('samples-median vs medians.csv mismatches:', len(mism), mism[:3], file=sys.stderr)

ratio1 = {}
ratio2 = {}
for arm in M['base-1']:
    ratio1[arm] = M['cand-1'][arm] / M['base-1'][arm]
    ratio2[arm] = M['cand-2'][arm] / M['base-2'][arm]

# compare.csv cross-check
cmp_rows = {}
with open(p(BASE + 'runs/compare/compare.csv')) as fh:
    for r in csv.DictReader(fh):
        cmp_rows[r['arm']] = r
def _d(a):
    return max(abs(ratio1[a] - float(cmp_rows[a]['r1'])), abs(ratio2[a] - float(cmp_rows[a]['r2'])))
xchk = max(_d(a) for a in cmp_rows if a in ratio1 and (a.startswith('tsy.') or a.startswith('kernel.')))
bad_prefixes = sorted({a.split('.')[0] for a in cmp_rows if a in ratio1 and _d(a) > 1e-4})
print('compare.csv arms differing from medians ratios, by prefix:', bad_prefixes, file=sys.stderr)
print('max |recomputed ratio - compare.csv ratio| =', xchk, file=sys.stderr)

tsy = [a for a in ratio1 if a.startswith('tsy.')]
fp64 = [a for a in ratio1 if re.match(r'kernel\.fp64\.(\d+)$', a) and int(a.split('.')[-1]) >= 4096]
w1 = [a for a in ratio1 if re.match(r'kernel\.w1\.(\d+)$', a) and int(a.split('.')[-1]) >= 4096]

p1 = [ratio1[a] for a in tsy]
p2 = [ratio2[a] for a in tsy]
S1 = max(max(ratio1[a], ratio2[a]) for a in tsy)
K1 = max(max(ratio1[a], ratio2[a]) for a in fp64)
C1 = max(max(abs(ratio1[a] - 1), abs(ratio2[a] - 1)) for a in w1)

claims = []
claims.append({'claimed': 'ratios 0.98–1.02 (pair 1, tsy arms)', 'computed': f"min r1={min(p1):.4f} max r1={max(p1):.4f} over {len(p1)} tsy arms",
               'definition': 'r1 = median(cand-1)/median(base-1) per tsy arm (medians.csv)', 'files': [BASE + 'runs/compare/compare.csv'],
               'status': 'match' if round(min(p1), 2) == 0.98 and round(max(p1), 2) == 1.02 else 'mismatch'})
claims.append({'claimed': 'ratios 1.02–1.08 (pair 2, tsy arms)', 'computed': f"min r2={min(p2):.4f} max r2={max(p2):.4f} over {len(p2)} tsy arms",
               'definition': 'r2 = median(cand-2)/median(base-2) per tsy arm', 'files': [BASE + 'runs/compare/compare.csv'],
               'status': 'match' if round(min(p2), 2) == 1.02 and round(max(p2), 2) == 1.08 else 'mismatch'})
claims.append({'claimed': 'S1 1.083 (no arm > +10%)', 'computed': f"max(r1,r2) over tsy arms = {S1:.4f}",
               'definition': 'max over the 6 tsy.* arms of max(r1,r2); threshold 1.10', 'files': [BASE + 'runs/compare/compare.csv'],
               'status': 'match' if round(S1, 3) == 1.083 else 'mismatch'})
claims.append({'claimed': 'positive control 0.44', 'computed': f"max(r1,r2) over kernel.fp64.* with B>=4096 ({len(fp64)} arms) = {K1:.4f}",
               'definition': 'K1 positive control rule from PROTOCOL.md', 'files': [BASE + 'runs/compare/compare.csv'],
               'status': 'match' if round(K1, 2) == 0.44 else 'mismatch'})
claims.append({'claimed': 'negative control within 4.5%', 'computed': f"max |r-1| over kernel.w1.* with B>=4096 ({len(w1)} arms) = {C1:.4f} ({100*C1:.2f}%)",
               'definition': 'C1 negative control max |r_k - 1|', 'files': [BASE + 'runs/compare/compare.csv'],
               'status': 'match' if C1 <= 0.045 + 1e-12 else 'mismatch'})

# identical gate lines
gates = {r: [l.rstrip('\n') for l in open(p(BASE + 'runs/' + r + '/gate.txt')) if l.strip()] for r in RUNS}
n_cmp = n_eq = n_diff = 0
for r in RUNS[1:]:
    for a, b in zip(gates['base-1'], gates[r]):
        n_cmp += 1
        if a == b:
            n_eq += 1
        else:
            n_diff += 1
    if len(gates[r]) != len(gates['base-1']):
        n_diff += 1
ident = {'claimed': True, 'n_compared': n_cmp, 'n_equal': n_eq, 'n_differ': n_diff,
         'files': [BASE + 'runs/' + r + '/gate.txt' for r in RUNS],
         'status': 'match' if n_diff == 0 and n_cmp > 0 else 'mismatch'}

study = [BASE + 'README.md', BASE + 'PROTOCOL.md', BASE + 'report.typ']
study_off = grep_lines(study, OFFREPO_RX)
study_aa = grep_lines(study, AA_RX)
study_iso = grep_lines(study, ISO_RX)
proto = BASE + 'PROTOCOL.md'
first_raw = BASE + 'runs/base-1/measurements.json'

row = {
    'row_no': 22, 'slug': '20261017-two-sylow-fp64',
    'claims': claims, 'identical': ident,
    'offrepo_refs': study_off, 'aa_lines': study_aa, 'isolation_lines': study_iso,
    'protocol_git_date': git_added(proto), 'first_raw_git_date': git_added(first_raw),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/22.py',
    'notes': (f"aggregation = per-arm median over reps (medians.csv; checked against samples.csv medians: {len(mism)} mismatches). "
              f"recomputed tsy/kernel ratios vs runs/compare/compare.csv max abs diff {xchk:.2e}; compare.csv ratios for prefixes {bad_prefixes} do not equal medians.csv ratios (not used by any headline claim). hwcost compare (Rust) not run (cargo forbidden); ratios recomputed in python. C1 claim 'within 4.5%' is an upper bound and is tested as C1 <= 0.045. "
              "Identical claim = gate.txt lines (the card's gate) compared against base-1 across four runs; gate.txt contains 2 lines per run."),
}
write_row(22, row, OUT)
