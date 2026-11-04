import sys, json, numpy as np
sys.path.insert(0,'.')
from gen import draw
def run(d, cmax):
    p,t,co=d['p'],d['t'],d['coeffs']; K=1<<t
    mons=[(a,b) for a in range(K) for b in range(K)]
    deg={m:bin(m[0]).count('1')+bin(m[1]).count('1') for m in mons}
    out=[]
    for c in range(2,cmax+1):
        cols=sorted([m for m in mons if deg[m]<=c], key=lambda m:(-deg[m],m)); idx={m:i for i,m in enumerate(cols)}; n=len(cols)
        f=np.zeros(n,dtype=np.int64)
        for e1 in range(3):
            for e2 in range(3):
                if co[e1][e2]%p: f[idx[(e1%K,e2%K)]]=(f[idx[(e1%K,e2%K)]]+co[e1][e2])%p
        # shift permutations for the 2t variables
        perms=[]
        for i in (0,1):
            for j in range(t):
                sh=1<<j; pm=np.full(n,-1)
                for m,k in idx.items():
                    mm=list(m); mm[i]=(mm[i]+sh)&(K-1); mm=tuple(mm)
                    pm[k]=idx.get(mm,-1)
                perms.append(pm)
        def echelon(M):
            M=M.copy()%p; r=0; pivs=[]
            for col in range(n):
                if r>=M.shape[0]: break
                nz=np.nonzero(M[r:,col])[0]
                if len(nz)==0: continue
                k=r+nz[0]; M[[r,k]]=M[[k,r]]
                M[r]=(M[r]*pow(int(M[r,col]),-1,p))%p
                others=np.nonzero(M[:,col])[0]
                for o in others:
                    if o!=r: M[o]=(M[o]-M[o,col]*M[r])%p
                pivs.append(col); r+=1
            return M[:r],pivs
        V=f[None,:]
        while True:
            E,pivs=echelon(V)
            low=[i for i,pc in enumerate(pivs) if deg[cols[pc]]<=c-1]
            new=[E]
            for i in low:
                for pm in perms:
                    v=np.zeros(n,dtype=np.int64); ok=pm>=0
                    v[pm[ok]]=E[i][ok]
                    assert (E[i][~ok]==0).all()
                    new.append(v[None,:])
            V2,piv2=echelon(np.vstack(new))
            if V2.shape[0]==E.shape[0]: break
            V=V2
        E,pivs=echelon(V)
        unit=any(deg[cols[pc]]==0 for pc in pivs)
        lowdim=sum(1 for pc in pivs if deg[cols[pc]]<=c-1)
        out.append((c,n,E.shape[0],lowdim,unit))
        if unit: break
    return out
if __name__=='__main__':
    kind,t,s0,n=sys.argv[1],int(sys.argv[2]),int(sys.argv[3]),int(sys.argv[4]); cmax=int(sys.argv[5])
    for s in range(s0,s0+n):
        d=draw(kind,2013265921,t,s)
        print(kind,t,s,run(d,cmax),flush=True)
