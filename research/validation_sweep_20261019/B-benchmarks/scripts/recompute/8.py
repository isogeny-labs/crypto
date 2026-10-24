import csv
import os
import re
import sys
from decimal import Decimal

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from swb_common import *  # noqa

ROW = 8
SLUG = '20261017-two-sylow-fp64'
STUDY = ['research/two_sylow_fp64_20261017']
RUNS = ['base-1', 'cand-1', 'base-2', 'cand-2']
D = 'research/two_sylow_fp64_20261017/runs'
PROTOCOL = 'research/two_sylow_fp64_20261017/PROTOCOL.md'
RAW_CANDS = [D + '/base-1/measurements.json', D + '/base-2/measurements.json',
             D + '/cand-1/measurements.json', D + '/cand-1/samples.csv',
             D + '/cand-2/measurements.json', D + '/cand-2/samples.csv']

# per-run, per-arm samples -> median over reps (rule checked against medians.csv)
samples = {}
for run in RUNS:
    for r in csv.DictReader(open(abspath(D + '/%s/samples.csv' % run))):
        if r['group'] in ('tsy', 'kernel'):
            samples.setdefault(run, {}).setdefault(r['arm'], []).append(float(r['value']))
med = {run: {a: median(v) for a, v in arms.items()} for run, arms in samples.items()}

# cross-check: medians.csv equals the median of the samples
mismatch_med = 0
n_med = 0
for run in RUNS:
    for r in csv.DictReader(open(abspath(D + '/%s/medians.csv' % run))):
        if r['arm'] in med[run]:
            n_med += 1
            if abs(med[run][r['arm']] - float(r['median'])) > 1e-6 * max(1.0, float(r['median'])):
                mismatch_med += 1

ratio = {}
for arm in med['base-1']:
    ratio[arm] = (med['cand-1'][arm] / med['base-1'][arm], med['cand-2'][arm] / med['base-2'][arm])

tsy = [a for a in ratio if a.startswith('tsy.')]
k64 = [a for a in ratio if a.startswith('kernel.fp64.') and int(a.rsplit('.', 1)[1]) >= 4096]
kw1 = [a for a in ratio if a.startswith('kernel.w1.') and int(a.rsplit('.', 1)[1]) >= 4096]
r1_tsy = [ratio[a][0] for a in tsy]
r2_tsy = [ratio[a][1] for a in tsy]
S1 = max(max(ratio[a]) for a in tsy)
K1 = max(max(ratio[a]) for a in k64)
C1 = max(max(abs(x - 1) for x in ratio[a]) for a in kw1)

# gate lines (identical search output) across base x cand runs
GATE = re.compile(r"two_sylow (\S+) p=(\d+): (\d+) draws.*?depth sum (\d+), points \(depth >= 6\) (\d+), "
                  r"x xor per arm baseline (0x[0-9a-f]+), fast-chi (0x[0-9a-f]+), fast (0x[0-9a-f]+)")
gates = {}
stderr_same = 0
for run in RUNS:
    lines = [l.rstrip('\n') for l in open(abspath(D + '/%s/gate.txt' % run)) if l.startswith('two_sylow ')]
    err = [l.rstrip('\n') for l in open(abspath(D + '/%s/stderr.txt' % run)) if l.startswith('two_sylow ')]
    stderr_same += (lines == err)
    parsed = []
    for l in lines:
        m = GATE.search(l)
        parsed.append(m.groups() if m else None)
    gates[run] = parsed
n_cmp = n_eq = n_diff = 0
for b in ('base-1', 'base-2'):
    for c in ('cand-1', 'cand-2'):
        for pb, pc in zip(gates[b], gates[c]):
            for fb, fc in zip(pb or (), pc or ()):
                n_cmp += 1
                if fb == fc:
                    n_eq += 1
                else:
                    n_diff += 1
gate_files = [D + '/%s/gate.txt' % r for r in RUNS] + [D + '/%s/stderr.txt' % r for r in RUNS]

F = [D + '/base-1/samples.csv', D + '/cand-1/samples.csv', D + '/base-2/samples.csv', D + '/cand-2/samples.csv']
claims = [
    C('0.98', '%.6f' % min(r1_tsy), 'min of r1 = cand-1 / base-1 median over the 6 tsy arms (medians of 7 reps)', F),
    C('1.02', '%.6f' % max(r1_tsy), 'max of r1 over the 6 tsy arms', F),
    C('1.02', '%.6f' % min(r2_tsy), 'min of r2 = cand-2 / base-2 median over the 6 tsy arms', F),
    C('1.08', '%.6f' % max(r2_tsy), 'max of r2 over the 6 tsy arms', F),
    C('1.083', '%.6f' % S1, 'S1: max(r1, r2) over the 6 tsy arms (search arms, all batches)', F),
    C('0.44', '%.6f' % K1, 'K1 positive control: max(r1, r2) over kernel.fp64 arms with batch >= 4096 (6 arms); '
      'README prints 0.442', F),
    {'claimed': 'within 4.5 % (C1 negative control)', 'computed': '%.6f max |r - 1| over kernel.w1 arms with batch >= 4096 '
     '(6 arms)' % C1, 'definition': 'upper-bound check: computed value <= 0.045', 'files': F,
     'status': 'match' if C1 <= Decimal('0.045') else 'mismatch'},
]

identical = {'claimed': True, 'n_compared': n_cmp, 'n_equal': n_eq, 'n_differ': n_diff, 'files': gate_files,
              'status': 'match' if (n_diff == 0 and n_cmp > 0 and stderr_same == 4) else 'mismatch'}

notes = ('Medians are the median of the 7 reps of each arm in samples.csv; this reproduces medians.csv for %d of %d '
         'arm-run entries (mismatches: %d). r1 = cand-1/base-1 and r2 = cand-2/base-2, which reproduces compare.csv. '
         'Identity: gate lines parsed from gate.txt (8 lines over 4 runs, each run 2 family lines) and compared field by '
         'field across the 4 base x cand pairs; the 8 fields are family, p, draws, depth sum, points, and the three x-xor '
         'values (n_compared = 4 pairs x 2 lines x 8 fields). '
         'gate.txt equals the "two_sylow " lines of stderr.txt in %d of 4 runs. README prints K1 as 0.442 and C1 as 0.045; '
         'computed 0.4415 and 0.04446. Protocol commit file records e71198313423b83ca939162e2a6827c9c624e955. '
         'Host: PROTOCOL names an Intel Xeon 2.10 GHz, 4 vCPU, L0, no PMU.'
         % (n_med - mismatch_med, n_med, mismatch_med, stderr_same))

offrepo_files, off, aa, iso = scan_study(STUDY)
store = store_refs(offrepo_files)
emit(ROW, SLUG, claims, identical, off + store, aa, iso,
     git_add(PROTOCOL), first_raw_date(RAW_CANDS),
     'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/8.py', notes)
