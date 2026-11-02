from tensor_closure import *
import statistics as st
def rand_inv(dim,rng):
    while True:
        M=[rng.getrandbits(dim) for _ in range(dim)]
        if f2rank(M)==dim: return M
def apply(M,x):  # M: list of column images; x bitvector
    r=0;i=0
    while x:
        if x&1: r^=M[i]
        x>>=1;i+=1
    return r
def arms(n,k,rng,which):
    m=modulus(n)
    V=rand_subspace(n,k,rng); z=rng.randrange(1,1<<n); a6=rng.randrange(1,1<<n)
    bil,affa,affb,c=semaev_parts(n,m,V,z,a6)
    L=lambda x: pmulmod(x,x,m,n)^pmulmod(z,x,m,n)
    out={}
    out['A']=to_eqs(n,k,bil,affa,affb,c)
    if 'Ap' in which:
        L1=rand_inv(k,rng);L2=rand_inv(k,rng);L3=rand_inv(n,rng)
        # new vars a=L1 a', b=L2 b'  -> tensor T'[j'][l'] = sum_{j in L1 col j'} sum_{l in L2 col l'} T[j][l]; then mix eqs by L3
        bp=[[0]*k for _ in range(k)]
        for jp in range(k):
            for lp in range(k):
                s=0
                for j in range(k):
                    if (L1[jp]>>j)&1:
                        for l in range(k):
                            if (L2[lp]>>l)&1: s^=bil[j][l]
                bp[jp][lp]=apply(L3,s)
        aa=[apply(L3,sum_x(affa,L1[jp])) for jp in range(k)]
        ab=[apply(L3,sum_x(affb,L2[lp])) for lp in range(k)]
        out['Ap']=to_eqs(n,k,bp,aa,ab,apply(L3,c))
    if 'B' in which:
        out['B']=to_eqs(n,k,[[rng.getrandbits(n) for _ in range(k)] for _ in range(k)],affa,affb,c)
    if 'C1' in which:
        W=rand_subspace(n,k,rng); out['C1']=to_eqs(n,k,mult_tensor(n,m,W,W,L),affa,affb,c)
    if 'C2' in which:
        W1=rand_subspace(n,k,rng);W2=rand_subspace(n,k,rng);Lr=rand_inv(n,rng)
        out['C2']=to_eqs(n,k,mult_tensor(n,m,W1,W2,lambda x:apply(Lr,x)),affa,affb,c)
    for R,tag in ((n,'D1'),(2*n,'D2'),(4*n,'D4')):
        if tag in which:
            T=[[0]*k for _ in range(k)]
            for _ in range(R):
                lam=rng.getrandbits(k);mu=rng.getrandbits(k);nu=rng.getrandbits(n)
                for j in range(k):
                    if (lam>>j)&1:
                        for l in range(k):
                            if (mu>>l)&1: T[j][l]^=nu
            out[tag]=to_eqs(n,k,T,affa,affb,c)
    return out
def sum_x(vec,mask):
    s=0;j=0
    while mask:
        if mask&1: s^=vec[j]
        mask>>=1;j+=1
    return s
ARMS=['A','Ap','B','C1','C2','D1','D2','D4']
if __name__=='__main__':
    import sys
    n=int(sys.argv[1]);k=int(sys.argv[2]);d=int(sys.argv[3]);draws=int(sys.argv[4])
    res={a:[] for a in ARMS}
    t0=time.process_time()
    for s in range(draws):
        rng=random.Random(1000*n+100*k+s)
        sy=arms(n,k,rng,ARMS)
        for a in ARMS:
            r=closure(sy[a],2*k,d); res[a].append([r['dim'],r['low'],int(r['one'])])
    print(json.dumps(dict(n=n,k=k,d=d,draws=draws,cpu=round(time.process_time()-t0,2),res=res)))
