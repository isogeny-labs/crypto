"""Cold equally cached block/monolithic exact fiber comparison."""
from pathlib import Path
import hashlib,json,resource,statistics,subprocess,sys,time
from research.norm_block_reuse_20261115_v36 import worker as o
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def child(method):
 begin=time.monotonic();prior=json.loads((R/'research/norm_block_reuse_20261115_v36/raw.json').read_text());assert prior['error']is None;expected=prior['out']['actual'];f,ctx,C=o.actual_context();setup=time.monotonic()-begin
 if method=='block':t=time.monotonic();C.prepare_blocks();setup+=time.monotonic()-t
 rows=[];core=verify=0
 for k in C.ks:
  for h in C.hs:
   t=time.monotonic();fiber=C.block(h,k)if method=='block'else C.mono(h,k);core+=time.monotonic()-t;t=time.monotonic();raw=o.original(f,ctx,h,k,fiber)
   B=[]if raw is None else o.b.canonical_basis(raw['basis']);part=None if raw is None else o.b.apply_residue(raw['particular'],B)
   row=dict(h=h,k=k,consistent=raw is not None,nullity=None if raw is None else len(B),canonical_sha256=hashlib.sha256(json.dumps([part,B],separators=(',',':')).encode()).hexdigest());assert row==expected['fibers'][len(rows)]
   rows.append(dict(h=h,k=k,particular=part,basis=B));verify+=time.monotonic()-t
 blob=json.dumps(rows,separators=(',',':')).encode();assert len(rows)==128
 print(json.dumps(dict(method=method,fibers=len(rows),full_output_sha256=hashlib.sha256(blob).hexdigest(),full_output_bytes=len(blob),core_seconds=core,setup_seconds=setup,restore_verify_seconds=verify,pre_stdout_seconds=time.monotonic()-begin,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss),separators=(',',':')))
def main():
 out=dict(records=[]);error=None
 try:
  for rep in range(3):
   for method in(['monolithic','block']if rep%2==0 else['block','monolithic']):
    t=time.monotonic();r=subprocess.run([sys.executable,'-m','research.norm_block_cost_20261115_v36.worker',method],cwd=R,capture_output=True,text=True,timeout=12);elapsed=time.monotonic()-t
    assert r.returncode==0,(r.returncode,r.stderr[-1000:]);row=json.loads(r.stdout);row.update(repetition=rep,cold_wall_seconds=elapsed);out['records'].append(row)
   assert time.monotonic()-START<78
  assert len({x['full_output_sha256']for x in out['records']})==1
  for key in['cold_wall_seconds','core_seconds','setup_seconds','restore_verify_seconds']:
   out['median_'+key]={m:statistics.median(x[key]for x in out['records']if x['method']==m)for m in['monolithic','block']}
  out['AA_spread']={m:max(x['cold_wall_seconds']for x in out['records']if x['method']==m)/min(x['cold_wall_seconds']for x in out['records']if x['method']==m)-1 for m in['monolithic','block']}
  out['paid_component_ratio']=out['median_cold_wall_seconds']['monolithic']/out['median_cold_wall_seconds']['block'];out['core_ratio']=out['median_core_seconds']['monolithic']/out['median_core_seconds']['block'];out['prediction_2x']=out['paid_component_ratio']>=2
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(out=out,error=error,worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),qualified_speedup=None,release_gate_claim=False,complete_global_root_search=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Paid exact affine-fiber block reuse comparison with equally cached strong monolithic baseline.',evidence=ev,next_actions=['If full gain below2x inspect canonical/output floor and reducedmatrix assembly; create changed variant rather than identical retry. Seek wholecell support exclusion and retain original/global gates.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<16384 and time.monotonic()-START<84;print(text)
if __name__=='__main__':
 if len(sys.argv)>1 and sys.argv[1]in['block','monolithic']:child(sys.argv[1])
 else:main()
