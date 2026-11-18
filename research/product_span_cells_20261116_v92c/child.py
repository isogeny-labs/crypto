import json,time,sys,os,resource
from pathlib import Path
START=time.monotonic()
from .kernel import certificate
from research.disjoint_separator_20261116_v92.kernel import Prefix
p=Path(sys.argv[1]);d=json.loads(p.read_text());spec=d['actual'];S=Prefix(spec['n'],spec['mod'],spec['d']);rows=[]
for k in d['dimensions']:
 for q in spec['queries']:
  u0=q['u']&~((1<<k)-1);v=certificate(S,q['y'],u0,k,q['B'],sys.argv[2]);rows.append({'id':q['id'],'kind':q['kind'],'k':k,'u0':u0,'y':q['y'],**v})
out={'complete':True,'certificates':rows,'certificate_count':len(rows),'cpu_affinity':sorted(os.sched_getaffinity(0)),'child_peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss};trial=json.dumps(out,separators=(',',':'))
if time.monotonic()-START>=10:out['complete']=False
print(json.dumps(out,separators=(',',':')))
