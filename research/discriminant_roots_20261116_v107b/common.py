from pathlib import Path
import json,sys,time,hashlib,resource
P=Path(__file__).resolve().parent
def finish(out,error,start,summary,next_action,budget=78):
    if time.monotonic()-start>=budget:error='worker lifetime budget exceeded'
    out['original_complete']=False;out['original_exact_root_count']=None
    raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();(P/(sys.argv[2]+'.json')).open('xb').write(raw)
    value={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':summary,'evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-start,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':[next_action]};text=json.dumps(value);assert time.monotonic()-start<budget+6;print(text)
