from pathlib import Path
import json,hashlib,statistics,subprocess,resource,sys,time
P=Path(__file__).resolve().parent;START=time.monotonic()

def main():
 out=dict(records=[]);error=None
 try:
  for repetition,order in enumerate([['reference','compressed'],['compressed','reference'],['reference','compressed']]):
   for method in order:
    assert time.monotonic()-START<65;t=time.monotonic();run=subprocess.run([sys.executable,'-m','research.root_compressed_cold_20261115_v48.child',method],capture_output=True,text=True,timeout=20);wall=time.monotonic()-t;assert run.returncode==0,run.stderr;row=json.loads(run.stdout);assert row['complete']and row['candidates']==249;row.update(repetition=repetition,cold_wall_seconds=wall,stderr=run.stderr);out['records'].append(row)
  assert len({r['candidate_sha256']for r in out['records']})==1 and all(not r['norms']for r in out['records'])
  out['median_cold_seconds']={m:statistics.median(r['cold_wall_seconds']for r in out['records']if r['method']==m)for m in ['reference','compressed']}
  out['AA_spread']={m:max(r['cold_wall_seconds']for r in out['records']if r['method']==m)/min(r['cold_wall_seconds']for r in out['records']if r['method']==m)-1 for m in ['reference','compressed']};out['conditional_ratio']=out['median_cold_seconds']['reference']/out['median_cold_seconds']['compressed']
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),separators=(',',':'))
 if time.monotonic()-START>=78:error='late serialized result';out['conditional_ratio']=None
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,median_cold_seconds=out.get('median_cold_seconds'),AA_spread=out.get('AA_spread'),conditional_ratio=out.get('conditional_ratio'),worker_seconds=time.monotonic()-START,parent_peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,native_peak_rss_kib=[r['native_peak_rss_kib']for r in out['records']],cold_python_peak_rss_kib=[r['parent_peak_rss_kib']for r in out['records']],raw_sha256=hashlib.sha256(raw).hexdigest(),qualified_speedup=None,qualified_isolation=False,complete_global_root_search=False,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Fully paid cold conditional-chart comparison, not a genuine Groebner or full-target ratio.',evidence=ev,next_actions=['Use measured phase costs to select native support/verification or point-table amortization; never infer total gain from native-only timing.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
