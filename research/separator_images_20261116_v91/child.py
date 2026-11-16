import json,time,sys,os,resource
from pathlib import Path
START=time.monotonic()
from .kernel import Solver
from research.twist_auxiliary_20261116_v86.algebra import s3
p=Path(sys.argv[1]);data=json.loads(p.read_text());spec=data['panels'][int(sys.argv[2])];method=sys.argv[3];S=Solver(spec['n'],spec['mod'],spec['d']);rows=[]
for q in spec['queries']:
 a,b,c=S.coefficients(q['y'],q['u'],q['B'])if q['kind']=='S3'else(q['a'],q['b'],q['c']);xs=S.solve(a,b,c,method)
 for x in xs:
  assert x<1<<S.d and S.f.mul(a,S.f.sq(x))^S.f.mul(b,x)==c
  if q['kind']=='S3':assert s3(x,q['y'],q['u'],0,q['B'],S.f)==0
 rows.append({'id':q['id'],'kind':q['kind'],'roots':xs})
out={'complete':True,'roots':rows,'exact_root_count':sum(len(r['roots'])for r in rows),'stats':{'queries':len(rows),'method':method},'cpu_affinity':sorted(os.sched_getaffinity(0)),'child_peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss};trial=json.dumps(out,separators=(',',':'))
if time.monotonic()-START>=10:out['complete']=False;out['exact_root_count']=None
print(json.dumps(out,separators=(',',':')))
