from pathlib import Path
from math import gcd
import json
root=Path(__file__).resolve().parent;c=json.loads((root.parent/'round52-p192/input.json').read_text());p,r,a,b=c['p'],c['r'],c['a'],c['b'];t=p+1-r;nt=p+1+t
assert p%4==3 and gcd(r,nt)==1 and gcd(6,nt)==1
for x in range(17):
 rhs=(x*x*x+a*x+b)%p
 if pow(rhs,(p-1)//2,p)==p-1:
  y=pow(-rhs%p,(p+1)//4,p);assert -y*y%p==rhs;break
else:raise AssertionError('frozen search exhausted')
def A(u,v):return ((u[0]+v[0])%p,(u[1]+v[1])%p)
def N(u):return (-u[0]%p,-u[1]%p)
def M(u,v):return ((u[0]*v[0]-u[1]*v[1])%p,(u[0]*v[1]+u[1]*v[0])%p)
def inv(u):
 z=pow((u[0]*u[0]+u[1]*u[1])%p,-1,p);return u[0]*z%p,-u[1]*z%p
def scale(n,u):return n*u[0]%p,n*u[1]%p
def add(P,Q):
 if P is None:return Q
 if Q is None:return P
 x,y=P;u,v=Q
 if x==u and A(y,v)==(0,0):return None
 s=M(A(scale(3,M(x,x)),(a,0)),inv(scale(2,y))) if P==Q else M(A(v,N(y)),inv(A(u,N(x))))
 X=A(A(M(s,s),N(x)),N(u));return X,A(M(s,A(x,N(X))),N(y))
def mul(n,P):
 out=None
 while n:
  if n&1:out=add(out,P)
  P=add(P,P);n//=2
 return out
S=((x,0),(0,y));assert M(S[1],S[1])==A(A(M(M(S[0],S[0]),S[0]),scale(a,S[0])),(b,0));assert mul(nt,S) is None
rS=mul(r,S);R=mul(6,S);assert rS is not None and R is not None and mul(r,R) is not None
out={'all_checks_pass':True,'p':p,'r':r,'a':a,'b':b,'trace':t,'twist_order':nt,'quadratic_relation':'W^2=-1','S':S,'R_6S':R,'rS':rS,'rR':mul(r,R),'base_field_twist_point':[(-x)%p,y],'domain':'every rational pair shifted by (S,6S) and every mixed-kernel summand is finite','trace_evaluated':False}
(root/'P192-shift.json').write_text(json.dumps(out,indent=2)+'\n');print('P192_QUADRATIC_SHIFT_PASS','x',x,'nonzero_rS',rS is not None)
