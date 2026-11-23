import sys, time
# EXP15: char-2 squared-variable reduction (s1,s2,s3 enter only as squares; t_i = s_i^2). Run from twist_lift/.
sys.argv=['x','5','1','1','2601','0','0','0']
src=open('exp14_char2_solve.sage').read(); src=src[:src.index('print("d Q curve')]
exec(preparse(src))
set_random_seed(2601)
W=TermOrder('wdegrevlex',(2,2,2,1))
Rd = PolynomialRing(Fq,'t1,t2,t3,s4',order='degrevlex'); Rw2 = PolynomialRing(Fq,'t1,t2,t3,s4',order=W)
def reduce(e,RR):
    g=RR.gens(); return sum(c*g[0]^(m.exponents()[0][0]//2)*g[1]^(m.exponents()[0][1]//2)*g[2]^(m.exponents()[0][2]//2)*g[3]^m.exponents()[0][3] for c,m in zip(e.coefficients(),e.monomials()))
X=PolynomialRing(Fq,'X').gen()
Rtl = PolynomialRing(Fq,'t1,t2,t3,s4',order='lex')
def ratpts(G,gens):
    sols=[[]]
    for k in range(3,-1,-1):
        new=[]
        for part in sols:
            sub={gens[k+1+i]:part[i] for i in range(len(part))}
            polys=[g.subs(sub) for g in G if min([i for i in range(4) if g.degree(gens[i])>0],default=-1)==k]
            polys=[p for p in polys if p!=0]
            if not polys: continue
            h=gcd([p.univariate_polynomial().change_ring(Fq).subs(X) if p.variables() else Fq(p)+0*X for p in polys])
            if h==0: continue
            for r,_ in h.roots(): new.append([r]+part)
        sols=new
    return [p for p in sols if all(g(*p)==0 for g in G)]
done=0
while done<2:
    b=K.random_element(); E=EllipticCurve(K,[1,0,0,0,b]); sb=b.sqrt()
    D=None
    for _ in range(20000):
        zz=K.random_element(); u=zz^Q+zz
        if u!=0 and Tr(sb/u)==0 and (sb/u^2)^Q!=sb/u^2: D=u; break
    if D is None: continue
    done+=1
    for r in range(3):
        Rt=2*E.random_point(); eqs=system(D,b,Rt[0])
        out=[]
        for RR in (Rd,Rw2):
            red=[reduce(e,RR) for e in eqs]
            c0=time.process_time(); gb=RR.ideal(red).groebner_basis('libsingular:std'); out.append(time.process_time()-c0)
        dg=len(Rw2.ideal(gb).normal_basis())
        # rational points of reduced system: field equations, then sqrt
        red=[reduce(e,Rd) for e in eqs]
        redl=[Rtl(str(f)) for f in red]
        G=list(Rtl.ideal(redl+[v^Q-v for v in Rtl.gens()]).groebner_basis('libsingular:stdfglm'))
        V=ratpts(G,Rtl.gens())
        split=0
        for P in V:
            s=[P[0].sqrt(),P[1].sqrt(),P[2].sqrt(),P[3]]
            rts=(X^4+s[0]*X^3+s[1]*X^2+s[2]*X+s[3]).roots()
            if sum(m for _,m in rts)==4 and all(x!=0 for x,_ in rts) and all(E.is_x_coord(D*emb(x)) for x,m in rts): split+=1
        bf=brute(E,D,Rt)
        c0=time.process_time(); lx=list(Rtl.ideal([Rtl(str(f)) for f in red]).groebner_basis('libsingular:stdfglm')); tl=time.process_time()-c0
        uv=[g for g in lx if g.variables()==(Rtl.gens()[3],)]
        if uv:
            h=uv[0].univariate_polynomial(); sq=h.degree()-h.gcd(h.derivative()).degree()
            mult={}
            for f_,e_ in h.factor(): mult[e_]=mult.get(e_,0)+f_.degree()
            print(f"   lex: len {len(lx)} s4-elim deg {h.degree()} sqfree-part {sq} multiplicity->degree {dict(sorted(mult.items()))} (stdfglm cpu {tl:.1f}s)",flush=True)
        else: print(f"   lex: len {len(lx)} no s4-univariate",flush=True)
        print(f"c{done} r{r} D: reduced deg {dg} | GB cpu degrevlex {out[0]:.2f}s wdegrevlex(2,2,2,1) {out[1]:.2f}s | Fq sols {len(V)} split {split} brute {bf}",flush=True)
