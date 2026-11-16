# Deterministic generator of polynomial constraints; hashes the stream instead of expanding it on disk.
from pathlib import Path
import json,hashlib,importlib.util
s=importlib.util.spec_from_file_location('point52',Path(__file__).resolve().parent.parent/'round52-p192/model.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
def plus(a,b):return ('+',a,b)
def sub(a,b):return ('-',a,b)
def times(a,b):return ('*',a,b)
def summation(xs):
 out=0
 for x in xs:out=plus(out,x)
 return out
def degree(e):
 if isinstance(e,int):return 0
 if isinstance(e,str):return 1
 return degree(e[1])+degree(e[2]) if e[0]=='*' else max(degree(e[1]),degree(e[2]))
class Compiler:
 def __init__(self,p,a,b,r,G,Q,keep=False):self.p=p;self.a=a;self.b=b;self.r=r;self.G=tuple(G)+(0,);self.Q=tuple(Q)+(0,);self.nv=0;self.ne=0;self.maximum_degree=0;self.digest=hashlib.sha256();self.equations=[] if keep else None
 def var(self):self.nv+=1;return 'v'+str(self.nv)
 def emit(self,e):
  self.ne+=1;self.maximum_degree=max(self.maximum_degree,degree(e));self.digest.update(json.dumps(e,separators=(',',':')).encode()+b'\n')
  if self.equations is not None:self.equations.append(e)
 def point(self):
  x,y,o=[self.var() for _ in range(3)];self.emit(times(o,sub(o,1)));self.emit(times(o,x));self.emit(times(o,y));self.emit(times(sub(1,o),sub(sub(sub(times(y,y),times(times(x,x),x)),times(self.a,x)),self.b)));return x,y,o
 def add(self,P,Q):
  x,y,o=P;u,v,q=Q;X,Y,O=self.point();selectors=[self.var() for _ in range(5)]
  for e in selectors:self.emit(times(e,sub(e,1)))
  self.emit(sub(summation(selectors),1));I,J,C,D,F=selectors;s,z=self.var(),self.var();off=sub(sub(1,D),F);self.emit(times(off,s));self.emit(times(off,z))
  cases=[[sub(o,1),sub(X,u),sub(Y,v),sub(O,q)],[o,sub(q,1),sub(X,x),sub(Y,y),sub(O,o)],[o,q,sub(x,u),plus(y,v),sub(O,1),X,Y],[o,q,sub(x,u),sub(y,v),O,sub(times(times(2,y),z),1),sub(s,times(plus(times(3,times(x,x)),self.a),z)),plus(sub(X,times(s,s)),times(2,x)),plus(sub(Y,times(s,sub(x,X))),y)],[o,q,O,sub(times(sub(u,x),z),1),sub(s,times(sub(v,y),z)),plus(plus(sub(X,times(s,s)),x),u),plus(sub(Y,times(s,sub(x,X))),y)]]
  for mask,eqs in zip(selectors,cases):
   for e in eqs:self.emit(times(mask,e))
  return (X,Y,O),selectors,s
 def scalar(self,P,n):
  A=P
  for bit in bin(n)[3:]:
   A=self.add(A,A)[0]
   if bit=='1':A=self.add(A,P)[0]
  return A
 def power(self,x,n):
  y=x
  for bit in bin(n)[3:]:
   z=self.var();self.emit(sub(z,times(y,y)));y=z
   if bit=='1':z=self.var();self.emit(sub(z,times(y,x)));y=z
  return y
 def miller(self,P,R,n):
  A=P;f=1
  for bit in bin(n)[3:]:
   for B,double in [(A,True)]+([(P,False)] if bit=='1' else []):
    S,ss,s=self.add(A,B);I,J,C,D,F=ss;x,y,o=A;X,Y,O=S;u,v,q=R;g=self.var()
    self.emit(times(q,sub(g,1)));self.emit(times(times(sub(1,q),plus(I,J)),sub(g,1)));self.emit(times(times(sub(1,q),C),plus(sub(g,u),x)))
    self.emit(times(times(sub(1,q),plus(D,F)),plus(plus(sub(times(g,sub(u,X)),v),y),times(s,sub(u,x)))))
    nf=self.var();self.emit(sub(nf,times(times(f,f),g) if double else times(f,g)));f=nf;A=S
  for actual,expected in zip(A,(0,0,1)):self.emit(sub(actual,expected))
  return f
 def dual(self):
  U,V=self.point(),self.point();GU=self.miller(self.G,U,self.r);UG=self.miller(U,self.G,self.r);QV=self.miller(self.Q,V,self.r);VQ=self.miller(V,self.Q,self.r)
  for P in [U,V]:
   H=self.scalar(P,self.p);fx=self.power(P[0],self.p);fy=self.power(P[1],self.p)
   for a,b in zip(H,(fx,fy,P[2])):self.emit(sub(a,b))
  signs=times(sub(times(2,U[2]),1),sub(times(2,V[2]),1));self.emit(sub(times(times(signs,GU),QV),times(UG,VQ)));return U,V
 def gamma(self):
  k=self.r.bit_length();bits=[self.var() for _ in range(k)]
  for e in bits:self.emit(times(e,sub(e,1)))
  states=[(0,0,1),(0,0,1)];bases=[self.G[:2],self.Q[:2]]
  for e in bits:
   for j in range(2):
    P=bases[j];selected=(times(e,P[0]),times(e,P[1]),sub(1,e));states[j]=self.add(states[j],selected)[0];bases[j]=m.add(P,P,self.p,self.a)
  return states
 def full(self):
  A,B=self.gamma();U,V=self.dual();return self.add(A,U)[0],self.add(B,V)[0]
 def result(self):return {'variables':self.nv,'equations':self.ne,'maximum_degree':self.maximum_degree,'constraint_stream_sha256':self.digest.hexdigest()}
def evaluate(e,values,F):
 if isinstance(e,int):return F(e)
 if isinstance(e,str):return values[int(e[1:])-1]
 a=evaluate(e[1],values,F);b=evaluate(e[2],values,F)
 return a+b if e[0]=='+' else a-b if e[0]=='-' else a*b
if __name__=='__main__':
 root=Path(__file__).resolve().parent;c=json.loads((root.parent/'round52-p192/input.json').read_text());out={}
 for full in [False,True]:
  comp=Compiler(c['p'],c['a'],c['b'],c['r'],c['G'],c['Q']);comp.full() if full else comp.dual();out['full_mixed' if full else 'dual']=comp.result()
 out['input_sha256']=hashlib.sha256((root.parent/'round52-p192/input.json').read_bytes()).hexdigest();(root/'P192-blueprint.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
