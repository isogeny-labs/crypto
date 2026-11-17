import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from swb_common import *  # noqa

ROW = 2
SLUG = '20261018-hec-rho-hash-mixing'
STUDY = ['research/notes/index-calculus/round10']
RAW = 'research/notes/index-calculus/round10/run1/R10.txt'
PROTOCOL = 'research/notes/index-calculus/round10/CARD-HEIC-ROUND10-1.yaml'
RAW_CANDS = ['research/notes/index-calculus/round10/diagnostic/seed_profile.txt',
             'research/notes/index-calculus/round10/run1/R10.txt',
             'research/notes/index-calculus/round10/run1/isolated_bench.jsonl',
             'research/notes/index-calculus/round10/run1/receipt.json']

# g p N plain fold plain+mix fold+mix fold/pl mix_f/pl ok
rows = []
for line in open(abspath(RAW)):
    f = line.split()
    if len(f) == 10 and f[0].isdigit():
        rows.append(dict(N=int(f[2]), plain=float(f[3]), fold=float(f[4]), pm=float(f[5]),
                         fm=float(f[6]), fpl=float(f[7]), mfpl=float(f[8]), ok=f[9]))
assert len(rows) == 18, len(rows)

# column definition check: printed 'mix f/pl' should equal fold+mix / plain+mix
max_dev = max(abs(r['fm'] / r['pm'] - r['mfpl']) for r in rows)
large = [r for r in rows if r['N'] >= 100000]
small = [r for r in rows if r['N'] <= 60000]
top = sorted(rows, key=lambda r: r['N'])
med_pm = median([r['pm'] for r in large])
med_mfpl = median([r['mfpl'] for r in large])
ctrl = abs(median([r['pm'] for r in small]) - median([r['plain'] for r in small]))
F = [RAW]

claims = [
    C('1.406', '%.4f' % med_pm, 'median of printed plain+mix S_walk over rows with N >= 1e5 (%d rows)' % len(large), F),
    C('2.240', '%.3f' % top[-1]['pm'], 'printed plain+mix S_walk at the largest N (16790591)', F),
    C('1.696', '%.3f' % top[-2]['pm'], 'printed plain+mix S_walk at the second-largest N (4620611)', F),
    C('0.960', '%.4f' % med_mfpl, 'median of printed mixed fold/plain column (fold+mix over plain+mix) over N >= 1e5', F),
    C('0.023', '%.4f' % ctrl,
      'abs difference of medians over the %d rows with N <= 6e4: plain+mix vs plain S_walk' % len(small), F),
    {'claimed': 'H1 registered bound 1.55 on the two largest rows, falsified',
     'computed': '%.3f and %.3f both > 1.55' % (top[-1]['pm'], top[-2]['pm']),
     'definition': 'comparison of the two largest-N plain+mix values with the 1.55 bound',
     'files': F, 'status': 'match' if min(top[-1]['pm'], top[-2]['pm']) > 1.55 else 'mismatch'},
    {'claimed': 'H2 registered bound 0.80, falsified',
     'computed': '%.4f > 0.80' % med_mfpl,
     'definition': 'comparison of the computed median mixed fold/plain with the registered bound',
     'files': F, 'status': 'match' if med_mfpl > 0.80 else 'mismatch'},
    C('every run verified', 'ok flag y on %d/%d rows; H4 verdict HOLDS' %
      (sum(r['ok'] == 'y' for r in rows), len(rows)),
      'per-run verification flags are not committed; only the per-row ok column and the H4 verdict line',
      F, 'partial'),
]

notes = ('Source table is run1/R10.txt. Per-seed records are not committed (diagnostic/seed_profile.txt holds one '
         'row only). The column header is "mix f/pl"; it was checked against fold+mix / plain+mix: max abs deviation '
         '%.5f over 18 rows (printed to 3 dp). Verdict lines in R10.txt: H1 median 1.406 with bounds written as '
         '(<= 1.45) for the median and (<= 1.55) for the two largest rows; H2 median 0.960 FALSIFIED (> 0.80); '
         'H3 control 0.023 HOLDS (< 0.15); H4 HOLDS. Max fruitless share fold+mix 6.0%%.' % max_dev)

offrepo_files, off, aa, iso = scan_study(STUDY)
store = store_refs(offrepo_files)
emit(ROW, SLUG, claims, no_identical(), off + store, aa, iso,
     git_add(PROTOCOL), first_raw_date([p for p in RAW_CANDS]),
     'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/2.py', notes)
