from pathlib import Path
import hashlib,json,resource,sys,time
from research.general_native_20261115_v54.solve import solve
P=Path(__file__).resolve().parent;R=P.parents[1]
if __name__=='__main__':
 begin=time.monotonic();method=sys.argv[1];data=json.loads((P/'DATA.json').read_text());rows=[]
 for entry in data['instances']:
  instance=json.loads((R/entry['path']).read_text());result=solve(instance,R/'research/general_native_20261115_v54'/method,R/'research/general_native_20261115_v54/validation.so');assert result['complete'];rows.append(dict(name=entry['name'],solutions=result['solutions'],complete=result['complete'],candidates=len(result['candidate_pairs']),candidate_sha256=hashlib.sha256(json.dumps(result['candidate_pairs'],separators=(',',':')).encode()).hexdigest(),native_wall_seconds=result['native_wall_seconds'],total_seconds=result['total_seconds'],native_peak_rss_kib=result['native']['peak_rss_kib']))
 print(json.dumps(dict(method=method,cases=rows,internal_seconds=time.monotonic()-begin,parent_peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss),separators=(',',':')))
