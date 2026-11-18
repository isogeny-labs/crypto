"""Complete tiny and quadratic-ANF norm identity oracle; no root search."""
from pathlib import Path
import hashlib,itertools,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.inversion_free_norm_20261115_v34 import compiler as c
P=Path(__file__).resolve().parent;START=time.monotonic();CURRENT={}
def naive(f,a,b):
 v=0
 while b:
  if b&1:v^=a
  b>>=1;a<<=1
 while v.bit_length()>f.degree:v^=f.modulus<<(v.bit_length()-f.degree-1)
 return v
class Small:
 def __init__(self,n,m):
  self.degree=n;self.modulus=m;self.order=1<<n
  self.table=[[naive(self,a,b)for b in range(self.order)]for a in range(self.order)]
 def mul(self,a,b):return self.table[a][b]
 def square(self,a):return self.table[a][a]
def trace(f,x):
 t=0
 for _ in range(f.degree):t^=x;x=f.square(x)
 assert t in(0,1);return t
def pmul(f,a,b):
 out=[0]*(len(a)+len(b)-1)
 for i,x in enumerate(a):
  if x:
   for j,y in enumerate(b):
    if y:out[i+j]^=f.mul(x,y)
 return out
def plus(a,b):return[(a[i]if i<len(a)else 0)^(b[i]if i<len(b)else 0)for i in range(max(len(a),len(b)))]
def literal(f,A,B,z,y,h,r,s):
 t=f.mul(f.square(z),z)^f.mul(r,f.square(z))^f.mul(s,z)^f.mul(f.mul(h,z),y)
 F=[t,s,r,1];G=[0,h]
 N=plus(plus(pmul(f,F,F),pmul(f,pmul(f,[0,1],F),G)),pmul(f,[B,0,A,1],pmul(f,G,G)))
 H=[0]*6;H[5]=N[6]
 for i in range(5,0,-1):H[i-1]=N[i]^f.mul(z,H[i])
 assert N[0]==f.mul(z,H[0])and H[5]==1
 return H
def context(f,A,z,y):
 roots=[a for a in range(f.order)if f.square(a)^a==A^z];assert len(roots)==2
 return [dict(status='ready',c=a,d=y^f.mul(z,a),z=z,y=y)for a in roots]
def tiny():
 records=[];states=zero=0
 for n,m in[(2,7),(3,11),(4,19),(5,37)]:
  f=Small(n,m)
  for A in[0,1]:
   z,y,B=next((z,y,f.square(y)^f.mul(z,y)^f.mul(f.square(z),z)^f.mul(A,f.square(z)))for z in range(1,f.order)for y in range(f.order)if trace(f,A^z)==0 and(f.square(y)^f.mul(z,y)^f.mul(f.square(z),z)^f.mul(A,f.square(z))))
   cs=context(f,A,z,y)
   if n%2:assert c.compile_context(f,A,z,y)in cs
   for ctx in cs:
    digest=hashlib.sha256();count=0
    for h,u,v in itertools.product(range(f.order),repeat=3):
     r,s=c.to_original(f,ctx,h,u,v);w=s^f.mul(z,r)
     assert u==r^z^f.mul(ctx['c'],h)and v==w^f.square(z)^f.mul(y,h)
     H=c.norm_coefficients(f,ctx,h,u,v);assert H==literal(f,A,B,z,y,h,r,s)
     digest.update(bytes([h,u,v,r,s]+H));count+=1;zero+=h==0;states+=1
    records.append(dict(n=n,modulus=m,A=A,B=B,z=z,y=y,c=ctx['c'],count=count,sha256=digest.hexdigest()))
   if n%2:
    z1=next(z for z in range(f.order)if trace(f,A^z));assert c.compile_context(f,A,z1,y)['status']=='unsupported_trace_one'
   assert time.monotonic()-START<65
 return dict(records=records,complete_states=states,h0_states=zero,all_maps_bijective=True,trace_one_refused=True,even_contexts_supplied_externally=True)
def actual():
 f=BinaryField(83,0x800000000200000000007);A=0;B=1;z=0x622425e96ba633969263b;y=0x507c02b90217be9ec635f
 assert f.square(y)^f.mul(z,y)==f.mul(f.square(z),z)^B
 for i in range(256):
  a=(i*0x123456789abcdef12345)%(1<<83);b=((i+7)*0x923456789abcdeffedcb)%(1<<83)
  assert f.mul(a,b)==naive(f,a,b)
 ctx=c.compile_context(f,A,z,y);assert ctx['status']=='ready';digest=hashlib.sha256();count=0;samples=[]
 points=itertools.chain([0],(1<<i for i in range(249)),((1<<i)|(1<<j)for j in range(249)for i in range(j)))
 mask=(1<<83)-1
 for bits in points:
  h=bits&mask;u=(bits>>83)&mask;v=bits>>166;r,s=c.to_original(f,ctx,h,u,v);H=c.norm_coefficients(f,ctx,h,u,v)
  assert H==literal(f,A,B,z,y,h,r,s)
  for a in H:digest.update(a.to_bytes(11,'little'))
  if count<8:samples.append(dict(bits=bits,H=H))
  count+=1
  if count%1024==0:assert time.monotonic()-START<78
 assert count==31126
 return dict(field_degree=83,modulus=f.modulus,A=A,B=B,z=z,y=y,context=ctx,boolean_variables=249,degree_bound=2,complete_anf_evaluations=count,sha256=digest.hexdigest(),samples=samples,k=0,all_h_including_zero=True,scope='Exact global binary k0 norm compiler identity only; not all original roots')
def main():
 out={};error=None
 try:out['tiny']=tiny();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<131072
 with(P/'raw.json').open('xb')as s:s.write(raw)
 ev=dict(error=error,tiny_states=out.get('tiny',{}).get('complete_states'),actual=out.get('actual'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False,external_reviewer=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Bijective inversionfree binaryk0 norm chart; independent complete tiny maps and actual degree2 ANF oracle.',evidence=ev,next_actions=['Compare paid strongest specialized k0 compiler; derive exact normalized block membership before whole-cell search. Retain all nonzero k and original source/ghost branches.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
