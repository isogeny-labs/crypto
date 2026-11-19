import sys, time, itertools
# EXP16a (v2): multiplicity structure of reduced char-2 systems (t_i = s_i^2), by line type.
# Fixes v1 (exp16a_doubled.sage): polynomial comparisons across rings with different variable names were always False.
# Q1: if the Krylov minpoly of s4 (one random vector) has degree n = dim A, then A = F_Q[s4]/charpoly exactly
#     (cyclic), so each factor f^e of charpoly is ONE local component F_Q[X]/(f^e): e>1 <=> non-reduced point.
# Q2: for each doubled factor (and a few simple ones as control) the point is computed over an extension L,
#     the 4 embedding-conjugate summation equations are solved on the curve: which sign patterns +-P1..+-P4 = +-R,
#     and which partial pair sums are T = (0, sqrt b) or O.
# usage: sage exp16a_doubled2.sage d types ncurves ntargets seed classify(0/1) [ncontrol]
d=int(sys.argv[1]); TYPES=sys.argv[2].split(','); NC=int(sys.argv[3]); NT=int(sys.argv[4]); SEED=int(sys.argv[5]); CLS=sys.argv[6]=='1'
NCTL=int(sys.argv[7]) if len(sys.argv)>7 else 2
sys.argv=['x',str(d),'1','1',str(SEED),'0','0','0']
src=open('exp14_char2_solve.sage').read(); src=src[:src.index('print("d Q curve')]
exec(preparse(src))
set_random_seed(SEED)
CPU=time.process_time
W=TermOrder('wdegrevlex',(2,2,2,1))
Rw2=PolynomialRing(Fq,'t1,t2,t3,s4',order=W); G4=Rw2.gens()
Px=PolynomialRing(Fq,'X')
def reduce(e,RR):
    g=RR.gens(); return sum(c*g[0]^(m.exponents()[0][0]//2)*g[1]^(m.exponents()[0][1]//2)*g[2]^(m.exponents()[0][2]//2)*g[3]^m.exponents()[0][3] for c,m in zip(e.coefficients(),e.monomials()))
def mults(p):
    mu={}
    for f,e in p.factor(): mu[e]=mu.get(e,0)+f.degree()
    return dict(sorted(mu.items()))

def classify_point(tag,f,g,red,u,b,Rt):
    k=f.degree(); ee=lcm(4,k)
    # first pass over L0 to find the splitting degree of the w-quartic
    def setup(bits):
        L=GF(2^bits,'l'); tL=K.modulus().change_ring(L).roots()[0][0]
        phK=lambda c,j=0: c.polynomial()(tL)^(Q^j)
        phQ=lambda c: phK(emb(c))
        PL=PolynomialRing(L,'Y')
        al=PL([phQ(c) for c in f.list()]).roots()[0][0]
        tt=[PL([phQ(c) for c in gi.list()])(al) for gi in g]
        ss=[x.sqrt() for x in tt]+[al]
        quart=PL([ss[3],ss[2],ss[1],ss[0],1])
        return L,tL,phK,phQ,PL,al,tt,ss,quart
    L,tL,phK,phQ,PL,al,tt,ss,quart=setup(d*ee)
    qd=lcm([ff.degree() for ff,_ in quart.factor()])
    L,tL,phK,phQ,PL,al,tt,ss,quart=setup(d*ee*qd*2)
    ws=[r for r,mm in quart.roots() for _ in range(mm)]
    pt=tt+[al]
    # Jacobian of the 4 descended reduced equations; left kernel -> embedding support
    def ev(p):
        return sum(phQ(c)*prod(pt[v]^ex for v,ex in enumerate(mm.exponents()[0])) for c,mm in zip(p.coefficients(),p.monomials())) if p!=0 else L(0)
    J=matrix(L,[[ev(r_.derivative(G4[i])) for i in range(4)] for r_ in red])
    Th=matrix(L,[[tL^(l*Q^j) for l in range(4)] for j in range(4)])
    lk=J.left_kernel().basis()
    musupp=[[j for j in range(4) if (vector(L,lam)*Th.inverse())[j]!=0] for lam in lk]
    wq=[w^Q==w for w in ws]
    info=f"split {len(ws)} wFQ {sum(wq)} wrep {len(ws)-len(set(ws))} w0 {sum(1 for w in ws if w==0)}"
    emb_rep=[]
    for j in range(4):
        uj=phK(u,j); bj=phK(b,j); sbj=bj.sqrt(); xr=phK(Rt[0],j)
        Ej=EllipticCurve(L,[1,0,0,0,bj]); Tj=Ej(0,sbj)
        xs=[uj*w for w in ws]
        if len(xs)<4 or any(x==0 for x in xs) or not all(Ej.is_x_coord(x) for x in xs+[xr]):
            emb_rep.append(f"j{j}:nolift"); continue
        P=[Ej.lift_x(x) for x in xs]; Rj=Ej.lift_x(xr)
        hits=[]
        for sg in itertools.product([1,-1],repeat=3):
            sp=[P[0]]+[sg[i]*P[i+1] for i in range(3)]
            S_=sum(sp)
            if S_==Rj or S_==-Rj:
                fl=[]
                for a_,c_ in itertools.combinations(range(4),2):
                    pp=sp[a_]+sp[c_]
                    if pp==Tj: fl.append(f"P{a_}+P{c_}=T")
                    if pp.is_zero(): fl.append(f"P{a_}+P{c_}=O")
                    if sp[a_]-sp[c_]==Tj: fl.append(f"P{a_}-P{c_}=T")
                for a_ in range(4):
                    if sp[a_]==Tj: fl.append(f"P{a_}=T")
                    if 2*sp[a_]==Tj: fl.append(f"2P{a_}=T")
                hits.append((sg,fl))
        tpairs=[(a_,c_) for a_,c_ in itertools.combinations(range(4),2) if xs[a_]*xs[c_]==sbj]
        emb_rep.append(f"j{j}: hits {len(hits)} flags {[h[1] for h in hits]} xaxc=sqrt(b) {tpairs}")
    frat=all(x^Q==x for x in pt)
    print(f"   [{tag}] factor deg {k}: FQ-rational(t,s4) {frat} | {info} | J rank {J.rank()} kernel emb-support {musupp}",flush=True)
    for r_ in emb_rep: print(f"      {r_}",flush=True)
    print(f"      w degrees over F_Q: {[min(i for i in range(1,200) if w^(Q^i)==w) for w in ws]}",flush=True)

def analyse(name,u,b,E,Rt):
    eqs=system(u,b,Rt[0])
    for e in eqs:
        assert all(m.exponents()[0][i]%2==0 for m in e.monomials() for i in range(3))
    red=[reduce(e,Rw2) for e in eqs]
    c0=CPU(); gb=Rw2.ideal(red).groebner_basis('libsingular:std'); tgb=CPU()-c0
    NB=Rw2.ideal(gb).normal_basis(); n=len(NB); idx={m:i for i,m in enumerate(NB)}
    def nfvec(f):
        v=vector(Fq,n); r=f.reduce(gb)
        for c,mm in zip(r.coefficients(),r.monomials()): v[idx[mm]]=c
        return v
    c0=CPU()
    M4=matrix(Fq,[nfvec(G4[3]*m) for m in NB])
    cp=M4.charpoly().change_ring(Fq); cp=Px(cp.list())
    # Krylov rank from the unit vector (class of 1): = dim F_Q[s4].1 = degree of the s4-eliminant
    one=nfvec(Rw2(1)); kry=[one]
    for _ in range(n-1): kry.append(kry[-1]*M4)
    KM=matrix(Fq,kry); kr=KM.rank()
    tl=CPU()-c0
    m4=mults(cp)
    sq=sum(f.degree() for f,_ in cp.factor())
    dbl=[(f,e) for f,e in cp.factor() if e>1]
    print(f"Q={Q} {name} | deg {n} gb {tgb:.2f}s linalg {tl:.1f}s | s4-eliminant deg (Krylov rank of 1) {kr} | charpoly(s4) mult {m4} distinct-s4 {sq} | cyclic {kr==n} => #points {'%d'%sq if kr==n else '?'} non-reduced(deg-weighted) {sum(f.degree() for f,e in dbl) if kr==n else '?'}",flush=True)
    print(f"   doubled s4-factor degrees {sorted(f.degree() for f,e in dbl)}; deg-1 simple factors {sum(1 for f,e in cp.factor() if e==1 and f.degree()==1)}",flush=True)
    if not (CLS and kr==n and dbl): return
    KMi=KM.inverse()
    g=[Px(list(nfvec(G4[i])*KMi)) for i in range(3)]
    # sanity: shape-lemma basis t_i - g_i(s4) lies in the ideal
    for i in range(3): assert (G4[i]-g[i](G4[3])).reduce(gb)==0
    c0=CPU()
    for f,e in sorted(dbl,key=lambda fe:fe[0].degree()):
        if f.degree()>8: print(f"   skip doubled factor deg {f.degree()}",flush=True); continue
        classify_point(f"mult{e}",f,g,red,u,b,Rt)
    sim=[f for f,e in cp.factor() if e==1 and f.degree()<=4][:NCTL]
    for f in sim: classify_point("simple",f,g,red,u,b,Rt)
    print(f"   [classify {CPU()-c0:.1f}s]",flush=True)

done=0
while done<NC:
    b=K.random_element()
    if b==0: continue
    E=EllipticCurve(K,[1,0,0,0,b]); sb=b.sqrt(); D=P1=P2=None
    for _ in range(20000):
        zz=K.random_element(); u=zz^Q+zz
        if u==0: continue
        if D is None and Tr(sb/u)==0 and (sb/u^2)^Q!=sb/u^2: D=u
        if P1 is None and Tr(u)==0 and Tr(sb/u)!=0: P1=u
        if D is not None and P1 is not None: break
    if D is None or P1 is None: continue
    for _ in range(20000):
        u=K.random_element()
        if u!=0 and Tr(u)!=0 and Tr(sb/u)==0: P2=u; break
    while True:
        Rl=K.random_element()
        if Rl!=0 and Tr(Rl)!=0 and Tr(sb/Rl)!=0: break
    done+=1
    lines={"D":D,"P1":P1,"P2":P2,"R":Rl}
    for r in range(NT):
        Rt=2*E.random_point()
        if Rt.is_zero(): continue
        for ty in TYPES:
            print(f"c{done} r{r} {ty}: Tr(u)==0:{int(Tr(lines[ty])==0)} Tr(sb/u)==0:{int(Tr(sb/lines[ty])==0)} sqrt(b)/u^2 in F_Q:{int((sb/lines[ty]^2)^Q==sb/lines[ty]^2)}",flush=True)
            analyse(ty,lines[ty],b,E,Rt)
