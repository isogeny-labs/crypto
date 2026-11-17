from pathlib import Path
import importlib.util
s=importlib.util.spec_from_file_location('tower54',Path(__file__).resolve().parent.parent/'round54-p192/tower.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m);K=m.K;F=m.F
s=importlib.util.spec_from_file_location('curve52',Path(__file__).resolve().parent.parent/'round52-p192/model.py');curve=importlib.util.module_from_spec(s);s.loader.exec_module(curve)
def add(P,Q):return curve.add(P,Q,11,2)
def mul(n,P):return curve.mul(n,P,11,2)
def neg(P):return None if P is None else (P[0],-P[1])
def read(P):return None if P is None else tuple(K(v) for v in P)
def code(P):return None if P is None else [v.code() for v in P]
def padd(A,B):
 C=[K()] * max(len(A),len(B))
 for i,v in enumerate(A):C[i]=C[i]+v
 for i,v in enumerate(B):C[i]=C[i]+v
 return C
def pscale(A,c):return [v*c for v in A]
def pmul(A,B):
 C=[K()]*(len(A)+len(B)-1)
 for i,a in enumerate(A):
  for j,b in enumerate(B):C[i+j]=C[i+j]+a*b
 return C
def rootdivide(A,z):
 Q=[K()]*(len(A)-1);Q[-1]=A[-1]
 for i in range(len(A)-2,0,-1):Q[i-1]=A[i]+z*Q[i]
 assert A[0]+z*Q[0]==0;return Q
def value(A,x):
 y=K()
 for a in reversed(A):y=y*x+a
 return y
def derivative(A):return [i*A[i] for i in range(1,len(A))]
def pointkey(P):return str(code(P))
def division_from_torsion(Er):
 roots={}
 for U in Er:
  if U is not None:roots.setdefault(tuple(U[0].code()),U)
 assert len(roots)==24
 psi=[K(7)]
 for U in roots.values():psi=pmul(psi,[-U[0],K(1)])
 assert all(not any(z.code()[1:]) for z in psi)
 return psi,list(roots.values())
def numerator(H,R0,psi,reps):
 D=pmul(psi,psi);images={pointkey(U):V for U,V in H};A=pscale([K()]+D,R0[0]);B=[K()]
 for U in reps:
  x,y=U;Vp=images[pointkey(U)];Vm=images[pointkey(neg(U))];up=add(R0,Vp)[0];um=add(R0,Vm)[0];q=rootdivide(rootdivide(D,x),x)
  A=padd(A,pscale(pmul(q,[2*x+14,K(2)+x*x,x]),up+um))
  B=padd(B,pscale(q,(um-up)*2*y))
 assert all(not any(z.code()[i] for i in [1,2,4,5]) for z in A+B)
 return A,B
def sqrt11(a):
 z=pow(a%11,3,11);return z if z*z%11==a%11 else None
def sqrt121(z):
 v=z.code();assert all(v[i]==0 for i in [1,2,4,5]);a,b=v[0],v[3]
 if b==0:
  u=sqrt11(a)
  if u is not None:return K(u)
  w=sqrt11(a*6%11);assert w is not None;return K(0,w)
 n=sqrt11((a*a-2*b*b)%11);assert n is not None
 for nn in [n,-n%11]:
  u=sqrt11((a+nn)*6%11)
  if u:
   w=b*pow(2*u,-1,11)%11;y=K(u,w)
   if y*y==z:return y
 raise ValueError('no square root')
def extract(U,A,B,psi,R0):
 assert U is not None;x,y=U;dp=value(derivative(psi),x);kappa=4*y*y*dp*dp;assert kappa
 raw=value(A,x)-y*value(B,x);v=raw/kappa;root=sqrt121(v*v*v+2*v+7)
 candidates=[add((v,root),neg(R0)),add((v,-root),neg(R0))]
 valid=[P for P in candidates if P is not None and all(not any(z.code()[1:]) for z in P) and mul(7,P) is None]
 assert len(valid)==1
 return valid[0],{'kappa':kappa.code(),'trace_principal_value':v.code(),'candidates':[code(P) for P in candidates]}
