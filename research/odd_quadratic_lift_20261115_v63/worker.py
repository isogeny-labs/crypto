"""Independent odd-characteristic S3 product-lifting identity and small root oracle."""
from pathlib import Path
import json,hashlib,itertools,resource,sys,time
P=Path(__file__).resolve().parent;START=time.monotonic()
class F:
 def __init__(self,p,r=None):self.p=p;self.r=r;self.q=p if r is None else p*p
 def add(self,a,b):
  p=self.p
  return(a+b)%p if self.r is None else((a%p+b%p)%p)+p*((a//p+b//p)%p)
 def neg(self,a):return self.scale(a,-1)
 def sub(self,a,b):return self.add(a,self.neg(b))
 def scale(self,a,k):
  p=self.p
  return a*k%p if self.r is None else(a%p*k%p)+p*(a//p*k%p)
 def mul(self,a,b):
  p=self.p
  if self.r is None:return a*b%p
  return((a%p*(b%p)+self.r*(a//p)*(b//p))%p)+p*((a%p*(b//p)+a//p*(b%p))%p)
 def sq(self,a):return self.mul(a,a)

def original(f,a,b,u,A,B):
 # Literal unfactored summation-polynomial formula.
 x=f.mul(f.sq(f.sub(a,b)),f.sq(u));y=f.mul(f.add(a,b),f.add(f.mul(a,b),A));y=f.add(y,f.scale(B,2))
 x=f.sub(x,f.scale(f.mul(y,u),2));x=f.add(x,f.sq(f.sub(f.mul(a,b),A)));return f.sub(x,f.scale(f.mul(B,f.add(a,b)),4))

def lifted(f,a,b,u,A,B,P,V,W):
 x=f.sq(W);x=f.sub(x,f.scale(f.mul(P,V),2));x=f.sub(x,f.scale(f.mul(A,V),2));x=f.sub(x,f.scale(f.mul(B,u),4));x=f.add(x,f.sq(f.sub(P,A)));return f.sub(x,f.scale(f.mul(B,f.add(a,b)),4))

def main():
 out={'profiles':[],'small_states':0,'auxiliary_mutations':0,'full_augmented_states':0,'requested_prime_identity_checks':[]};error=None
 try:
  for p,r in[(3,None),(5,None),(7,None),(3,2),(5,2)]:
   f=F(p,r)
   if r is not None:assert all(v*v%p!=r for v in range(p))
   for A,B in[(0,0),(1,1)]:
    roots=[]
    for a,b,u in itertools.product(range(f.q),repeat=3):
     aux=[f.mul(a,b),f.mul(f.add(a,b),u),f.mul(f.sub(a,b),u)];want=original(f,a,b,u,A,B);assert lifted(f,a,b,u,A,B,*aux)==want
     if want==0:roots.append([a,b,u])
     for j in range(3):
      bad=aux.copy();bad[j]=f.add(bad[j],1);definitions=[f.sub(bad[0],f.mul(a,b)),f.sub(bad[1],f.mul(f.add(a,b),u)),f.sub(bad[2],f.mul(f.sub(a,b),u))];assert any(definitions);out['auxiliary_mutations']+=1
     out['small_states']+=1
    if p==3 and r is None and A==B==1:
     augmented=[]
     for a,b,u,Pc,Vc,Wc in itertools.product(range(3),repeat=6):
      defs=[f.sub(Pc,f.mul(a,b)),f.sub(Vc,f.mul(f.add(a,b),u)),f.sub(Wc,f.mul(f.sub(a,b),u))]
      if not any(defs)and lifted(f,a,b,u,A,B,Pc,Vc,Wc)==0:augmented.append([a,b,u])
      out['full_augmented_states']+=1
     assert sorted(augmented)==sorted(roots)
    out['profiles'].append({'p':p,'extension_degree':1 if r is None else 2,'T_squared':r,'A':A,'B':B,'states':f.q**3,'root_count':len(roots),'ordered_roots_sha256':hashlib.sha256(json.dumps(roots,separators=(',',':')).encode()).hexdigest()})
  primes=[('P192',(1<<192)-(1<<64)-1),('P224',(1<<224)-(1<<96)+1),('P256',(1<<256)-(1<<224)+(1<<192)+(1<<96)-1),('P384',(1<<384)-(1<<128)-(1<<96)+(1<<32)-1),('P521',(1<<521)-1)]
  for name,p in primes:
   f=F(p);A=p-3;B=7;state=63003
   for i in range(24):
    v=[]
    for j in range(3):state=int.from_bytes(hashlib.sha256(str(state).encode()).digest(),'big');v.append(state%p)
    a,b,u=v;aux=[f.mul(a,b),f.mul(f.add(a,b),u),f.mul(f.sub(a,b),u)];assert original(f,a,b,u,A,B)==lifted(f,a,b,u,A,B,*aux)
   out['requested_prime_identity_checks'].append({'field':name,'p_hex':hex(p),'cases':24,'curve_coefficients':'A=-3,B=7 algebraic diagnostic; not the standard NIST curve instance','solver_performance_claim':False})
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<75
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Odd characteristic exact product-lifting identity with characteristic3 and odd-extension all-root oracles.','evidence':{'out':out,'error':error,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'worker_seconds':time.monotonic()-START,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Implement field-coordinate odd-prime/extension encoder and retained domain constraints; compare specialized factor methods and genuine all-root GB on matching hard instances.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
