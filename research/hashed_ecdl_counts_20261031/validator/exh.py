import sys,math,collections
import os; sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import recompute as R
B=R.load('/dev/shm/restore/snapshot-1730/sessions/B/records.jsonl')
rho={(r['workload']['workload_id']):r for r in B if r['arm']=='rho-neg'}
tab=collections.defaultdict(list)
for r in B:
    if r['arm'].startswith('native') and not r['arm'].endswith('-aa') and r['outcome']['status']=='exhausted':
        k=r['workload']['workload_id']; 
        for w in (0.0,1.0):
            s=R.priced_s(r,w); rs=R.priced_s(rho[k],w)
            tab[(r['arm'],R.curve_key(r),w)].append(s/rs)
print('exhausted native rows: spent S / paired rho S (min..max) per arm,curve,w')
for k,v in sorted(tab.items()):
    print(k,len(v),'min %.3g max %.3g'%(min(v),max(v)), 'below1:',sum(x<1 for x in v))
# also: all rows (including verified) native vs rho
allv=collections.defaultdict(list)
for r in B:
    if r['arm'].startswith('native') and not r['arm'].endswith('-aa'):
        k=r['workload']['workload_id']; allv[(r['arm'],r['outcome']['status'])].append(R.priced_s(r,0.0)/R.priced_s(rho[k],0.0))
for k,v in sorted(allv.items()): print(k,len(v),'min ratio %.3g'%min(v))
