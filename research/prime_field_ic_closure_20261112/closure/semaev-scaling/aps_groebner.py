# O3: Amadori-Pintore-Sala style system {S4(x1,x2,x3,X), f(x1), f(x2), f(x3)}, f = prod_{v in Vx}(x - v),
# sympy groebner over GF(p), grevlex. Per-query wall time vs p (tiny p only).
import sys, time, math, random, json
import sympy as sp
from ec import rand_curve, rand_point, lift, POLYS
bits=int(sys.argv[1]); seed=int(sys.argv[2])
rng=random.Random(seed); p,a,b=rand_curve(bits,rng); m=3; s=math.ceil(bits/m)
Vx=[x for x in range(2**s) if lift(x,p,a,b)]
X=rand_point(p,a,b,rng)[0]
x1,x2,x3=sp.symbols('x1 x2 x3')
xs=[x1,x2,x3]
S=0
for mon,c in POLYS['S4'].items():
    t=int(c)*pow(a,mon[0],p)*pow(b,mon[1],p)*pow(X,mon[5],p)%p
    if t: S+=t*x1**mon[2]*x2**mon[3]*x3**mon[4]
f=lambda x: sp.prod([x-v for v in Vx])
sysm=[sp.expand(S)]+[sp.expand(f(x)) for x in xs]
t0=time.time()
G=sp.groebner(sysm,*xs,modulus=p,order='grevlex')
dt=time.time()-t0
print(json.dumps(dict(bits=bits,seed=seed,p=p,s=s,Vx=len(Vx),secs=round(dt,3),gb_len=len(G.exprs),unsat=G.exprs==[1],
      maxdeg=max(sp.Poly(g,*xs).total_degree() for g in G.exprs))),flush=True)
