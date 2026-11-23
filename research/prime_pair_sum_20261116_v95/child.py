import json,sys,time,resource,os
from pathlib import Path
START=time.monotonic()
from .kernel import solve
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
d=json.loads(Path(sys.argv[1]).read_text());c=d['cold'][int(sys.argv[2])];method=sys.argv[3];v=solve(c,method);f=Field(c['p'],[0,1]);assert len(v['roots'])==len(set(map(tuple,v['roots'])))
for x,y in v['roots']:assert 0<=x<=c['M']and 0<=y<=c['M']and s3(x,y,c['u'],c['A'],c['B'],f)==0
v['cpu_affinity']=sorted(os.sched_getaffinity(0));v['process_peak_rss_kib']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss;v['input_id']=c['id'];serialized=json.dumps(v,separators=(',',':'))
if time.monotonic()-START>=d['child_report_seconds']:v.update({'complete':False,'exact_root_count':None,'error':'child reporting deadline'})
print(json.dumps(v,separators=(',',':')))
