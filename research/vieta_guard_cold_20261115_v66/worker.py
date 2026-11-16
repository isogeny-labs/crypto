from pathlib import Path
import json,hashlib,os,resource,statistics,sys,time
from research.quadratic_baseline_pilot_20261115_v63.worker import invoke
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 import research.quadratic_baseline_pilot_20261115_v63.worker as proc
 proc.P=P;out={'runs':[],'qualified_speedup':None};error=None
 try:
  data=json.loads((P/'DATA.json').read_text());expected=set(map(tuple,json.loads(Path(data['expected']).read_text())['out']['original_tuples']));env=os.environ.copy();env.update(RAYON_NUM_THREADS='1',OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1');times={k:[]for k in ['reference','g6','g7','g8']}
  for rep,order in enumerate(data['order']):
   for backend in order:
    assert time.monotonic()-START<63,'insufficient remaining child budget';name=backend+'_'+str(rep);binary=data['reference']if backend=='reference'else data['candidate'];mode='reference'if backend=='reference'else'guard';prefix='0'if backend=='reference'else backend[1:];cmd=[sys.executable,'-m','research.vieta_guard_trials_20261115_v66.child',data['descriptor'],binary,mode,prefix];r=invoke(cmd,env,name);v=json.loads(r['stdout']);assert r['exit_code']==0 and not r['outer_timeout']and v['complete_original_three_source'];assert set(map(tuple,v['original_tuples']))==expected;r['parsed']=v;payload=json.dumps(r,sort_keys=True,separators=(',',':')).encode()
    with(P/(name+'.json')).open('xb')as f:f.write(payload)
    times[backend].append(r['wall_seconds']);s=v['native_stats'];out['runs'].append({'backend':backend,'rep':rep,'wall_seconds':r['wall_seconds'],'wait4_peak_rss_kib':r['wait4_peak_rss_kib'],'native_peak_rss_kib':v['native_peak_rss_kib'],'exact_root_count':v['exact_root_count'],'guard_tests':s.get('guard_tests',0),'guard_rejected':s.get('guard_rejected',0),'raw_sha256':hashlib.sha256(payload).hexdigest()})
  out['medians']={k:statistics.median(v)for k,v in times.items()};out['repeat_span_fraction']={k:(max(v)-min(v))/statistics.median(v)for k,v in times.items()};out['own_reference_ratios']={k:out['medians']['reference']/out['medians'][k]for k in ['g6','g7','g8']};out['comparison_scope']='All frozen prefixes on one exposed ordinaryGF83three-source input;complete paid cold pipeline;not genuineGB or50xqualification'
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<78
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Four cold complete-root repetitions for unguardedpacked and all three frozen projected support guards.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Promote only a paid improvement andretain allprefixes;derive exact whole-cell rejections for50xresearch rather than multiply microbenchmark ratios.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
