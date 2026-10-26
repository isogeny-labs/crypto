import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _common import *
import csv

RUNS = 'research/isogeny-volcano-rho/runs/'
STUDY = ['research/isogeny-volcano-rho/README.md', 'research/isogeny-volcano-rho/report.typ']
OUT = p('research/validation_sweep_20261019/B-benchmarks/recompute/rows/row17.json')


def load(f):
    with open(p(f)) as fh:
        return list(csv.DictReader(fh))


def stats(rows, pred, key='iters'):
    xs = [float(r[key]) for r in rows if pred(r)]
    return mean_sd_ci(xs) if len(xs) > 1 else None


def is_iso_ctrl(r):
    return r['arm'].startswith('iso')


def is_plain_v(r):
    return r['arm'].startswith('v') and '+aut' not in r['arm']


def fmt(s):
    return f"n={s['n']} mean={s['mean']:.1f} ci95={s['ci95']:.1f}"


claims = []
A = load(RUNS + 'run-20261016T2302Z/trials.csv')
B = load(RUNS + 'run-20261016T2315Z/trials.csv')
C = load(RUNS + 'run-20261017T0049Z/trials.csv')
D = load(RUNS + 'run-20261017T0140Z/trials.csv')
PB = load(RUNS + 'run-20261017T0111Z/trials.csv')

levels = {}
for lv, claimed_mean, claimed_ci, label in [
    (0, 700483, 47849, 'L0 (crater)'), (1, 681766, 26525, 'L1'),
    (2, 637158, 20410, 'L2'), (3, 659589, 20993, 'L3 floor')]:
    s = stats(A, lambda r, lv=lv: is_plain_v(r) and int(r['level']) == lv)
    levels[lv] = s
    ok = rel_status(claimed_mean, s['mean']) == 'match' and rel_status(claimed_ci, s['ci95']) == 'match'
    claims.append({'claimed': f"{label} {claimed_mean:,} ± {claimed_ci:,}",
                   'computed': fmt(s),
                   'definition': 'curve A (run-20261016T2302Z/trials.csv): isogenous arms (arm not iso*) with level==L; mean iters, ci95=1.96*sample_sd/sqrt(n)',
                   'files': [RUNS + 'run-20261016T2302Z/trials.csv'],
                   'status': 'match' if ok else 'mismatch'})

allA = stats(A, is_plain_v)
claims.append({'claimed': '659,263 ± 12,422 (all isogenous, curve A)', 'computed': fmt(allA),
               'definition': 'all isogenous arms of curve A, mean and ci95 as above',
               'files': [RUNS + 'run-20261016T2302Z/trials.csv'],
               'status': 'match' if rel_status(659263, allA['mean']) == 'match' and rel_status(12422, allA['ci95']) == 'match' else 'mismatch'})

ctrlA = stats(A, is_iso_ctrl)
claims.append({'claimed': '674,042 ± 40,816 (isomorphic control, curve A)', 'computed': fmt(ctrlA),
               'definition': 'arms iso0..iso2 of curve A, mean and ci95 as above',
               'files': [RUNS + 'run-20261016T2302Z/trials.csv'],
               'status': 'match' if rel_status(674042, ctrlA['mean']) == 'match' and rel_status(40816, ctrlA['ci95']) == 'match' else 'mismatch'})

n_A = 272748451513
ref_A = math.sqrt(math.pi * n_A / 2)
claims.append({'claimed': '√(πn/2) = 654,547 (curve A, n = 272748451513)', 'computed': f"{ref_A:.1f}",
               'definition': 'sqrt(pi*n/2) with n from MEASURED text (no raw file field)',
               'files': [RUNS + 'run-20261016T2302Z/trials.csv'],
               'status': rel_status(654547, ref_A)})

sB = stats(B, is_plain_v)
claims.append({'claimed': '1,090,981 ± 17,380 (all isogenous, curve B)', 'computed': fmt(sB),
               'definition': 'curve B (run-20261016T2315Z/trials.csv): all isogenous arms, mean and ci95',
               'files': [RUNS + 'run-20261016T2315Z/trials.csv'],
               'status': 'match' if rel_status(1090981, sB['mean']) == 'match' and rel_status(17380, sB['ci95']) == 'match' else 'mismatch'})

sC = stats(C, is_plain_v)
claims.append({'claimed': '497,666 ± 9,336 (all isogenous, curve C)', 'computed': fmt(sC),
               'definition': 'curve C (run-20261017T0049Z/trials.csv): all isogenous arms, mean and ci95',
               'files': [RUNS + 'run-20261017T0049Z/trials.csv'],
               'status': 'match' if rel_status(497666, sC['mean']) == 'match' and rel_status(9336, sC['ci95']) == 'match' else 'mismatch'})

plainD = stats(D, is_plain_v, 'ops')
crD = [float(r['ops']) for r in D if r['arm'] == 'v0+aut']
print('plain D mean incl iso ctrl:', mean_sd_ci([float(r['ops']) for r in D if not r['arm'].endswith('+aut')])['mean'], file=sys.stderr)
folded = mean_sd_ci(crD)
ratio = plainD['mean'] / folded['mean']
rel_a = plainD['ci95'] / plainD['mean']
rel_b = folded['ci95'] / folded['mean']
ratio_ci = ratio * math.sqrt(rel_a ** 2 + rel_b ** 2) * 1.96 / 1.96
claims.append({'claimed': '1.89 ± 0.09× fewer ops (curve D, folded crater vs plain)',
               'computed': f"ratio={ratio:.4f} delta_ci95={ratio_ci:.4f}; plain_mean={plainD['mean']:.1f} (n={plainD['n']}); folded_v0+aut_mean={folded['mean']:.1f} (n={folded['n']})",
               'definition': 'ratio of mean ops: all plain (non +aut) arms of run-20261017T0140Z / arm v0+aut; ci = delta method with 1.96*sd/sqrt(n) per mean',
               'files': [RUNS + 'run-20261017T0140Z/trials.csv'],
               'status': 'match' if rel_status(1.89, ratio) == 'match' and round(ratio_ci, 2) == 0.09 else 'mismatch'})

# identical claim: pull-back control, same number of steps on every curve per seed
by_seed = {}
for r in PB:
    by_seed.setdefault(r['seed'], {})[r['arm']] = r['iters']
n_cmp = n_eq = n_diff = 0
arms_per_seed = set()
for seed, d in by_seed.items():
    ref = d.get('v0')
    arms_per_seed.add(len(d))
    for arm, it in d.items():
        if arm == 'v0' or ref is None:
            continue
        n_cmp += 1
        if it == ref:
            n_eq += 1
        else:
            n_diff += 1
ident = {'claimed': True, 'n_compared': n_cmp, 'n_equal': n_eq, 'n_differ': n_diff,
         'files': [RUNS + 'run-20261017T0111Z/trials.csv'],
         'status': 'match' if n_diff == 0 and n_cmp > 0 else 'mismatch'}

print('pull-back seeds:', len(by_seed), 'arms per seed:', sorted(arms_per_seed), file=sys.stderr)

study_off = grep_lines(STUDY, OFFREPO_RX)
study_aa = grep_lines(STUDY, AA_RX)
study_iso = grep_lines(STUDY, ISO_RX)

proto = 'research/isogeny-volcano-rho/runs/run-20261017T0415Z-instance/PROTOCOL.md'
first_raw = RUNS + 'run-20261016T2315Z/results.json'

row = {
    'row_no': 17, 'slug': '20261016-isogeny-volcano-rho',
    'claims': claims, 'identical': ident,
    'offrepo_refs': study_off, 'aa_lines': study_aa, 'isolation_lines': study_iso,
    'protocol_git_date': git_added(proto), 'first_raw_git_date': git_added(first_raw),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/17.py',
    'notes': ("protocol path used: " + proto + " (the only PROTOCOL.md in the study dirs). first raw file = first RAW CANDIDATES entry as listed. "
              "Level pooling uses the level column of trials.csv. Curve D ratio ci95 is delta method on the two means, printed-precision match (0.0922 -> 0.09). "
              "Curve D plain set = v* arms not ending in +aut (iso controls excluded by prefix; their plain mean is printed to stderr). "
              "Ref sqrt(pi*n/2) uses n from the MEASURED text. Row 17 study has no protocol file with a row-17 date; dates shown are for the instance PROTOCOL.md."),
}
write_row(17, row, OUT)
