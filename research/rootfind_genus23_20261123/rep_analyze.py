#!/usr/bin/env python3
"""AMENDMENT.md R2 analysis: pooled sibling - forward split rate at g = 3, p = 65537; also both passes combined
(descriptive). Writes rep_analysis.json. [pcc:unverified]"""
import json, math, os
H = os.path.dirname(os.path.abspath(__file__))
def pool(rows):
    ks = sum(r['sib_split'] for r in rows); ns = sum(r['sib_cand'] for r in rows)
    kf = sum(r['fwd_split'] for r in rows); nf = sum(r['fwd_cand'] for r in rows)
    a, b = ks / ns, kf / nf
    se = math.sqrt(a * (1 - a) / ns + b * (1 - b) / nf)
    return dict(fwd_rate=b, sib_rate=a, diff=a - b, ci=[a - b - 1.96 * se, a - b + 1.96 * se], z=(a - b) / se,
                rel=(a - b) / b, n_sib=ns, n_fwd=nf,
                per_curve=per_curve(rows))


def per_curve(rows):
    # Pool the counts of every row of a curve before taking rates. (The first version used a dict
    # comprehension over rows, so for rep + first the later first-pass row overwrote the replication row.)
    acc = {}
    for r in rows:
        a = acc.setdefault(r['curve_seed'], [0, 0, 0, 0])
        a[0] += r['sib_split']; a[1] += r['sib_cand']; a[2] += r['fwd_split']; a[3] += r['fwd_cand']
    return {cs: a[0] / a[1] - a[2] / a[3] for cs, a in sorted(acc.items())}
rep = [json.loads(l) for l in open(os.path.join(H, 'raw', 'rep_n.jsonl'))]
first = [json.loads(l) for l in open(os.path.join(H, 'raw', 'runs.jsonl'))]
first = [r for r in first if r['arm_label'] == 'B' and r['g'] == 3]
R = pool(rep); F = pool(first); C = pool(rep + first)
reading = 'R-real' if R['ci'][1] < 0 else 'R-noise'
an = json.load(open(os.path.join(H, 'analysis.json')))
q = an['cells']['g3_p65537']['emission']
rho_w = an['cells']['g3_p65537']['rho_w_mean']
# net Cantor cost per relation if the combined deficit were real: candidates per relation rise by
# (1 + q) / (1 + q * sib/fwd)
infl = (1 + q) / (1 + q * C['sib_rate'] / C['fwd_rate'])
out = dict(replication=R, first_pass=F, combined_descriptive=C, reading=reading,
           net_walk_ratio_if_combined_deficit_real=rho_w * infl, inflation=infl, standing='[pcc:unverified]')
json.dump(out, open(os.path.join(H, 'rep_analysis.json'), 'w'), indent=1)
print(json.dumps({k: (v if not isinstance(v, dict) else {kk: vv for kk, vv in v.items() if kk != 'per_curve'}) for k, v in out.items()}, indent=1))
