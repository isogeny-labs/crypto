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
out=['# Development-check rows: tower vs multiples vs null (`[pcc:unverified]`, local host)','',
     '| file | m | t | N | target | d_F tower | d_F multiples | Δ | d_F null | Δ null | null width | ffd tower / mult / null | falls tower / mult / null | n_vars t/m | n_eq t/m | wall s t/m/n |','|:--|--:|--:|--:|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|']
for (f,m,t,ti,tg),c in cells.items():
    a=c.get('tower'); b=c.get('multiples'); u=c.get('null')
    da,db,du=dF(a),dF(b),dF(u)
    def dl(x,y):
        if x and y and x.get('last_fall_degree') is not None and y.get('last_fall_degree') is not None:
            return str(y['last_fall_degree']-x['last_fall_degree'])
        return '—'
    delta=dl(a,b); deltan=dl(a,u)
    g=lambda r,k: '—' if r is None else str(r.get(k))
    w=lambda r: '—' if r is None else f"{r.get('wall_s',0):.0f}"
    N=(a or b or u)['N']
    nw=g(u,'null_width') if u and 'null_width' in u else '—'
    out.append(f"| {f} | {m} | {t} | {N} | {tg} #{ti} | {da} | {db} | {delta} | {du} | {deltan} | {nw} | {g(a,'first_fall_degree')} / {g(b,'first_fall_degree')} / {g(u,'first_fall_degree')} | {g(a,'falls')} / {g(b,'falls')} / {g(u,'falls')} | {g(a,'n_vars')} / {g(b,'n_vars')} | {g(a,'n_equations')} / {g(b,'n_equations')} | {w(a)} / {w(b)} / {w(u)} |")
open(os.path.join(here,'table.md'),'w').write('\n'.join(out)+'\n')
print('\n'.join(out))
