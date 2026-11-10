#!/usr/bin/env python3
"""Lay out the devcheck rows (m*_t*.jsonl, agent_smoke_*.jsonl) as table.md: per
(file, m, t, target) the tower and multiples last fall degrees side by side. Reads
recorded values only; computes nothing beyond the difference of two recorded degrees."""
import json, glob, os, collections
here=os.path.dirname(os.path.abspath(__file__))
cells=collections.OrderedDict()
for f in sorted(glob.glob(os.path.join(here,'*.jsonl'))):
    for line in open(f):
        line=line.strip()
        if not line: continue
        r=json.loads(line)
        if 'm' not in r: continue  # the run's trailing summary row
        key=(os.path.basename(f), r['m'], r['t'], r.get('target_index'), r.get('target'))
        c=cells.setdefault(key,{})
        c[r['control']]=r
def dF(r):
    if r is None: return '—'
    d=r.get('last_fall_degree')
    if d is None:
        cen=r.get('censored_at')
        return f'> {cen} (censored)' if cen else 'none (planted/consistent)' if r.get('target')=='planted' else f'> {r.get("hky_cmax")}'
    return str(d)
out=['# Development-check rows: tower vs multiples (`[pcc:unverified]`, local host)','',
     '| file | m | t | N | target | d_F tower | d_F multiples | Δ | ffd tower / mult | falls tower / mult | n_vars t/m | n_eq t/m | wall s t/m |','|:--|--:|--:|--:|:--|:--|:--|:--|:--|:--|:--|:--|:--|']
for (f,m,t,ti,tg),c in cells.items():
    a=c.get('tower'); b=c.get('multiples')
    da,db=dF(a),dF(b)
    delta='—'
    if a and b and a.get('last_fall_degree') is not None and b.get('last_fall_degree') is not None:
        delta=str(b['last_fall_degree']-a['last_fall_degree'])
    g=lambda r,k: '—' if r is None else str(r.get(k))
    w=lambda r: '—' if r is None else f"{r.get('wall_s',0):.0f}"
    N=(a or b)['N']
    out.append(f"| {f} | {m} | {t} | {N} | {tg} #{ti} | {da} | {db} | {delta} | {g(a,'first_fall_degree')} / {g(b,'first_fall_degree')} | {g(a,'falls')} / {g(b,'falls')} | {g(a,'n_vars')} / {g(b,'n_vars')} | {g(a,'n_equations')} / {g(b,'n_equations')} | {w(a)} / {w(b)} |")
open(os.path.join(here,'table.md'),'w').write('\n'.join(out)+'\n')
print('\n'.join(out))
