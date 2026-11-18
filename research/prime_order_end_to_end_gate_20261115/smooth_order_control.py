#!/usr/bin/env python3
"""Complete prime-field ECDLP positive control via binary Pohlig--Hellman.

For Mersenne primes p=2^k-1 (k=31,61), E/F_p: y^2=x^3+x has
cyclic group of order p+1=2^k. Solves all public Q in <G> by recovering
k binary digits. This is classical smooth-order Pohlig--Hellman, NOT an
attack on cryptographic prime-order curves, and NOT a novel method.
Arithmetic is producer-local [pcc:unverified].
"""
import json
import math
import random
from statistics import mean

class ECC:
    def __init__(self,p):
        self.p=p
        self.ops=0
        self.invs=0
    def add(self,P,Q):
        if P is None:return Q
        if Q is None:return P
        self.ops+=1
        p=self.p
        x,y=P;z,w=Q
        if x==z and (y+w)%p==0:return None
        if x==z:
            assert y==w and y!=0
            num=3*x*x+1;den=2*y
        else:num=w-y;den=z-x
        self.invs+=1
        slope=(num*pow(den%p,-1,p))%p
        u=(slope*slope-x-z)%p
        return u,(slope*(x-u)-y)%p
    def neg(self,P):
        return None if P is None else (P[0],(-P[1])%self.p)
    def mul(self,n,P):
        assert n>=0
        res=None
        while n:
            if n&1:res=self.add(res,P)
            P=self.add(P,P)
            n>>=1
        return res


def lucas_lehmer(k):
    assert k>=3 and k%2
    m=(1<<k)-1
    s=4
    for _ in range(k-2):s=(s*s-2)%m
    return s==0


def generator(p,k):
    e=ECC(p)
    for x in range(1,200):
        f=(x**3+x)%p
        if f==0:continue
        y=pow(f,(p+1)//4,p)
        if y*y%p!=f:continue
        G=(x,y)
        if e.mul(1<<(k-1),G) is not None and e.mul(1<<k,G) is None:
            return G
    raise RuntimeError('no full-order generator found')


def binary_ph(group,G,Q,k):
    base=group.mul(1<<(k-1),G)
    if base is None or group.add(base,base) is not None:
        raise AssertionError("generator order certificate failed")
    R=Q;P=G;d=0
    for i in range(k):
        T=R
        for _ in range(k-i-1):T=group.add(T,T)
        if T is None:bit=0
        elif T==base:bit=1
        else:raise ValueError('Q not in expected cyclic group')
        if bit:
            d|=1<<i
            R=group.add(R,group.neg(P))
        P=group.add(P,P)
    if R is not None: raise AssertionError("residual not zero")
    if group.mul(d,G)!=Q: raise AssertionError("target replay failed")
    return d


def bsgs(group,G,Q,n):
    m=math.isqrt(n)
    if m*m<n:m+=1
    table={};q=None
    for i in range(m):
        table.setdefault(q,i)
        q=group.add(q,G)
    step=group.neg(group.mul(m,G))
    s=Q
    for j in range(m+1):
        if s in table:
            d=(j*m+table[s])%n
            if group.mul(d,G)!=Q: raise AssertionError("BSGS replay failed")
            return d
        s=group.add(s,step)
    raise ValueError('BSGS failed')


def run(k,with_bsgs):
    assert lucas_lehmer(k)
    p=(1<<k)-1
    assert p%4==3
    G=generator(p,k)
    n=p+1
    # For p==3(mod4), chi(x^3+x)+chi((-x)^3-x)=0; #E=p+1.
    # x^3+x=0 has only x=0; thus E[2](F_p) has size 2.
    # A finite abelian 2-group with unique involution is cyclic.
    assert ECC(p).mul(n,G) is None and ECC(p).mul(n//2,G) is not None
    rng=random.Random(4188+k)
    inputs=[0,1,2,3,n//2-1,n//2,n//2+1,n-1]
    inputs+=rng.sample(range(4,n//2-1), 8)
    out=[]
    for d in inputs:
        Q=ECC(p).mul(d,G)
        h=ECC(p)
        got=binary_ph(h,G,Q,k)
        assert got==d
        entry=dict(log=d,ph_point_ops=h.ops,ph_field_invs=h.invs)
        if with_bsgs:
            b=ECC(p)
            got2=bsgs(b,G,Q,n)
            assert got2==got
            entry.update(bsgs_point_ops=b.ops,bsgs_field_invs=b.invs)
        out.append(entry)
    result=dict(field_p=p,group_order=n,curve_a=1,curve_b=0,
                generator=G,target_count=len(out),
                ph_mean_ops=mean(e['ph_point_ops'] for e in out),
                ph_max_ops=max(e['ph_point_ops'] for e in out),
                ph_min_ops=min(e['ph_point_ops'] for e in out),
                primality='Lucas-Lehmer passed',
                cyclic_group_proof='p+1=2^k, unique rational nonidentity two-torsion',
                same_curve_known_order_given_to_both=True,
                rows=out)
    if with_bsgs:
        result['bsgs_mean_ops']=mean(e['bsgs_point_ops'] for e in out)
        result['bsgs_over_ph_mean_ratio']=result['bsgs_mean_ops']/result['ph_mean_ops']
    return result


def main():
    return dict(standing='[pcc:unverified]',
                novelty='none; established binary Pohlig-Hellman',
                scope='smooth 2-power group-order prime-field curves only; excludes prime-order target groups',
                p31=run(31,True),p61=run(61,False))

if __name__=='__main__':print(json.dumps(main(),sort_keys=True,indent=2))
