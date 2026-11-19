import sys, time
# EXP16a probe: build one reduced D system (Q=32), weighted GB, multiplication matrices, minpolys. Timing test.
d=int(sys.argv[1]) if len(sys.argv)>1 else 5
sys.argv=['x',str(d),'1','1','1601','0','0','0']
src=open('exp14_char2_solve.sage').read(); src=src[:src.index('print("d Q curve')]
exec(preparse(src))
set_random_seed(1601)
W=TermOrder('wdegrevlex',(2,2,2,1))
Rw2=PolynomialRing(Fq,'t1,t2,t3,s4',order=W)
def reduce(e,RR):
    g=RR.gens(); return sum(c*g[0]^(m.exponents()[0][0]//2)*g[1]^(m.exponents()[0][1]//2)*g[2]^(m.exponents()[0][2]//2)*g[3]^m.exponents()[0][3] for c,m in zip(e.coefficients(),e.monomials()))
while True:
    b=K.random_element(); E=EllipticCurve(K,[1,0,0,0,b]); sb=b.sqrt(); D=None
    for _ in range(20000):
        zz=K.random_element(); u=zz^Q+zz
        if u!=0 and Tr(sb/u)==0 and (sb/u^2)^Q!=sb/u^2: D=u; break
    if D is not None: break
Rt=2*E.random_point(); eqs=system(D,b,Rt[0]); red=[reduce(e,Rw2) for e in eqs]
c0=time.process_time(); gb=Rw2.ideal(red).groebner_basis('libsingular:std'); print("gb",time.process_time()-c0,flush=True)
c0=time.process_time(); NB=Rw2.ideal(gb).normal_basis(); idx={m:i for i,m in enumerate(NB)}; n=len(NB); print("nb",n,time.process_time()-c0,flush=True)
def mulmat(f):
    M=matrix(Fq,n,n)
    for i,m in enumerate(NB):
        r=(f*m).reduce(gb)
        for c,mm in zip(r.coefficients(),r.monomials()): M[i,idx[mm]]=c
    return M
c0=time.process_time(); M4=mulmat(Rw2.gens()[3]); print("mulmat",time.process_time()-c0,type(M4),flush=True)
c0=time.process_time(); mp=M4.minpoly(); print("minpoly",time.process_time()-c0,mp.degree(),flush=True)
c0=time.process_time(); cp=M4.charpoly(); print("charpoly",time.process_time()-c0,flush=True)
print([(f.degree(),e) for f,e in cp.factor() if e>1])
