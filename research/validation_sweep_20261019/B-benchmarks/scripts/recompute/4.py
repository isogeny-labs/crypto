import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from swb_common import *  # noqa

ROW = 4
SLUG = '20261018-k-conversion'
STUDY = ['research/k_conversion_20261018']
RAW = 'research/k_conversion_20261018/raw.jsonl'
PROTOCOL = 'research/k_conversion_20261018/PROTOCOL.md'
RAW_CANDS = ['research/k_conversion_20261018/PROVENANCE.txt',
             'research/k_conversion_20261018/RESULT.json',
             'research/k_conversion_20261018/raw.jsonl']
F = [RAW]

R = [json.loads(l) for l in open(abspath(RAW))]

# denominator: rho Ir per walk step, rep 0, toggled collection, per field cell
steps = {}
for r in R:
    if r['kind'] == 'rho' and r['rep'] == 0 and r['toggle']:
        steps.setdefault(r['cell'], []).append(r['ir'] / r['walk_steps'])
istep = {c: median(v) for c, v in steps.items()}
rho_n = sorted({r['n'] for r in R if r['kind'] == 'rho'})

# numerator: closure Ir per column, rep 0, toggled collection
clos = {}
for r in R:
    if r['kind'] == 'closure' and r['toggle'] and r['rep'] == 0:
        clos.setdefault(r['cell'], []).append(r)

def K_of(rec, den):
    return (rec['ir'] / rec['columns']) / istep[den]

m4_cells = ['m4-11-5', 'm4-12-3', 'm4-12-5']
pool_n83 = [K_of(r, 'n83') for c in m4_cells for r in clos[c]]
pool_n61 = [K_of(r, 'n61') for c in m4_cells for r in clos[c]]
k_med = median(pool_n83)
k_min, k_max = min(pool_n83), max(pool_n83)
deg4 = clos['m3-11-5-deg4']
deg4_K = median([K_of(r, 'n83') for r in deg4])
deg4_cols = sorted({r['columns'] for r in deg4})
deg4_nvars = sorted({r['n_vars'] for r in deg4})
log2k = math.log2(k_med)
conv_gap_lo = 7.62 + log2k
conv_gap_hi = 10.32 + log2k

claims = [
    C('4.48', '%.4f' % k_med, 'median over the 12 degree-3 m=4 draws (cells m4-11-5, m4-12-3, m4-12-5), n=83 denominator', F),
    C('3.96', '%.4f' % k_min, 'minimum of the same 12 K values, n=83 denominator', F),
    C('16.77', '%.4f' % k_max, 'maximum of the same 12 K values, n=83 denominator', F),
    C('83', '%.4f' % deg4_K, 'median of the 2 degree-4 draws of cell m3-11-5-deg4 (n=83 denominator)', F),
    C('17902', ','.join(str(c) for c in deg4_cols), 'columns field of the degree-4 cell m3-11-5-deg4', F),
    C('26', ','.join(str(v) for v in deg4_nvars), 'n_vars field of the degree-4 cell (N in the protocol)', F),
    C('9.78', '%.4f' % conv_gap_lo, 'converted gap: 7.62 + log2(median K) with median K from the n=83 pool', F),
    C('12.48', '%.4f' % conv_gap_hi, 'converted gap: 10.32 + log2(median K) with median K from the n=83 pool', F),
    {'claimed': '131 not measurable', 'computed': 'rho n values present in raw.jsonl: %s' % rho_n,
     'definition': 'presence of any n=131 rho record', 'files': F,
     'status': 'match' if 131 not in rho_n else 'mismatch'},
]

# cross-check against the committed RESULT.json (not used for the computed values)
res = json.load(open(abspath('research/k_conversion_20261018/RESULT.json')))
xchk = abs(res['pooled_m4_deg3']['n83']['median'] - k_med)

notes = ('Recomputed from raw.jsonl with the analyze.py rules: rho denominator = ir/walk_steps (rep 0, toggled, '
         'per n-cell, median over 3 seeds); closure numerator = ir/columns (rep 0, toggled); K = numerator / median '
         'denominator. Cross-check vs RESULT.json pooled median: abs diff %.2e. The A/A control records (m3-16-6 draw 0, '
         'rep 0 and rep 1) are in raw.jsonl; the record counts per cell are in the Counter of raw kinds (23 closure, 7 rho). '
         'Protocol names the degree-4 cell as N=26 growth cell; the raw n_vars field is what is computed above.' % xchk)

offrepo_files, off, aa, iso = scan_study(STUDY)
store = store_refs(offrepo_files)
emit(ROW, SLUG, claims, no_identical(), off + store, aa, iso,
     git_add(PROTOCOL), first_raw_date(RAW_CANDS),
     'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/4.py', notes)
