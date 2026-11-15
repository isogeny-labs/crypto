import time, random, statistics as st
from sympy import symbols, groebner, expand, Poly
import sys
p=int(sys.argv[1]) if len(sys.argv)>1 else 1009; a=int(sys.argv[2]) if len(sys.argv)>2 else 5; x0=int(sys.argv[3]) if len(sys.argv)>3 else 3; b=(-x0**3-a*x0)%p; t=(3*x0*x0+a)%p; w=x0*t%p; A=(a-5*t)%p; B=(b-7*w)%p
u,v=symbols('u v')
def S3(X1,X2,X3,a_,b_): return (X1-X2)**2*X3**2-2*((X1+X2)*(X1*X2+a_)+2*b_)*X3+(X1*X2-a_)**2-4*b_*(X1+X2)
issq=lambda z: z%p==0 or pow(z,(p-1)//2,p)==1
import itertools
assert t and (4*a**3+27*b*b)%p and (4*A**3+27*B*B)%p
xs1=list(itertools.islice((x for x in range(p) if x!=x0 and issq(x**3+a*x+b)),40))           # x-coords on E1
xsE=list(itertools.islice((x for x in range(p) if issq(x**3+A*x+B)),40))                      # x-coords on E
phix=lambda x:(x*x-x0*x+t)*pow(x-x0,-1,p)%p
def plain_sys(F,xR):
    fF=lambda z: expand(eval('*'.join(f'(z-{c})' for c in F)) if F else 1)
    return [expand(S3(u,v,xR,A,B)), expand(fF(u)), expand(fF(v))]
def pulled_sys(F1,xR):
    n=lambda z:z*z-x0*z+t; d=lambda z:z-x0
    S=(n(u)*d(v)-n(v)*d(u))**2*xR**2-2*((n(u)*d(v)+n(v)*d(u))*(n(u)*n(v)+A*d(u)*d(v))+2*B*d(u)**2*d(v)**2)*xR+(n(u)*n(v)-A*d(u)*d(v))**2-4*B*d(u)*d(v)*(n(u)*d(v)+n(v)*d(u))
    prod=lambda z: eval('*'.join(f'(z-{c})' for c in F1))
    return [expand(S), expand(prod(u)), expand(prod(v))]
def timeit(sys_):
    t0=time.perf_counter(); G=groebner(sys_,u,v,modulus=p,order='grevlex'); return time.perf_counter()-t0, G
def nsol_plain(F,xR): return sum(1 for X1 in F for X2 in F if S3(X1,X2,xR,A,B)%p==0)
def nsol_pulled(F1,xR): return sum(1 for x in F1 for y in F1 if S3(phix(x),phix(y),xR,A,B)%p==0)
arms_n=lambda k,arms: next(ns for kk,_,ns in arms if kk==k)
R=random.Random(7)
print(f"p={p} E1:y^2=x^3+{a}x+{b}  E:y^2=x^3+{A}x+{B}  grevlex, sympy groebner, 6 targets per cell, seconds median[min,max]")
print("n | plain small-x on E | plain x(phi(F1)) on E | pulled-back on E1 | solutions plain/image/pulled (summed)")
assert len(xs1)>=16
for n in (4,8,12,16):
    F1=xs1[:n]; FI=sorted({phix(x) for x in F1}); FE=xsE[:len(FI)]
    T={'E':[],'I':[],'P':[]}; sol=[0,0,0]
    def rand_xE():
        while True:
            z=R.randrange(p)
            if issq(z**3+A*z+B): return z
    targets=[rand_xE() for _ in range(6)]
    for xR in targets:
        arms=[('E',plain_sys(FE,xR),nsol_plain(FE,xR)),('I',plain_sys(FI,xR),nsol_plain(FI,xR)),('P',pulled_sys(F1,xR),nsol_pulled(F1,xR))]
        from collections import Counter
        mult=Counter(phix(x) for x in F1)
        assert nsol_pulled(F1,xR)==sum(mult[X1]*mult[X2] for X1 in FI for X2 in FI if S3(X1,X2,xR,A,B)%p==0)
        R.shuffle(arms)  # randomized arm order per target
        for k,s,ns in arms:
            dt,G=timeit(s); T[k].append(dt)
            # factor-base roots are all in F_p, so G == [1] iff no solution
            assert (list(G.exprs)==[1])==(ns==0), (k,xR,ns,G.exprs)
        sol[0]+=arms_n('E',arms); sol[1]+=arms_n('I',arms); sol[2]+=arms_n('P',arms)
    f=lambda L:f"{st.median(L):.3f}[{min(L):.3f},{max(L):.3f}]"
    print(f"{n:2d} (|x(phi F1)|={len(FI)}) | {f(T['E'])} | {f(T['I'])} | {f(T['P'])} | {sol}", flush=True)
