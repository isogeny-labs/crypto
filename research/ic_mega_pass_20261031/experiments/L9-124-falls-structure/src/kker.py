"""Explicit K-relations: K-polys Q = sum_m c_m m (m = a^e b^f K-monomials from window, bit-degree<=3) with Q in W^(r)_low (x) K.
Unknowns: bits of c_m. Constraint: Tr(x^t Q) in W for t=0..n-1. Returns K-basis in reduced echelon form (over K)."""
import sys, itertools, json
from krel import *
def kinv(x,mul,n):
    r=1; e=(1<<n)-2; b=x
    while e:
        if e&1: r=mul(r,b)
        b=mul(b,b); e>>=1
    return r
def solve(n,name,s,pref,win,rd,maxdeg=3):
    k=(n+1)//2; mul,frob,trm=field(n); V,V2=subspaces(n,k,s,name)
    monos,_=load(pref,0); idx={m:i for i,m in enumerate(monos)}
    WB=basis(load(pref,rd)[1]).full()
    M=[];P=[]
    for da in range(maxdeg+1):
        for db in range(maxdeg+1-da):
            for Ia in itertools.combinations(win,da):
                for Ib in itertools.combinations(win,db):
                    M.append((Ia,Ib)); P.append(kmono(Ia,Ib,V,V2,k,mul,frob))
    L=len(monos)
    # image of unknown (mi,u): concat_t red(Tr(x^t * x^u * m))
    imgs=[]
    for mi,p in enumerate(P):
        trs=[]
        for t in range(2*n-1):
            th=1
            # theta = x^t mod modulus
            th=1<<t if t<n else None
            trs.append(t)
        # precompute Tr(x^j m) rows for j=0..2n-2 using field element x^j
        xs=[1]
        for j in range(2*n-2): xs.append(mul(xs[-1],2))
        rows=[]
        for j in range(2*n-1):
            r=0
            for mm,c in p.items():
                if bin(mul(xs[j],c)&trm).count('1')&1: r^=1<<idx[mm]
            rows.append(WB.nf(r))
        for u in range(n):
            v=0
            for t in range(n): v|=rows[t+u]<<(t*L)
            imgs.append(v)
    # kernel of unknowns -> imgs
    res={}; comb={}; ker=[]
    for i,v in enumerate(imgs):
        c=1<<i
        while v:
            lb=v&-v; q=res.get(lb)
            if q is None: res[lb]=v; comb[lb]=c; break
            v^=q; c^=comb[lb]
        if not v: ker.append(c)
    # convert kernel vectors to K-coefficient vectors, then K-RREF
    vecs=[]
    for c in ker:
        cf=[0]*len(M)
        for i in range(len(M)*n):
            if c>>i&1: cf[i//n]^=1<<(i%n)
        vecs.append(cf)
    # K-RREF
    rref=[]; pivs=[]
    for v in vecs:
        v=v[:]
        for (pc,rv) in zip(pivs,rref):
            if v[pc]:
                f=v[pc]; v=[a^mul(f,b) for a,b in zip(v,rv)]
        nz=[i for i,x in enumerate(v) if x]
        if not nz: continue
        pc=nz[0]; iv=kinv(v[pc],mul,n); v=[mul(iv,x) for x in v]
        for j in range(len(rref)):
            if rref[j][pc]:
                f=rref[j][pc]; rref[j]=[a^mul(f,b) for a,b in zip(rref[j],v)]
        rref.append(v); pivs.append(pc)
    return M,len(ker),rref,pivs
def fmt(Ia,Ib):
    e='*'.join(['a%d'%i for i in Ia]+['b%d'%j for j in Ib]); return e or '1'
if __name__=='__main__':
    n,name,s,pref=int(sys.argv[1]),sys.argv[2],int(sys.argv[3]),sys.argv[4]; win=[int(x) for x in sys.argv[5].split(',')]; rd=int(sys.argv[6])
    md=int(sys.argv[7]) if len(sys.argv)>7 else 3
    M,kd,rref,pivs=solve(n,name,s,pref,win,rd,md)
    out=dict(n=n,arm=name,s=s,round=rd,window=win,nmono=len(M),F2_kernel=kd,K_dim=len(rref),
             relations=[{fmt(*M[i]):hex(c) for i,c in enumerate(v) if c} for v in rref])
    print(json.dumps(out))
