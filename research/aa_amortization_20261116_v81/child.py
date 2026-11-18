import json,time,sys,os,resource
from pathlib import Path
START=time.monotonic()
from .inputs import materialize,digest
from research.prime_orbit_roots_20261116_v71.solver import solve
Path(sys.argv[4]).write_text('post-exec imports complete\n');d=json.loads(Path(sys.argv[1]).read_text());n=int(sys.argv[2]);instances=materialize(n,d);h=digest(instances);assert h==d['input_hashes'][str(n)]
results=[]
for delta,inst in enumerate(instances):
 v=solve(inst);assert v['complete']and v['roots']==[[t-delta,u]for t,u in d['base_roots']];results.append(v)
out={'complete':True,'batch_size':n,'input_sha256':h,'roots':[v['roots']for v in results],'candidate_shifts':[v['candidate_shifts']for v in results],'orbit_polynomial':[v['orbit_polynomial']for v in results],'exact_root_count':sum(len(v['roots'])for v in results),'child_peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'cpu_affinity':sorted(os.sched_getaffinity(0))};trial=json.dumps(out,separators=(',',':'))
if time.monotonic()-START>=7:out['complete']=False;out['exact_root_count']=None
print(json.dumps(out,separators=(',',':')))
