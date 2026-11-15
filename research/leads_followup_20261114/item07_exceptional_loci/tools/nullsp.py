import numpy as np, itertools
def rref_mod(M,p):
    M=M.copy()%p; r=0; piv=[]
    rows,cols=M.shape
    for c in range(cols):
        if r==rows: break
        nz=np.nonzero(M[r:,c])[0]
        if nz.size==0: continue
        i=r+nz[0]; M[[r,i]]=M[[i,r]]
        M[r]=M[r]*pow(int(M[r,c]),p-2,p)%p
        idx=np.nonzero(M[:,c])[0]
        for j in idx:
            if j!=r: M[j]=(M[j]-M[j,c]*M[r])%p
        piv.append(c); r+=1
    return M[:r],piv
def nullspace(M,p):
    R,piv=rref_mod(M,p); cols=M.shape[1]; free=[c for c in range(cols) if c not in piv]
    N=[]
    for f in free:
        v=np.zeros(cols,dtype=np.int64); v[f]=1
        for r,c in enumerate(piv): v[c]=(-R[r,f])%p
        N.append(v)
    return np.array(N,dtype=np.int64).reshape(len(N),cols)
def monos(nvars,deg):
    return [e for e in itertools.product(range(deg+1),repeat=nvars) if sum(e)<=deg]
def evalmat(pts,mons,p):
    P=np.array(pts,dtype=np.int64)
    cols=[]
    for e in mons:
        v=np.ones(len(P),dtype=np.int64)
        for i,k in enumerate(e):
            for _ in range(k): v=v*P[:,i]%p
        cols.append(v)
    return np.stack(cols,1)
