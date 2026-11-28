#!/usr/bin/env python3
"""Independent actual Prime31 EC verification of partial-rank target closure.
Known-scalar planted relation fixtures ONLY; candidate never receives target log.
"""
import random
P,N=2147483647,2147446831
A,B=847514846,2087444724
G=(625037715,2100627251)

def add(U,V):
    if U is None:return V
    if V is None:return U
    x,y=U;u,v=V
    if x==u and (y+v)%P==0:return None
    if x==u:s=(3*x*x+A)*pow(2*y,-1,P)%P
    else:s=(v-y)*pow(u-x,-1,P)%P
    t=(s*s-x-u)%P
    return (t,(s*(x-t)-y)%P)
def mul(k,V):
    q=None
    while k:
        if k&1:q=add(q,V)
        V=add(V,V);k>>=1
    return q

def signed(s,x):return x if s==1 else (-x)%N
class AffineGraph:
    def __init__(self,n):
        self.p=list(range(n));self.s=[1]*n;self.d=[0]*n;self.fixed=[None]*n
    def root(self,i):
        parent=self.p[i]
        if i==parent:return i,1,0
        r,sign,shift=self.root(parent)
        self.p[i]=r;self.d[i]=(self.d[i]+signed(self.s[i],shift))%N
        self.s[i]*=sign
        return r,self.s[i],self.d[i]
    def fix(self,u,v):
        if self.fixed[u] is None:self.fixed[u]=v
        else:assert self.fixed[u]==v
    def relation(self,i,a,j,b,k):
        u,s,di=self.root(i);v,t,dj=self.root(j)
        c1=a*s;c2=b*t
        rhs=(k-signed(a,di)-signed(b,dj))%N
        if u==v:
            coeff=c1+c2
            if coeff==0:assert rhs==0
            else:self.fix(u,rhs*pow(coeff%N,-1,N)%N)
        else:
            sign=-c1*c2
            shift=signed(c2,rhs)
            self.p[v]=u;self.s[v]=sign;self.d[v]=shift
            if self.fixed[v] is not None:
                self.fix(u,signed(sign,(self.fixed[v]-shift)%N))
    def known_signed_sum(self,i,a,j,b):
        u,s,di=self.root(i);v,t,dj=self.root(j)
        const=(signed(a,di)+signed(b,dj))%N
        c1=a*s;c2=b*t
        if u==v:
            coeff=c1+c2
            if coeff==0:return const
            if self.fixed[u] is not None:return (const+coeff*self.fixed[u])%N
        elif self.fixed[u] is not None and self.fixed[v] is not None:
            return (const+c1*self.fixed[u]+c2*self.fixed[v])%N
        return None

def test(seed):
    rng=random.Random(seed)
    # The original curve is not weakened or changed; only this planted
    # correctness fixture uses reference base scalars.
    logs=[rng.randrange(1,N) for _ in range(12)]
    points=[mul(k,G) for k in logs]
    graph=AffineGraph(len(logs))
    for i,a,j,b in [(0,1,1,1),(1,1,2,1),(0,1,2,1)]:
        k=(a*logs[i]+b*logs[j])%N
        expected=add(mul(a%N,points[i]),mul(b%N,points[j]))
        assert mul(k,G)==expected
        graph.relation(i,a,j,b,k)
    secret=rng.randrange(1,N)
    public_target=mul(secret,G)
    shift=(logs[0]-logs[1]-secret)%N
    assert add(public_target,mul(shift,G))==add(points[0],mul(N-1,points[1]))
    known=graph.known_signed_sum(0,1,1,-1)
    assert known is not None
    candidate=(known-shift)%N
    assert mul(candidate,G)==public_target and candidate==secret
    # Partial rank: there are base logs not fixed; this is NOT all-log recovery.
    assert graph.known_signed_sum(10,1,11,1) is None

if __name__=="__main__":
    for trial in range(64):test(0x58a661f1+7919*trial)
    print("PASS: 64 original-Prime31 EC group-law target recoveries, planted relation fixtures only")
