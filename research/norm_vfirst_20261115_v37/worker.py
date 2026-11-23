"""Changed representation: complete tiny oracle then paid three-arm comparison."""
from pathlib import Path
import hashlib,itertools,json,resource,statistics,subprocess,sys,time
from research.norm_block_reuse_20261115_v36 import worker as o
from research.norm_block_cost_20261115_v36 import worker as prior
from research.norm_vfirst_20261115_v37 import backend as a
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def check_controls():
 count=states=0
 for n,m in[(2,7),(3,11)]:
  f=o.old.Small(n,m);q=f.order
  for A in[0,1]:
   z,y,B=next((z,y,f.square(y)^f.mul(z,y)^f.mul(f.square(z),z)^f.mul(A,f.square(z)))for z in range(1,q)for y in range(q)if o.old.trace(f,A^z)==0 and(f.square(y)^f.mul(z,y)^f.mul(f.square(z),z)^f.mul(A,f.square(z))))
   ctx=o.old.context(f,A,z,y)[0]
   for dim in[1,2]:
    widths=tuple(min(n,j*(dim-1)+1)for j in[2,3,4]);C=a.Context(f,z,ctx['d'],list(range(q)),list(range(q)),widths);C.prepare_blocks()
    for h,k in itertools.product(range(q),repeat=2):
     got=o.points(C.raw_block(ctx,h,k));want=set()
     for r,s in itertools.product(range(q),repeat=2):
      if o.pack(o.literal(f,A,B,z,y,h,k,r,s),n,widths)==0:want.add(r|(s<<n))
      states+=1
     assert got==want;count+=1
 return dict(complete_fibers=count,literal_states=states)
def child(method):
 if method=='monolithic':prior.child(method);return
 if method=='sparse':
  from research.norm_sparse_lift_20261115_v36 import worker as previous
  previous.child('sparse');return
 begin=time.monotonic();d=json.loads((R/'research/norm_block_reuse_20261115_v36/raw.json').read_text());assert d['error']is None;expected=d['out']['actual'];f,ctx,oldC=o.actual_context();C=a.Context(f,ctx['z'],ctx['d'],oldC.hs,oldC.ks,oldC.widths);C.prepare_blocks();setup=time.monotonic()-begin;rows=[];core=verify=0
 for k in C.ks:
  for h in C.hs:
   t=time.monotonic();raw=C.raw_block(ctx,h,k);core+=time.monotonic()-t;t=time.monotonic()
   B=[]if raw is None else o.b.canonical_basis(raw['basis']);part=None if raw is None else o.b.apply_residue(raw['particular'],B)
   row=dict(h=h,k=k,consistent=raw is not None,nullity=None if raw is None else len(B),canonical_sha256=hashlib.sha256(json.dumps([part,B],separators=(',',':')).encode()).hexdigest());assert row==expected['fibers'][len(rows)]
   rows.append(dict(h=h,k=k,particular=part,basis=B));verify+=time.monotonic()-t
 blob=json.dumps(rows,separators=(',',':')).encode()
 print(json.dumps(dict(method=method,fibers=len(rows),full_output_sha256=hashlib.sha256(blob).hexdigest(),full_output_bytes=len(blob),core_seconds=core,setup_seconds=setup,restore_verify_seconds=verify,pre_stdout_seconds=time.monotonic()-begin,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss),separators=(',',':')))
def main():
 out=dict(records=[]);error=None
 try:
  out['controls']=check_controls()
  for rep in range(3):
   for method in(['monolithic','sparse','vfirst']if rep%2==0 else['vfirst','sparse','monolithic']):
    t=time.monotonic();r=subprocess.run([sys.executable,'-m','research.norm_vfirst_20261115_v37.worker',method],cwd=R,capture_output=True,text=True,timeout=10);elapsed=time.monotonic()-t
    assert r.returncode==0,(r.returncode,r.stderr[-1000:]);row=json.loads(r.stdout);row.update(repetition=rep,cold_wall_seconds=elapsed);out['records'].append(row)
   assert time.monotonic()-START<78
  assert len({x['full_output_sha256']for x in out['records']})==1
  for key in['cold_wall_seconds','core_seconds','setup_seconds','restore_verify_seconds']:
   out['median_'+key]={m:statistics.median(x[key]for x in out['records']if x['method']==m)for m in['monolithic','sparse','vfirst']}
  out['AA_spread']={m:max(x['cold_wall_seconds']for x in out['records']if x['method']==m)/min(x['cold_wall_seconds']for x in out['records']if x['method']==m)-1 for m in['monolithic','sparse','vfirst']}
  out['paid_vs_fastest_previous']=min(out['median_cold_wall_seconds'][m]for m in['monolithic','sparse'])/out['median_cold_wall_seconds']['vfirst'];out['paid_vs_monolithic']=out['median_cold_wall_seconds']['monolithic']/out['median_cold_wall_seconds']['vfirst']
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(out=out,error=error,worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),qualified_speedup=None,release_gate_claim=False,complete_global_root_search=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='v-first exact fibers with rankdeficient fallback; exact complete fibers and paid sameoutput three-arm comparison.',evidence=ev,next_actions=['Preserve modest/negative result; quantify normalized wholecell rank guards or compact lazyfiber contracts before further timing. Do not infer shrinkage of73bit search or originalGBgain.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<24576 and time.monotonic()-START<84;print(text)
if __name__=='__main__':
 if len(sys.argv)>1 and sys.argv[1]in['monolithic','sparse','vfirst']:child(sys.argv[1])
 else:main()
