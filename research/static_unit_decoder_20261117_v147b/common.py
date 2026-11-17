from pathlib import Path
import json,sys,time,hashlib,resource
P=Path(__file__).resolve().parent

def memory_snapshot():
 result={}
 for line in Path('/proc/self/status').read_text().splitlines():
  key,_,value=line.partition(':')
  if key in ['VmHWM','VmRSS','VmSize']:result[key]=value.strip()
 return result

MEMORY_START=memory_snapshot()

def finish(out,error,start,summary,next_action):
 out['post_exec_memory']={'start':MEMORY_START,'finish':memory_snapshot()}

 if time.monotonic()-start>=78:error='worker lifetime budget exceeded'
 if error:out['complete']=False;out['exact_root_count']=None;out['all_registered_contexts_complete']=False;out['complete_early_census']=False;out['conditional_parameter_upper_bound']=None;out['global_original_root_count']=None
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode()
 if time.monotonic()-start>=78:
  error='postserialization lifetime budget exceeded';out.update(complete=False,exact_root_count=None,all_registered_contexts_complete=False,complete_early_census=False,conditional_parameter_upper_bound=None,global_original_root_count=None);raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode()
 with(P/(sys.argv[2]+'.json')).open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':summary,'evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-start,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':[next_action]};text=json.dumps(result);assert time.monotonic()-start<84;print(text)
