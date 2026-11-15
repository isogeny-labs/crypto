#!/usr/bin/env python3
"""Skeptic experiment for MP-ml_empirical-3.
m=2 binary Weil descent of S_3(X1,X2,z) on E: y^2+xy=x^3+a6 (a6=1, Koblitz-like) over F_{2^n}.
V = l-dim F_2-subspace. Boolean vars c_1 (l), c_2 (l). Measures for UNSAT targets z:
  D* = least D with 1 in M_D (Macaulay/closure refutation degree, Boolean ring),
  rankE = dim span of the n descended equations (= dim_F2 span{v_j^2 v_k^2 + v_j v_k z, v_j^2 z^2, a6}),
  dimVV = dim V.V, e = n - l - dimVV (linearization excess, cryptanalysis fit).
Families: R random, P polynomial span <1,t,..,t^{l-1}>, G geometric progression c*P, T random in ker Tr,
  PR = <1,t,..,t^{l-2}, r> (one random vector spoils the progression), GL = GL(l) re-basis of a random V (control).
"""
import sys, random, json, itertools, time
IRRED = {11:(1<<11)|(1<<2)|1, 13:(1<<13)|(1<<4)|(1<<3)|1|(1<<1), 17:(1<<17)|(1<<3)|1, 19:(1<<19)|(1<<5)|(1<<2)|(1<<1)|1, 23:(1<<23)|(1<<5)|1}

class GF:
    def __init__(s,n):
        s.n=n; s.mod=IRRED[n]
    def mul(s,a,b):
        r=0
        while b:
            if b&1: r^=a
            b>>=1; a<<=1
            if a>>s.n: a^=s.mod
        return r
    def sq(s,a): return s.mul(a,a)
    def tr(s,a):
        t=0; x=a
        for _ in range(s.n): t^=x; x=s.sq(x)
        return t&1
    def pw(s,a,e):
        r=1
        while e:
            if e&1: r=s.mul(r,a)
            a=s.mul(a,a); e>>=1
        return r
    def inv(s,a): return s.pw(a,(1<<s.n)-2)

def f2rank(vecs):
    basis={}
    for v in vecs:
        while v:
            h=v.bit_length()-1
            if h in basis: v^=basis[h]
            else: basis[h]=v; break
    return len(basis)

def span_dim(elems): return f2rank(list(elems))

def is_xcoord(F,z,a6):
    # y^2+xy=x^3+a6 has a point with x=z iff Tr(z + a6/z^2)=0 (a2=0)
    return F.tr(z ^ F.mul(a6, F.inv(F.sq(z))))==0

def build(F,V,z,a6):
    """return list of n Boolean quadratics as dict monomial->1; monomials: frozenset of var indices (c1_j=j, c2_k=l+k)"""
    l=len(V); n=F.n
    z2=F.sq(z)
    coeffs={}  # monomial -> field element
    coeffs[frozenset()]=a6
    for j in range(l):
        u=F.mul(F.sq(V[j]),z2)
        coeffs[frozenset([j])]=u
        coeffs[frozenset([l+j])]=u
    for j in range(l):
        for k in range(l):
            w=F.mul(F.sq(V[j]),F.sq(V[k])) ^ F.mul(F.mul(V[j],V[k]),z)
            coeffs[frozenset([j,l+k])]=w
    eqs=[]
    for t in range(n):
        eq=set()
        for m,c in coeffs.items():
            if (c>>t)&1: eq.add(m)
        eqs.append(frozenset(eq))
    return eqs, coeffs

def sat(F,V,z,a6):
    l=len(V)
    pts=[]
    for c in range(1<<l):
        x=0
        for j in range(l):
            if (c>>j)&1: x^=V[j]
        pts.append(x)
    z2=F.sq(z)
    for x1 in pts:
        x1s=F.sq(x1)
        for x2 in pts:
            x2s=F.sq(x2)
            val=F.mul(x1s,x2s)^F.mul(x1s^x2s,z2)^F.mul(F.mul(x1,x2),z)^a6
            if val==0: return True
    return False

def dstar(eqs,N,Dmax=7):
    """least D with 1 in span{m*f : deg m <= D-2} in Boolean ring; returns D or Dmax+1"""
    # monomial indexing by degree
    idx={}
    monos_by_deg={}
    for d in range(Dmax+1):
        monos_by_deg[d]=[frozenset(c) for c in itertools.combinations(range(N),d)]
        for m in monos_by_deg[d]: idx[m]=len(idx)
    one=1<<idx[frozenset()]
    def tovec(poly):
        v=0
        for m in poly: v^=1<<idx[m]
        return v
    basis={}
    def insert(v):
        while v:
            h=v.bit_length()-1
            if h in basis: v^=basis[h]
            else: basis[h]=v; return True
        return False
    def reduces_to_zero(v):
        while v:
            h=v.bit_length()-1
            if h in basis: v^=basis[h]
            else: return False
        return True
    for D in range(2,Dmax+1):
        d=D-2
        for m in monos_by_deg[d]:
            for f in eqs:
                p=set()
                for mm in f:
                    p ^= {mm|m}  # Boolean product: union of variable sets (x^2=x)
                # careful: union may collide for distinct mm -> handled by symmetric difference
                insert(tovec(p))
        if reduces_to_zero(one): return D
    return Dmax+1

def mk_families(F,n,l,rng):
    fams={}
    rand=lambda: rng.randrange(1,1<<n)
    def indep(vs): return f2rank(vs)==len(vs)
    def randV():
        while True:
            V=[rand() for _ in range(l)]
            if indep(V): return V
    def polyV(t):
        return [F.pw(t,j) for j in range(l)]
    def trace0V():
        while True:
            V=[]
            while len(V)<l:
                x=rand()
                if F.tr(x)==0: V.append(x)
            if indep(V): return V
    fams['R']=randV
    fams['P']=lambda: polyV(rand())
    fams['G']=lambda: (lambda c,t:[F.mul(c,F.pw(t,j)) for j in range(l)])(rand(),rand())
    fams['T']=trace0V
    def PR():
        while True:
            t=rand(); V=[F.pw(t,j) for j in range(l-1)]+[rand()]
            if indep(V): return V
    fams['PR']=PR
    return fams

def VV_dim(F,V):
    return span_dim(F.mul(a,b) for a in V for b in V)

def main():
    n=int(sys.argv[1]); l=int(sys.argv[2]); reps=int(sys.argv[3]); ntargets=int(sys.argv[4]); seed=int(sys.argv[5]) if len(sys.argv)>5 else 1
    rng=random.Random(seed)
    F=GF(n); a6=1
    fams=mk_families(F,n,l,rng)
    out=[]
    for fam,gen in fams.items():
        for r in range(reps if fam in('R',) else max(3,reps//4)):
            V=gen()
            dvv=VV_dim(F,V)
            # GL(l) control for the first random V
            found=0; tries=0
            while found<ntargets and tries<2000:
                tries+=1
                z=rng.randrange(1,1<<n)
                if not is_xcoord(F,z,a6): continue
                if sat(F,V,z,a6): continue
                eqs,coeffs=build(F,V,z,a6)
                rankE=span_dim(coeffs.values())
                neq=f2rank([sum(1<<i for i,m in enumerate(sorted(coeffs,key=lambda m:(len(m),sorted(m)))) if m in e) for e in eqs])
                D=dstar(eqs,2*l)
                trV0=int(all(F.tr(v)==0 for v in V)); l0c=F.tr(F.mul(a6,F.inv(F.sq(z))))
                rec=dict(n=n,l=l,fam=fam,rep=r,z=z,dimVV=dvv,e=n-l-dvv,rankW=rankE,rankEq=neq,trV0=trV0,l0const=l0c,Dstar=D)
                if fam=='R' and found==0:
                    # GL(l) control: random invertible re-basis of V
                    while True:
                        M=[[rng.randrange(2) for _ in range(l)] for _ in range(l)]
                        if f2rank([sum(M[i][j]<<j for j in range(l)) for i in range(l)])==l: break
                    V2=[0]*l
                    for i in range(l):
                        for j in range(l):
                            if M[i][j]: V2[i]^=V[j]
                    eqs2,_=build(F,V2,z,a6)
                    rec['GL_control_Dstar']=dstar(eqs2,2*l)
                out.append(rec); found+=1
                print(json.dumps(rec),flush=True)
    return out

if __name__=='__main__':
    main()
