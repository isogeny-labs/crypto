#!/usr/bin/env python3
"""CT114: scalar-blind local oriented CRT projector; prime-switch ambiguity.

Arithmetic in A_m=(Z/m)[X,Y]/(psi11(X),Y^2+Y-rhs(X)), m=59 or 59^2.
Source validation uses only encoded (x,y), not discrete logarithms.
"""
import json
P=59
D=[-79,-40,-4,4];f3=[-21,-237,-60,-4,3];f4=[-5821,-1496,-210,-790,-100,-4,2]

def req(a,m):
    if not a:raise ArithmeticError(m)
def trim(f):
    while len(f)>1 and f[-1]==0:f.pop()
    return f
def mul(a,b,M):
    out=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        if x:
            for j,y in enumerate(b):out[i+j]=(out[i+j]+x*y)%M
    return trim(out)
def minus(a,b,M):
    return trim([((a[i] if i<len(a) else 0)-(b[i] if i<len(b) else 0))%M for i in range(max(len(a),len(b)))])
def ppower(a,n,M):
    r=[1]
    while n:
        if n&1:r=mul(r,a,M)
        a=mul(a,a,M);n>>=1
    return r

def division_polynomial(M):
    f5=minus(mul(ppower(D,2,M),f4,M),ppower(f3,3,M),M)
    f6=mul(f3,minus(f5,ppower(f4,2,M),M),M)
    f7=minus(mul(f5,ppower(f3,3,M),M),mul(ppower(D,2,M),ppower(f4,3,M),M),M)
    f11=minus(mul(f7,ppower(f5,3,M),M),mul(mul(ppower(D,2,M),f4,M),ppower(f6,3,M),M),M)
    return trim([z%M for z in f11])
def ev(a,x,M):
    v=0
    for z in reversed(a):v=(v*x+z)%M
    return v

def divlinear_monic(F,r,M):
    # F=(X-r)*Q, synthesized from high coefficients.
    n=len(F)-1;q=[0]*n
    q[-1]=F[-1]%M
    for j in range(n-2,-1,-1):q[j]=(F[j+1]+r*q[j+1])%M
    req((F[0]+r*q[0])%M==0,"synthetic quotient remainder")
    return trim(q)

def reduce(a,F,M):
    a=[x%M for x in a]
    inv=pow(F[-1],-1,M);n=len(F)-1
    for k in range(len(a)-1,n-1,-1):
        t=a[k]*inv%M
        if t:
            for j in range(n+1):a[k-n+j]=(a[k-n+j]-t*F[j])%M
    return trim(a[:n] or [0])

def gcdpoly(a,b,M):
    while b!=[0]:
        a,b=b,reduce(a,b,M)
    return [z*pow(a[-1],-1,M)%M for z in a]

class Etale:
    def __init__(self,M):
        self.M=M;self.F=division_polynomial(M)
        req(len(self.F)==61 and self.F[-1]==11,"wrong psi11 leading")
        self.rhs=[(-20)%M,(-10)%M,(-1)%M,1]
    def z(self):return ([0],[0])
    def o(self):return ([1],[0])
    def add(self,u,v):return (trim([(a+b)%self.M for a,b in zip_pad(u[0],v[0])]),trim([(a+b)%self.M for a,b in zip_pad(u[1],v[1])]))
    def sub(self,u,v):return (minus(u[0],v[0],self.M),minus(u[1],v[1],self.M))
    def prod(self,u,v):
        a,b=u;c,d=v;M=self.M
        bd=mul(b,d,M)
        return (reduce([(x+y)%M for x,y in zip_pad(mul(a,c,M),mul(bd,self.rhs,M))],self.F,M),
                reduce(minus([(x+y)%M for x,y in zip_pad(mul(a,d,M),mul(b,c,M))],bd,M),self.F,M))
    def eq(self,u,v):return all(reduce(minus(u[i],v[i],self.M),self.F,self.M)==[0] for i in (0,1))
    def source_idempotent(self,x,y):
        p=P
        req(0<=x<p and 0<=y<p,"bad canonical encoding")
        req(ev(self.F,x,p)==0,"not an eleven torsion x")
        req((y*y+y-(x*x*x-x*x-10*x-20))%p==0,"bad ordinate")
        q=divlinear_monic([v%p for v in self.F],x,p)
        deriv=sum(i*(self.F[i]%p)*pow(x,i-1,p) for i in range(1,len(self.F)))%p
        req(deriv!=0 and (2*y+1)%p!=0,"ramified source")
        ex=[z*pow(deriv,-1,p)%p for z in q]
        k=pow((2*y+1)%p,-1,p)
        return ([((y+1)*k*z)%p for z in ex],[(k*z)%p for z in ex])
    def lift2(self,e):
        # Universal idempotent Newton polynomial, no division or labels.
        e2=self.prod(e,e);e3=self.prod(e2,e)
        return self.sub(self.add(self.add(e2,e2),e2),self.add(e3,e3))

def zip_pad(a,b):
    for i in range(max(len(a),len(b))): yield (a[i] if i<len(a) else 0),(b[i] if i<len(b) else 0)

A=Etale(59);B=Etale(59**2)
req(gcdpoly(A.F,[i*A.F[i]%P for i in range(1,len(A.F))],P)==[1],
    "psi11 fails squarefree test at source prime")
req(gcdpoly(A.F,[z%P for z in D],P)==[1],"11-torsion x has branched y fiber")
xs=[x for x in range(P) if ev(A.F,x,P)==0]
req(xs==[19,20,22,43,46],"unexpected five linear factors")
points=[(x,y) for x in xs for y in range(P) if (y*y+y-(x*x*x-x*x-10*x-20))%P==0]
req(len(points)==10,"unexpected source places")
req(not any((x,y)==(19,23) for x,y in points),"bad negative input")
selectors=[];lifted=[]
for x,y in points:
    e=A.source_idempotent(x,y)
    req(A.eq(A.prod(e,e),e),"source selector not idempotent")
    E=B.lift2(e)
    req(B.eq(B.prod(E,E),E),"Newton lift not idempotent modulo 59^2")
    req(all([v%P for v in E[i]]==[v%P for v in e[i]] for i in (0,1)),
        "lift altered mod59 projector")
    req(all((ev(e[0],q_x,P)+ev(e[1],q_x,P)*q_y)%P==(1 if (q_x,q_y)==(x,y) else 0)
            for q_x,q_y in points),"oriented selector point evaluations")
    selectors.append(e);lifted.append(E)
req(all(A.eq(A.prod(selectors[i],selectors[j]),A.z()) for i in range(10) for j in range(i)),
    "selectors not orthogonal")
req(all(B.eq(B.prod(lifted[i],lifted[j]),B.z()) for i in range(10) for j in range(i)),
    "lifted selectors not orthogonal")
unit=A.z()
for e in selectors:unit=A.add(unit,e)
req(A.eq(A.prod(unit,unit),unit) and not A.eq(unit,A.o()),
    "source degree-one summand should be proper")
# An integer-coefficient representative E and E+59^2 have the SAME
# source mod-59^2 class but differing binary residue (59^2 is odd).
req((59**2)%2==1,"CRT target ambiguity not found")
req(not A.eq(selectors[0],A.o()) and not A.eq(selectors[0],A.z()),
    "trivial selector")
for invalid in [(19,23),(0,0),(-1,0),(19,59)]:
    try:A.source_idempotent(*invalid)
    except (ValueError,ArithmeticError):pass
    else:raise ArithmeticError("invalid encoded point was admitted")
print(json.dumps({"case":"ct114-source-etale-crt-idempotents","source_modulus":59,"source_order":11,
  "source_linear_x_roots":xs,"source_oriented_points":len(points),"primitive_idempotents_mod59":len(selectors),
  "lifted_idempotents_mod3481":len(lifted),"pairwise_orthogonality":True,
  "source_degree_one_summand_proper":True,"two_to_fiftynine_squared_ambiguity":True,
  "global_idempotent_in_K":False,"fixed_binary_target_value_selected":False,
  "online_scalar_used":False,"full_cold_cost_measured":False},sort_keys=True,separators=(',',':')))
