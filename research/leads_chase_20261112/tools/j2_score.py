#!/usr/bin/env python3
"""J2 scorer: success on attempt 1 per (q, c), paired charged M-eq (w = 0) per seed, c vs 2.00; calib medians per q."""
import json, glob, os, sys, statistics as st
U = sys.argv[1] if len(sys.argv) > 1 else 'j2/units'
def recs(f): return [json.loads(l) for l in open(f) if l.startswith('{')]
calib = {}
for q in (65479, 262111, 1048507, 4194271):
    rs = [r for i in (1, 2, 3) if os.path.exists(f'{U}/calib_{q}_{i}.jsonl') for r in recs(f'{U}/calib_{q}_{i}.jsonl')]
    if rs: calib[q] = dict(r_inv=st.median(r['r_inv'] for r in rs), r_lmul=st.median(r['r_lmul'] for r in rs))
PH = ('fb', 'relations', 'matrix', 'linear_algebra', 'log_recovery', 'descent', 'verify')
def meq(r, q):
    c = calib[q]; return sum(r.get(f'{p}.mul', 0) + c['r_inv'] * r.get(f'{p}.inv', 0) + c['r_lmul'] * r.get(f'{p}.lmul', 0) for p in PH)
rows = {}
for f in sorted(glob.glob(f'{U}/ic_*_c*.jsonl')):
    base = os.path.basename(f)[3:-6]; q, sd, c = base.split('_'); q, sd, c = int(q), int(sd), float(c[1:])
    rs = [r for r in recs(f) if r.get('rec') == 'ic' and r.get('arm') == 'DL1']
    if not rs: rows[(q, c, sd)] = dict(missing=True); continue
    first = rs[0]; ok1 = first['ok'] and first['attempt'] == 1
    total = sum(meq(r, q) for r in rs)       # every attempt charged (N4v2 rule)
    rows[(q, c, sd)] = dict(ok_attempt1=bool(ok1), attempts=len(rs), final_ok=bool(rs[-1]['ok']), meq=total, k_found_ok=all(r['k_found'] == r['k'] for r in rs if r['ok']))
print('calib medians:', json.dumps(calib))
print(f"{'q':>8} {'c':>5} | attempt-1 success (seeds) | charged DL1 M-eq per seed | ratio to c=2.00 (paired)")
for q in sorted({k[0] for k in rows}):
    for c in sorted({k[1] for k in rows if k[0] == q}):
        cells = [rows.get((q, c, sd)) for sd in (1, 2, 3)]
        succ = ['-' if (x is None or x.get('missing')) else ('Y' if x['ok_attempt1'] else 'n') for x in cells]
        ratios = []
        for sd in (1, 2, 3):
            a, b = rows.get((q, c, sd)), rows.get((q, 2.0, sd))
            if a and b and not a.get('missing') and not b.get('missing'): ratios.append(a['meq'] / b['meq'])
        me = ['-' if (x is None or x.get('missing')) else f"{x['meq']:.3e}" for x in cells]
        print(f"{q:>8} {c:>5.2f} | {''.join(succ):>8} | {' '.join(me)} | {' '.join(f'{r:.3f}' for r in ratios)}")
aa = f'{U}/aa_65479_1_c1.75_repeat.jsonl'
if os.path.exists(aa):
    a = [r for r in recs(aa) if r.get('arm') == 'DL1']; b = [r for r in recs(f'{U}/ic_65479_1_c1.75.jsonl') if r.get('arm') == 'DL1']
    same = all({k: v for k, v in x.items() if not k.endswith(('wall', 'cpu')) and k != 'load1'} == {k: v for k, v in y.items() if not k.endswith(('wall', 'cpu')) and k != 'load1'} for x, y in zip(a, b)) and len(a) == len(b)
    print('A/A repeat of (65479, seed 1, c 1.75): counters identical =', same)
