import random,math,numpy as np,json,sys
from scipy import stats
polys={17:(1<<17)|(1<<3)|1,19:(1<<19)|(1<<5)|(1<<2)|(1<<1)|1,23:(1<<23)|(1<<5)|1}
def run(n,seed):
    M=polys[n];rnd=random.Random(seed)
    def mul(a,b):
        r=0
        while b:
            if b&1:r^=a
            b>>=1;a<<=1
            if a>>n:a^=M
        return r
    def pw(a,e):
        r=1
        while e:
            if e&1:r=mul(r,a)
            a=mul(a,a);e>>=1
        return r
    inv=lambda a:pw(a,(1<<n)-2)
    def tr(a):
        t=a;s=a
        for _ in range(n-1):t=mul(t,t);s^=t
        return s
    def solve(c):  # z^2+z=c, n odd: half-trace
        h=c;t=c
        for _ in range((n-1)//2):t=mul(mul(t,t),1);t=mul(t,t);h^=t
        return h
    a,b=1,rnd.randrange(1,1<<n)
    l=n//3
    basis=[rnd.randrange(1,1<<n) for _ in range(l)]
    V=set([0])
    for v in basis: V|={x^v for x in V}
    pts=[]
    for x in V:
        if x==0:continue
        c=x^a ^ mul(b,inv(mul(x,x)))  # y=xz: z^2+z = x+a+b/x^2
        if tr(c)==0:
            z=solve(c);pts.append((x,mul(x,z)));pts.append((x,mul(x,z^1)))
    sums=[]
    for i in range(len(pts)):
        for j in range(i+1,len(pts)):
            (x1,y1),(x2,y2)=pts[i],pts[j]
            if x1==x2:continue
            lam=mul(y1^y2,inv(x1^x2))
            sums.append(mul(lam,lam)^lam^x1^x2^a)
    sums=np.array(sums,dtype=np.int64)
    res=[]
    for k in range(60):
        # random complement: matrix whose kernel is V -> project via random linear map with kernel V
        Wl=n-l
        # build linear map: random n x Wl matrix, then kill V by Gaussian approach: use random functionals vanishing on V
        funcs=[]
        while len(funcs)<Wl:
            f=rnd.randrange(1,1<<n)
            if all(bin(f&v).count('1')%2==0 for v in basis): funcs.append(f)
        w=np.zeros(len(sums),dtype=np.int64)
        for f in funcs:
            w+=np.array([bin(int(s)&f).count('1')&1 for s in sums]) if False else np.vectorize(lambda s:bin(int(s)&f).count('1')&1)(sums)
        # rank check skipped (prob high); compare to Binomial(Wl,1/2)
        cdf=lambda x:stats.binom.cdf(x,Wl,0.5)
        mean_def=(Wl/2-w.mean())/(Wl/2)
        ks=stats.kstest(w,lambda x:stats.binom.cdf(np.floor(x),Wl,0.5)).pvalue
        res.append((mean_def,ks))
    md=[r[0] for r in res];ps=[r[1] for r in res]
    return dict(n=n,l=l,pairs=len(sums),max_abs_rel_mean_deficit=max(map(abs,md)),mean_rel_deficit=float(np.mean(md)),min_ks_p=min(ps),frac_ks_below_0p01=sum(p<0.01 for p in ps)/len(ps),expected_frac_null=0.01)
out=[run(n,s) for n in (17,19,23) for s in (1,2)]
print(json.dumps(out))
