import json,sys
import os; sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import recompute as R
def aa(recs,A,B,label):
    ra={(r['workload']['workload_id'],r['round']):r for r in recs if r['arm']==A}
    rb={(r['workload']['workload_id'],r['round']):r for r in recs if r['arm']==B}
    n=0;bad=[];stat=0;seed=0;ctr=0;ph=0;ex=0
    for k in sorted(set(ra)&set(rb)):
        x,y=ra[k],rb[k]; n+=1
        if x['outcome']['status']!=y['outcome']['status']: stat+=1
        if x['algorithm_seed']!=y['algorithm_seed']: seed+=1
        if (x['cost']['s'],x['cost']['total_gae'])!=(y['cost']['s'],y['cost']['total_gae']): bad.append(k)
        if x['counters']!=y['counters']: ctr+=1
        px=[(p['name'],p['adds'],p['doubles'],p['scalar_mults'],p['gae'],p['native']) for p in x['phases']]
        py=[(p['name'],p['adds'],p['doubles'],p['scalar_mults'],p['gae'],p['native']) for p in y['phases']]
        if px!=py: ph+=1
        if x['outcome']['status']=='exhausted': ex+=1
    print(f"{label}: pairs={n} status_mismatch={stat} seed_mismatch={seed} S/GAE_mismatch={len(bad)} counters_mismatch={ctr} phase_mismatch={ph} exhausted_pairs={ex} armsA={len(ra)} armsB={len(rb)}")
C1014=R.load('/dev/shm/restore/snapshot-1730/sessions/C-10-14/records.jsonl')
C16=R.load('/dev/shm/restore/v2/sessions/C-16/records.jsonl')
B=R.load('/dev/shm/restore/snapshot-1730/sessions/B/records.jsonl')
aa(C1014,'rho-neg','rho-neg-aa','C-10-14 rho-neg vs rho-neg-aa')
aa(C16,'rho-neg','rho-neg-aa','C-16 rho-neg vs rho-neg-aa')
aa(B,'rho-neg','rho-neg-aa','B rho-neg vs rho-neg-aa')
aa(B,'native-selectors','native-selectors-aa','B native-selectors vs -aa')
# seed sharing across arms per workload
for name,recs in [('C-10-14',C1014),('C-16',C16),('B',B)]:
    by={}
    for r in recs: by.setdefault((r['workload']['workload_id'],r['round']),set()).add(r['algorithm_seed'])
    print(name,'workload-rounds',len(by),'with >1 seed:',sum(1 for v in by.values() if len(v)>1))
# status census
import collections
for name,recs in [('C-10-14',C1014),('C-16',C16),('B',B)]:
    print(name,dict(collections.Counter(r['outcome']['status'] for r in recs)), 'warmup rows:',sum(r['warmup'] for r in recs),'rounds:',sorted({r['round'] for r in recs}))
