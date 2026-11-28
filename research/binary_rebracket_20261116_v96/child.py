import sys,json,time,resource,os
from pathlib import Path
START=time.monotonic()
from .kernel import solve
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
d=json.loads(Path(sys.argv[1]).read_text());c=d['cold'][int(sys.argv[2])];v=solve(c,sys.argv[3]);f=Field(2,c['mod']);assert len(v['roots'])==len(set(map(tuple,v['roots'])))
for x,y,z in v['roots']:assert 0<=x<1<<c['d']and 0<=y<1<<c['d']and 0<=z<1<<c['n']and s3(c['a'],x,z,0,c['B'],f)==s3(z,y,c['b'],0,c['B'],f)==0
v.update({'cpu_affinity':sorted(os.sched_getaffinity(0)),'process_peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss});text=json.dumps(v,separators=(',',':'))
if time.monotonic()-START>=d['report_seconds']:v.update({'complete':False,'exact_root_count':None,'error':'child report deadline'})
print(json.dumps(v,separators=(',',':')))
