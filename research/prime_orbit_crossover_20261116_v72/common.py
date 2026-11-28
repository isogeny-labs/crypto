from pathlib import Path
import json,hashlib,resource,sys,time
P=Path(__file__).resolve().parent
def finish(out,error,start,summary,next_action):
 if time.monotonic()-start>=78:error='worker lifetime exceeded evidence budget'
 if error:
  out['complete']=False;out['exact_root_count']=None
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode()
 with(P/(sys.argv[2]+'.json')).open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':summary,'evidence':{'out':out,'error':error,'raw_sha256':hashlib.sha256(raw).hexdigest(),'worker_seconds':time.monotonic()-start,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'qualified_speedup':None,'release_gate_claim':False},'next_actions':[next_action]}
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-start<84;print(text)
