#!/usr/bin/env python3
"""Compare assess/indep_*.jsonl with raw/ and summarise (assessor). Usage: compare.py P B FILE..."""
import json, sys
def med(x):
    x=sorted(x);n=len(x);return None if n==0 else (x[n//2] if n%2 else (x[n//2-1]+x[n//2])/2)
p,b=int(sys.argv[1]),int(sys.argv[2])
cell={};ex={}
for l in open(f'raw/cell_p{p}_b{b}.jsonl'):
    r=json.loads(l); cell[(r['seed'],r['arm'])]=r
try:
    for l in open(f'raw/exact_p{p}_b{b}.jsonl'):
        r=json.loads(l); ex[(r['seed'],r['arm'])]=r
except FileNotFoundError: pass
mis=[]; rp=[]; rs=[]; gA=[]; gB=[]; defic=[]; n=0
for f in sys.argv[3:]:
  for l in open(f):
    o=json.loads(l); s=o['seed']; n+=1
    chk=[o['T_A_uf']==cell[(s,'A')]['T'], o['T_B_uf']==cell[(s,'B')]['T'], o['N1']==cell[(s,'A')]['N1'], o['pencil_rows']==cell[(s,'B')]['pencil_rows']]
    if (s,'B') in ex: chk+= [o['T_B_ex1']==ex[(s,'B')]['T'], o['A_full_trials']==ex[(s,'A')]['T'], o['A_dim_max']==ex[(s,'A')]['dim']]
    if not all(chk): mis.append((s,chk))
    defic.append((o['A_ncols']-o['A_dim_max'], o['k_shared']))
    rp.append(1-o['T_B_uf']/o['T_A_uf'])
    gA.append(o['A_T_dim_max']/o['A_T_uf']-1)
    if not o['cens_B_ex1']:
        rs.append(1-o['T_B_ex1']/o['A_T_dim_max']); gB.append(o['T_B_ex1']/o['T_B_uf']-1)
print(f'p={p} beta_idx={b} seeds={n} mismatches_vs_raw={mis}')
print(' A deficiency n_cols-dim_max (k_shared):', defic)
print(' median proxy r_B', round(med(rp),3))
print(' A: T to own saturation / T_uf - 1: median', round(med(gA),3), 'range', round(min(gA),3), round(max(gA),3))
print(' B uncensored at n_cols-1:', len(rs), 'of', n, '; B T_exact/T_uf - 1 median', None if not gB else round(med(gB),3))
print(' r_B with each arm at its exact stop (A: own saturation, B: n_cols-1), B-uncensored seeds: median', None if not rs else round(med(rs),3), 'range', None if not rs else (round(min(rs),3), round(max(rs),3)))
