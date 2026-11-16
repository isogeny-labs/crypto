from pathlib import Path
import json,sys,time,resource,os
START=time.monotonic()
from research.prime_orbit_roots_20261116_v71.solver import solve
from .portfolio import portfolio,method
Path(sys.argv[3]).write_text('post-exec imports complete\n')
inst=json.loads(Path(sys.argv[1]).read_text());m=sys.argv[2]
out=portfolio(inst)if m=='portfolio'else solve(inst,m)
out['chosen_method']=method(inst)if m=='portfolio'else m
out['exact_root_count']=len(out['roots']);out['child_peak_rss_kib']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss;out['cpu_affinity']=sorted(os.sched_getaffinity(0))
trial=json.dumps(out,separators=(',',':'))
if time.monotonic()-START>=7:
 out['complete']=False;out['exact_root_count']=None;out['reason']='late child result'
print(json.dumps(out,separators=(',',':')))
