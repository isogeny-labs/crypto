#!/usr/bin/env python3
"""Encoded F4 genus-2 Mumford divisors -> two F1024 elliptic quotients.
Reuses CT62 Cantor arithmetic; transports divisors without DLP labels.
"""
import json,sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'ct62'))
from ct62_genus2_cantor import classes,compose
K=10; MOD=0x409; Q=1<<K

def check(ok,why):
    if not ok:raise ArithmeticError(why)
def mul(a,b):
    o=0
    while b:
        if b&1:o^=a
        b>>=1;a<<=1
        if a&Q:a^=MOD
    return o&(Q-1)
def power(a,n):
    s=1
    while n:
        if n&1:s=mul(s,a)
        a=mul(a,a);n>>=1
    return s
def inv(a):
    check(a!=0,'zero inverse')
    return power(a,Q-2)
OMEGA=next(i for i in range(2,Q) if mul(i,i)^i^1==0)
F4=(0,1,OMEGA,OMEGA^1)
IDX={x:i for i,x in enumerate(F4)}
def m4(a,b):return IDX[mul(F4[a],F4[b])]

# Fixed lifting witnesses; exhaustive search and alternatives checked in JS.
LIFTS=((20,6,120,432),(534,794,636,668))
def quotient(t):
    r,a,b,c=t
    check(mul(a,mul(r,r))==mul(b,r),'translation does not square to 1')
    A=mul(a,inv(r)); C=mul(c^mul(A,power(r,3)),inv(r))
    co=(inv(r),r,mul(C,inv(r)))
    check(mul(A,A)==co[0] and mul(co[0],power(r,3))==(OMEGA^A),'quotient cubic')
    check(mul(co[1],mul(r,r))^co[2]==mul(C,C),'quotient quadratic')
    check(mul(co[2],r)==C,'quotient linear')
    return co,A,C

class Base:
    def c(self,a):return a
    def add(self,a,b):return a^b
    def mul(self,a,b):return mul(a,b)
    def inv(self,a):return inv(a)
class Quadratic:
    def __init__(self,u0,u1):self.u0=u0;self.u1=u1
    def c(self,a):return a,0
    def add(self,a,b):return a[0]^b[0],a[1]^b[1]
    def mul(self,a,b):
        p=mul(a[0],b[0])^mul(mul(a[1],b[1]),self.u0)
        q=mul(a[0],b[1])^mul(a[1],b[0])^mul(mul(a[1],b[1]),self.u1)
        return p,q
    def inv(self,a):
        t=a[0]^mul(a[1],self.u1)
        n=mul(a[0],t)^mul(mul(a[1],a[1]),self.u0)
        z=inv(n)
        return mul(t,z),mul(a[1],z)
B=Base()
def powx(F,x,n):
    s=F.c(1)
    while n:
        if n&1:s=F.mul(s,x)
        x=F.mul(x,x);n>>=1
    return s
def eadd(F,P,R,co):
    if P is None:return R
    if R is None:return P
    x,y=P;z,w=R;a,b,g=map(F.c,co)
    if x==z:
        if y!=w:return None
        m=F.add(F.mul(a,powx(F,x,2)),g)
    else:m=F.mul(F.add(y,w),F.inv(F.add(x,z)))
    x3=F.add(F.add(x,z),F.mul(F.add(b,powx(F,m,2)),F.inv(a)))
    y3=F.add(F.add(F.mul(m,F.add(x,x3)),y),F.c(1))
    return x3,y3

def project(F,P,t):
    x,y=P;r,_,_,_=t;co,A,C=quotient(t)
    u=F.add(powx(F,x,2),F.mul(F.c(r),x))
    v=F.add(y,F.add(F.mul(F.c(A),powx(F,x,3)),F.mul(F.c(C),x)))
    eq=F.add(F.mul(F.c(co[0]),powx(F,u,3)),F.add(F.mul(F.c(co[1]),powx(F,u,2)),F.mul(F.c(co[2]),u)))
    check(F.add(powx(F,v,2),v)==eq,'image off quotient')
    return u,v

def push(D,t):
    u,v=D;co=quotient(t)[0]
    if len(u)==1:return None,'zero'
    if len(u)==2:
        return project(B,(F4[u[0]],F4[v[0]] if v else 0),t),'linear'
    check(len(u)==3,'nonreduced divisor')
    roots=[x for x in range(4) if m4(x,x)^m4(u[1],x)^u[0]==0]
    if roots:
        S=None
        for x in roots:
            y=v[0]^(m4(v[1],x) if len(v)>1 else 0) if v else 0
            p=project(B,(F4[x],F4[y]),t)
            S=eadd(B,S,p,co)
            if len(roots)==1:S=eadd(B,S,p,co)
        return S,'repeated' if len(roots)==1 else 'split'
    # Irreducible degree-2 F4 support remains nonsplit over F1024 (extension 5 odd).
    F=Quadratic(F4[u[0]],F4[u[1]])
    x=(0,1);x2=(F4[u[1]],1)
    pts=[]
    for X in (x,x2):
        Y=F.add(F.c(F4[v[0]] if v else 0),F.mul(F.c(F4[v[1]] if len(v)>1 else 0),X))
        pts.append(project(F,(X,Y),t))
    out=eadd(F,*pts,co)
    check(out is None or (out[0][1]==out[1][1]==0),'non-rational quadratic pushforward')
    return (None if out is None else (out[0][0],out[1][0])),'nonsplit'

def main():
    check(OMEGA==236,'field model')
    co=[quotient(t)[0] for t in LIFTS]
    check(co==[(941,20,336),(249,534,39)],'explicit quotient')
    # CT62 classes() indexes h[0:3] directly: retain its canonical padded ABI.
    f=(0,0,0,2,0,1,0);h=(1,0,0,0)
    group=classes(f,h)
    check(len(group)==31,'wrong source group order')
    G=((0,1),());check(G in group,'generator missing')
    images={};support={'zero':0,'linear':0,'split':0,'repeated':0,'nonsplit':0}
    for D in group:
        p,t=push(D,LIFTS[0]);q,s=push(D,LIFTS[1]);check(t==s,'support mismatch')
        support[t]+=1;images[D]=(p,q)
    check(images[G]==((0,0),(0,0)),'incorrect oriented generator')
    for A in group:
        for D in group:
            S=compose(A,D,f,h)
            expected=(eadd(B,images[A][0],images[D][0],co[0]),eadd(B,images[A][1],images[D][1],co[1]))
            check(images[S]==expected,'not a group homomorphism')
    # The source scalar k is used only for the known-answer check, never by push().
    P=group[0];logs={P:0}
    for k in range(1,31):
        P=compose(P,G,f,h);logs[P]=k
    check(len(logs)==31 and compose(P,G,f,h)==group[0],'source cycle')
    for D in group:
        for i in range(2):
            R=None;found=None
            for k in range(31):
                if R==images[D][i]:found=k;break
                R=eadd(B,R,images[G][i],co[i])
            check(found==logs[D],'toy DLP mismatch')
    print(json.dumps({'quotients':co,'source_classes':len(group),'source_support':support,
                      'homomorphism_pairs':961,'destination_logs':62,
                      'generator_images':images[G]},sort_keys=True,separators=(',',':')))
if __name__=='__main__':main()
