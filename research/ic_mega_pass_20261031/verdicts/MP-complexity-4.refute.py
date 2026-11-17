import numpy as np, json, sys
from sympy import nextprime
def run(nb,k,a=3,b=7,seed=1):
    p=nextprime(2**nb-2**(nb-3)); n=nb
    # points with x<2^k
    sq={}
    for y in range(p): sq.setdefault(y*y%p,[]).append(y)
    base=[]
    for x in range(1,2**k):
        r=(x**3+a*x+b)%p
        for y in sq.get(r,[]): base.append((x,y))
    def add(P,Q):
        (x1,y1),(x2,y2)=P,Q
        if x1==x2:
            if (y1+y2)%p==0: return None
            l=(3*x1*x1+a)*pow(2*y1,-1,p)%p
        else: l=(y2-y1)*pow(x2-x1,-1,p)%p
        x3=(l*l-x1-x2)%p; return (x3,(l*(x1-x3)-y1)%p)
    dec=np.zeros(2**n,np.int8)
    for i in range(len(base)):
        for j in range(i,len(base)):
            R=add(base[i],base[j])
            if R: dec[R[0]]=1
    curve=np.zeros(2**n,np.int8)
    for x in range(p):
        if (x**3+a*x+b)%p in sq: curve[x]=1
    def wht(f):
        F=(1-2*f.astype(np.float64)).copy(); h=1
        while h<len(F):
            F=F.reshape(-1,2,h); F=np.stack([F[:,0]+F[:,1],F[:,0]-F[:,1]],1).reshape(-1); h*=2
        return F/len(F)
    def anfdeg(f):
        g=f.astype(np.uint8).copy(); h=1
        while h<len(g):
            g=g.reshape(-1,2,h); g[:,1]^=g[:,0]; g=g.reshape(-1); h*=2
        idx=np.nonzero(g)[0]; return int(max(bin(i).count('1') for i in idx))
    wt=np.array([bin(i).count('1') for i in range(2**n)])
    def lowW(f,t):
        F=wht(f); F[0]=0; return float((F[wt<=t]**2).sum())
    rng=np.random.default_rng(seed)
    cidx=np.nonzero(curve)[0]; dens=dec.sum()/len(cidx)
    out={'p':int(p),'n':n,'k':k,'B':len(base),'dec_count':int(dec.sum()),'curve_x':len(cidx),'density_on_curve':float(dens),'anf_deg_dec':anfdeg(dec)}
    for t in (1,2,3,4):
        obs=lowW(dec,t); rs=[]
        for _ in range(50):
            g=np.zeros(2**n,np.int8); g[rng.choice(cidx,int(dec.sum()),replace=False)]=1; rs.append(lowW(g,t))
        rs=np.array(rs); out[f'W<={t}']=obs; out[f'z<={t}']=float((obs-rs.mean())/rs.std())
    out['anf_deg_random_eqdens']=anfdeg(g)
    return out
res=[run(16,7),run(18,8)]
print(json.dumps(res))
