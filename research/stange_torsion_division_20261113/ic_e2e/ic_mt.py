"""End-to-end toy index calculus (m = 3 summands, hash-table decomposition) vs Pollard rho,
both counted in elliptic-curve point additions (group ops) on the same prime-order curve.
Exploratory; not a bench.record run. Usage: ic_mt.py p seed n1,n2,...  (n = factor-base size)"""
import sys, random, json, math, time
p=int(sys.argv[1]); SEED=int(sys.argv[2]); NS=[int(v) for v in sys.argv[3].split(',')]; LP='--lp' in sys.argv
SLACK=next((int(a.split('=')[1]) for a in sys.argv if a.startswith('--slack=')),5)
rng=random.Random(SEED)
def legendre(a): a%=p; return 0 if a==0 else (1 if pow(a,(p-1)//2,p)==1 else -1)
def isprime(n): return n>1 and all(n%k for k in range(2,int(n**.5)+1))
# find a curve y^2=x^3+Ax+B with prime order r (brute point count)
while True:
    A,B=rng.randrange(1,p),rng.randrange(1,p)
    if (4*A**3+27*B*B)%p==0: continue
    r=p+1+sum(legendre(x*x*x+A*x+B) for x in range(p))
    if isprime(r): break
def msqrt(a):
    a%=p
    if a==0: return 0
    q,s=p-1,0
    while q%2==0: q//=2; s+=1
    z=2
    while legendre(z)!=-1: z+=1
    M,c,t,R_=s,pow(z,q,p),pow(a,q,p),pow(a,(q+1)//2,p)
    while t!=1:
        i,tt=0,t
        while tt!=1: tt=tt*tt%p; i+=1
        b=pow(c,1<<(M-i-1),p); M,c,t,R_=i,b*b%p,t*b*b%p,R_*b%p
    return R_
OPS=[0]
def add(P,Q):
    OPS[0]+=1
    if P is None: return Q
    if Q is None: return P
    if P[0]==Q[0] and (P[1]+Q[1])%p==0: return None
    l=(3*P[0]*P[0]+A)*pow(2*P[1],-1,p)%p if P==Q else (Q[1]-P[1])*pow(Q[0]-P[0],-1,p)%p
    x=(l*l-P[0]-Q[0])%p; return (x,(l*(P[0]-x)-P[1])%p)
neg=lambda P: None if P is None else (P[0],(-P[1])%p)
def mul(k,P):  # not counted as ops in the walk budget; used only for setup/verification
    sv=OPS[0]; R_=None
    while k:
        if k&1: R_=add(R_,P)
        P=add(P,P); k>>=1
    OPS[0]=sv; return R_
def rand_point():
    while True:
        x=rng.randrange(p); v=(x**3+A*x+B)%p
        if legendre(v)==1: return (x,msqrt(v))
P=rand_point(); w=rng.randrange(1,r); Q=mul(w,P)
# ---------- Pollard rho (r-adding walk, 20 multipliers, Floyd-free distinguished-point-free: Brent) ----------
def rho():
    OPS[0]=0
    K=20; coef=[(rng.randrange(r),rng.randrange(r)) for _ in range(K)]
    M=[add(mul(a,P),mul(b,Q)) for a,b in coef]; OPS[0]=0
    a0,b0=rng.randrange(r),rng.randrange(r); X=add(mul(a0,P),mul(b0,Q)); OPS[0]=0
    def step(X,a,b):
        i=hash(X)%K if X else 0
        return add(X,M[i]),(a+coef[i][0])%r,(b+coef[i][1])%r
    # Brent cycle finding
    power=lam=1; T,ta,tb=X,a0,b0; H,ha,hb=step(X,a0,b0)
    while T!=H:
        if power==lam: T,ta,tb=H,ha,hb; power*=2; lam=0
        H,ha,hb=step(H,ha,hb); lam+=1
    if (hb-tb)%r==0: return None,OPS[0]
    return (ta-ha)*pow(hb-tb,-1,r)%r, OPS[0]
def rho_store(negmap):
    # r-adding walk storing every visited point; first repeat (or x-repeat with negation matching) solves the DLP
    OPS[0]=0
    K=20; coef=[(rng.randrange(r),rng.randrange(r)) for _ in range(K)]
    M=[add(mul(a,P),mul(b,Q)) for a,b in coef]; OPS[0]=0
    a,b=rng.randrange(r),rng.randrange(r); X=add(mul(a,P),mul(b,Q)); OPS[0]=0
    seen={}
    while True:
        key=X[0] if negmap else X
        if key in seen:
            a0,b0,X0=seen[key]
            if X0==X: da,db=(a0-a)%r,(b-b0)%r          # a0P+b0Q = aP+bQ
            else:     da,db=(-a0-a)%r,(b+b0)%r         # a0P+b0Q = -(aP+bQ)
            if db: return da*pow(db,-1,r)%r, OPS[0]
            return None, OPS[0]
        seen[key]=(a,b,X)
        i=hash(X)%K; X=add(X,M[i]); a=(a+coef[i][0])%r; b=(b+coef[i][1])%r
# ---------- index calculus with hash-table 3-decomposition ----------
def ic(n, lp=False):
    OPS[0]=0
    FB=[]; x=1
    while len(FB)<n:
        v=(x**3+A*x+B)%p
        if legendre(v)==1: FB.append((x,msqrt(v)))
        x+=1
    idx={F[0]:i for i,F in enumerate(FB)}
    tab={}
    for i in range(n):
        for j in range(i,n):
            for sj in (1,-1):
                S=add(FB[i],FB[j] if sj==1 else neg(FB[j]))
                tab.setdefault('O' if S is None else S[0],[]).append((i,j,sj))
    table_ops=OPS[0]
    K=20; coef=[(rng.randrange(r),rng.randrange(r)) for _ in range(K)]
    sv=OPS[0]; M=[add(mul(a,P),mul(b,Q)) for a,b in coef]; OPS[0]=sv
    a,b=rng.randrange(r),rng.randrange(r); sv=OPS[0]; R_=add(mul(a,P),mul(b,Q)); OPS[0]=sv
    rows=[]; targets=0; need=n+SLACK; partial={}; nlp=0; retries=0
    while True:
        while len(rows)<need:
            k=rng.randrange(K); R_=add(R_,M[k]); a=(a+coef[k][0])%r; b=(b+coef[k][1])%r; targets+=1
            if R_ is None: continue
            found=None
            for kk in range(n):
                for s3 in (1,-1):
                    T=add(R_, neg(FB[kk]) if s3==1 else FB[kk])   # T = R - s3*F_k
                    key='O' if T is None else T[0]
                    if T is None:   # R = s3*F_k exactly
                        e=[0]*n; e[kk]+=s3; found=e; break
                    if lp:
                        hit=partial.get(key)
                        if hit is not None and hit[0]!=targets:
                            _,L0,a0,b0,k0,sg0=hit
                            e=[0]*n
                            if L0==T:   # R - s3 F_kk = R0 - sg0 F_k0
                                e[kk]+=s3; e[k0]-=sg0; ra,rb=(a-a0)%r,(b-b0)%r
                            else:       # R - s3 F_kk = -(R0 - sg0 F_k0)
                                e[kk]+=s3; e[k0]+=sg0; ra,rb=(a+a0)%r,(b+b0)%r
                            rows.append((e,ra,rb)); nlp+=1
                            partial[key]=(targets,T,a,b,kk,s3)
                            if len(rows)>=need: break
                        elif hit is None:
                            partial[key]=(targets,T,a,b,kk,s3)
                    for (i,j,sj) in tab.get(key,()):
                        # x(F_i + sj F_j) = x(T): so F_i + sj F_j = +-T ; resolve sign (no extra ops counted: one compare)
                        S=add(FB[i],FB[j] if sj==1 else neg(FB[j]))
                        sg=1 if S==T else -1
                        e=[0]*n; e[i]+=sg; e[j]+=sg*sj; e[kk]+=s3
                        found=e; break
                    if found: break
                if found or len(rows)>=need: break
            if found and len(rows)<need: rows.append((found,a,b))
        walk_ops=OPS[0]-table_ops
        # linear algebra mod r (not counted in group ops; reported separately as row ops)
        m=len(rows); Mx=[[c%r for c in e]+[1 if t==k else 0 for t in range(m)] for k,(e,_,_) in enumerate(rows)]
        piv=0
        for col in range(n):
            pr=next((i for i in range(piv,m) if Mx[i][col]),None)
            if pr is None: continue
            Mx[piv],Mx[pr]=Mx[pr],Mx[piv]; inv=pow(Mx[piv][col],-1,r)
            Mx[piv]=[v*inv%r for v in Mx[piv]]
            for i in range(m):
                if i!=piv and Mx[i][col]:
                    f=Mx[i][col]; Mx[i]=[(vi-f*vp)%r for vi,vp in zip(Mx[i],Mx[piv])]
            piv+=1
        for i in range(piv,m):
            lam=Mx[i][n:]
            sa=sum(l*rows[k][1] for k,l in enumerate(lam))%r; sb=sum(l*rows[k][2] for k,l in enumerate(lam))%r
            if sb: return (-sa*pow(sb,-1,r))%r, table_ops, walk_ops, targets, len(rows), nlp, retries
        need+=max(1,SLACK); retries+=1   # no usable dependency yet: collect more relations (ops keep counting)
        if retries>50: return None, table_ops, OPS[0]-table_ops, targets, len(rows), nlp, retries
out={'p':p,'r':r,'seed':SEED,'curve':[A,B]}
rr=[]
for _ in range(5):
    wr,ops=rho(); assert wr is None or wr==w, 'rho wrong'; rr.append(ops)
out['rho_ops_mean']=sum(rr)/len(rr); out['rho_ops_runs']=rr; out['sqrt_r']=math.isqrt(r)
for nm,ng in (('rho_store',False),('rho_store_neg',True)):
    v=[]
    for _ in range(5):
        wr,ops=rho_store(ng); assert wr is None or wr==w, nm+' wrong'; v.append(ops)
    out[nm+'_ops_mean']=sum(v)/len(v); out[nm+'_ops_runs']=v
out['slack']=SLACK
out['ic']=[]
for n in NS:
    t0=time.time(); wi,to,wo,tg,nr,nl,rt=ic(n, LP)
    assert wi==w, ('ic wrong', n, wi, w)
    out['ic'].append({'n':n,'large_prime':LP,'table_ops':to,'walk_ops':wo,'total_ops':to+wo,'targets':tg,'relations':nr,'lp_relations':nl,'la_retries':rt,'seconds':round(time.time()-t0,2)})
print(json.dumps(out))
