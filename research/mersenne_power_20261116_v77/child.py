from pathlib import Path
import sys,json,time,resource,os
START=time.monotonic()
from research.prime_orbit_roots_20261116_v71.solver import solve as reference
from .solver import solve
from .kernel import reset,COUNTS
Path(sys.argv[3]).write_text('post-exec imports complete\n');v=json.loads(Path(sys.argv[1]).read_text());reset();out=(solve if sys.argv[2]=='chain'else reference)(v);out['exact_root_count']=len(out['roots']);out['operation_counts']=dict(COUNTS);out['child_peak_rss_kib']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss;out['cpu_affinity']=sorted(os.sched_getaffinity(0));trial=json.dumps(out,separators=(',',':'))
if time.monotonic()-START>=7:out['complete']=False;out['exact_root_count']=None;out['reason']='late child serialization'
print(json.dumps(out,separators=(',',':')))
