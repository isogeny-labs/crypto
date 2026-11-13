#!/usr/bin/env python3
"""CT61: exact ordinary-binary-target construction, toy point replay."""
import json
from math import isqrt

PRIMES = [3, 5, 7, 11, 19, 43, 163, 257, 65537, 2147483647, 4294967291]
TOYS = {3, 5, 7, 11, 19, 43, 163}

def require(ok, why):
    if not ok:
        raise RuntimeError(why)

def target(ell, k):
    q = 1 << k
    radius = isqrt(4*q)
    m = (q+1+ell)//(2*ell)
    if m == 0:
        m = 1
    N = 2*ell*m
    t = q+1-N
    return (q,N,t,abs(t)<=radius)

def fmul(a, b, poly, k):
    q = 1 << k
    out = 0
    while b:
        if b & 1: out ^= a
        b >>= 1
        a <<= 1
        if a & q: a ^= poly
    return out

def fpow(a, n, poly, k):
    out = 1
    while n:
        if n & 1: out = fmul(out, a, poly, k)
        a = fmul(a, a, poly, k)
        n >>= 1
    return out

def prem(a, b):
    while a and a.bit_length() >= b.bit_length():
        a ^= b << (a.bit_length()-b.bit_length())
    return a

def pgcd(a,b):
    while b:
        a,b = b,prem(a,b)
    return a

def is_irreducible(poly,k):
    x = 2
    power = x
    for i in range(1,k+1):
        power = fmul(power,power,poly,k)
        if i <= k//2 and k%i == 0 and pgcd(power^x,poly)!=1:
            return False
    return power == x

def find_poly(k):
    for poly in range((1<<k)|1,1<<(k+1),2):
        if is_irreducible(poly,k):
            return poly
    raise RuntimeError('no irreducible polynomial')

def witness(ell,k,N,t):
    q = 1 << k
    poly = find_poly(k)
    square = [fmul(x,x,poly,k) for x in range(q)]
    tr = []
    for x in range(q):
        v=x; s=0
        for _ in range(k):
            s ^= v
            v=square[v]
        require(s in (0,1),'field trace not in F2')
        tr.append(s)
    a2one = next(x for x in range(q) if tr[x]==1)
    invsq = [0] + [square[fpow(x,q-2,poly,k)] for x in range(1,q)]
    AS = {}
    for z in range(q): AS[square[z]^z] = z
    for a6 in range(1,q):
        for a2 in [0,a2one]:
            count = 2 + 2*sum(tr[x^a2^fmul(a6,invsq[x],poly,k)]==0 for x in range(1,q))
            if count != N: continue
            def is_on(P):
                if P is None: return True
                x,y=P
                return square[y]^fmul(x,y,poly,k) == fmul(square[x],x,poly,k)^fmul(a2,square[x],poly,k)^a6
            def add(P,Q):
                if P is None: return Q
                if Q is None: return P
                x,y=P; u,v=Q
                if x == u:
                    if y^v == x: return None
                    if x == 0: return None
                    slope=x^fmul(y,fpow(x,q-2,poly,k),poly,k)
                    X=square[slope]^slope^a2
                    Y=square[x]^fmul(slope^1,X,poly,k)
                else:
                    slope=fmul(y^v,fpow(x^u,q-2,poly,k),poly,k)
                    X=square[slope]^slope^x^u^a2
                    Y=fmul(slope,x^X,poly,k)^X^y
                return X,Y
            def smul(n,P):
                Q=None
                while n:
                    if n&1: Q=add(Q,P)
                    P=add(P,P);n>>=1
                return Q
            for x in range(1,q):
                rhs=x^a2^fmul(a6,invsq[x],poly,k)
                if rhs not in AS: continue
                P=(x,fmul(x,AS[rhs],poly,k))
                R=smul(N//ell,P)
                if R is None: continue
                require(is_on(P) and is_on(R),'point equation')
                require(smul(ell,R) is None,'bad order')
                Q=smul(7%ell,R)
                recovered=next(d for d in range(ell) if smul(d,R)==Q)
                require(recovered==7%ell,'DLP control failed')
                return {'ell':ell,'k':k,'q':q,'N':N,'t':t,'poly':poly,'a2':a2,'a6':a6,'P':P,'R':R,'Q':Q,'d':recovered}
    raise RuntimeError(f'no concrete witness for ell={ell}, k={k}')

def main():
    rows=[]; samples=[]
    for ell in PRIMES:
        m=ell.bit_length(); bound=2*m-2
        q,N,t,ok=target(ell,bound)
        require(ok and t%2 and N%ell==0,'uniform bound failed')
        first=next(k for k in range(1,bound+1) if target(ell,k)[-1])
        q,N,t,_=target(ell,first)
        require(abs(t)<=isqrt(4*q) and t%2 and N%ell==0,'trace construction failed')
        rows.append([ell,first,t,N,bound])
        if ell in TOYS: samples.append(witness(ell,first,N,t))
    print(json.dumps({'rows':rows,'toys':samples},separators=(',',':')))

if __name__=='__main__':main()
