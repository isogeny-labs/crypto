from pathlib import Path
import json,sys,time,resource
from research.orbit_roots_20261116_v70.solver import solve
from .field_first import solve as field_first
START=time.monotonic()
v=json.loads(Path(sys.argv[1]).read_text());method=sys.argv[2];out=field_first(v)if method=='field_first'else solve(v,method);trial=json.dumps(out,separators=(',',':'))
if time.monotonic()-START>=8:out['complete']=False;out['exact_root_count']=None;out['reason']='late childserialization'
else:out['exact_root_count']=len(out['roots'])
out['child_peak_rss_kib']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss;print(json.dumps(out,separators=(',',':')))
