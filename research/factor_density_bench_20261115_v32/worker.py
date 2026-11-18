"""Prepared cold component comparison; no executions yet."""
from pathlib import Path
import json,hashlib,resource,time,sys,subprocess,statistics,collections
from rootfinder import binaryfield as bf
from research.compiled_subspace_oracles_20261115_v31 import worker as o
from research.gold5_cells_20261115_v22 import support
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
INPUT=R/'research/factor_density_bench_20261115_v32/DATA.json'
def trace_result(F,H,precheck):
 if precheck and not support.supported(F.field,H,18):return dict(supported=False,roots=None)
 result=bf.binary_field_roots(H,F.field,seed=0,max_trials=128);assert result['status']=='complete'
 work=list(H);roots=[]
 for x in result['roots']:
  count=0
  while len(work)>1 and bf.evaluate_poly(work,x,F.field)==0:
   q,rem=bf.divmod_poly(work,[x,1],F.field);assert not rem;work=list(q);count+=1
  if count:roots.append((x,count))
 good=len(work)==1 and all(x<1<<18 for x,m in roots)
 return dict(supported=good,roots=roots if good else None)
def child(method):
 begin=time.monotonic();data=json.loads(INPUT.read_text());F=o.Binary(83,data['modulus']);B=data['basis'];rows=[];solve_seconds=verify_seconds=0;profile_seconds={}
 for group in data['groups']:
  group_start=time.monotonic()
  for case in group['cases']:
   H=case['H'];planted=case['planted_roots'];t=time.monotonic();assert o.from_roots(F,planted)==H
   for x in H+B:F.field.validate(x)
   verify_seconds+=time.monotonic()-t;t=time.monotonic()
   result=o.a.supported_roots(F,H,B)if method=='subspace'else trace_result(F,H,method=='support_trace')
   solve_seconds+=time.monotonic()-t;t=time.monotonic();good=all(x<1<<18 for x in planted);expected=sorted(collections.Counter(planted).items())if good else None
   assert result['supported']==good and result['roots']==expected;rows.append(dict(profile=group['name'],supported=good,roots=expected));verify_seconds+=time.monotonic()-t
  profile_seconds[group['name']]=time.monotonic()-group_start
 value=dict(method=method,rows=rows,roots_sha256=hashlib.sha256(json.dumps(rows,sort_keys=True,separators=(',',':')).encode()).hexdigest(),profile_seconds=profile_seconds,solve_seconds=solve_seconds,verify_seconds=verify_seconds,pre_stdout_seconds=time.monotonic()-begin,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss)
 print(json.dumps(value,separators=(',',':')))

def main():
 out=dict(records=[]);error=None
 try:
  for repetition in range(3):
   order=['trace','support_trace','subspace']if repetition%2==0 else['subspace','support_trace','trace']
   for method in order:
    t=time.monotonic();run=subprocess.run([sys.executable,'-m','research.factor_density_bench_20261115_v32.worker',method],cwd=R,capture_output=True,text=True,timeout=8)
    elapsed=time.monotonic()-t;assert run.returncode==0,(run.returncode,run.stderr[-1000:]);value=json.loads(run.stdout);value.update(repetition=repetition,cold_wall_seconds=elapsed,stdout_bytes=len(run.stdout.encode()));out['records'].append(value)
  assert len({v['roots_sha256']for v in out['records']})==1
  out['median_cold_wall']={m:statistics.median(v['cold_wall_seconds']for v in out['records']if v['method']==m)for m in['trace','support_trace','subspace']}
  out['AA_spread']={m:max(v['cold_wall_seconds']for v in out['records']if v['method']==m)/min(v['cold_wall_seconds']for v in out['records']if v['method']==m)-1 for m in out['median_cold_wall']}
  out['component_ratio']=min(out['median_cold_wall'][m]for m in['trace','support_trace'])/out['median_cold_wall']['subspace']
  out['profile_median_loop']={p:{m:statistics.median(v['profile_seconds'][p]for v in out['records']if v['method']==m)for m in ['trace','support_trace','subspace']}for p in ['reject_only','sparse_support','supported_only']}
  out['profile_component_ratios']={p:min(v[m]for m in ['trace','support_trace'])/v['subspace']for p,v in out['profile_median_loop'].items()}
 except Exception as exc:error=type(exc).__name__+': '+str(exc);out['component_ratio']=None
 if time.monotonic()-START>82:error='late parent reporting';out['component_ratio']=None;out['profile_component_ratios']=None
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<524288
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(profile_median_loop=out.get('profile_median_loop'),profile_component_ratios=out.get('profile_component_ratios'),error=error,median_cold_wall=out.get('median_cold_wall'),component_ratio=out.get('component_ratio'),AA_spread=out.get('AA_spread'),cold_calls=len(out['records']),child_rss_kib=[v['peak_rss_kib']for v in out['records']],parent_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),external_reviewer=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False,qualified_isolation=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Cold paid matching support-qualified root/multiplicity component comparison, notF4/F5/F6 or globalroot speed.',evidence=ev,next_actions=['Use fastestcompleted specialized arm andAAspread; ifslow optimize compiledprefix reuse andexactlowdegree leaves in freshvariant, not identicalretry. Fulloriginalrootgate remainsopen.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':
 if sys.argv[1]in['trace','support_trace','subspace']:child(sys.argv[1])
 else:main()
