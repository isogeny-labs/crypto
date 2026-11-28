import json,time,sys,resource,os
from pathlib import Path
START=time.monotonic()
from .kernel import solve,Prefix
from research.twist_auxiliary_20261116_v86.algebra import s3
p=Path(sys.argv[1]);data=json.loads(p.read_text());spec=data['cases'][int(sys.argv[2])];method=sys.argv[3];roots,stats=solve(spec,method);S=Prefix(spec['n'],spec['mod'],spec['d'])
for x,u in roots:assert x<1<<S.d and s3(x,spec['y'],u,0,spec['B'],S.f)==0
out={'complete':True,'roots':roots,'exact_root_count':len(roots),'stats':stats,'cpu_affinity':sorted(os.sched_getaffinity(0)),'child_peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss};trial=json.dumps(out,separators=(',',':'))
if time.monotonic()-START>=10:out['complete']=False;out['exact_root_count']=None
print(json.dumps(out,separators=(',',':')))
