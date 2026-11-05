import numpy as np, random
from cypari2 import Pari
pari=Pari()
def run(p,k,M,seed):
    random.seed(seed)
    while True:
        a,b=random.randrange(p),random.randrange(p)
        E=pari.ellinit([a,b],p); N=int(pari.ellcard(E))
        if pari.isprime(N): break
    G=pari.ellgenerators(E)[0]
    # x-coordinate of n*G for all n: iterate
    xs=np.zeros(N,dtype=np.int64); P=G
    for n in range(1,N):
        xs[n]=int(pari.lift(P[0])) ; P=pari.elladd(E,P,G)
    xs[0]=-1
    L=p//k  # interval width -> |F| ~ N/k points
    idx=np.arange(1,N)
    def triples(mask):
        F=idx[mask[1:]]; S=np.zeros(N,bool); S[F]=True
        # count unordered a+b+c=0 mod N, approx via a,b in F
        s=(-(F[:,None]+F[None,:]))%N
        return int(S[s].sum())
    out=[];szs=[]
    for _ in range(M):
        u=random.randrange(1,p); r=random.randrange(p)
        y=(u*u*xs+r)%p; m=(xs>=0)&(y<L); f=m.sum(); t=triples(m); out.append(t/(f**3/N)); szs.append(f)
    nul=[]
    for _ in range(M):
        m=np.zeros(N,bool); m[1+np.random.choice(N-1,int(np.mean([1])) or 1,replace=False)]=False
        sz=int(((xs>=0)&(((xs)%p)<L)).sum())
        m[1+np.random.choice(N-1,sz,replace=False)]=True; nul.append(triples(m)/(sz**3/N))
    out=np.array(out);nul=np.array(nul)
    from scipy.stats import ks_2samp
    return dict(p=p,N=N,mean_model=out.mean(),var_model=out.var(),mean_null=nul.mean(),var_null=nul.var(),
      var_ratio=out.var()/nul.var(),ks_p=ks_2samp(out,nul).pvalue,max_model=int(out.max()),max_null=int(nul.max()),
      maxratio_model=out.max()/out.mean(),maxratio_null=nul.max()/nul.mean(),size_cv=np.std(szs)/np.mean(szs),lam=float(np.mean(szs))**3/N)
for p in (4099,16411):
    print(run(p,8,1500,p))
