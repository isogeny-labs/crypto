import itertools, random, os, json, subprocess
from sympy import Poly, symbols, GF, cyclotomic_poly
X=symbols('x')
ORD={11:991,23:4196903}
def setup(n):
    p=2*n+1
    fl=Poly(cyclotomic_poly(p,X),X,modulus=2).factor_list()[1]
    f=sorted(fl,key=lambda t:str(t[0]))[0][0]
    co=f.all_coeffs()[::-1]; F=sum((int(c)%2)<<i for i,c in enumerate(co))
    def mul(a,b):
        r=0
        while b:
            if b&1: r^=a
            b>>=1; a<<=1
            if a>>n&1: a^=F
        return r
    g=[1]
    for k in range(1,p): g.append(mul(g[-1],2))
    assert mul(g[-1],2)==1
    onb=[g[pow(2,i,p)]^g[(-pow(2,i,p))%p] for i in range(n)]
    # inverse: express poly bits in onb coords: solve matrix
    rows=[(onb[i],1<<i) for i in range(n)]
    basis={}  # pivot bit -> (vec,tag)
    for v,t in rows:
        for b in sorted(basis,reverse=True):
            if v>>b&1: v^=basis[b][0]; t^=basis[b][1]
        assert v, "ONB not basis"
        hb=v.bit_length()-1
        for b in list(basis):
            if basis[b][0]>>hb&1: basis[b]=(basis[b][0]^v,basis[b][1]^t)
        basis[hb]=(v,t)
    conv=[basis[b][1] for b in range(n)]  # poly bit b -> onb coord vector
    def to_onb(x):
        c=0
        for b in range(n):
            if x>>b&1: c^=conv[b]
        return c
    return dict(n=n,p=p,F=F,mul=mul,g=g,onb=onb,conv=conv,to_onb=to_onb,r=ORD[n])
def from_onb(S,c):
    x=0
    for i in range(S['n']):
        if c>>i&1: x^=S['onb'][i]
    return x
def cyc(S,w):
    d={}
    for T in itertools.combinations(range(S['p']),w):
        x=0
        for k in T: x^=S['g'][k]
        if x: d.setdefault(x,[]).append(T)
    return d
def ham(S,r):
    out=set()
    for k in range(1,r+1):
        for T in itertools.combinations(range(S['n']),k):
            out.add(from_onb(S,sum(1<<i for i in T)))
    return out
def subspace(S,k,seed):
    rnd=random.Random(seed); n=S['n']
    while True:
        vs=[rnd.randrange(1,1<<n) for _ in range(k)]
        span={0}
        for v in vs: span|={s^v for s in span}
        if len(span)==1<<k: return vs, span-{0}
