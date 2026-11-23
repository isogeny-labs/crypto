import sys, time, itertools
# EXP16a: structure of the multiplicity-2 part of reduced char-2 systems (t_i = s_i^2), by line type.
# usage: sage exp16a_doubled.sage d types(e.g. D,P1,P2,R) ncurves ntargets seed classify(0/1)
d=int(sys.argv[1]); TYPES=sys.argv[2].split(','); NC=int(sys.argv[3]); NT=int(sys.argv[4]); SEED=int(sys.argv[5]); CLS=sys.argv[6]=='1'
sys.argv=['x',str(d),'1','1',str(SEED),'0','0','0']
src=open('exp14_char2_solve.sage').read(); src=src[:src.index('print("d Q curve')]
exec(preparse(src))
set_random_seed(SEED)
CPU=time.process_time
W=TermOrder('wdegrevlex',(2,2,2,1))
Rw2=PolynomialRing(Fq,'t1,t2,t3,s4',order=W); G4=Rw2.gens()
def reduce(e,RR):
    g=RR.gens(); return sum(c*g[0]^(m.exponents()[0][0]//2)*g[1]^(m.exponents()[0][1]//2)*g[2]^(m.exponents()[0][2]//2)*g[3]^m.exponents()[0][3] for c,m in zip(e.coefficients(),e.monomials()))
def mults(p):
    mu={}
    for f,e in p.factor(): mu[e]=mu.get(e,0)+f.degree()
    return dict(sorted(mu.items()))

def analyse(name,u,b,E,Rt):
    eqs=system(u,b,Rt[0])
    for e in eqs:   # verify the even-exponent property used by reduce()
        assert all(m.exponents()[0][i]%2==0 for m in e.monomials() for i in range(3))
    red=[reduce(e,Rw2) for e in eqs]
    c0=CPU(); gb=Rw2.ideal(red).groebner_basis('libsingular:std'); tgb=CPU()-c0
    NB=Rw2.ideal(gb).normal_basis(); n=len(NB); idx={m:i for i,m in enumerate(NB)}
    def nfvec(f):
        v=vector(Fq,n); r=f.reduce(gb)
        for c,mm in zip(r.coefficients(),r.monomials()): v[idx[mm]]=c
        return v
    c0=CPU()
    M=[matrix(Fq,[nfvec(G4[i]*m) for m in NB]) for i in range(4)]
    print("    [mats %.1fs]"%(CPU()-c0),flush=True)
    Px_=PolynomialRing(Fq,'X')
    def kmin(Mx,v):
        # minimal polynomial of v under Mx (Krylov + echelon); divides minpoly(Mx)
        rows=[v]
        for _ in range(n): rows.append(rows[-1]*Mx)
        KK=matrix(Fq,rows).transpose()
        ker=KK.right_kernel_matrix()   # relations sum c_j Mx^j v = 0
        # lowest-degree relation: echelon form with reversed columns
        km=ker.matrix_from_columns(list(range(n,-1,-1))).echelon_form()
        r=km.row(km.nrows()-1)[::-1]
        return Px_(list(r)).monic()
    def minp(Mx):
        f=Px_(1)
        for _ in range(2): f=lcm(f,kmin(Mx,random_vector(Fq,n)))
        return f
    mps=[minp(Mi) for Mi in M]; print("    [krylov minpolys %.1fs]"%(CPU()-c0),flush=True)
    cps=[None,None,None,M[3].charpoly()]
    cr=[Fq.random_element() for _ in range(4)]
    Ml=sum(cr[i]*M[i] for i in range(4)); cpl=Ml.charpoly(); mpl=minp(Ml)
    print("    [form %.1fs]"%(CPU()-c0),flush=True)
    sqf=[prod(f for f,_ in mpi.factor()) for mpi in mps]
    # if minpoly(form)==charpoly(form) then A = Fq[form]/charpoly, so #distinct points = deg sqfree(charpoly) exactly
    nrad=sum(f.degree() for f,_ in cpl.factor()) if mpl==cpl else -1
    elimsq=[ (mpi.degree(), sum(f.degree() for f,_ in mpi.factor())) for mpi in mps]
    print("    eliminant (deg, sqfree deg) t1,t2,t3,s4:",elimsq,flush=True)
    mp4=mps[3]
    tlin=CPU()-c0
    m4=mults(cps[3]); ml=mults(cpl)
    shape= (mp4==cps[3])
    print(f"Q={Q} {name} | deg {n} gb {tgb:.2f}s linalg {tlin:.2f}s | s4: minpoly==charpoly {shape} mult {m4} | random form: distinct {sum(f.degree() for f,_ in cpl.factor())} mult {ml} minpoly==charpoly {mpl==cpl} | radical deg {nrad}",flush=True)
    dbl=[(f,e) for f,e in cps[3].factor() if e>1]
    print(f"   doubled s4-factor degrees {sorted(f.degree() for f,e in dbl)}; doubled form-factor degrees {sorted(f.degree() for f,e in cpl.factor() if e>1)}",flush=True)
    if not (CLS and shape and dbl): return
    # shape-lemma basis: t_i = g_i(s4) in A = Fq[s4]/cp
    one=nfvec(Rw2(1)); kry=[one]
    for _ in range(n-1): kry.append(kry[-1]*M[3])
    KM=matrix(Fq,kry)   # row j = s4^j
    KMi=KM.inverse()
    Px=PolynomialRing(Fq,'X')
    g=[Px(list(nfvec(G4[i])*KMi)) for i in range(3)]
    for f,e in dbl:
        k=f.degree(); ee=lcm(4,k); dl=d*ee*12
        L=GF(2^dl,'l'); tL=K.modulus().change_ring(L).roots()[0][0]
        phK=lambda c,j=0: c.polynomial()(tL)^(Q^j)
        phQ=lambda c: phK(emb(c))
        PL=PolynomialRing(L,'Y')
        al=PL([phQ(c) for c in f.list()]).roots()[0][0]
        tt=[PL([phQ(c) for c in gi.list()])(al) for gi in g]
        ss=[x.sqrt() for x in tt]+[al]
        quart=PL([ss[3],ss[2],ss[1],ss[0],1])
        ws=[r for r,mm in quart.roots() for _ in range(mm)]
        # Jacobian of the 4 descended reduced eqs at the point
        pt=tt+[al]
        J=matrix(L,[[PL(0).parent()(0)+ sum(phQ(c)*prod(pt[v]^ex for v,ex in enumerate(mm.exponents()[0])) for c,mm in zip(r_.derivative(G4[i]).coefficients(),r_.derivative(G4[i]).monomials())) if r_.derivative(G4[i])!=0 else L(0) for i in range(4)] for r_ in red])
        Th=matrix(L,[[tL^(l*Q^j) for l in range(4)] for j in range(4)])
        lk=J.left_kernel().basis()
        mus=[ (vector(L,lam)*Th.inverse()) for lam in lk]
        musupp=[[j for j in range(4) if mu[j]!=0] for mu in mus]
        info=[]
        info.append(f"wsplit {len(ws)}")
        info.append(f"wQrat {sum(1 for w in ws if w^Q==w)}")
        info.append(f"w0 {sum(1 for w in ws if w==0)}")
        info.append(f"wrep {len(ws)-len(set(ws))}")
        fl=[]
        if len(ws)==4:
            for j in range(4):
                uj=phK(u,j); bj=phK(b,j); sbj=bj.sqrt(); xr=phK(Rt[0],j); xs=[uj*w for w in ws]
                for a,c in itertools.combinations(range(4),2):
                    if xs[a]*xs[c]==sbj: fl.append(f"Tpair j{j} ({a}{c})")
                    if S3(xs[a],xs[c],xr,bj)==0: fl.append(f"S3(xa,xb,xR) j{j} ({a}{c})")
                for tr in itertools.combinations(range(4),3):
                    if S3(xs[tr[0]],xs[tr[1]],xs[tr[2]],bj)==0: fl.append(f"S3 triple j{j} {tr}")
                for a in range(4):
                    if xs[a]==xr: fl.append(f"x=xR j{j} {a}")
        frat=all(x^Q==x for x in pt)
        print(f"   factor deg {k}: point F_Q-rational {frat} | {' '.join(info)} | J rank {J.rank()} kernel embedding-support {musupp} | flags {fl}",flush=True)
        print(f"      t {[x^Q==x for x in tt]} s4inFQ {al^Q==al}  w-orbit: {[min(i for i in range(1,13*ee+1) if w^(Q^i)==w) for w in ws]}",flush=True)

done=0
while done<NC:
    b=K.random_element()
    if b==0: continue
    E=EllipticCurve(K,[1,0,0,0,b]); sb=b.sqrt(); D=P1=P2=None
    for _ in range(20000):
        zz=K.random_element(); u=zz^Q+zz
        if u==0: continue
        if D is None and Tr(sb/u)==0 and (sb/u^2)^Q!=sb/u^2: D=u
        if P1 is None and Tr(sb/u)!=0: P1=u
        if D is not None and P1 is not None: break
    if D is None: continue
    for _ in range(20000):
        u=K.random_element()
        if u!=0 and Tr(u)!=0 and Tr(sb/u)==0: P2=u; break
    done+=1
    lines={"D":D,"P1":P1,"P2":P2,"R":K.random_element()}
    for r in range(NT):
        Rt=2*E.random_point()
        if Rt.is_zero(): continue
        for ty in TYPES:
            print(f"c{done} r{r} {ty}: Tr(u)==0:{int(Tr(lines[ty])==0)} Tr(sb/u)==0:{int(Tr(sb/lines[ty])==0)}",flush=True)
            analyse(ty,lines[ty],b,E,Rt)
