"""Field-operation-counted version of ../ic_e2e/ic_mt.py (copied from sha256 087c0d32...ba8, then extended).

Counts F_p multiplications (M), squarings (S) and inversions (I) in every arm.
Exploratory; not a bench.record run.

Usage: ic_pm.py p seed n1,n2,... --curve=A,B,r [--slack=1] [--reps-ic=R] [--reps-rho=R] [--W=1,16,64]

IC arms (large-prime IC, m = 3, hash-table decomposition, the algorithm of ic_mt.py --lp):
  base     : one target walk; every leftover R -+ F_k is a full affine addition (1 I each) = ic_mt.py as written.
  pm       : one target walk; x(R-F_k) and x(R+F_k) share one inversion of (x_F - x_R); x-only leftovers.
  bb<W>    : W parallel target walks, Montgomery batch inversion per round, but NO +- sharing: R-F_k and R+F_k
             each take a batch slot and are full affine points (the batching-only control for pmb<W>).
  pmb<W>   : W parallel target walks; +- sharing, and ONE Montgomery batch inversion per round over all
             W*(n+1) denominators (n leftovers pairs + the walk step of every walk).
Rho arms (rho_store_neg of ic_mt.py: r-adding walk, every visited point stored by x, matches +-X):
  rho<W>,m : W parallel walks with a shared store, one Montgomery batch inversion per round (W=1: unbatched);
             m > 0 also stores x(X -+ S_k), k < m, with the same shared-inversion +- formula as the IC (--m=0,1,4).
Starting points: walk 0 (and the IC table, multipliers) are set up with uncounted scalar multiplications,
as in ic_mt.py. Walks 1..W-1 start at X0 + SM_j with W-1 precomputed (uncounted) random combinations SM_j;
those W-1 additions are COUNTED and share one batch inversion. Same scheme in IC and rho.
"""
import sys, random, json, math, time
p=int(sys.argv[1]); SEED=int(sys.argv[2]); NS=[int(v) for v in sys.argv[3].split(',')]
arg=lambda k,d: next((a.split('=',1)[1] for a in sys.argv if a.startswith('--'+k+'=')),d)
SLACK=int(arg('slack','1')); REPS_IC=int(arg('reps-ic','10')); REPS_RHO=int(arg('reps-rho','20'))
WS=[int(v) for v in arg('W','1,16,64').split(',')]; MS=[int(v) for v in arg('m','0,1,4').split(',')]
A,B,r=[int(v) for v in arg('curve','').split(',')]
rng=random.Random(SEED)
def legendre(a): a%=p; return 0 if a==0 else (1 if pow(a,(p-1)//2,p)==1 else -1)
def isprime(n): return n>1 and all(n%k for k in range(2,int(n**.5)+1))
assert (4*A**3+27*B*B)%p and isprime(r) and abs(p+1-r)<=2*math.isqrt(p)+2
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
# ---------- uncounted reference arithmetic (setup / verification only) ----------
def add(P,Q):
    if P is None: return Q
    if Q is None: return P
    if P[0]==Q[0] and (P[1]+Q[1])%p==0: return None
    l=(3*P[0]*P[0]+A)*pow(2*P[1],-1,p)%p if P==Q else (Q[1]-P[1])*pow(Q[0]-P[0],-1,p)%p
    x=(l*l-P[0]-Q[0])%p; return (x,(l*(P[0]-x)-P[1])%p)
neg=lambda P: None if P is None else (P[0],(-P[1])%p)
def mul(k,P):
    R_=None
    while k:
        if k&1: R_=add(R_,P)
        P=add(P,P); k>>=1
    return R_
def rand_point():
    while True:
        x=rng.randrange(p); v=(x**3+A*x+B)%p
        if legendre(v)==1: return (x,msqrt(v))
P=rand_point(); assert mul(r,P) is None   # r prime in Hasse interval and r*P = O  =>  #E = r
w=rng.randrange(1,r); Q=mul(w,P)
# ---------- counted field arithmetic ----------
C={'M':0,'S':0,'I':0}
def fm(a,b): C['M']+=1; return a*b%p
def fs(a): C['S']+=1; return a*a%p
def fi(a): C['I']+=1; return pow(a,-1,p)
def batch_inv(vals):
    """Montgomery's trick: 1 I + 3(n-1) M for n nonzero values."""
    n=len(vals)
    if n==0: return []
    pre=[vals[0]]
    for v in vals[1:]: pre.append(fm(pre[-1],v))
    t=fi(pre[-1]); out=[0]*n
    for i in range(n-1,0,-1): out[i]=fm(t,pre[i-1]); t=fm(t,vals[i])
    out[0]=t; return out
def addc(P,Q):
    """counted affine addition: add 1I+2M+1S, doubling 1I+2M+2S (constant multiples are additions)."""
    if P is None: return Q
    if Q is None: return P
    if P[0]==Q[0]:
        if (P[1]+Q[1])%p==0: return None
        l=fm((3*fs(P[0])+A)%p, fi(2*P[1]%p))
    else: l=fm((Q[1]-P[1])%p, fi((Q[0]-P[0])%p))
    x=(fs(l)-P[0]-Q[0])%p; return (x,(fm(l,(P[0]-x)%p)-P[1])%p)
def add_inv(P,Q,inv):
    """P+Q given inv = 1/(xQ-xP): 2M+1S."""
    l=fm((Q[1]-P[1])%p,inv); x=(fs(l)-P[0]-Q[0])%p; return (x,(fm(l,(P[0]-x)%p)-P[1])%p)
def pm_x(R,F,inv):
    """x(R-F), x(R+F) from one shared inv = 1/(xF-xR): 2M+2S. Returns lambdas too (y costs 1M more on demand)."""
    lm=fm((-F[1]-R[1])%p,inv); lp_=fm((F[1]-R[1])%p,inv)
    s=(R[0]+F[0])%p
    return (fs(lm)-s)%p,(fs(lp_)-s)%p,lm,lp_
def snap(): return dict(C)
def diff(a,b): return {k:b[k]-a[k] for k in C}
def starts(W, a0, b0, X0, SM, scoef):
    """walk 0 from an uncounted start; walk j = X0 + SM[j-1], the W-1 additions done with ONE batch inversion (counted).
    SM are W-1 precomputed random combinations aP+bQ (uncounted setup, like the walk multipliers)."""
    out=[(X0,a0,b0)]
    if W==1: return out
    den=[(SM[j][0]-X0[0])%p for j in range(W-1)]
    nz=[j for j in range(W-1) if den[j]]; iv=dict(zip(nz,batch_inv([den[j] for j in nz])))
    for j in range(W-1):
        Y=add_inv(X0,SM[j],iv[j]) if j in iv else addc(X0,SM[j])
        if Y is None: continue
        out.append((Y,(a0+scoef[j][0])%r,(b0+scoef[j][1])%r))
    return out
def randcomb(K):
    """K random combinations aP+bQ, resampled while equal to O (uncounted setup)."""
    co=[];pts=[]
    while len(co)<K:
        a,b=rng.randrange(r),rng.randrange(r); X=add(mul(a,P),mul(b,Q))
        if X is not None: co.append((a,b)); pts.append(X)
    return co,pts
# ---------- rho_store_neg with W parallel walks, shared x-store, batched inversion ----------
def rho_par(W, m=0):
    """rho_store_neg, W parallel walks, shared x-store, one batch inversion per round.
    m > 0 gives rho the same +- tool as the IC: for every walk point X also store x(X - S_k) and x(X + S_k),
    k < m, for m fixed random combinations S_k (uncounted setup), sharing X's batch slot per k (x-only, 2M+2S
    per pair, y computed on demand at a hit for 1M). Side points are dead ends: they are stored, not walked."""
    c0=snap(); K=20; coef,M=randcomb(K); scoef,SM=randcomb(max(1,W-1))
    sco,SS=randcomb(m) if m else ([],[])
    (a0,b0),X0=[v[0] for v in randcomb(1)]
    walks=[list(t) for t in starts(W,a0,b0,X0,SM,scoef)]; Wreq=W; W=len(walks)   # a start equal to O is dropped
    seen={}; cnt={'pts':0,'side':0,'restarts':0,'rounds':0}
    def res(): return {'W':Wreq,'m':m,'walks':W,'ops':diff(c0,C),'points':cnt['pts'],'side_points':cnt['side'],
                       'restarts':cnt['restarts'],'rounds':cnt['rounds'],'log_ok':True}
    def restart(X,a,b):
        """new walk start X + T, T a fresh random combination (uncounted setup), the addition counted."""
        while True:
            (ta,tb),T=[v[0] for v in randcomb(1)]; Y=addc(X,T)
            if Y is not None: return [Y,(a+ta)%r,(b+tb)%r]
    def lookup(key,a,b,full):
        """returns ('sol',w) | ('degenerate',None) | ('new',None). full(): the full point (may cost 1M)."""
        if key not in seen: return 'new',None
        a1,b1,f1=seen[key]; X1=f1(); X=full()
        if X1==X: da,db=(a1-a)%r,(b-b1)%r
        else:
            assert X1==neg(X); da,db=(-a1-a)%r,(b+b1)%r
        if db:
            sol=da*pow(db,-1,r)%r; assert sol==w,'rho wrong'; return 'sol',sol
        return 'degenerate',None
    def check(j):
        X,a,b=walks[j]
        st_,_=lookup(X[0],a,b,lambda X=X:X)
        if st_=='sol': return True
        if st_=='degenerate':   # same representation (walk merged into its own trail): restart, counted
            cnt['restarts']+=1; walks[j]=restart(X,a,b); return check(j)
        seen[X[0]]=(a,b,lambda X=X:X); cnt['pts']+=1; return False
    def side(X,a,b,k,inv):
        xm,xp,lm,lp_=pm_x(X,SS[k],inv)
        for xT,l,sg in ((xm,lm,-1),(xp,lp_,1)):
            ya=lambda xT=xT,l=l:(xT,(fm(l,(X[0]-xT)%p)-X[1])%p)   # y on demand, 1M
            aa,bb=(a+sg*sco[k][0])%r,(b+sg*sco[k][1])%r
            st_,_=lookup(xT,aa,bb,ya)
            if st_=='sol': return True
            if st_=='new': seen[xT]=(aa,bb,ya); cnt['side']+=1
        return False
    for j in range(W):
        if check(j): return res()
    while True:
        cnt['rounds']+=1; idx=[hash(X[0]^X[1])%K for X,_,_ in walks]; den=[]
        for j in range(W):
            X=walks[j][0]; den.append((M[idx[j]][0]-X[0])%p); den.extend((S[0]-X[0])%p for S in SS)
        ok=[t for t in range(len(den)) if den[t]]; inv=dict(zip(ok,batch_inv([den[t] for t in ok])))
        for j in range(W):
            X,a,b=walks[j]; i=idx[j]; base=j*(1+m)
            for k in range(m):
                if base+1+k in inv and side(X,a,b,k,inv[base+1+k]): return res()
            Y=add_inv(X,M[i],inv[base]) if base in inv else addc(X,M[i])
            if Y is None:   # walk hit O (probability ~1/r): restart from a fresh counted start
                cnt['restarts']+=1; walks[j]=restart(X0,a0,b0)
                if check(j): return res()
                continue
            walks[j]=[Y,(a+coef[i][0])%r,(b+coef[i][1])%r]
            if check(j): return res()
# ---------- large-prime index calculus, four field-op modes (base, pm, bb<W>, pmb<W>) ----------
def la_solve(rows,n):
    m=len(rows); Mx=[[c%r for c in e]+[1 if t==k else 0 for t in range(m)] for k,(e,_,_) in enumerate(rows)]
    piv=0
    for col in range(n):
        pr=next((i for i in range(piv,m) if Mx[i][col]),None)
        if pr is None: continue
        Mx[piv],Mx[pr]=Mx[pr],Mx[piv]; iv=pow(Mx[piv][col],-1,r)
        Mx[piv]=[v*iv%r for v in Mx[piv]]
        for i in range(m):
            if i!=piv and Mx[i][col]:
                f=Mx[i][col]; Mx[i]=[(vi-f*vp)%r for vi,vp in zip(Mx[i],Mx[piv])]
        piv+=1
    for i in range(piv,m):
        lam=Mx[i][n:]
        sa=sum(l*rows[k][1] for k,l in enumerate(lam))%r; sb=sum(l*rows[k][2] for k,l in enumerate(lam))%r
        if sb: return (-sa*pow(sb,-1,r))%r
    return None
def ic(n, mode, W=1):
    c0=snap()
    FB=[]; x=1
    while len(FB)<n:
        v=(x**3+A*x+B)%p
        if legendre(v)==1: FB.append((x,msqrt(v)))
        x+=1
    tab={}
    for i in range(n):
        for j in range(i,n):
            for sj in (1,-1):
                S=addc(FB[i],FB[j] if sj==1 else neg(FB[j]))   # counted (n^2 table, full adds)
                tab.setdefault('O' if S is None else S[0],[]).append((i,j,sj,S))
    c_table=snap()
    K=20; coef,M=randcomb(K); scoef,SM=randcomb(max(1,W-1))
    (a0,b0),X0=[v[0] for v in randcomb(1)]
    walks=[list(t) for t in starts(W,a0,b0,X0,SM,scoef)] if mode in ('pmb','bb') else [[X0,a0,b0]]
    Wreq=W; W=len(walks)
    st={'rows':[],'need':n+SLACK,'partial':{},'nlp':0,'targets':0}
    def table_hit(key,T,kk,s3):
        for (i,j,sj,S) in tab.get(key,()):
            sg=1 if S==T else -1
            e=[0]*n; e[i]+=sg; e[j]+=sg*sj; e[kk]+=s3; return e
        return None
    def partial_hit(key,T_full,R,a,b,kk,s3):
        """T_full: callable returning the full point T (y computed on demand, counted)."""
        rows=st['rows']; hit=st['partial'].get(key)
        if hit is None:
            st['partial'][key]=(st['targets'],R,a,b,kk,s3); return
        if hit[0]==st['targets']: return
        t0,R0,a1,b1,k0,sg0=hit
        T=T_full(); L0=addc(R0, neg(FB[k0]) if sg0==1 else FB[k0])   # recompute old leftover (rare, counted)
        e=[0]*n
        if L0==T: e[kk]+=s3; e[k0]-=sg0; ra,rb=(a-a1)%r,(b-b1)%r
        else:
            assert L0==neg(T); e[kk]+=s3; e[k0]+=sg0; ra,rb=(a+a1)%r,(b+b1)%r
        rows.append((e,ra,rb)); st['nlp']+=1
        st['partial'][key]=(st['targets'],R,a,b,kk,s3)
    def process_target(R,a,b,invs):
        """invs: None (base / pm computes its own inversions) or list of n shared inverses 1/(xF_k - xR)."""
        st['targets']+=1; rows=st['rows']
        for kk in range(n):
            F=FB[kk]
            if R[0]==F[0]:   # R = +-F_k
                e=[0]*n; e[kk]+=1 if R==F else -1; rows.append((e,a,b)); return
            if mode in ('base','bb'):
                for s3 in (1,-1):
                    G=neg(F) if s3==1 else F
                    T=addc(R,G) if mode=='base' else add_inv(R,G,invs[kk][0 if s3==1 else 1])   # T = R - s3 F, full point
                    e=table_hit(T[0],T,kk,s3)
                    if e: rows.append((e,a,b)); return
                    partial_hit(T[0],lambda T=T:T,R,a,b,kk,s3)
                    if len(rows)>=st['need']: return
            else:
                inv=fi((F[0]-R[0])%p) if invs is None else invs[kk]
                xm,xp,lm,lp_=pm_x(R,F,inv)
                for s3,xT,l in ((1,xm,lm),(-1,xp,lp_)):
                    def full(xT=xT,l=l): return (xT,(fm(l,(R[0]-xT)%p)-R[1])%p)   # y on demand: 1M
                    if xT in tab:
                        e=table_hit(xT,full(),kk,s3)
                        if e: rows.append((e,a,b)); return
                    partial_hit(xT,full,R,a,b,kk,s3)
                    if len(rows)>=st['need']: return
    retries=0
    while True:
        while len(st['rows'])<st['need']:
            if mode in ('base','pm'):
                X,a,b=walks[0]; k=rng.randrange(K); Y=addc(X,M[k])
                walks[0]=[Y,(a+coef[k][0])%r,(b+coef[k][1])%r]
                if Y is None: walks[0]=[X0,a0,b0]; continue
                process_target(Y,walks[0][1],walks[0][2],None)
            else:
                # one round: batch all W*(n+1) denominators; walk j's new point R_j' = R_j + M_k is the target
                # processed next round, so its leftovers share the round's batch with its own walk step.
                ks=[rng.randrange(K) for _ in range(W)]; den=[]; dup=2 if mode=='bb' else 1
                for j,(X,_,_) in enumerate(walks):
                    den.append((M[ks[j]][0]-X[0])%p)
                    den.extend((F[0]-X[0])%p for F in FB for _ in range(dup))   # bb: R-F and R+F each get their own slot
                nz=[i for i,d in enumerate(den) if d]; iv=dict(zip(nz,batch_inv([den[i] for i in nz])))
                for j in range(W):
                    X,a,b=walks[j]; base=j*(dup*n+1)
                    if len(st['rows'])<st['need']:
                        if all((base+1+kk) in iv for kk in range(dup*n)):
                            if dup==1: process_target(X,a,b,[iv[base+1+kk] for kk in range(n)])
                            else: process_target(X,a,b,[(iv[base+1+2*kk],iv[base+2+2*kk]) for kk in range(n)])
                        else:   # some x_R == x_F: R = +-F_k, relation directly
                            for kk,F in enumerate(FB):
                                if X[0]==F[0]:
                                    st['targets']+=1; e=[0]*n; e[kk]+=1 if X==F else -1; st['rows'].append((e,a,b)); break
                    Y=add_inv(X,M[ks[j]],iv[base]) if base in iv else addc(X,M[ks[j]])
                    if Y is None: walks[j]=[X0,a0,b0]; continue
                    walks[j]=[Y,(a+coef[ks[j]][0])%r,(b+coef[ks[j]][1])%r]
        sol=la_solve(st['rows'],n)
        if sol is not None:
            assert sol==w,('ic wrong',n,mode,sol,w)
            tot=diff(c0,C); tb=diff(c0,c_table)
            return {'n':n,'mode':mode,'W':Wreq,'walks':W,'ops':tot,'table_ops':tb,'targets':st['targets'],'relations':len(st['rows']),
                    'lp_relations':st['nlp'],'la_retries':retries,'log_ok':True}
        st['need']+=max(1,SLACK); retries+=1
        assert retries<=50,'ic gave up'
# ---------- self-test of the counted formulas against the reference arithmetic (counters reset after) ----------
for _ in range(200):
    R0,F0=rand_point(),rand_point()
    if R0[0]==F0[0]: continue
    xm,xp,_,_=pm_x(R0,F0,pow((F0[0]-R0[0])%p,-1,p))
    assert xm==add(R0,neg(F0))[0] and xp==add(R0,F0)[0]
    assert addc(R0,F0)==add(R0,F0) and addc(R0,R0)==add(R0,R0)
vs=[rng.randrange(1,p) for _ in range(50)]; assert all(v*iv%p==1 for v,iv in zip(vs,batch_inv(vs)))
c=snap(); batch_inv(vs); d=diff(c,C); assert d=={'M':3*49,'S':0,'I':1},d
for k in C: C[k]=0
out={'p':p,'r':r,'seed':SEED,'curve':[A,B],'slack':SLACK,'reps_ic':REPS_IC,'reps_rho':REPS_RHO,'rho':[],'ic':[]}
t0=time.time()
for W in WS:
    for m in MS:
        for _ in range(REPS_RHO): out['rho'].append(rho_par(W,m))
for n in NS:
    for _ in range(REPS_IC):
        s=rng.randrange(1<<30)
        for mode,W in [('base',1),('pm',1)]+[(md,W) for W in WS for md in ('bb','pmb')]:
            rng.seed(s)   # same seed per rep: base / pm walk the same targets
            out['ic'].append(ic(n,mode,W))
out['seconds']=round(time.time()-t0,1)
print(json.dumps(out))
