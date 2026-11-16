from pathlib import Path
import hashlib,json,resource,statistics,subprocess,sys,time
P=Path(__file__).resolve().parent;START=time.monotonic()

def main():
 out=dict(records=[]);error=None
 try:
  preflight=json.loads((P/'preflight.json').read_text());assert preflight['error']is None;expected={r['h_index']:r for r in preflight['out']['charts']}
  for repetition,order in enumerate([['reference','compressed'],['compressed','reference'],['reference','compressed']]):
   for method in order:
    assert time.monotonic()-START<65;t=time.monotonic();run=subprocess.run([sys.executable,'-m','research.root_validated_cold_20261115_v50.child',method],capture_output=True,text=True,timeout=25);wall=time.monotonic()-t
    out['last_subprocess']=dict(exit_code=run.returncode,stdout=run.stdout,stderr=run.stderr);assert run.returncode==0,run.stderr;row=json.loads(run.stdout)
    for chart in row['charts']:
     ref=expected[chart['h_index']];assert chart['complete']and chart['candidate_sha256']==ref['candidate_sha256']and chart['norms']==ref['norms']
    row.update(repetition=repetition,cold_wall_seconds=wall,exit_code=run.returncode,stdout=run.stdout,stderr=run.stderr,rss_sum_peak_upper_bound_kib=row['parent_peak_rss_kib']+max(c['native_peak_rss_kib']for c in row['charts']));out['records'].append(row)
  out['median_cold_seconds']={m:statistics.median(r['cold_wall_seconds']for r in out['records']if r['method']==m)for m in ['reference','compressed']};out['AA_spread']={m:max(r['cold_wall_seconds']for r in out['records']if r['method']==m)/min(r['cold_wall_seconds']for r in out['records']if r['method']==m)-1 for m in ['reference','compressed']};out['conditional_ratio']=out['median_cold_seconds']['reference']/out['median_cold_seconds']['compressed'];out.pop('last_subprocess')
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),separators=(',',':'))
 if time.monotonic()-START>=78:error='late serialized result';out['conditional_ratio']=None
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Four-chart fully verified cold conditional solve, same native validator in both specialized scanner arms.',evidence=dict(error=error,median_cold_seconds=out.get('median_cold_seconds'),AA_spread=out.get('AA_spread'),conditional_ratio=out.get('conditional_ratio'),rss_sum_upper_bounds_kib=[r['rss_sum_peak_upper_bound_kib']for r in out['records']],worker_seconds=time.monotonic()-START,parent_peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),qualified_isolation=False,qualified_speedup=None,complete_global_root_search=False,release_gate_claim=False),next_actions=['Expand general h/k and curve/field coverage; these specialized conditional timings do not satisfy the original20-instance genuineF4/F5/F6 release gate.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
