import os
import sys
from decimal import Decimal

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from swb_common import *  # noqa

ROW = 3
SLUG = '20261019-hec-rho-branch-width'
STUDY = ['research/notes/index-calculus/round12']
RAW = 'research/notes/index-calculus/round12/run1/R12.txt'
PROTOCOL = 'research/notes/index-calculus/round12/CARD-HEIC-ROUND12-1.yaml'
RAW_CANDS = ['research/notes/index-calculus/round12/run1/R12.txt',
             'research/notes/index-calculus/round12/run1/isolated_bench.jsonl',
             'research/notes/index-calculus/round12/run1/receipt.json']

# g p N jac_8b jac_full int_8b int_full full_pre ok
rows = []
for line in open(abspath(RAW)):
    f = line.split()
    if len(f) == 9 and f[0].isdigit():
        rows.append(dict(N=int(f[2]), j8=float(f[3]), jf=float(f[4]), i8=float(f[5]), if_=float(f[6]),
                         pre=int(f[7]), ok=f[8]))
assert len(rows) == 18, len(rows)

large = [r for r in rows if r['N'] >= 100000]
small = [r for r in rows if r['N'] <= 60000]
top = sorted(rows, key=lambda r: r['N'])
big = top[-1]
F = [RAW]

med_jf = median([r['jf'] for r in large])
med_if = median([r['if_'] for r in large])
ctrl = Decimal(str(median([r['jf'] for r in small]))) - Decimal(str(median([r['j8'] for r in small])))
ctrl = abs(ctrl)
ratio_big = Decimal(str(big['j8'])) / Decimal(str(big['jf']))

claims = [
    C('1.390', '%.4f' % med_jf, 'median of printed Jacobian full-width S_walk over N >= 1e5 (%d rows)' % len(large), F),
    C('2.100', '%.3f' % big['j8'], 'printed Jacobian 8-bit S_walk at the largest N (16790591)', F),
    C('1.481', '%.3f' % big['jf'], 'printed Jacobian full-width S_walk at the largest N', F),
    C('1.418', '%.4f' % ratio_big, 'printed Jacobian 8-bit / full-width S_walk at the largest N, computed from printed values', F),
    C('1.303', '%.4f' % med_if, 'median of printed integer full-width S_walk over N >= 1e5 (%d rows)' % len(large), F),
    C('0.025', '%s' % ctrl,
      'abs difference of medians over the %d rows with N <= 6e4: Jacobian full-width vs 8-bit S_walk' % len(small), F),
    C('every run verified', 'ok flag y on %d/%d rows; W4 verdict HOLDS' %
      (sum(r['ok'] == 'y' for r in rows), len(rows)),
      'per-run verification flags are not committed; only the per-row ok column and the W4 verdict line',
      F, 'partial'),
]

notes = ('Source table is run1/R12.txt (report.typ cites ../../run1/R12.txt). Per-seed records are not committed; '
         'the table holds means over 40 seeds per row. Columns: jac 8b, jac full, int 8b, int full, full pre (op count, '
         'not an S_walk). The 0.025 control value is computed as exactly 0.0245 from printed 3 dp values and is at the '
         'half-unit boundary of 3 dp; it is recorded as matching at the printed precision. Verdict lines in R12.txt: '
         'W1 median 1.303 in [1.10, 1.40] HOLDS; W2 median 1.390 HOLDS; W3 control 0.025 HOLDS; W4 HOLDS.')

offrepo_files, off, aa, iso = scan_study(STUDY)
store = store_refs(offrepo_files)
emit(ROW, SLUG, claims, no_identical(), off + store, aa, iso,
     git_add(PROTOCOL), first_raw_date(RAW_CANDS),
     'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/3.py', notes)
