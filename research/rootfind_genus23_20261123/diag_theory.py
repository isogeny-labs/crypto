#!/usr/bin/env python3
"""Post-hoc diagnostic (not pre-registered): exact share of reduced divisors whose u splits
completely, N_split / #J, on each genus-3 curve, against the measured forward and sibling rates."""
import json, math, os
H = os.path.dirname(os.path.abspath(__file__))
cells = [c for c in json.load(open(os.path.join(H, 'cells.json'))) if c['g'] == 3]
rows = {}
for l in open(os.path.join(H, 'raw', 'runs.jsonl')):
    d = json.loads(l)
    if d['g'] == 3 and d['arm_label'] in ('B', 'AA'):
        rows[(d['curve_seed'], d['arm_label'])] = d
for l in open(os.path.join(H, 'raw', 'rep_n.jsonl')):
    d = json.loads(l); rows[(d['curve_seed'], 'R')] = d
out = []; T = dict(exp=0.0, n=0, fwd=0, sib=0, nsib=0)
for c in cells:
    p, f = c['p'], c['f']
    s = z = 0
    for x in range(p):
        acc = 0
        for cf in reversed(f):
            acc = (acc * x + cf) % p
        if acc == 0: z += 1
        elif pow(acc, (p - 1) // 2, p) == 1: s += 1
    nsplit = math.comb(s, 3) * 8 + math.comb(s, 2) * z * 4 + s * math.comb(z, 2) * 2 + math.comb(z, 3)
    exp = nsplit / c['order']
    cs = c['curve_seed']
    fk = sum(rows[(cs, a)]['fwd_split'] for a in ('B', 'AA', 'R')); fn = sum(rows[(cs, a)]['fwd_cand'] for a in ('B', 'AA', 'R'))
    sk = sum(rows[(cs, a)]['sib_split'] for a in ('B', 'R')); sn = sum(rows[(cs, a)]['sib_cand'] for a in ('B', 'R'))
    out.append(dict(curve=cs, s=s, z=z, expected=exp, fwd=fk / fn, sib=sk / sn, n_fwd=fn, n_sib=sn))
    T['exp'] += exp * fn; T['n'] += fn; T['fwd'] += fk; T['sib'] += sk; T['nsib'] += sn
e = T['exp'] / T['n']
zf = (T['fwd'] / T['n'] - e) / math.sqrt(e * (1 - e) / T['n'])
zs = (T['sib'] / T['nsib'] - e) / math.sqrt(e * (1 - e) / T['nsib'])
res = dict(per_curve=out, pooled_expected=e, pooled_fwd=T['fwd'] / T['n'], pooled_sib=T['sib'] / T['nsib'], z_fwd=zf, z_sib=zs,
           note='post hoc; forward pooled over arms B (=A walk), AA, R2; sibling over B and R2; [pcc:unverified]')
json.dump(res, open(os.path.join(H, 'raw', 'diag_theory.json'), 'w'), indent=1)
for r in out: print(r)
print({k: v for k, v in res.items() if k != 'per_curve'})
