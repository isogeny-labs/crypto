import sys,json,resource,os,time
from pathlib import Path
START=time.monotonic()
from .kernel import solve
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
d=json.loads(Path(sys.argv[1]).read_text());c=d['cases'][int(sys.argv[2])];v=solve(c,sys.argv[3]);f=Field(c['p'],[0,1]);assert len(v['roots'])==len(c['targets'])
for u,roots in zip(c['targets'],v['roots']):
 assert len(roots)==len(set(map(tuple,roots)))
 for x,y in roots:assert 0<=x<=c['M']and 0<=y<=c['M']and s3(x,y,u,c['A'],c['B'],f)==0
v['cpu_affinity']=sorted(os.sched_getaffinity(0));v['process_peak_rss_kib']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss;serialized=json.dumps(v,separators=(',',':'))
if time.monotonic()-START>=d['report_seconds']:v.update({'complete':False,'exact_root_counts':None,'error':'reporting deadline'})
print(json.dumps(v,separators=(',',':')))
