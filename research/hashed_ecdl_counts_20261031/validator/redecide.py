import os
import sys,json,importlib.util
from pathlib import Path
spec=importlib.util.spec_from_file_location('decide',''+os.path.dirname(os.path.abspath(__file__))+'/../analysis/decide.py')
D=importlib.util.module_from_spec(spec); spec.loader.exec_module(D)
raw=Path(''+os.path.dirname(os.path.abspath(__file__))+'/../results/raw')
dec=json.load(open(''+os.path.dirname(os.path.abspath(__file__))+'/../results/decision.json'))
dirs={'C':[Path('/dev/shm/restore/sessions/C-10-14'),Path('/dev/shm/sessions/C-16')],'B':[Path('/dev/shm/restore/sessions/B')]}
recs={t:[r for d in ds for r in D.read_records(d)] for t,ds in dirs.items()}
band={}
for t in 'CB':
    for rk in json.load(open(raw/f'band-{t}.json'))['ranks']:
        band[(rk['curve'],rk['a'],rk['b'])]=rk['verdict']; band[(rk['curve'],rk['b'],rk['a'])]=rk['verdict']
cells=[]
for cid,t,a,b,lb in D.FAMILY:
    for d in dirs[t]:
        by_w={w:(json.load(open(raw/f'compare-{cid}-{d.name}-w{w}.json'))['result'],None) for w in D.WEIGHTS}
        slugs=sorted({r['workload']['curve']['slug'] for r in D.read_records(d)})
        for slug in slugs:
            c=D.cell_from_compares(by_w,D.arm_rows(recs[t],a,slug),D.arm_rows(recs[t],b,slug),slug)
            c.update({'comparison':cid,'track':t,'a':a,'b':b,'lower_bound_only':lb})
            raw_o=band.get((slug,a,b)); c['band_order']=raw_o
            c['p'],c['pre']=D.p_value(c,lb,raw_o,a,b)
            cells.append(c)
D.holm(cells)
# compare to decision.json
bad=0
key=lambda c:(c['comparison'],c['slug'])
dm={key(c):c for c in dec['cells']}
for c in cells:
    d=dm[key(c)]
    for f in ['complete','p','pre','verdict','holm_step','holm_level','band_order','ratio','ci95_linearised_t','descriptive_ratio_incomplete_unknown','verified_pairs']:
        if c.get(f)!=d.get(f): bad+=1; print('DIFF',key(c),f,c.get(f),d.get(f))
print('cells',len(cells),'diffs',bad)
# P1/P4
c_rho=sorted((c for c in cells if c['comparison']=='C-rho'),key=lambda c:c.get('log2_r',99))
p1=len(c_rho)==4 and all(c['verdict']=='supported_within_scope' and c['direction']=='B > A' for c in c_rho) and all(c_rho[i]['ratio'][j]<c_rho[i+1]['ratio'][j] for i in range(3) for j in range(3))
print('P1 recomputed',p1,'decision',dec['P1'])
native=[c for c in cells if c['lower_bound_only'] and c['complete']]
print('P4 complete cells',[(c['comparison'],c['slug'][:14],c['verdict']) for c in native],'decision',dec['P4']['verdict'])
