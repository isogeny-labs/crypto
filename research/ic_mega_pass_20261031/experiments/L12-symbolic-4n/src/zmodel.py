"""L12 generic Frobenius model Z: K-valued functions a^e b^f (e,f nonneg ints over a shifted position range),
product = exponent addition (a_i a_i = a_{i+1}), bit-degree = popcount(e)+popcount(f).
W2 = M4 ∩ P<=3,  W3 = (M4 + Var*W2) ∩ P<=3,  M4 = span{ mono(wt<=2) * F_j }.  Linear algebra over K by ./kech."""
import os, sys, subprocess, itertools, json
HERE=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0,os.path.join(HERE,'..','..','MP-comm_alg-1'))
from tensor_closure import pmulmod, modulus
KECH=os.path.join(HERE,'kech')
wt=lambda e,f: bin(e).count('1')+bin(f).count('1')
class Field:
    def __init__(s,n,mod=None): s.n=n; s.m=mod or modulus(n)
    def mul(s,x,y): return pmulmod(x,y,s.m,s.n)
    def sq(s,x): return s.mul(x,x)
    def frob(s,x,i):
        for _ in range(i%s.n if s.n else i): x=s.sq(x)
        return x
    def inv(s,x):
        r=1
        for _ in range(s.n-1): x=s.sq(x); r=s.mul(r,x)
        return r
def Fj(K,z,c,j,lo):
    """F_j as dict {(e,f):coef}; position p -> bit p-lo. Coeffs z_j=z^(2^j) (j may be negative: use n-periodicity)."""
    zj=K.frob(z,j); zj1=K.sq(zj); cj=K.frob(c,j)
    A=lambda p: 1<<(p-lo)
    P={}
    def add(k,v):
        P[k]=P.get(k,0)^v
        if not P[k]: del P[k]
    add((A(j+1),A(j+1)),1); add((A(j),A(j)),zj); add((A(j+1),0),zj1); add((0,A(j+1)),zj1); add((0,0),cj)
    return P
def pmono(P,e,f):
    Q={}
    for (e1,f1),v in P.items():
        k=(e1+e,f1+f); Q[k]=Q.get(k,0)^v
        if not Q[k]: del Q[k]
    return Q
def vars_(lo,hi): return [(1<<(p-lo),0) for p in range(lo,hi)]+[(0,1<<(p-lo)) for p in range(lo,hi)]
def inwin(key,lo,win):
    e,f=key; mask=0
    for p in win: mask|=1<<(p-lo)
    return (e & ~mask)==0 and (f & ~mask)==0
def kech(K,rows,colorder,split):
    idx={k:i for i,k in enumerate(colorder)}
    lines=["%d %d %d %d %d"%(K.n,K.m,len(colorder),split,len(rows))]
    for P in rows: lines.append(" ".join([str(len(P))]+["%d %d"%(idx[k],v) for k,v in P.items()]))
    r=subprocess.run([KECH],input="\n".join(lines)+"\n",capture_output=True,text=True)
    if r.returncode: raise RuntimeError(r.stderr+str(r.returncode))
    t=r.stdout.split("\n"); rank=int(t[0]); nout=int(t[1]); out=[]
    for i in range(nout):
        a=list(map(int,t[2+i].split())); out.append({colorder[a[1+2*j]]:a[2+2*j] for j in range(a[0])})
    return rank,out
def order(keys,lo,win):
    keys=set(keys)
    hi4=sorted(k for k in keys if wt(*k)>=4)
    lowo=sorted(k for k in keys if wt(*k)<=3 and not inwin(k,lo,win))
    loww=sorted(k for k in keys if wt(*k)<=3 and inwin(k,lo,win))
    return hi4+lowo+loww, len(hi4), len(hi4)+len(lowo)
def closure(K,z,c,lo,hi,win,mult_hi=None,extra_j=None):
    V=vars_(lo,hi)
    mults=[(0,0)]+V+[(a[0]+b[0],a[1]+b[1]) for a,b in itertools.combinations(V,2)]
    js=range(lo,hi-1) if extra_j is None else extra_j
    gens=[pmono(Fj(K,z,c,j,lo),*mu) for j in js for mu in mults]
    gens=[g for g in gens if g]
    keys=set(k for g in gens for k in g)
    col,s_low,s_win=order(keys,lo,win)
    rk2,W2=kech(K,gens,col,s_low)           # W2 = M4 ∩ P<=3 (RREF)
    W2win=[P for P in W2 if all(inwin(k,lo,win) for k in P)]
    prods=[pmono(P,*v) for P in W2 for v in V]
    rows=gens+prods
    keys=set(k for g in rows for k in g)
    col,s_low,s_win=order(keys,lo,win)
    rk3,W3w=kech(K,rows,col,s_win)
    _,W2w=kech(K,gens,*order(set(k for g in gens for k in g),lo,win)[::2])
    return dict(rankM4=rk2,dimW2=len(W2),rankS3=rk3,dimW2win=len(W2w),dimW3win=len(W3w),ngens=len(gens),nprods=len(prods)),W2w,W3w,W2
