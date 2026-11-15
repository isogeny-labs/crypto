import numpy as np, cypari2, json, time, sys, platform
pari=cypari2.Pari(); t0=time.time()
def inv(a,p):
    r=np.ones_like(a); b=a.copy(); e=p-2
    while e:
        if e&1: r=r*b%p
        b=b*b%p; e>>=1
    return r
def add1(x1,y1,x2,y2,a,p):  # scalar
    if x1 is None: return x2,y2
    if x1==x2:
        if (y1+y2)%p==0: return None,None
        l=(3*x1*x1+a)*pow(2*y1,-1,p)%p
    else: l=(y2-y1)*pow(x2-x1,-1,p)%p
    x3=(l*l-x1-x2)%p; return x3,(l*(x1-x3)-y1)%p
def curve(p,seed):
    rng=np.random.default_rng(seed)
    while True:
        a=int(rng.integers(1,p));b=int(rng.integers(1,p))
        if (4*a**3+27*b*b)%p==0: continue
        E=pari.ellinit([a,b],p); N=int(pari.ellcard(E))
        if pari.isprime(N):
            G=pari.ellgenerators(E)[0]; return a,b,N,int(pari.lift(G[0])),int(pari.lift(G[1]))
def enum(p,a,N,gx,gy):
    m=int(np.ceil(np.sqrt(N)))+1; bx=[None];by=[None]; x,y=None,None
    for j in range(1,m):
        x,y=add1(x,y,gx,gy,a,p); bx.append(x);by.append(y)
    mx,my=add1(bx[-1],by[-1],gx,gy,a,p)  # mG
    BX=np.array(bx[1:],dtype=np.int64);BY=np.array(by[1:],dtype=np.int64)
    X=np.zeros(N,dtype=np.int64); cx,cy=None,None
    for i in range(0,N//m+1):
        base=i*m
        # k=base+0
        if base<N: X[base]=0 if cx is None else cx
        js=np.arange(1,m); ks=base+js; ok=ks<N
        if cx is None: xs=BX
        else:
            d=(BX-cx)%p; bad=d==0; d[bad]=1
            l=(BY-cy)%p*inv(d,p)%p; xs=(l*l-cx-BX)%p
            if bad.any():
                for t in np.nonzero(bad)[0]:
                    r=add1(cx,cy,int(BX[t]),int(BY[t]),a,p)[0]; xs[t]=0 if r is None else r
        X[ks[ok]]=xs[ok]
        cx,cy=add1(cx,cy,mx,my,a,p)
    return X
def fval(c,X,p):
    r=np.zeros_like(X)
    for ci in c: r=(r*ci+0)%p; r=r  # Horner below
    r=np.zeros_like(X)
    for ci in c: r=(r*X+ci)%p
    return r
MU=1/256
def lam(F):
    mu=F.mean(); S=np.abs(np.fft.rfft(F-mu)); S[0]=0
    return float(S.max()/np.sqrt(len(F)*mu)), int(S.argmax())
def mask(c,X,p): return (fval(c,X,p)<p//256).astype(np.float64)
def climb(X,p,d,rng,budget):
    best=None;bl=-1
    c=[int(v) for v in rng.integers(1,p,d+1)]
    for it in range(budget):
        if it%8==0: cand=[int(v) for v in rng.integers(1,p,d+1)]; 
        else:
            cand=list(best); i=int(rng.integers(0,d+1)); cand[i]=int(rng.integers(1,p))
        l=lam(mask(cand,X,p))[0]
        if l>bl: bl,best=l,cand
    return best,bl
out={'per_p':[], 'versions':{'numpy':np.__version__,'pari':str(pari.version()),'python':platform.python_version()}}
for k in [14,16,18,20,22]:
    p=int(pari.nextprime(2**k)); seed=20261031+k*100
    a,b,N,gx,gy=curve(p,seed); X=enum(p,a,N,gx,gy)
    a2,b2,N2,g2x,g2y=curve(p,seed+1); X2=enum(p,a2,N2,g2x,g2y)
    rng=np.random.default_rng(seed); Xp=rng.permutation(X)
    bud=40 if k<=18 else 16
    rec={'k':k,'p':p,'N':N,'curve':[a,b],'N2':N2,'rows':[]}
    n2=[lam((rng.random(N)<MU).astype(float))[0] for _ in range(16)]
    rec['null2_max']=max(n2); rec['null2_mean']=float(np.mean(n2))
    for d in range(1,7):
        r2=np.random.default_rng(seed+d)
        c,lr=climb(X,p,d,r2,bud); ln=climb(Xp,p,d,r2,bud)[1]
        lh=lam(mask(c,X2,p))[0]; F=mask(c,X,p)
        rec['rows'].append({'d':d,'f':c,'hits':int(F.sum()),'lam_real_climbed':lr,'lam_null1_climbed':ln,'lam_heldout':lh,
            'ratio_real_null2':lr/rec['null2_max'],'ratio_heldout_null2':lh/rec['null2_max']})
    # d=1 identity control
    rec['lam_identity_x']=lam(mask([1,0],X,p))[0]
    # planted control
    F=mask([3,5,7],X,p); j0=int(np.random.default_rng(seed).integers(1,N//2))
    pos=np.cos(2*np.pi*j0*np.arange(N)/N)>0; mem=np.nonzero(F)[0]; nf=int(0.05*len(mem))
    r3=np.random.default_rng(seed+99); drop=r3.choice(mem[~pos[mem]],min(nf,(~pos[mem]).sum()),replace=False)
    Fp=F.copy(); Fp[drop]=0; addc=r3.choice(np.nonzero(pos&(F==0))[0],len(drop),replace=False); Fp[addc]=1
    lp,jp=lam(Fp); rec['planted']={'lam':lp,'found_j':jp,'j0':j0,'ratio_null2':lp/rec['null2_max']}
    if k==20:
        th=[]
        for theta in [0.25,0.35,0.45,0.55]:
            s=int(round(p**theta)); r4=np.random.default_rng(seed+int(theta*100))
            for arm,XX in [('real',X),('null1',Xp)]:
                c=rec['rows'][1]['f']  # climbed d=2 filter
                M=(fval(c,XX,p)<p//256)
                for kind in ['random','interval','adv_top']:
                    if kind=='random': A=r4.integers(0,N,s);B=r4.integers(0,N,s)
                    elif kind=='interval': A=np.arange(s)+int(r4.integers(0,N));B=np.arange(s)+int(r4.integers(0,N))
                    else:
                        B=r4.integers(0,N,s); pool=r4.integers(0,N,4*s)
                        if 4*s*s>4e7: pool=pool[:int(4e7//s)]
                        rows=M[(pool[:,None]+B[None,:])%N].sum(1); A=pool[np.argsort(-rows)[:s]]
                    if s*s>4e7: A=A[:int(4e7//s)]
                    h=int(M[(A[:,None]+B[None,:])%N].sum()); n=len(A)*len(B); e=n*M.mean()
                    th.append({'theta':theta,'size':s,'arm':arm,'kind':kind,'pairs':n,'hits':h,'mass_ratio':h/e,'z':(h-e)/np.sqrt(e)})
        rec['theta_sweep']=th
    rec['secs']=time.time()-t0; out['per_p'].append(rec); print(json.dumps({k2:v for k2,v in rec.items() if k2!='theta_sweep'}),flush=True)
out['runtime_s']=time.time()-t0
json.dump(out,open(sys.argv[1],'w'),indent=1,default=float)
