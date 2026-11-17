import itertools, json, sys, time
import numpy as np
from scipy.sparse import csr_matrix
from scipy.sparse.csgraph import maximum_flow, breadth_first_order

def pmul(a,b):
    r={}
    for ea,ca in a.items():
        for eb,cb in b.items():
            e=tuple(x+y for x,y in zip(ea,eb)); r[e]=r.get(e,0)+ca*cb
    return {e:c for e,c in r.items() if c}
def ppow(a,k,n):
    r={(0,)*n:1}
    for _ in range(k): r=pmul(r,a)
    return r
def S3(ia,ib,ic,n,A=7,B=11,zc=None):
    # (a-b)^2c^2 - 2((a+b)(ab+A)+2B)c + (ab-A)^2 ; c may be constant zc
    def mono(**d):
        e=[0]*n
        for k,v in d.items(): e[k]+=v
        return tuple(e)
    import sympy as sp
    a,b,c=sp.symbols('a b c')
    expr=sp.expand((a-b)**2*c**2-2*((a+b)*(a*b+A)+2*B)*c+(a*b-A)**2)
    if zc is not None: expr=sp.expand(expr.subs(c,zc))
    P=sp.Poly(expr,a,b,c); r={}
    for (ea,eb,ec),co in P.terms():
        e=[0]*n; e[ia]+=ea; e[ib]+=eb
        if ic is not None: e[ic]+=ec
        e=tuple(e); r[e]=r.get(e,0)+int(co)
    return r

def solve(polys, bounds, box, l, order, mult_s=None, bigvars=(), tol=1e-5, allowed=None):
    """polys: list of dicts; bounds: function delta->list of log_p bounds per var; box: max deg per var;
    order: weight vector (lex tiebreak). mult_s: max total degree of multiplier in bigvars."""
    n=len(box)
    key=lambda e:(sum(w*x for w,x in zip(order,e)),e)
    LM=[max(p,key=key) for p in polys]
    monos=list(itertools.product(*[range(b+1) for b in box])); idx={m:i for i,m in enumerate(monos)}
    ks=[kk for kk in itertools.product(range(l+1),repeat=len(polys)) if sum(kk)<=l and (allowed is None or all(kk[i]==0 for i in range(len(polys)) if i not in allowed))]
    cache={}
    def G(kk):
        if kk not in cache:
            r={(0,)*n:1}
            for p,k in zip(polys,kk): r=pmul(r,ppow(p,k,n))
            cache[kk]=r
        return cache[kk]
    K=np.zeros(len(monos)); rows=[];cols=[]; nbig=np.zeros(len(monos),int)
    for i,m in enumerate(monos):
        best=None
        for kk in sorted(ks,key=lambda t:-sum(t)):
            if best is not None and sum(kk)<sum(best[0]): break
            lm=tuple(sum(k*L[j] for k,L in zip(kk,LM)) for j in range(n))
            mul=tuple(a-b for a,b in zip(m,lm))
            if min(mul)<0: continue
            if mult_s is not None and sum(mul[j] for j in bigvars)>mult_s: continue
            deps=[tuple(a+b for a,b in zip(mul,e)) for e in G(kk)]
            if all(all(d[j]<=box[j] for j in range(n)) for d in deps):
                best=(kk,mul,deps); break
        if best is None: continue
        K[i]=sum(best[0]); nbig[i]=sum(best[1][j] for j in bigvars)
        for d in best[2]:
            if d!=m: rows.append(i); cols.append(idx[d])
    E=np.array(monos)
    N=len(monos); S=N; T=N+1
    def maxclosure(delta):
        b=np.array(bounds(delta)); w=K-E@b
        W=np.round(w*1e6).astype(np.int64)
        INF=np.int64(1)<<50
        r=list(rows)+[S]*int((W>0).sum())+list(np.nonzero(W<0)[0])
        c=list(cols)+list(np.nonzero(W>0)[0])+[T]*int((W<0).sum())
        cap=[INF]*len(rows)+list(W[W>0])+list(-W[W<0])
        M=csr_matrix((np.array(cap,dtype=np.int32 if False else np.int64),(r,c)),shape=(N+2,N+2))
        # scipy requires int32 capacities
        M=csr_matrix((np.minimum(np.array(cap),2**30).astype(np.int32),(r,c)),shape=(N+2,N+2))
        f=maximum_flow(M,S,T)
        val=W[W>0].sum()-f.flow_value
        R=M-f.flow; R.data[R.data<0]=0; R.eliminate_zeros()
        R=csr_matrix(R)
        reach=breadth_first_order(R,S,directed=True,return_predecessors=False)
        sel=np.array([x for x in reach if x<N],int)
        return val,sel
    lo,hi=0.0,1.0
    while hi-lo>tol:
        mid=(lo+hi)/2; v,_=maxclosure(mid)
        if v>0: lo=mid
        else: hi=mid
    v,sel=maxclosure(lo)
    return dict(delta=lo, dim=int(len(sel)), maxbig=int(nbig[sel].max()) if len(sel) else 0,
                n_big_ge2=int((nbig[sel]>=2).sum()) if len(sel) else 0, LM=[list(x) for x in LM], nmonos=N)
