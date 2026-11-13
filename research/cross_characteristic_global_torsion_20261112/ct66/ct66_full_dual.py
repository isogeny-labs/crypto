#!/usr/bin/env python3
"""Full F1024 Mumford pullback + dual check for complementary genus-two elliptic quotients.
Standalone field arithmetic, Cantor, split/nonsplit divisor pushforward and toy all-31 check.
"""
import json
Q=1024;MOD=0x409

def check(cond,msg):
    if not cond:raise AssertionError(msg)
def m(a,b):
    o=0
    while b:
        if b&1:o^=a
        b>>=1;a<<=1
        if a&Q:a^=MOD
    return o&(Q-1)
def pw(a,n):
    o=1
    while n:
        if n&1:o=m(o,a)
        a=m(a,a);n>>=1
    return o
def inv(a):
    check(a!=0,'zero inverse')
    return pw(a,Q-2)
def norm(p):
    v=list(p)
    while v and v[-1]==0:v.pop()
    return tuple(v)
def pa(a,b):
    z=[0]*max(len(a),len(b))
    for i,v in enumerate(a):z[i]^=v
    for i,v in enumerate(b):z[i]^=v
    return norm(z)
def pm(a,b):
    if not a or not b:return ()
    z=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):z[i+j]^=m(x,y)
    return norm(z)
def ps(a,c):return norm(m(x,c) for x in a)
def pd(a,b):
    a=list(norm(a));b=norm(b)
    check(bool(b),'poly denominator')
    out=[0]*max(1,len(a)-len(b)+1)
    while a and len(a)>=len(b):
        d=len(a)-len(b);t=m(a[-1],inv(b[-1]));out[d]^=t
        for i,x in enumerate(b):a[i+d]^=m(t,x)
        while a and a[-1]==0:a.pop()
    return norm(out),norm(a)
def rem(a,b):return pd(a,b)[1]
def exact(a,b):
    out,left=pd(a,b)
    check(not left,('nonexact',a,b,left))
    return out
def monic(a):return ps(a,inv(a[-1])) if a else ()
def xgcd(a,b):
    r0,r1=norm(a),norm(b);s0,s1=(1,),();t0,t1=(),(1,)
    while r1:
        q,r=pd(r0,r1)
        r0,r1=r1,r;s0,s1=s1,pa(s0,pm(q,s1));t0,t1=t1,pa(t0,pm(q,t1))
    if not r0:return (),(),()
    z=inv(r0[-1])
    return ps(r0,z),ps(s0,z),ps(t0,z)
omega=next(x for x in range(2,Q) if m(x,x)^x^1==0)
check(omega==236,'omega')
f=(0,0,0,omega,0,1);h=(1,)
O=((1,),())
def canon(u,v):
    u=monic(u)
    if not u:return O
    v=rem(v,u)
    while len(u)-1>2:
        u2=monic(exact(pa(f,pa(pm(v,v),pm(h,v))),u))
        check(bool(u2),'zero reduction')
        v=rem(pa(h,v),u2);u=u2
    check(not rem(pa(f,pa(pm(v,v),pm(h,v))),u),('bad mumford',u,v))
    return u,v
def compose(A,B):
    u,v=A;w,z=B
    if len(u)==1:return B
    if len(w)==1:return A
    d1,e1,e2=xgcd(u,w)
    d,g1,g2=xgcd(d1,pa(pa(v,z),h))
    s1=pm(g1,e1);s2=pm(g1,e2);s3=g2
    U=exact(pm(u,w),pm(d,d))
    t=pa(pa(pm(pm(s1,u),z),pm(pm(s2,w),v)),pm(s3,pa(pm(v,z),f)))
    V=rem(exact(t,d),U)
    return canon(U,V)
def scalar(A,k):
    out=O
    while k:
        if k&1:out=compose(out,A)
        A=compose(A,A);k>>=1
    return out

def qp(t):
    r,a,b,c=t
    A=m(a,inv(r));D=m(c^m(A,pw(r,3)),inv(r))
    co=(inv(r),r,m(D,inv(r)))
    return {'r':r,'a':a,'b':b,'c':c,'A':A,'D':D,'coef':co}
T0=qp((20,6,120,432));T1=qp((20,6,120,433))
check(T0['coef']==(941,20,336) and T1['coef']==(941,20,714),'quotients')

class Base:
    def c(self,a):return a
    def add(self,a,b):return a^b
    def mul(self,a,b):return m(a,b)
    def inv(self,a):return inv(a)
class Ext:
    def __init__(self,u):self.u0=u[0];self.u1=u[1]
    def c(self,a):return a,0
    def add(self,a,b):return a[0]^b[0],a[1]^b[1]
    def mul(self,a,b):
        p=m(a[0],b[0])^m(m(a[1],b[1]),self.u0)
        z=m(a[0],b[1])^m(a[1],b[0])^m(m(a[1],b[1]),self.u1)
        return p,z
    def inv(self,a):
        t=a[0]^m(a[1],self.u1)
        n=m(a[0],t)^m(m(a[1],a[1]),self.u0)
        z=inv(n)
        return m(t,z),m(a[1],z)
B=Base()
def exp(F,a,n):
    out=F.c(1)
    while n:
        if n&1:out=F.mul(out,a)
        a=F.mul(a,a);n>>=1
    return out
def eadd(F,P,R,co):
    if P is None:return R
    if R is None:return P
    x,y=P;z,w=R;a,b,g=map(F.c,co)
    if x==z:
        if y!=w:return None
        slope=F.add(F.mul(a,exp(F,x,2)),g)
    else:slope=F.mul(F.add(y,w),F.inv(F.add(x,z)))
    x3=F.add(F.add(x,z),F.mul(F.add(b,exp(F,slope,2)),F.inv(a)))
    y3=F.add(F.add(F.mul(slope,F.add(x,x3)),y),F.c(1))
    return x3,y3
def project(F,P,t):
    x,y=P
    r=t['r'];A=t['A'];D=t['D'];co=t['coef']
    u=F.add(exp(F,x,2),F.mul(F.c(r),x))
    v=F.add(y,F.add(F.mul(F.c(A),exp(F,x,3)),F.mul(F.c(D),x)))
    eq=F.add(F.mul(F.c(co[0]),exp(F,u,3)),F.add(F.mul(F.c(co[1]),exp(F,u,2)),F.mul(F.c(co[2]),u)))
    check(F.add(exp(F,v,2),v)==eq,'quotient image')
    return u,v

def push(D,t):
    u,v=D
    if len(u)==1:return None,'zero'
    if len(u)==2:
        return project(B,(u[0],v[0] if v else 0),t),'linear'
    check(len(u)==3,'bad degree')
    roots=[x for x in range(Q) if m(x,x)^m(u[1],x)^u[0]==0]
    if roots:
        out=None
        for x in roots:
            y=(v[0] if v else 0)^(m(v[1],x) if len(v)>1 else 0)
            P=project(B,(x,y),t)
            out=eadd(B,out,P,t['coef'])
            if len(roots)==1:out=eadd(B,out,P,t['coef'])
        return out,'repeated' if len(roots)==1 else 'split'
    F=Ext(u);x=(0,1);x2=(u[1],1)
    out=None
    for X in (x,x2):
        Y=F.add(F.c(v[0] if v else 0),F.mul(F.c(v[1] if len(v)>1 else 0),X))
        out=eadd(F,out,project(F,(X,Y),t),t['coef'])
    check(out is None or (out[0][1]==out[1][1]==0),'non rational norm')
    return (None if out is None else (out[0][0],out[1][0])),'nonsplit'

def pull(P,t):
    if P is None:return O
    U,V=P;r=t['r'];A=t['A'];D=t['D']
    u=(U,r,1)
    # y = V + A*x^3 + D*x; reduce mod x^2+r*x+U
    v=rem((V,D,0,A),u)
    return canon(u,v)

def negE(P):return None if P is None else (P[0],P[1]^1)
def multE(P,n,co):
    out=None
    while n:
        if n&1:out=eadd(B,out,P,co)
        P=eadd(B,P,P,co);n>>=1
    return out

def toF4classes():
    # The source F4 group is generated by G=[(0,0)-infty] of order 31.
    G=((0,1),())
    check(canon(*G)==G,'source point')
    return [scalar(G,k) for k in range(31)]

classes=toF4classes()
support={'zero':0,'linear':0,'split':0,'repeated':0,'nonsplit':0}
checked=0
for D in classes:
    e0,t0=push(D,T0);e1,t1=push(D,T1)
    check(t0==t1,'support discrepancy');support[t0]+=1
    back=compose(pull(e0,T0),pull(e1,T1))
    check(back==scalar(D,2),('dual norm != 2',D,back,scalar(D,2)))
    check(scalar(back,16)==D,('inverse failed',D))
    check(push(back,T0)[0]==multE(e0,2,T0['coef']),'E0 dual')
    check(push(back,T1)[0]==multE(e1,2,T1['coef']),'E1 dual')
    checked+=1

# Explicit target-only samples not limited to the source F4-rational subgroup.
# Over F1024 all 31-torsion is rational and #E=31^2.
def findE(t):
    co=t['coef'];out=[]
    rts={}
    for v in range(Q):rts.setdefault(m(v,v)^v,[]).append(v)
    for U in range(Q):
        rhs=m(co[0],pw(U,3))^m(co[1],pw(U,2))^m(co[2],U)
        for V in rts.get(rhs,[]):out.append((U,V))
    check(len(out)==960,'E count')
    return out
E0=findE(T0);E1=findE(T1)
for i in (0,1,5,23,83,137):
    e0=E0[i];e1=E1[(i*7+91)%len(E1)]
    D=compose(pull(e0,T0),pull(e1,T1))
    check(push(D,T0)[0]==multE(e0,2,T0['coef']),('target E0',i))
    check(push(D,T1)[0]==multE(e1,2,T1['coef']),('target E1',i))
    R=scalar(D,16)
    check(push(R,T0)[0]==e0 and push(R,T1)[0]==e1,('full target inverse',i))
print(json.dumps({'source_31_classes':checked,'types':support,'target_31_torsion_pairs_tested':6,'quotients':[T0['coef'],T1['coef']],'dual_scalar':2,'inverse_scalar_mod31':16},sort_keys=True,separators=(',',':')))
