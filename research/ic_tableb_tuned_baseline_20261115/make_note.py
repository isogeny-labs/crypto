"""Compile numbers from the two Table B ksum studies (no new computation beyond ratios)."""
import json, sys
R='..'
P=json.load(open(f'{R}/ic_tableb_ksum_20261115/summary.json'))
B=json.load(open(f'{R}/ic_tableb_binary_ksum_20261115/summary.json'))
rows=[]
for kind,J,ks in (('prime',P,'ksum5'),('binary',B,'ksum5')):
    for r in J['rows']:
        tb=r['table_b']
        for w in ('0','0.1','1'):
            s=r['weights'][w]['s']
            inc={k:v for k,v in s.items() if k.startswith('inc-') and k!='inc-tb'}
            kb=min(inc,key=inc.get)
            rows.append(dict(kind=kind,slug=r['slug'],role=r['role'],w=w,
              rec_ic=tb['s_ic'],rec_rho=tb['s_rho'],rec_ratio=tb['ic_over_rho'],
              inc_tb=s['inc-tb'],best_name=kb,best=inc[kb],strict=r['weights'][w]['strictest_incumbent'],
              ksum4=s['ksum4'],ksum5=s['ksum5'],rho=s['rho-neg'],
              src=f"research/ic_tableb_{'ksum' if kind=='prime' else 'binary_ksum'}_20261115/summary.json"))
json.dump(rows,open('table.json','w'),indent=1)
def f(x): return f'{x:.2f}'
out=['| row | w | rec. S_IC / S_rho (ratio) | inc-tb S | best non-tb incumbent S | inc-tb / best | ksum4 S | ksum5 S | rho-neg S | rec. IC/rho | inc-tb/rho-neg | best/rho-neg | ksum5/rho-neg |','|:--|--:|:--|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|']
for x in rows:
    out.append(f"| {x['slug'].split('-')[1]} | {x['w']} | {f(x['rec_ic'])} / {f(x['rec_rho'])} | {f(x['inc_tb'])} | {x['best_name']}: {f(x['best'])} | {x['inc_tb']/x['best']:.2f} | {f(x['ksum4'])} | {f(x['ksum5'])} | {f(x['rho'])} | {x['rec_ratio']:.1f} | {x['inc_tb']/x['rho']:.2f} | {x['best']/x['rho']:.2f} | {x['ksum5']/x['rho']:.2f} |")
open('table.md','w').write('\n'.join(out)+'\n')
