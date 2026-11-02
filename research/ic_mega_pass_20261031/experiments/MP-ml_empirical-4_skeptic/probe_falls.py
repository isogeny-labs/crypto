"""Skeptic probe for MP-ml_empirical-4: exact F_2 Macaulay spaces M_c of the binary
Weil-descended S_3 on V (m=2), fall dimensions Fall_c = dim(M_c cap deg<c) - dim(M_{c-1} cap deg<c),
and D* = min c with 1 in M_c. Random F_2-subspace V, a2=1,a6=1, xR real abscissa (UNSAT draws).
Also computes the trace-coefficient count N_c = dim_F2{ g in M_c^K(S) : top coefficients in K_0 }
via the identity M_c(F) = Tr(M_c^K(S)) (checked by rank equality)."""
import sys, random, time, json, itertools
sys.path.insert(0,'/Volumes/SSD990-2/crypto-autoresearcher/research/verification')
import ffd_semaev_core as C
from math import comb
C.MODS.update({11:0x805,13:0x201b,15:0x8003,17:0x20009,19:0x80027,21:0x200005})
def gpow(a,e,n):
    r=1
    while e:
        if e&1: r=C.gmul(r,a,n)
        a=C.gmul(a,a,n); e>>=1
    return r
def tr(a,n):
    t=0; x=a
    for _ in range(n): t^=x; x=C.gmul(x,x,n)
    return t
def real_xR(n,a2,a6,rng):
    # x is an abscissa iff Tr((x^3+a2 x^2+a6)/x^2)=0 (Artin-Schreier)
    while True:
        x=rng.randrange(1,1<<n)
        inv=gpow(x,(1<<n)-2,n); inv2=C.gmul(inv,inv,n)
        rhs=C.gmul(C.gmul(x,x,n),x,n)^C.gmul(a2,C.gmul(x,x,n),n)^a6
        if tr(C.gmul(rhs,inv2,n),n)==0: return x

def popcount(x): return bin(x).count('1')

def monos_upto(N, D):
    ms=[m for m in range(1<<N) if popcount(m)<=D]
    ms.sort(key=lambda m:(popcount(m),m)); return ms

def rank_rows(rows):
    piv={}
    r=0
    for row in rows:
        cur=row
        while cur:
            h=cur.bit_length()-1
            if h in piv: cur^=piv[h]
            else: piv[h]=cur; r+=1; break
    return r, piv

def macaulay(eqs, N, c, idx, degs):
    """rows of M_c with columns ordered so that degree-c monomials occupy the HIGH bits
    (so eliminating high bits first leaves the low-degree subspace readable)."""
    rows=[]
    for f in eqs:
        for m in range(1<<N):
            if popcount(m)>c-2: continue
            prod=0
            for mm in f:
                prod ^= 1<<idx[mm|m]
            rows.append(prod)
    return rows

def analyse(n,l,seed,cmax):
    rng=random.Random(seed)
    xR=real_xR(n,1,1,rng)
    # random F_2-subspace basis of dim l
    while True:
        basis=[rng.randrange(1,1<<n) for _ in range(l)]
        # independence
        r,_=rank_rows(basis)
        if r==l: break
    eqs=C.build_system(n,l,1,xR,basis)
    N=2*l
    out=dict(n=n,l=l,N=N,neqs=len(eqs),seed=seed,falls={},dimM={},lowdim={},Dstar=None,secs={})
    for c in range(2,cmax+1):
        t=time.time()
        ms=[m for m in range(1<<N) if popcount(m)<=c]
        # order: low degree -> low bits, degree-c -> high bits
        ms.sort(key=lambda m:(popcount(m),m))
        idx={m:i for i,m in enumerate(ms)}
        degs=[popcount(m) for m in ms]
        nlow=sum(1 for d in degs if d<c)
        rows=macaulay(eqs,N,c,idx,degs)
        rk,piv=rank_rows(rows)
        # low part: pivots with leading bit < nlow are elements of degree < c (fully reduced wrt high bits)
        low=sum(1 for h in piv if h<nlow)
        out['dimM'][c]=rk; out['lowdim'][c]=low
        one_in = (0 in piv) and piv[0]==1  # constant 1 as a pivot row with only bit 0... need reduce
        # test 1 in span: reduce vector 1
        cur=1
        while cur:
            h=cur.bit_length()-1
            if h in piv: cur^=piv[h]
            else: break
        if cur==0 and out['Dstar'] is None: out['Dstar']=c
        prev = out['dimM'].get(c-1,0)
        out['falls'][c]= low-prev
        out['secs'][c]=round(time.time()-t,2)
        if cur==0: break
    return out

if __name__=='__main__':
    res=[]
    for n in [11,13,15,17,19,21]:
        for l,tag in [(-(-n//3),'l=ceil(n/3)'),(-(-n//2),'diag')]:
            if tag=='diag' and n>17: cmax=4
            else: cmax=5
            if tag=='diag' and n==21: continue
            r=analyse(n,l,1000+n,cmax); r['tag']=tag
            print(json.dumps(r),flush=True); res.append(r)
    json.dump(res,open(sys.argv[1],'w'),indent=1)
