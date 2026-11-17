from pathlib import Path
import importlib.util,hashlib,json
from field import F
s=importlib.util.spec_from_file_location('complete52',Path(__file__).resolve().parent.parent/'round52-p192/model.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
p,a,b,r=11,2,7,7
O=(0,0,1)
def enc(P):return None if P[2] else [list(F(P[0]).c),list(F(P[1]).c)]
def key(P):return json.dumps(enc(P),separators=(',',':'))
def add(P,Q):return m.witness(P,Q,p,a,b)[0]
def mul(n,P):
 R=O
 while n:
  if n&1:R=add(R,P)
  P=add(P,P);n//=2
 return R
class Trace:
 def __init__(self):self.variables=0;self.equations=0;self.digest=hashlib.sha256();self.gates=0;self.values=[]
 def reg(self,values,equations):
  self.values.extend(F(v) for v in values);self.variables+=len(values);self.equations+=equations;self.digest.update(json.dumps([list(F(v).c) for v in values],separators=(',',':')).encode())
 def point(self,P):
  x,y,o=P;assert o in [0,1]
  assert (not o and y*y-x*x*x-a*x-b==0) or (o and x==y==0)
  self.reg(P,4)
 def group(self,A,B):
  C,ss,s,z=m.witness(A,B,p,a,b);self.reg(list(C)+ss+[s,z],44);self.gates+=1;return C,ss,s
 def scalar(self,P,n):
  A=P
  for bit in bin(n)[3:]:
   A=self.group(A,A)[0]
   if bit=='1':A=self.group(A,P)[0]
  return A
 def power(self,x,n):
  y=x
  for bit in bin(n)[3:]:
   y=y*y;self.reg([y],1)
   if bit=='1':y=y*x;self.reg([y],1)
  return y
 def miller(self,P,R,n):
  A=P;f=F(1)
  for bit in bin(n)[3:]:
   for B,double in [(A,True)]+([(P,False)] if bit=='1' else []):
    C,ss,s=self.group(A,B);I,J,K,D,L=ss;x,y,o=A;X,Y,oo=C;u,v,q=R
    if q or I or J:g=F(1)
    elif K:g=u-x
    else:g=(v-y-s*(u-x))/(u-X)
    residual=[q*(g-1),(1-q)*(I+J)*(g-1),(1-q)*K*(g-u+x),(1-q)*(D+L)*(g*(u-X)-v+y+s*(u-x))]
    assert not any(residual);self.reg([g],4)
    f=f*f*g if double else f*g;self.reg([f],1);A=C
  assert A==O;self.equations+=3
  return f
 def finish(self):return {'variables':self.variables,'equations':self.equations,'group_gates':self.gates,'witness_sha256':self.digest.hexdigest()}
def dual(U,V,G,Q,with_values=False):
 tr=Trace();tr.point(U);tr.point(V)
 fGU=tr.miller(G,U,r);fUG=tr.miller(U,G,r);fQV=tr.miller(Q,V,r);fVQ=tr.miller(V,Q,r)
 for P in [U,V]:
  H=tr.scalar(P,p);fx=tr.power(F(P[0]),p);fy=tr.power(F(P[1]),p);assert H==(fx,fy,P[2]);tr.equations+=3
 residual=(2*U[2]-1)*(2*V[2]-1)*fGU*fQV-fUG*fVQ;tr.equations+=1
 return (residual==0,tr.finish(),tr.values) if with_values else (residual==0,tr.finish())
def weil(P,R):
 if P[2] or R[2]:return F(1)
 tr=Trace();return -tr.miller(P,R,r)/tr.miller(R,P,r)
