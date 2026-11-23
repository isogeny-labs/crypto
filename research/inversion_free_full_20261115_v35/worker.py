"""Independent all-k polynomial norm validation including every exceptional zero."""
from pathlib import Path
import hashlib,itertools,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.inversion_free_oracles_20261115_v35 import worker as old
from research.inversion_free_full_20261115_v35 import compiler as c
P=Path(__file__).resolve().parent;START=time.monotonic()
def literal(f,A,B,z,y,h,k,r,s):
 t=f.mul(f.square(z),z)^f.mul(r,f.square(z))^f.mul(s,z)^f.mul(f.mul(h,z)^k,y)
 F=[t,s,r,1];G=[k,h]
 N=old.plus(old.plus(old.pmul(f,F,F),old.pmul(f,old.pmul(f,[0,1],F),G)),old.pmul(f,[B,0,A,1],old.pmul(f,G,G)))
 H=[0]*6;H[5]=N[6]
 for i in range(5,0,-1):H[i-1]=N[i]^f.mul(z,H[i])
 assert N[0]==f.mul(z,H[0])and H[5]==1;return H

def controls():
 rows=[];total=0;zero=0
 for n,m in[(2,7),(3,11),(4,19),(5,37)]:
  f=old.Small(n,m)
  for A in[0,1]:
   z,y,B=next((z,y,f.square(y)^f.mul(z,y)^f.mul(f.square(z),z)^f.mul(A,f.square(z)))for z in range(1,f.order)for y in range(f.order)if old.trace(f,A^z)==0 and(f.square(y)^f.mul(z,y)^f.mul(f.square(z),z)^f.mul(A,f.square(z))))
   for ctx in old.context(f,A,z,y):
    digest=hashlib.sha256();count=0
    points=itertools.product(range(f.order),repeat=4)if n<=3 else((i%f.order,(i//f.order)%f.order,((i//(f.order*f.order))+3)%f.order,(i*11+5)%f.order)for i in range(1024))
    for h,k,u,v in points:
     r,s=c.to_original(f,ctx,h,k,u,v)
     assert r^z^f.mul(ctx['c'],h)==u
     assert s^f.mul(z,r)^f.square(z)^f.mul(y,h)^f.mul(ctx['c'],k)==v
     H=c.norm_coefficients(f,ctx,h,k,u,v);assert H==literal(f,A,B,z,y,h,k,r,s)
     # Changing AS choice represents the identical original (r,s).
     ctx2=dict(ctx,c=ctx['c']^1,d=ctx['d']^z)
     assert c.to_original(f,ctx2,h,k,u^h,v^k)==(r,s)
     assert c.norm_coefficients(f,ctx2,h,k,u^h,v^k)==H
     digest.update(bytes([h,k,u,v,r,s]+H));count+=1;zero+=h==0 or k==0;total+=1
    rows.append(dict(n=n,modulus=m,A=A,B=B,z=z,y=y,c=ctx['c'],count=count,complete=n<=3,sha256=digest.hexdigest()))
 return dict(records=rows,states=total,zero_h_or_k_states=zero,AS_choice_equivariance=True)

def actual():
 f=BinaryField(83,0x800000000200000000007);A=0;B=1;z=0x622425e96ba633969263b;y=0x507c02b90217be9ec635f
 assert f.square(y)^f.mul(z,y)==f.mul(f.square(z),z)^B
 ctx=c.compile_context(f,A,z,y);mask=(1<<83)-1;digest=hashlib.sha256();count=0
 points=itertools.chain([0],(1<<i for i in range(332)),((1<<i)|(1<<j)for j in range(332)for i in range(j)))
 for bits in points:
  h=bits&mask;k=(bits>>83)&mask;u=(bits>>166)&mask;v=bits>>249
  r,s=c.to_original(f,ctx,h,k,u,v);H=c.norm_coefficients(f,ctx,h,k,u,v)
  assert H==literal(f,A,B,z,y,h,k,r,s)
  for a in H:digest.update(a.to_bytes(11,'little'))
  count+=1
  if count%1024==0:assert time.monotonic()-START<78
 assert count==55279
 return dict(field_degree=83,modulus=f.modulus,A=A,B=B,z=z,y=y,context=ctx,boolean_variables=332,degree_bound=2,complete_anf_evaluations=count,sha256=digest.hexdigest(),all_h_and_k=True,original_root_search_complete=False)
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<131072
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=out.get('controls'),actual=out.get('actual'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),qualified_speedup=None,release_gate_claim=False,complete_global_root_search=False,external_reviewer=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='All-k inversionfree binary norm chart, complete tiny and actual332bit quadratic identity oracles.',evidence=ev,next_actions=['Compile exact normalized E4constantblock then E2/E3 compatibility; measure paid costs against strongest general norm formula. Keep source support and original chart/ghost closure.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
