import json,time,sys,resource,os
from pathlib import Path
START=time.monotonic()
from .solver import solve
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
p=Path(sys.argv[1]);d=json.loads(p.read_text());case=d['cases'][int(sys.argv[2])];r=solve(case,sys.argv[3]);f=Field(case['p'],[0,1]);m=case['arity']
for row in r['roots']:
 u=row[m:];assert s3(row[0],row[1],u[0],case['A'],case['B'],f)==0
 for i in range(2,m-1):assert s3(u[i-2],row[i],u[i-1],case['A'],case['B'],f)==0
 assert s3(u[-1],row[m-1],case['Q'][0],case['A'],case['B'],f)==0
r['cpu_affinity']=sorted(os.sched_getaffinity(0));r['child_peak_rss_kib']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss;trial=json.dumps(r,separators=(',',':'))
if time.monotonic()-START>=10:r['complete']=False;r['exact_root_count']=None
print(json.dumps(r,separators=(',',':')))
