from pathlib import Path
import json,time,sys,resource,os
START=time.monotonic()
from research.prime_orbit_roots_20261116_v71.solver import solve
Path(sys.argv[3]).write_text('post-exec imports complete\n');instances=json.loads(Path(sys.argv[1]).read_text());results=[solve(v)for v in instances];out={'complete':all(v['complete']for v in results),'roots':[v['roots']for v in results],'candidate_shifts':[v['candidate_shifts']for v in results],'orbit_polynomial':[v['orbit_polynomial']for v in results],'exact_root_count':sum(len(v['roots'])for v in results),'batch_size':len(results),'child_peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'cpu_affinity':sorted(os.sched_getaffinity(0))};trial=json.dumps(out,separators=(',',':'))
if time.monotonic()-START>=7:out['complete']=False;out['exact_root_count']=None;out['reason']='late reference batch'
print(json.dumps(out,separators=(',',':')))
