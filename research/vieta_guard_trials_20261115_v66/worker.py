from pathlib import Path
import json,hashlib,os,resource,sys,time
from research.quadratic_baseline_pilot_20261115_v63.worker import invoke
from research.vieta_guard_20261115_v66.oracle import F,rootpoly,peval
P=Path(__file__).resolve().parent;START=time.monotonic()
def rank(values):
 piv={}
 for v in values:
  while v:
   j=v.bit_length()-1
   if j in piv:v^=piv[j]
   else:piv[j]=v;break
 return len(piv)
def main():
 import research.quadratic_baseline_pilot_20261115_v63.worker as proc
 proc.P=P;out={'records':[],'qualified_speedup':None};error=None
 try:
  data=json.loads((P/'DATA.json').read_text());expected=set(map(tuple,json.loads(Path(data['expected']).read_text())['out']['original_tuples']));v=json.loads(Path(data['descriptor']).read_text());f=F(v['field_degree'],v['modulus']);env=os.environ.copy();env.update(RAYON_NUM_THREADS='1',OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1')
  for prefix in data['prefixes']:
   L=rootpoly(range(1<<prefix),f);assert all(peval(L,1<<i,f)==0 for i in range(prefix));W=[peval(L,1<<i,f)for i in range(prefix,v['d'])];assert rank(W)==v['d']-prefix;want_rank=rank(f.mul(a,b)for i,a in enumerate(W)for b in W[i:]);name='prefix'+str(prefix);cmd=[sys.executable,'-m','research.vieta_guard_trials_20261115_v66.child',data['descriptor'],data['binary'],'verified',str(prefix)];r=invoke(cmd,env,name);child=json.loads(r['stdout']);assert not r['outer_timeout']and r['exit_code']==0 and child['complete_original_three_source'];assert set(map(tuple,child['original_tuples']))==expected;s=child['native_stats'];assert s['guard_prefix']==prefix and s['guard_tests']==261457 and s['guard_rejected']==s['guard_verified_rejected']and s['guard_product_rank']==want_rank;r['parsed']=child;payload=json.dumps(r,sort_keys=True,separators=(',',':')).encode()
   with(P/(name+'.json')).open('xb')as file:file.write(payload)
   out['records'].append({'prefix':prefix,'product_rank':want_rank,'quotient_dimension':83-want_rank,'image_dimension':len(W),'tests':s['guard_tests'],'rejected':s['guard_rejected'],'verified_rejected':s['guard_verified_rejected'],'exact_root_count':child['exact_root_count'],'wall_seconds_including_verification':r['wall_seconds'],'wait4_peak_rss_kib':r['wait4_peak_rss_kib'],'raw_sha256':hashlib.sha256(payload).hexdigest()});assert time.monotonic()-START<65
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<75
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Independent product-space ranks and full oracle verification of every actual guard rejection for all preregistered prefixes.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Measure all frozen guards with clean complete-pipeline cold costs;promote only if paid total cost improves.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
