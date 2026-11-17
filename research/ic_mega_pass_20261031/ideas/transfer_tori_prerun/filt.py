# x-functional filter from the F_2[Z/n]-module (group-algebra) structure of the trace-zero hyperplane H of F_{2^n}:
# H ~ F_2[X]/Phi_n(X) ~ F_{2^(n-1)} as F_2[sigma]-module; bucket(P) = chi_m(u(x(P))) with chi_m a multiplicative character of F_{2^(n-1)}.
# Test: does bucket(P+Q) depend on (bucket P, bucket Q) more than chance? K_0: y^2+xy=x^3+1 over F_{2^13}, <G> of prime order 2003.
import numpy as np, random, sys
n=13; MOD=(1<<13)|(1<<4)|(1<<3)|(1<<1)|1
def mul(a,b):
    r=0
    while b:
        if b&1: r^=a
        b>>=1; a<<=1
        if a>>n: a^=MOD
    return r
def sq(a): return mul(a,a)
def inv(a):
    # a^(2^n-2)
    r=1; e=(1<<n)-2
    while e:
        if e&1: r=mul(r,a)
        a=mul(a,a); e>>=1
    return r
def tr(a):
    t=0; x=a
    for i in range(n): t^=x; x=sq(x)
    return t
def halftrace_solve(c):
    # solve l^2+l=c (Tr c = 0), n odd: l = sum_{i=0}^{(n-1)/2} c^(2^(2i))
    l=0; x=c
    for i in range((n+1)//2):
        l^=x; x=sq(sq(x))
    return l
# curve y^2+xy=x^3+1 (a=0,b=1). Point addition in affine coords; O = None
def add(P,Q):
    if P is None: return Q
    if Q is None: return P
    x1,y1=P; x2,y2=Q
    if x1==x2:
        if y1!=y2 or x1==0: return None
        lam=x1^mul(y1,inv(x1))
        x3=sq(lam)^lam  # a=0
        y3=sq(x1)^mul(lam^1,x3)
        return (x3,y3)
    lam=mul(y1^y2,inv(x1^x2))
    x3=sq(lam)^lam^x1^x2
    y3=mul(lam,x1^x3)^x3^y1
    return (x3,y3)
def smul(k,P):
    R=None; Q=P
    while k:
        if k&1: R=add(R,Q)
        Q=add(Q,Q); k>>=1
    return R
def lift(x):
    # y^2+xy=x^3+1 -> (y/x)^2+(y/x)=x+1/x^2 ; need Tr(x+1/x^2)=0
    if x==0: return None
    c=x^sq(inv(x))
    if tr(c): return None
    l=halftrace_solve(c)
    return (x,mul(l,x))
random.seed(1)
# find point of order 2003
N=8012; r=2003
while True:
    x=random.randrange(1,1<<n); P=lift(x)
    if P is None: continue
    G=smul(4,P)
    if G is not None and smul(r,G) is None: break
pts=[None]*r; Q=None
for i in range(r):
    pts[i]=Q; Q=add(Q,G)
assert Q is None
xs=[p[0] for p in pts[1:]]
assert all(tr(x)==0 for x in xs), "x(<G>) not in trace-zero hyperplane?"
# module isomorphism H -> F_2[X]/Phi_13: pick h0 in H, basis sigma^i h0 (i<12)
h0=xs[0]
basis=[]; v=h0
for i in range(n-1): basis.append(v); v=sq(v)
# solve coordinates by Gaussian elimination over F_2 (13-bit vectors, 12 basis vectors)
def coords(h):
    # brute: build matrix once
    return None
# build inverse map via linear algebra: matrix M (13 x 12) columns basis; find c with M c = h
import itertools
rows=[[ (basis[j]>>i)&1 for j in range(n-1)] for i in range(n)]
# augmented solving each time is slow; precompute pivot structure using numpy over GF(2)
M=np.array(rows,dtype=np.uint8)
def solve(h):
    A=np.concatenate([M, np.array([[(h>>i)&1] for i in range(n)],dtype=np.uint8)],axis=1)
    A=A.copy(); piv=[]
    rr=0
    for c in range(n-1):
        pr=None
        for i in range(rr,n):
            if A[i,c]: pr=i;break
        if pr is None: continue
        A[[rr,pr]]=A[[pr,rr]]
        for i in range(n):
            if i!=rr and A[i,c]: A[i]^=A[rr]
        piv.append(c); rr+=1
    assert rr==n-1
    assert not A[rr:, -1].any(), "h not in span"
    c=np.zeros(n-1,dtype=np.uint8)
    for i,cc in enumerate(piv): c[cc]=A[i,-1]
    return int(''.join(str(int(b)) for b in c[::-1]),2)  # polynomial in X as int bits
# arithmetic in F_2[X]/Phi_13(X), Phi_13 = X^12+...+1
PHI=(1<<13)-1  # bits 0..12 set
def pmul(a,b):
    r=0
    while b:
        if b&1: r^=a
        b>>=1; a<<=1
        if a>>12: a^=PHI
    return r
def ppow(a,e):
    r=1
    while e:
        if e&1: r=pmul(r,a)
        a=pmul(a,a); e>>=1
    return r
us=[solve(x) for x in xs]
# sanity: u(sigma h) = X*u(h)
assert solve(sq(xs[5]))==pmul(2,us[5])
Qm=(1<<12)-1  # 4095 = 3^2*5*7*13
print("n=13, |<G>|=2003, H ~ F_2[X]/Phi_13 ~ F_4096; u(sigma h)=X u(h) verified")
rng=np.random.default_rng(0)
for m in [3,5,7,9,13,15,21,35,39,45,63,65,91,105]:
    e=Qm//m
    vals=[ppow(u,e) for u in us]
    codes={v:i for i,v in enumerate(sorted(set(vals)))}
    assert len(codes)<=m
    b=np.array([codes[v] for v in vals])  # index i -> point (i+1)G
    # bucket of kG for k=1..2002; pairs (i,j) with i+j != 0 mod r
    k=np.arange(1,r)
    bi=b[:,None]; bj=b[None,:]
    s=(k[:,None]+k[None,:])%r
    mask=s!=0
    bs=np.full(s.shape,-1); bs[mask]=b[s[mask]-1]
    def modal_rate(b_i,b_j,b_s,mask):
        key=(b_i*m+b_j)*m+b_s
        key=key[mask]
        cnt=np.bincount(key,minlength=m*m*m).reshape(m,m,m)
        return cnt.max(axis=2).sum()/cnt.sum()
    real=modal_rate(bi,bj,bs,mask)
    nulls=[]
    for t in range(5):
        bp=rng.permutation(b)
        bsp=np.full(s.shape,-1); bsp[mask]=bp[s[mask]-1]
        nulls.append(modal_rate(bp[:,None],bp[None,:],bsp,mask))
    print(f"m={m:3d} buckets_used={len(codes):3d} modal-predict rate real={real:.4f}  null={np.mean(nulls):.4f}+-{np.std(nulls):.4f}  chance=1/m={1/m:.4f}")
