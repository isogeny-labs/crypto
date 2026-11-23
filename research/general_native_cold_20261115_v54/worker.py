from pathlib import Path
import hashlib,json,resource,statistics,subprocess,sys,time
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()

def main():
 out=dict(records=[]);error=None
 try:
  expected={}
  for entry in json.loads((P/'DATA.json').read_text())['instances']:
   ref=json.loads((R/'research/general_native_20261115_v54'/(entry['name']+'_reference.json')).read_text());expected[entry['name']]=dict(solutions=ref['solutions'],candidate_sha256=hashlib.sha256(json.dumps(ref['candidate_pairs'],separators=(',',':')).encode()).hexdigest())
  for rep,order in enumerate([['reference','compressed'],['compressed','reference'],['reference','compressed']]):
   for method in order:
    assert time.monotonic()-START<65;t=time.monotonic();run=subprocess.run([sys.executable,'-m','research.general_native_cold_20261115_v54.child',method],capture_output=True,text=True,timeout=25);wall=time.monotonic()-t;out['last_subprocess']=dict(stdout=run.stdout,stderr=run.stderr,exit_code=run.returncode);assert run.returncode==0,run.stderr;row=json.loads(run.stdout)
    for case in row['cases']:
     e=expected[case['name']];assert case['complete']and case['solutions']==e['solutions']and case['candidate_sha256']==e['candidate_sha256']
    row.update(repetition=rep,cold_wall_seconds=wall,stdout=run.stdout,stderr=run.stderr,exit_code=run.returncode,rss_sum_upper_bound_kib=row['parent_peak_rss_kib']+max(c['native_peak_rss_kib']for c in row['cases']));out['records'].append(row)
  out.pop('last_subprocess');out['medians']={m:statistics.median(r['cold_wall_seconds']for r in out['records']if r['method']==m)for m in ['reference','compressed']};out['AA_spread']={m:max(r['cold_wall_seconds']for r in out['records']if r['method']==m)/min(r['cold_wall_seconds']for r in out['records']if r['method']==m)-1 for m in ['reference','compressed']};out['conditional_ratio']=out['medians']['reference']/out['medians']['compressed']
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),separators=(',',':'))
 if time.monotonic()-START>=78:error='late serialized output';out['conditional_ratio']=None
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Matched fully paid cold positive ordinary/Koblitz generalh/k conditional norm-chart solves.',evidence=dict(error=error,median_cold_seconds=out.get('medians'),AA_spread=out.get('AA_spread'),conditional_ratio=out.get('conditional_ratio'),rss_sum_upper_bounds_kib=[r['rss_sum_upper_bound_kib']for r in out['records']],worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),qualified_isolation=False,qualified_speedup=None,complete_global_root_search=False,release_gate_claim=False),next_actions=['Use actual phase/memory evidence to optimize generic compilation or add complete original auxiliary reconstruction; no repeated unchanged timing retry.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
