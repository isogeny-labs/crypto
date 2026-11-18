"""Complete AS trace-rank census on the certified centroid h-domain."""
from pathlib import Path
import json,hashlib,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.centroid_cover_20261115_v33 import worker as old
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic();CURRENT={}
def trace_masks(f,m):
 powers=[];v=1
 for _ in range(f.degree+m-1):powers.append(old.trace(f,v));v=f.mul(v,2)
 return [sum(powers[i+j]<<i for i in range(f.degree))for j in range(m)]
def first_pivot(masks,value):return next((j for j,m in enumerate(masks)if(m&value).bit_count()%2),None)
def controls():
 systems=literal=zero=rankzero=empty=0
 for n,mod in[(2,7),(3,11),(4,19),(5,37)]:
  f=BinaryField(n,mod);As=sorted({0,1,1<<(n-1)})
  for d in range(1,min(n,3)+1):
   width=min(2*d-1,n);masks=trace_masks(f,width)
   for value in range(f.order):
    for j,M in enumerate(masks):assert (M&value).bit_count()%2==old.trace(f,old.naive(f,1<<j,value),True)
   for A in As:
    for z in range(f.order):
     tau=old.trace(f,A^z,True)
     for h in range(f.order):
      global CURRENT
      CURRENT=dict(n=n,modulus=mod,d=d,A=A,z=z,h=h);h2=old.naive(f,h,h);centroid=h2^h^z;C2=old.naive(f,h2,A)^old.naive(f,z,centroid)
      if not h:want=1<<width;zero+=1
      else:
       inv=next(x for x in range(1,f.order)if old.naive(f,h,x)==1);inv2=old.naive(f,inv,inv);pivot=first_pivot(masks,inv2)
       assert old.trace(f,old.naive(f,C2,inv2),True)==tau
       want=(1<<width)if pivot is not None else(0 if tau else 1<<(width+1));rankzero+=pivot is None;empty+=want==0
      count=0
      for r in range(f.order):
       count+=(old.naive(f,r,r)^old.naive(f,h,r)^C2)<1<<width;literal+=1;assert literal<=500000
      assert count==want;systems+=1
 return dict(complete_r_fibers=systems,literal_r_checks=literal,h0_fibers=zero,trace_rank_zero_fibers=rankzero,empty_fibers=empty,all_sets_counts_equal=True)
def actual(out):
 prior=json.loads((R/'research/centroid_cover_20261115_v33/raw.json').read_text());assert prior['error']is None;record=prior['out']['actual'];domain=record['domain'];f=BinaryField(record['field_degree'],record['modulus']);z=record['target_z'];B=domain['basis'];p=domain['particular'];N=domain['count'];assert len(B)==18 and N==1<<18 and domain['all_h_nonzero']
 masks=trace_masks(f,35);tau=old.trace(f,z);assert tau==0
 out.update(complete=False,seen=0,total=N,degenerate=0,pivot_histogram=[0]*35,witnesses=[],samples=[],trace_rhs=tau,masks=masks,modulus=f.modulus,basis=B,particular=p,target_z=z)
 digest=hashlib.sha256();product=1;inverse_sum=0;h=p
 for i in range(N):
  if i:h^=B[(i&-i).bit_length()-1]
  assert h;inv=f.inv(h);assert f._raw_mul(h,inv)==1;inv2=f._raw_square(inv);pivot=first_pivot(masks,inv2);product=f._raw_mul(product,h);inverse_sum^=inv;digest.update(h.to_bytes(11,'little')+inv.to_bytes(11,'little'))
  if pivot is None:
   out['degenerate']+=1
   if len(out['witnesses'])<16:out['witnesses'].append(dict(h=h,inverse=inv))
  else:out['pivot_histogram'][pivot]+=1
  if i<32:out['samples'].append(dict(h=h,inverse=inv,pivot=pivot))
  out['seen']=i+1
  if i%4096==0:assert time.monotonic()-START<78,'census deadline'
 assert product==domain['total_product']and inverse_sum==domain['total_inverse_sum']
 out.update(complete=True,total_product=product,total_inverse_sum=inverse_sum,sequence_sha256=digest.hexdigest(),ordinary_fibers=N-out['degenerate'],predicted_no_degeneracy=out['degenerate']==0,scope='Allcentroidh E2trace ranks only; no E3/support/source/originalroot solve')
def main():
 out=dict(actual={});error=None
 try:out['controls']=controls();actual(out['actual'])
 except Exception as exc:error=type(exc).__name__+': '+str(exc);out['actual']['complete']=False;out['failed_case']=CURRENT
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<524288
 with(P/'raw.json').open('xb')as f:f.write(raw)
 aa=out['actual'];ev=dict(error=error,controls=out.get('controls'),actual={k:v for k,v in aa.items()if k not in['masks','basis','samples','witnesses']},worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),external_reviewer=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Exact constant trace RHS and exhaustive globalh E2rank census with checksumcertificates.',evidence=ev,next_actions=['Use completecensus to retainexceptionalfibers orcertifyordinaryrank; derive inversionfree affine particular r0=z+c*h when c2+c=A+z. Keep E3/support andoriginalsourceghost branches open.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
