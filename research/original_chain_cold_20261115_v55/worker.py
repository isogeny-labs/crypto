from pathlib import Path
import hashlib,json,resource,statistics,subprocess,sys,time
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def main():
 out={'records':[]};error=None
 try:
  expected={}
  for entry in json.loads((P/'DATA.json').read_text())['instances']:
   values=json.loads((R/'research/original_chain_lift_20261115_v55'/(entry['name']+'.json')).read_text())['tuples'];expected[entry['name']]=values
  for rep,order in enumerate([['reference','compressed'],['compressed','reference'],['reference','compressed']]):
   for method in order:
    assert time.monotonic()-START<68;t=time.monotonic();run=subprocess.run([sys.executable,'-m','research.original_chain_cold_20261115_v55.child',method],capture_output=True,text=True,timeout=27);out['last_subprocess']={'stdout':run.stdout,'stderr':run.stderr,'exit_code':run.returncode};assert run.returncode==0,run.stderr;row=json.loads(run.stdout)
    for case in row['cases']:
     values=case.pop('original_tuples');assert case['complete']and values==expected[case['name']];case['original_tuple_count']=len(values);case['original_tuple_sha256']=hashlib.sha256(json.dumps(values,separators=(',',':')).encode()).hexdigest()
    wall=time.monotonic()-t;row.update(repetition=rep,cold_wall_seconds=wall,stdout=run.stdout,stderr=run.stderr,exit_code=run.returncode,rss_sum_upper_bound_kib=row['parent_peak_rss_kib']+max(c['native_peak_rss_kib']for c in row['cases']));raw=json.dumps(row,sort_keys=True,separators=(',',':')).encode();path=P/(method+'_'+str(rep)+'.json')
    with path.open('xb')as file:file.write(raw)
    out['records'].append({'method':method,'repetition':rep,'cold_wall_seconds':wall,'rss_sum_upper_bound_kib':row['rss_sum_upper_bound_kib'],'raw_path':str(path.relative_to(R)),'raw_sha256':hashlib.sha256(raw).hexdigest(),'counts':[c['original_tuple_count']for c in row['cases']]})
  out.pop('last_subprocess');out['medians']={m:statistics.median(r['cold_wall_seconds']for r in out['records']if r['method']==m)for m in ['reference','compressed']};out['AA_spread']={m:max(r['cold_wall_seconds']for r in out['records']if r['method']==m)/min(r['cold_wall_seconds']for r in out['records']if r['method']==m)-1 for m in ['reference','compressed']};out['conditional_ratio']=out['medians']['reference']/out['medians']['compressed']
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps({'out':out,'error':error},separators=(',',':'))
 if time.monotonic()-START>=80:error='late serialized output'
 if error:out['conditional_ratio']=None
 raw=json.dumps({'out':out,'error':error},sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if not error else'inconclusive','summary':'Cold complete original-chain reconstruction conditional on all supported norm source multisets, with full literal verification and output serialization.','evidence':{'error':error,'medians':out.get('medians'),'AA_spread':out.get('AA_spread'),'conditional_ratio':out.get('conditional_ratio'),'rss_sum_upper_bounds_kib':[r['rss_sum_upper_bound_kib']for r in out['records']],'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_isolation':False,'qualified_speedup':None,'complete_global_root_search':False,'release_gate_claim':False},'next_actions':['Use full original-output phase measurements to target the remaining setup/verification bottleneck; separately establish original-to-norm global chart coverage and matching GB interfaces.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<86;print(text)
if __name__=='__main__':main()
