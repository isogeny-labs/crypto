#!/usr/bin/env python3
"""Independent Prime31 run62 field/exceptional Jacobian certificate (not solver)."""
import random

P = 2147483647
A = 847514846
N = 2147446831
G = (625037715, 2100627251)

def affine_add(q, r):
    if q is None: return r
    if r is None: return q
    x, y = q
    u, v = r
    if x == u and (y + v) % P == 0: return None
    if x == u: slope = (3*x*x+A)*pow(2*y, -1, P) % P
    else: slope = (v-y)*pow(u-x, -1, P) % P
    xx = (slope*slope-x-u) % P
    return xx, (slope*(x-xx)-y) % P

def scalar(k, q):
    acc = None
    while k:
        if k & 1: acc = affine_add(acc, q)
        q = affine_add(q, q)
        k >>= 1
    return acc

def mul(a,b):
    product = a*b
    t = (product & P) + (product >> 31)
    return t-P if t>=P else t

def add(a,b):
    t=a+b
    return t-P if t>=P else t

def sub(a,b):
    return a-b if a>=b else a+P-b

def double(j):
    x,y,z=j
    if y==0: return None
    xx=mul(x,x); yy=mul(y,y); yyyy=mul(yy,yy)
    z2=mul(z,z); z4=mul(z2,z2)
    dbl=lambda v:add(v,v)
    d=dbl(sub(sub(mul(add(x,yy),add(x,yy)),xx),yyyy))
    m=add(add(xx,dbl(xx)),mul(A,z4))
    x3=sub(mul(m,m),dbl(d))
    y3=sub(mul(m,sub(d,x3)),dbl(dbl(dbl(yyyy))))
    return x3,y3,mul(dbl(y),z)

def shift(j):
    if j is None: return G[0],G[1],1
    x,y,z=j
    zz=mul(z,z); u=mul(G[0],zz); s=mul(G[1],mul(z,zz))
    h=sub(u,x); r=sub(s,y)
    if h==0: return double(j) if r==0 else None
    hh=mul(h,h); hhh=mul(h,hh); v=mul(x,hh)
    x3=sub(sub(mul(r,r),hhh),add(v,v))
    y3=sub(mul(r,sub(v,x3)),mul(y,hhh))
    return x3,y3,mul(z,h)

def normalize(j):
    if j is None: return None
    x,y,z=j
    inv=pow(z,-1,P);inv2=mul(inv,inv)
    return mul(x,inv2),mul(y,mul(inv2,inv))

def main():
    rnd=random.Random(0x62fe11)
    for a in (0,1,2,P-2,P-1):
        for b in (0,1,2,P-2,P-1):
            assert (add(a,b),sub(a,b),mul(a,b)) == ((a+b)%P,(a-b)%P,(a*b)%P)
    for _ in range(120000):
        a=rnd.randrange(P);b=rnd.randrange(P)
        assert (add(a,b),sub(a,b),mul(a,b)) == ((a+b)%P,(a-b)%P,(a*b)%P)
    assert scalar(N,G) is None
    ks=[0,1,2,3,N-1,N-2,N//2,1_000_000]
    ks.extend(rnd.randrange(N) for _ in range(88))
    for k in ks:
        q=scalar(k,G)
        j=None if q is None else (q[0],q[1],1)
        for i in range(128):
            q=affine_add(q,G);j=shift(j)
            assert normalize(j)==q,(k,i)
    print("PASS: 120025 field operand pairs; 96 complete Jacobian chains, 12288 steps; identity, doubling, inverse, order wrap")

if __name__=="__main__": main()
