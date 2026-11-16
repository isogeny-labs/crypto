from pathlib import Path
import json,hashlib,os,resource,sys,time
from research.quadratic_baseline_pilot_20261115_v63.worker import invoke
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 import research.quadratic_baseline_pilot_20261115_v63.worker as proc
 proc.P=P;out={'records':[],'qualified_speedup':None};error=None
 try:
  data=json.loads((P/'DATA.json').read_text());expected=set(map(tuple,json.loads(Path(data['expected']).read_text())['out']['original_tuples']));env=os.environ.copy();env.update(RAYON_NUM_THREADS='1',OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1')
  for mode in ['verified','profile']:
   cmd=[sys.executable,'-m','research.packed_cubic_trials_20261115_v65.child',data['descriptor'],data['binary'],mode];r=invoke(cmd,env,mode);v=json.loads(r['stdout']);assert not r['outer_timeout']and r['exit_code']==0 and v['complete_original_three_source'];assert set(map(tuple,v['original_tuples']))==expected;stats=v['native_stats'];assert stats['packed_calls']==262144 and stats['wide_inner_calls']==0
   if mode=='verified':assert stats['verified_packed_calls']==262144
   else:
    assert stats['verified_packed_calls']==0 and stats['profile_total_ns']>0;total=stats['profile_total_ns'];out['instrumented_native_stage_fractions']={name:stats['profile_'+name+'_ns']/total for name in ['setup','fiber','support']};f=out['instrumented_native_stage_fractions']['fiber'];out['conditional_amdahl_limit_if_only_fiber_accelerated']=1/(1-f);out['profile_model_assumption']='Instrumented native stage fractions; unchanged other-stage cost; excludes final serialization and parent startup/lift. This is not an observed speedup or a clean-runtime bound.'
   r['parsed']=v;payload=json.dumps(r,sort_keys=True,separators=(',',':')).encode()
   with(P/(mode+'.json')).open('xb')as f:f.write(payload)
   out['records'].append({'mode':mode,'wall_seconds':r['wall_seconds'],'wait4_peak_rss_kib':r['wait4_peak_rss_kib'],'native_stats':stats,'exact_root_count':v['exact_root_count'],'raw_sha256':hashlib.sha256(payload).hexdigest()})
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<75
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Verify every full-width packed affine certificate,then separately profile complete original-root pipeline.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Use clean cold comparisons separately;50xrequires measured savings across dominant stages or exact search pruning.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
