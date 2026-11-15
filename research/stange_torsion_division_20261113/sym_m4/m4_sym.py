"""Symmetrized m = 4 decomposition: does a multiplicative-subgroup (binomial) factor base help?
Decompose target xR on E: S4(X1,X2,X3,xR) = 0, X_i in factor base.
Arms: D  dense prod(X-c)=0 per variable, unsymmetrized
      Bn binomial X^n = 1 per variable, unsymmetrized
      Bs binomial, symmetrized: vars e1,e2,e3; S4 symmetrized; e3^n=1, p_n=3, (p_n^2-p_2n)/2=3
      Ds dense, symmetrized: sigma_k(F(X1),F(X2),F(X3)) = 0, k=1..3, written in e1,e2,e3 (only if --ds)
Ground truth by brute force; assert each arm's basis is [1] iff brute force finds no solution
(solutions of D/Bn: ordered triples; Bs: multisets). Time: min of 3 msolve wall times minus baseline.
Exploratory; not a bench.record run. Frozen before data: curve a=5,b=7 on E directly, seed, 8 targets."""
import sys, time, subprocess, random, statistics as st, tempfile, os, json, itertools
from sympy import symbols, resultant, Poly, expand
from sympy.polys.polyfuncs import symmetrize
p=int(sys.argv[1]); n=int(sys.argv[2]); DS='--ds' in sys.argv; SEED=11
ARMS=next((a.split('=',1)[1].split(',') for a in sys.argv if a.startswith('--arms=')),['D','Bn','Bs']+(['Ds'] if DS else []))
assert (p-1)%n==0
A,B=5,7; assert (4*A**3+27*B*B)%p
x1,x2,x3,x4,X,e1,e2,e3=symbols('x1 x2 x3 x4 X e1 e2 e3')
def S3(a,b,c): return (a-b)**2*c**2-2*((a+b)*(a*b+A)+2*B)*c+(a*b-A)**2-4*B*(a+b)
t0=time.time()
S4=Poly(resultant(S3(x1,x2,X),S3(x3,x4,X),X),x1,x2,x3,x4,modulus=p)
print(f"# S4 built ({len(S4.terms())} terms, deg/var {[S4.degree(v) for v in (x1,x2,x3,x4)]}) {time.time()-t0:.1f}s",file=sys.stderr)
issq=lambda z: z%p==0 or pow(z%p,(p-1)//2,p)==1
S4f=lambda a,b,c,d: S4.eval({x1:a,x2:b,x3:c,x4:d})%p
g=next(z for z in range(2,p) if all(pow(z,(p-1)//q,p)!=1 for q in range(2,p) if (p-1)%q==0 and all(q%k for k in range(2,int(q**.5)+1))))
MU=sorted({pow(g,(p-1)//n*k,p) for k in range(n)}); assert len(MU)==n
FD=[x for x in range(p) if issq(x**3+A*x+B)][:n]
def msqrt(a):
    a%=p
    if a==0: return 0
    q,s_=p-1,0
    while q%2==0: q//=2; s_+=1
    z=2
    while pow(z,(p-1)//2,p)!=p-1: z+=1
    M_,c,t_,r=s_,pow(z,q,p),pow(a,q,p),pow(a,(q+1)//2,p)
    while t_!=1:
        i,tt=0,t_
        while tt!=1: tt=tt*tt%p; i+=1
        b=pow(c,1<<(M_-i-1),p); M_,c,t_,r=i,b*b%p,t_*b*b%p,r*b%p
    assert r*r%p==a
    return r
def padd(P,Q):
    if P is None: return Q
    if Q is None: return P
    if P[0]==Q[0] and (P[1]+Q[1])%p==0: return None
    l=(3*P[0]*P[0]+A)*pow(2*P[1],-1,p)%p if P==Q else (Q[1]-P[1])*pow(Q[0]-P[0],-1,p)%p
    x=(l*l-P[0]-Q[0])%p; return (x,(l*(P[0]-x)-P[1])%p)
def pneg(P): return None if P is None else (P[0],(-P[1])%p)
PTS=[(x,msqrt(x**3+A*x+B)) for x in FD]
PART=[FD[i::3] for i in range(3)]   # APS-style disjoint parts (round-robin by x order)
def build_pair_table():
    tab={}
    for i in range(len(PTS)):
        for j in range(i,len(PTS)):
            for S in (padd(PTS[i],PTS[j]),padd(PTS[i],pneg(PTS[j]))):
                tab.setdefault('O' if S is None else S[0],set()).add((PTS[i][0],PTS[j][0]))
    return tab
def mitm_table(xR,tab):
    R_=(xR,msqrt(xR**3+A*xR+B)); out=set()
    for P3 in PTS:
        for T in (padd(R_,pneg(P3)),padd(R_,P3)):
            for (a,b) in tab.get('O' if T is None else T[0],()):
                out.add(tuple(sorted((a,b,P3[0]))))
    return out
def mitm(xR):
    # all multisets {x1,x2,x3} in FD with +-P1 +-P2 +-P3 = +-R, via hashing x(P1 +- P2) against x(R +- P3)
    R_=(xR,msqrt(xR**3+A*xR+B)); want={}
    for k,P3 in enumerate(PTS):
        for T in (padd(R_,pneg(P3)),padd(R_,P3)):
            if T is not None: want.setdefault(T[0],set()).add(P3[0])
            elif True: want.setdefault('O',set()).add(P3[0])
    out=set()
    for i in range(len(PTS)):
        for j in range(i,len(PTS)):
            for S in (padd(PTS[i],PTS[j]),padd(PTS[i],pneg(PTS[j]))):
                key='O' if S is None else S[0]
                for x3 in want.get(key,()):
                    out.add(tuple(sorted((PTS[i][0],PTS[j][0],x3))))
    return out
def ms(P,vs):  # Poly -> msolve string
    out=[]
    for mon,c in P.terms():
        c=int(c)%p
        if c==0: continue
        out.append(str(c)+''.join(f"*{v}^{e}" for v,e in zip(vs,mon) if e))
    return '+'.join(out) if out else '0'
def run(polys,vs):
    with tempfile.TemporaryDirectory() as d:
        fi=os.path.join(d,'i'); fo=os.path.join(d,'o')
        open(fi,'w').write(",".join(vs)+f"\n{p}\n"+",\n".join(polys)+"\n")
        best=None
        for _ in range(3):
            t1=time.perf_counter(); r=subprocess.run(['msolve','-g','2','-t','1','-f',fi,'-o',fo],capture_output=True,text=True); dt=time.perf_counter()-t1
            if r.returncode!=0:
                import shutil; shutil.copy(fi,f'fail_p{p}_n{n}.ms'); raise SystemExit(f"msolve rc={r.returncode}")
            best=dt if best is None else min(best,dt)
        o=open(fo).read()
    body=o[o.rfind('#---')+4:].replace('\n','').replace(' ','')
    return best, body in ('[1]:','[1*1]:') or body.startswith('[1]') or body.startswith('[1*1]')
base=min(run(["u-1","v-2"],["u","v"])[0] for _ in range(5))
# symmetric pieces
def esym(expr):  # symmetric poly in x1,x2,x3 -> poly in e1,e2,e3 (asserted exact)
    expr_s,rem,defs=symmetrize(expand(expr),[x1,x2,x3],formal=True)
    assert rem==0
    rep={d[0]:v for d,v in zip(defs,[e1,e2,e3])}
    return expand(expr_s.subs(rep))
def newton_ps(K):  # power sums p_1..p_K of 3 vars in e's, mod p
    P=[3]; E=[1,e1,e2,e3]
    for k in range(1,K+1):
        s=0
        for i in range(1,min(k,3)+1):
            pk=P[k-i] if k-i>0 else None
            s+= (-1)**(i-1)*E[i]*(P[k-i] if k-i>0 else k)  # p_k = sum (-1)^{i-1} e_i p_{k-i}, with p_0 term = k*e_k
        P.append(Poly(expand(s),e1,e2,e3,modulus=p).as_expr())
    return P
t0=time.time()
PS=newton_ps(2*n)
pn,p2n=PS[n],PS[2*n]
_r=random.Random(5)
for _ in range(3):  # numeric check of Newton sums
    xs=[_r.randrange(p) for _ in range(3)]
    ev={e1:sum(xs)%p,e2:(xs[0]*xs[1]+xs[0]*xs[2]+xs[1]*xs[2])%p,e3:xs[0]*xs[1]*xs[2]%p}
    for k in (n,2*n): assert Poly(PS[k],e1,e2,e3,modulus=p).eval(ev)%p==sum(pow(z,k,p) for z in xs)%p
bs_cons=[Poly(e3**n-1,e1,e2,e3,modulus=p), Poly(pn-3,e1,e2,e3,modulus=p), Poly(expand((pn**2-p2n)*pow(2,-1,p))-3,e1,e2,e3,modulus=p)]
print(f"# Newton sums to {2*n}: {time.time()-t0:.1f}s",file=sys.stderr)
R=random.Random(SEED)
TAB=None; TAB_s=None
if 'Mt' in ARMS:
    t1=time.perf_counter(); TAB=build_pair_table(); TAB_s=time.perf_counter()-t1
def rand_xE():
    while True:
        z=R.randrange(p)
        if issq(z**3+A*z+B): return z
T={k:[] for k in ARMS}; PREL={'Dp':0}; REL={'D':0,'Bn':0,'Bs':0}; MS={'dense':0,'binomial':0}; SOLS={}
if DS:
    FX=lambda v: eval('*'.join(f'({v}-{c})' for c in FD))
for ti in range(8):
    xR=rand_xE()
    S4R=Poly(S4.as_expr().subs(x4,xR),x1,x2,x3,modulus=p)
    S4sym=Poly(esym(S4R.as_expr()),e1,e2,e3,modulus=p)
    # ground truth
    solD=[(a,b,c) for a in FD for b in FD for c in FD if S4f(a,b,c,xR)==0]
    solB=[(a,b,c) for a in MU for b in MU for c in MU if S4f(a,b,c,xR)==0]
    valid=lambda tr: all(issq(z**3+A*z+B) for z in tr)
    REL['D']+=sum(map(valid,solD)); REL['Bn']+=sum(map(valid,solB)); REL['Bs']+=len({tuple(sorted(s)) for s in solB if valid(s)})
    MS['dense']+=len({tuple(sorted(s)) for s in solD if valid(s)}); MS['binomial']+=len({tuple(sorted(s)) for s in solB if valid(s)})
    fD=Poly(expand(eval('*'.join(f'(X-{c})' for c in FD))),X,modulus=p)
    uniD=lambda v: ms(Poly(fD.as_expr().subs(X,symbols(v)),symbols(v),modulus=p),[v])
    arms=[('D',[ms(S4R,['x1','x2','x3']),uniD('x1'),uniD('x2'),uniD('x3')],['x1','x2','x3'],bool(solD)),
          ('Bn',[ms(S4R,['x1','x2','x3'])]+[f"{v}^{n}-1" for v in ('x1','x2','x3')],['x1','x2','x3'],bool(solB)),
          ('Bs',[ms(S4sym,['e1','e2','e3'])]+[ms(c,['e1','e2','e3']) for c in bs_cons],['e1','e2','e3'],bool(solB))]
    Yv=symbols('Y')
    S3a=Poly(expand(S3(x1,x2,Yv)),x1,x2,x3,Yv,modulus=p); S3b=Poly(expand(S3(x3,xR,Yv)),x1,x2,x3,Yv,modulus=p)
    V4=['x1','x2','x3','Y']
    arms+=[('Dy',[ms(S3a,V4),ms(S3b,V4),uniD('x1'),uniD('x2'),uniD('x3')],V4,bool(solD)),
           ('By',[ms(S3a,V4),ms(S3b,V4)]+[f"{v}^{n}-1" for v in ('x1','x2','x3')],V4,bool(solB))]
    if DS:
        sig=[esym(expand(FX('x1')+FX('x2')+FX('x3'))), esym(expand(FX('x1')*FX('x2')+FX('x1')*FX('x3')+FX('x2')*FX('x3'))), esym(expand(FX('x1')*FX('x2')*FX('x3')))]
        arms.append(('Ds',[ms(S4sym,['e1','e2','e3'])]+[ms(Poly(s,e1,e2,e3,modulus=p),['e1','e2','e3']) for s in sig],['e1','e2','e3'],bool(solD)))
    solP=[(a,b,c) for a in PART[0] for b in PART[1] for c in PART[2] if S4f(a,b,c,xR)==0]
    fP=[Poly(expand(eval('*'.join(f'(X-{c})' for c in Pk))),X,modulus=p) for Pk in PART]
    arms.append(('Dp',[ms(S4R,['x1','x2','x3'])]+[ms(Poly(fP[k].as_expr().subs(X,symbols(v)),symbols(v),modulus=p),[v]) for k,v in enumerate(('x1','x2','x3'))],['x1','x2','x3'],bool(solP)))
    PREL['Dp']+=len({tuple(sorted(s_)) for s_ in solP if valid(s_)})
    if 'M' in ARMS:
        t1=time.perf_counter(); mm=min((mitm(xR),time.perf_counter()-t1) for _ in range(1))[0]
        best=None
        for _ in range(3):
            t1=time.perf_counter(); mm=mitm(xR); dt=time.perf_counter()-t1; best=dt if best is None else min(best,dt)
        T['M'].append(best)
        truth={tuple(sorted(s_)) for s_ in solD if valid(s_)}
        assert mm==truth, ('mitm mismatch',xR,len(mm),len(truth))
    if 'Mt' in ARMS:
        best=None
        for _ in range(3):
            t1=time.perf_counter(); mt=mitm_table(xR,TAB); dt=time.perf_counter()-t1; best=dt if best is None else min(best,dt)
        T['Mt'].append(best)
        assert mt=={tuple(sorted(s_)) for s_ in solD if valid(s_)}, ('table mismatch',xR)
    arms=[a for a in arms if a[0] in ARMS]
    R.shuffle(arms)
    for k,polys,vs,has in arms:
        dt,trivial=run(polys,vs); T[k].append(dt-base)
        assert trivial==(not has), (k,ti,xR,trivial,has)
    print(f"# target {ti} xR={xR} done",file=sys.stderr,flush=True)
res={'p':p,'n':n,'baseline_s':base,'median_s':{k:st.median(v) for k,v in T.items()},'min_s':{k:min(v) for k,v in T.items()},
     'max_s':{k:max(v) for k,v in T.items()},'valid_relations_ordered_or_multiset':REL,'valid_relation_multisets':MS,'partitioned_relation_multisets':PREL,'M_timing':'in-process python, min of 3 (not msolve)','Mt_table_build_s':TAB_s,'targets':8,'seed':SEED,'msolve':'0.6.5',
     'S4_terms':len(S4.terms()),'Bs_constraint_terms':[len(c.terms()) for c in bs_cons]}
print(json.dumps(res))
