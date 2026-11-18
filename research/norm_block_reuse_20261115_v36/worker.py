"""Coefficient membership fibers, independent tiny/original norm oracles."""
from pathlib import Path
import hashlib,itertools,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.inversion_free_oracles_20261115_v35 import worker as old
from research.inversion_free_full_20261115_v35 import compiler as nc
from research.inversion_free_full_20261115_v35.worker import literal
from research.norm_block_reuse_20261115_v36 import backend as b
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def pack(H,n,widths):
 a,c,d=widths;return(H[3]>>a)|((H[2]>>c)<<(n-a))|((H[1]>>d)<<(2*n-a-c))
def points(s):
 if s is None:return set()
 return{s['particular']^b.xor(v for i,v in enumerate(s['basis'])if t>>i&1)for t in range(1<<len(s['basis']))}
def rank(cols):
 out={}
 for x in cols:
  while x:
   p=(x&-x).bit_length()-1
   if p not in out:out[p]=x;break
   x^=out[p]
 return len(out)
def original(f,ctx,h,k,s):
 if s is None:return None
 n=f.degree;mask=(1<<n)-1
 def delta(v):
  u=v&mask;w=v>>n;return u|((w^f.mul(ctx['z'],u))<<n)
 p=s['particular'];r,t=nc.to_original(f,ctx,h,k,p&mask,p>>n)
 return dict(particular=r|(t<<n),basis=[delta(v)for v in s['basis']],rank=s['rank'])
def controls():
 fibers=states=zero=empty=0;profiles=[]
 for n,m in[(2,7),(3,11)]:
  f=old.Small(n,m);q=f.order
  for A in[0,1]:
   z,y,B=next((z,y,f.square(y)^f.mul(z,y)^f.mul(f.square(z),z)^f.mul(A,f.square(z)))for z in range(1,q)for y in range(q)if old.trace(f,A^z)==0 and(f.square(y)^f.mul(z,y)^f.mul(f.square(z),z)^f.mul(A,f.square(z))))
   ctx=old.context(f,A,z,y)[0]
   for dim in[1,2]:
    widths=tuple(min(n,j*(dim-1)+1)for j in[2,3,4]);C=b.Context(f,z,ctx['d'],list(range(q)),list(range(q)),widths);C.prepare_blocks()
    for h,k in itertools.product(range(q),repeat=2):
     mono=C.mono(h,k);block=C.block(h,k);assert b.same(mono,block)
     got=points(original(f,ctx,h,k,block));want=set()
     for r,s in itertools.product(range(q),repeat=2):
      if pack(literal(f,A,B,z,y,h,k,r,s),n,widths)==0:want.add(r|(s<<n))
      states+=1
     assert got==want;fibers+=1;zero+=h==0 or k==0;empty+=not want
    profiles.append(dict(n=n,A=A,source_dim=dim,widths=widths,E2_dims=[len(C.cache2[h][2])for h in range(q)],E4_dims=[len(C.cache4[k])for k in range(q)]))
 return dict(complete_fibers=fibers,literal_states=states,zero_h_or_k_fibers=zero,empty_fibers=empty,profiles=profiles)
def actual_context():
 src=json.loads((R/'research/centroid_cover_20261115_v33/raw.json').read_text())['out']['actual'];f=BinaryField(83,src['modulus']);z=src['target_z'];y=0x507c02b90217be9ec635f;ctx=nc.compile_context(f,0,z,y);D=src['domain'];h=D['particular'];hs=[]
 for i in range(32):
  if i:h^=D['basis'][(i&-i).bit_length()-1]
  assert f.square(h)^h^z<1<<18;hs.append(h)
 ks=[0,1,55,1<<82];return f,ctx,b.Context(f,z,ctx['d'],hs,ks,(35,52,69))
def actual():
 f,ctx,C=actual_context();C.prepare_blocks();n=f.degree;rows=[];digest=hashlib.sha256();directions=0
 for k in C.ks:
  for h in C.hs:
   mono=C.mono(h,k);block=C.block(h,k);assert b.same(mono,block)
   H0=literal(f,0,1,ctx['z'],ctx['y'],h,k,0,0);p0=pack(H0,n,C.widths);cols=[]
   for i in range(2*n):
    rr=(1<<i)if i<n else 0;ss=(1<<(i-n))if i>=n else 0
    cols.append(pack(literal(f,0,1,ctx['z'],ctx['y'],h,k,rr,ss),n,C.widths)^p0)
   direct=b.solve(b.rows_from(cols,p0,93),2*n);got=original(f,ctx,h,k,block);assert b.same(direct,got)
   if got is not None:
    assert b.apply(cols,got['particular'])==p0 and all(b.apply(cols,v)==0 for v in got['basis'])
    assert rank(got['basis'])==len(got['basis'])==2*n-rank(cols);directions+=len(got['basis'])
   B=[]if got is None else b.canonical_basis(got['basis']);part=None if got is None else b.apply_residue(got['particular'],B)
   row=dict(h=h,k=k,consistent=got is not None,nullity=None if got is None else len(B),canonical_sha256=hashlib.sha256(json.dumps([part,B],separators=(',',':')).encode()).hexdigest());rows.append(row);digest.update(json.dumps(row,sort_keys=True,separators=(',',':')).encode())
   assert time.monotonic()-START<78
 return dict(field_degree=83,modulus=f.modulus,hs=C.hs,ks=C.ks,widths=C.widths,fibers=rows,restored_kernel_directions=directions,E2_dims={str(h):len(C.cache2[h][2])for h in C.hs},E4_dims={str(k):len(C.cache4[k])for k in C.ks},literal_original_evaluations=128*167,sequence_sha256=digest.hexdigest(),scope='Complete coefficient-membership affine fibers only; no source support or original allroots solve')
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<262144
 with(P/'raw.json').open('xb')as f:f.write(raw)
 aa=out.get('actual',{});ev=dict(error=error,controls=out.get('controls'),actual_fibers=len(aa.get('fibers',[])),literal_original_evaluations=aa.get('literal_original_evaluations'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False,external_reviewer=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Exact reusable E2/E4 coefficient blocks with full affine-fiber lift and independent original norm oracles.',evidence=ev,next_actions=['If passes measure complete paid cached block versus equally cached monolithic compiler, restoring every original coordinate/kernel and serializing canonical outputs; keep global source support/ghost closure.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
