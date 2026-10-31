import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from swb_common import *  # noqa

ROW = 1
SLUG = '20261018-hec-rho-negation'
STUDY = ['research/notes/index-calculus/round9']
RAW = 'research/notes/index-calculus/round9/run5/R9.txt'
PROTOCOL = 'research/notes/index-calculus/round9/CARD-HEIC-ROUND9-1.yaml'
RAW_CANDS = ['research/notes/index-calculus/round9/CARD-HEIC-ROUND9-2.yaml',
             'research/notes/index-calculus/round9/CARD-HEIC-ROUND9-3.yaml',
             'research/notes/index-calculus/round9/CARD-HEIC-ROUND9-4.yaml',
             'research/notes/index-calculus/round9/CARD-HEIC-ROUND9-5.yaml',
             'research/notes/index-calculus/round9/run5/executor_report.txt',
             'research/notes/index-calculus/round9/run5/receipt.json']

# parse the printed table: g p N walk_plain walk_fold ratio fruitless S_p S_f S_ratio ok
rows = []
for line in open(abspath(RAW)):
    f = line.split()
    if len(f) == 11 and f[0].isdigit():
        rows.append(dict(g=int(f[0]), p=int(f[1]), N=int(f[2]), wp=int(f[3]), wf=int(f[4]),
                         ratio=float(f[5]), sp=float(f[7]), sf=float(f[8]), ok=f[10]))
assert len(rows) == 18, len(rows)

small = [r for r in rows if r['N'] <= 60000]
large = [r for r in rows if r['N'] >= 100000]
big2 = sorted(rows, key=lambda r: r['N'])[-2:]
r_big = {r['N']: r for r in big2}
ideal_plain = math.sqrt(math.pi / 2)
# consistency of printed ratio with printed S values (S_f/S_p == walk ratio)
max_dev = max(abs(r['sf'] / r['sp'] - r['ratio']) for r in rows)

claims = []
F = [RAW]
claims.append(C('0.67', '%.3f' % min(r['ratio'] for r in small),
                'min of printed folded/plain ratio over rows with N <= 6e4 (%d rows)' % len(small), F))
claims.append(C('0.89', '%.3f' % max(r['ratio'] for r in small),
                'max of printed folded/plain ratio over rows with N <= 6e4 (%d rows)' % len(small), F))
med = median([r['ratio'] for r in large])
claims.append(C('0.968', '%.4f' % med,
                'median of printed folded/plain ratio over rows with N >= 1e5 (%d rows)' % len(large), F))
claims.append({'claimed': 'R1 registered <= 0.80, falsified', 'computed': '%.4f > 0.80' % med,
               'definition': 'comparison of the computed median with the registered bound; '
                             'printed verdict line reads FALSIFIED',
               'files': F, 'status': 'match' if med > 0.80 else 'mismatch'})
claims.append(C('1.82', '%.3f' % r_big[4620611]['sp'],
                'printed S_walk plain at N=4620611 (second-largest row)', F))
claims.append(C('2.10', '%.3f' % r_big[16790591]['sp'],
                'printed S_walk plain at N=16790591 (largest row)', F))
claims.append(C('1.2533', '%.6f' % ideal_plain,
                'ideal plain S_walk sqrt(pi/2) computed here', [], None))
claims.append(C('18', len(rows), 'count of printed table rows (instances)', F))
claims.append(C('1321', min(r['N'] for r in rows), 'smallest N in printed table', F))
claims.append(C('16790591', max(r['N'] for r in rows), 'largest N in printed table', F))
claims.append(C('every run verified', 'ok flag y on %d/%d rows; R3 verdict HOLDS' %
                (sum(r['ok'] == 'y' for r in rows), len(rows)),
                'per-run verification flags are not committed; only the per-row ok column and the R3 verdict line',
                F, 'partial'))

notes = ('Source table is run5/R9.txt (the file report.typ cites as its source). Per-seed records are not '
         'committed: the table holds means over 40 seeds per row, so claims are computed from those printed means. '
         'Walk ratio column recomputed from printed S values agrees to rounding (max abs dev %.5f). '
         'The S ratio column is not used by any headline. Verdict lines in R9.txt: R1 median 0.968 FALSIFIED (> 0.80); '
         'R2 6.5%% HOLDS; R3 HOLDS. Ideal constants as printed: plain 1.2533, folded 0.8862.' % max_dev)

offrepo_files, off, aa, iso = scan_study(STUDY)
store = store_refs(offrepo_files)
emit(ROW, SLUG, claims, no_identical(), off + store, aa, iso,
     git_add(PROTOCOL), first_raw_date([p for p in RAW_CANDS if not p.endswith('.yaml')] + [RAW]),
     'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1.py', notes)
