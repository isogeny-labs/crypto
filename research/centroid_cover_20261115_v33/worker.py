"""Exact global centroid h-domain and compact whole-chart invariants."""
from pathlib import Path
import json,hashlib,resource,sys,time,itertools
from rootfinder.binaryfield import BinaryField
from research.shared_block_20261115_v26 import worker as b
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic();CURRENT={}
def xor(vs):
 x=0
 for v in vs:x^=v
 return x
def naive(f,a,c):
 v=0
 for i in range(f.degree):
  if c>>i&1:v^=a<<i
 while v.bit_length()>f.degree:v^=f.modulus<<(v.bit_length()-f.degree-1)
 return v
def trace(f,x,independent=False):
 out=0
 for _ in range(f.degree):out^=x;x=naive(f,x,x)if independent else f.square(x)
 assert out in(0,1);return out
def points(p,B):return sorted(p^xor(v for i,v in enumerate(B)if t>>i&1)for t in range(1<<len(B)))
def residual(v,B):
 for bit,row in sorted(b.reduce_basis(B).items(),reverse=True):
  if v>>bit&1:v^=row
 return v
def linearized(f,B):
 coeff=[1]
 for v in B:
  x=v;lam=0
  for c in coeff:lam^=f.mul(c,x);x=f.square(x)
  assert lam;new=[0]*(len(coeff)+1)
  for i,c in enumerate(coeff):new[i]^=f.mul(lam,c);new[i+1]^=f.square(c)
  coeff=new
 return coeff
def evaluate(f,L,x):
 value=0
 for c in L:value^=f.mul(c,x);x=f.square(x)
 return value
def compile_domain(f,V,z):
 n=f.degree;assert len(b.reduce_basis(V))==len(V)
 Q,_=b.s.nullspace(V,n);AS=[f.square(1<<i)^(1<<i)for i in range(n)];rows=[(sum(((q&v).bit_count()%2)<<i for i,v in enumerate(AS)),(q&z).bit_count()%2)for q in Q];sol=b.solve(rows,n);tz=trace(f,z);tv=int(any(trace(f,v)for v in V));expected=bool(tv or tz==0);assert (sol is not None)==expected
 if sol is None:return dict(consistent=False,count=0,rows=rows,trace_z=tz,trace_rank=tv,domain_dimension=None)
 B=sol['basis'];p=sol['particular'];assert len(B)==len(V)+1-tv and len(b.reduce_basis(B))==len(B)
 assert residual(f.square(p)^p^z,V)==0 and all(residual(f.square(v)^v,V)==0 for v in B)
 L=linearized(f,B);assert L[-1]==1 and L[0]and all(evaluate(f,L,v)==0 for v in B);constant=evaluate(f,L,p)
 return dict(consistent=True,count=1<<len(B),rows=rows,trace_z=tz,trace_rank=tv,domain_dimension=len(B),particular=p,basis=B,linearized_coefficients=L,constant=constant,all_h_nonzero=constant!=0,total_product=constant,total_inverse_sum=f.mul(L[0],f.inv(constant))if constant else None)
def controls():
 profiles=systems=states=empty=degenerate=0;counts=[]
 for n,mod in[(2,7),(3,11),(4,19),(5,37)]:
  f=BinaryField(n,mod);Vs=[tuple(1<<i for i in range(d))for d in range(min(n,3)+1)]
  for V in[(1,1<<(n-1)),(1<<(n-1),)]:
   if V not in Vs:Vs.append(V)
  for V in Vs:
   space=set(points(0,V));profiles+=1
   for z in range(f.order):
    got=compile_domain(f,V,z);truth=[h for h in range(f.order)if naive(f,h,h)^h^z in space];states+=f.order;systems+=1;found=points(got['particular'],got['basis'])if got['consistent']else[];assert found==truth
    assert got['trace_z']==trace(f,z,True)and got['trace_rank']==int(any(trace(f,v,True)for v in V))
    if not truth:empty+=1;continue
    degenerate+=got['domain_dimension']!=len(V);product=1;inverse_sum=0
    for h in truth:
     product=naive(f,product,h)
     if h:
      inv=next(x for x in range(1,f.order)if naive(f,h,x)==1);inverse_sum^=inv
    assert product==got['total_product']
    if got['all_h_nonzero']:assert inverse_sum==got['total_inverse_sum']
    counts.append((n,len(V),got['trace_rank'],got['domain_dimension']))
 return dict(subspaces=profiles,complete_domains=systems,literal_h_checks=states,empty_domains=empty,trace_degenerate_nonempty=degenerate,dimension_profiles=sorted(set(counts)),all_complete_sets_equal=True)
def actual():
 data=json.loads((R/'research/advance/instances/n83-d18-m5-T001-offset0.json').read_text());f=BinaryField(83,int(data['extension_field']['modulus_hex'],16));z=int(data['target']['x_hex'],16);V=[1<<i for i in range(18)];got=compile_domain(f,V,z);assert got['consistent']and got['domain_dimension']==18 and len(got['rows'])==65 and got['all_h_nonzero']
 h=0x4208336d0d441df79c400;assert all((m&h).bit_count()%2==r for m,r in got['rows']);assert f.square(h)^h^z==55740
 for h0 in[0,1]:assert not all((m&h0).bit_count()%2==r for m,r in got['rows'])
 return dict(field_degree=83,modulus=f.modulus,target_z=z,factorbase_basis=V,domain=got,known_fixed_h=h,scope='Allcentroid hcharts for finite quinticnorm representation only; no r/s/k/support/sourceghost/globalroot solve')
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc);out['failed_case']=CURRENT
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<524288
 with(P/'raw.json').open('xb')as f:f.write(raw)
 aa=out.get('actual',{});d=aa.get('domain',{});ev=dict(error=error,controls=out.get('controls'),actual=dict(consistent=d.get('consistent'),count=d.get('count'),dimension=d.get('domain_dimension'),rows=len(d.get('rows',[])),all_h_nonzero=d.get('all_h_nonzero'),total_product=d.get('total_product'),total_inverse_sum=d.get('total_inverse_sum')),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),external_reviewer=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Complete centroid h-domain certificate including trace-degenerate binary fields and compact product/inverse-sum identities.',evidence=ev,next_actions=['Use certifiedhcover asinput to whole-cell support rejection, not262144timesfixedh brute search; preserve norm/source/ghost scope. Batchinversion canbechecked against compactproduct identity butneeds paidcost proof.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
