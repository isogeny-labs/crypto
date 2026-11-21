import sys, time
# EXP15: char-2 squared-variable reduction (s1,s2,s3 enter only as squares; t_i = s_i^2). Run from twist_lift/.
sys.argv=['x','5','1','1','2501','0','0','0']
src=open('exp14_char2_solve.sage').read(); src=src[:src.index('print("d Q curve')]
exec(preparse(src))
set_random_seed(2501)
Rt_ = PolynomialRing(Fq,'t1,t2,t3,s4',order='degrevlex'); Rtl = PolynomialRing(Fq,'t1,t2,t3,s4',order='lex')
def reduce(e):
    return sum(c*prod(Rt_.gens()[i]^(m.exponents()[0][i]//2 if i<3 else m.exponents()[0][i]) for i in range(4)) for c,m in zip(e.coefficients(),e.monomials()))
b=K.random_element(); E=EllipticCurve(K,[1,0,0,0,b]); sb=b.sqrt()
for name,u in [("R",K.random_element()),("R2",K.random_element())]:
    R0=2*E.random_point(); eqs=system(u,b,R0[0]); red=[reduce(e) for e in eqs]
    t0=time.time(); gb=Rt_.ideal(red).groebner_basis('libsingular:std'); tg=time.time()-t0
    dg=len(Rt_.ideal(gb).normal_basis())
    t0=time.time(); lx=Rtl.ideal([Rtl(g) for g in red]).groebner_basis('libsingular:stdfglm'); tf=time.time()-t0
    uv=[g for g in lx if g.variables()==(Rtl.gens()[3],)]; h=uv[0].univariate_polynomial()
    print(name,"reduced deg",dg,"gb %.2fs"%tg,"stdfglm %.2fs"%tf,"lex len",len(lx),"s4-elim deg",h.degree(),"sqfree",h.gcd(h.derivative()).degree()==0)
    # count F_Q solutions both ways
    G2=Rq.ideal(eqs+[v^Q-v for v in Rq.gens()]); n1=len(Rq.ideal(G2.groebner_basis('libsingular:std')).normal_basis())
    G3=Rt_.ideal(red+[v^Q-v for v in Rt_.gens()]); n2=len(Rt_.ideal(G3.groebner_basis('libsingular:std')).normal_basis())
    print("   F_Q-solution count: original",n1,"reduced",n2)
