import json,sys,time,resource,os
from pathlib import Path
START=time.monotonic()
from .kernel import solve
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
d=json.loads(Path(sys.argv[1]).read_text());c=d['cold'][int(sys.argv[2])];v=solve(c,sys.argv[3]);f=Field(2,c['mod']);assert len(v['roots'])==len(set(map(tuple,v['roots'])))
for x1,x2,x3,x4,x5,a,z,b in v['roots']:
 assert all(0<=x<1<<c['d']for x in[x1,x2,x3,x4,x5])and all(0<=u<1<<c['n']for u in[a,z,b]);assert all(s3(r,s,t,0,1,f)==0 for r,s,t in[(x1,x2,a),(a,x3,z),(z,x4,b),(b,x5,c['T'])])
v.update({'cpu_affinity':sorted(os.sched_getaffinity(0)),'process_peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss});serialized=json.dumps(v,separators=(',',':'))
if time.monotonic()-START>=7:v.update({'complete':False,'exact_root_count':None,'error':'reportdeadline'})
print(json.dumps(v,separators=(',',':')))
