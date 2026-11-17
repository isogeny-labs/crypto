import random, sys, json, time, itertools, math
from math import comb
sys.path.insert(0,'/Volumes/SSD990-2/crypto-autoresearcher/research/verification')

# ---------- GF(2)[x] and K=F_{2^n} ----------
def pmulmod(a,b,m,n):
    r=0
    while b:
        if b&1: r^=a
        b>>=1; a<<=1
        if (a>>n)&1: a^=m
    return r
def pgcd(a,b):
    while b:
        while a and a.bit_length()>=b.bit_length(): a^=b<<(a.bit_length()-b.bit_length())
        a,b=b,a
    return a
def irreducible(m,n):
    x=2; t=x
    for i in range(1,n//2+1):
        t=pmulmod(t,t,m,n)
        if pgcd(m, t^x)!=1: return False
    return True
def modulus(n):
    for c in range(1,n):
        m=(1<<n)|(1<<c)|1
        if irreducible(m,n): return m
    for a in range(1,n):
        for b in range(a+1,n):
            for c in range(b+1,n):
                m=(1<<n)|(1<<c)|(1<<b)|(1<<a)|1
                if irreducible(m,n): return m
def rand_subspace(n,k,rng):
    while True:
        B=[rng.getrandbits(n) for _ in range(k)]
        if f2rank(B)==k: return B
def f2rank(vecs):
    piv={}
    r=0
    for v in vecs:
        while v:
            p=v.bit_length()-1
            if p in piv: v^=piv[p]
            else: piv[p]=v; r+=1; break
    return r

# ---------- system: T[i] = list of (j,l) bitmatrix as ints; affine ---------
def semaev_parts(n,m,V,z,a6):
    k=len(V); L=lambda x: pmulmod(x,x,m,n)^pmulmod(z,x,m,n)
    z2=pmulmod(z,z,m,n)
    bil=[[L(pmulmod(V[j],V[l],m,n)) for l in range(k)] for j in range(k)]  # K-valued
    affa=[pmulmod(z2,pmulmod(V[j],V[j],m,n),m,n) for j in range(k)]
    return bil, affa, affa[:], a6
def mult_tensor(n,m,W1,W2,Lmap):
    k=len(W1); return [[Lmap(pmulmod(W1[j],W2[l],m,n)) for l in range(k)] for j in range(k)]
def to_eqs(n,k,bil,affa,affb,c):
    # vars: a_j -> bit j, b_l -> bit k+l ; eq i = set of monomask
    eqs=[]
    for i in range(n):
        f=set()
        for j in range(k):
            for l in range(k):
                if (bil[j][l]>>i)&1: f.add((1<<j)|(1<<(k+l)))
            if (affa[j]>>i)&1: f.add(1<<j)
            if (affb[j]>>i)&1: f.add(1<<(k+j))
        if (c>>i)&1: f.add(0)
        eqs.append(frozenset(f))
    return eqs

# ---------- Boolean closure W_d ----------
def closure(eqs,N,d):
    monos=[]
    for e in range(d,-1,-1):           # highest degree -> highest bit
        for S in itertools.combinations(range(N),e): monos.append(sum(1<<s for s in S))
    monos.reverse()
    idx={mm:i for i,mm in enumerate(monos)}
    nlow=sum(comb(N,e) for e in range(d))  # bits < nlow are degree<=d-1
    deg=lambda mm: bin(mm).count('1')
    piv={}
    def insert(row):
        while row:
            p=row.bit_length()-1
            q=piv.get(p)
            if q is None: piv[p]=row; return p
            row^=q
        return None
    def row_of(P):
        r=0
        for mm in P: r^=1<<idx[mm]
        return r
    # bit -> monomial
    def poly_of(row):
        out=[]
        while row:
            p=row.bit_length()-1; out.append(monos[p]); row^=1<<p
        return out
    for f in eqs:
        if f and max(map(deg,f))<=d: insert(row_of(f))
    done=set()
    while True:
        new=False
        for p in list(piv.keys()):
            if p>=nlow or p in done: continue
            done.add(p)
            g=poly_of(piv[p])
            for v in range(N):
                b=1<<v
                prod={}
                for mm in g:
                    t=mm|b; prod[t]=prod.get(t,0)^1
                r=row_of([t for t,c in prod.items() if c])
                if insert(r) is not None: new=True
        if not new: break
    dim=len(piv); low=sum(1 for p in piv if p<nlow)
    one = 0 in piv and piv[0]==1 or (insert(1) is None)
    return dict(dim=dim, low=low, one=bool(one), ncols=len(monos))
