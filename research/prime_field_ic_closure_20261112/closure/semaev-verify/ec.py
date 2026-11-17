import random, pickle, os
from sympy import randprime, sqrt_mod
HERE = os.path.dirname(os.path.abspath(__file__))
POLYS = pickle.load(open(os.path.join(HERE,'semaev_polys.pkl'),'rb'))
def rand_curve(bits, rng):
    p = randprime(2**(bits-1), 2**bits)
    while True:
        a = rng.randrange(p); b = rng.randrange(1,p)
        if (4*a**3+27*b**2) % p: return p,a,b
def add(P,Q,p,a):
    if P is None: return Q
    if Q is None: return P
    x1,y1=P; x2,y2=Q
    if x1==x2:
        if (y1+y2)%p==0: return None
        l=(3*x1*x1+a)*pow(2*y1,-1,p)%p
    else:
        l=(y2-y1)*pow(x2-x1,-1,p)%p
    x3=(l*l-x1-x2)%p
    return (x3,(l*(x1-x3)-y1)%p)
def neg(P,p): return None if P is None else (P[0],(-P[1])%p)
def lift(x,p,a,b):
    r=(x**3+a*x+b)%p
    if r==0: return [(x,0)]
    if pow(r,(p-1)//2,p)!=1: return []
    y=sqrt_mod(r,p); return [(x,y),(x,(-y)%p)]
def rand_point(p,a,b,rng):
    while True:
        pts=lift(rng.randrange(p),p,a,b)
        if pts: return pts[0]
def semaev_eval(name, xs, p, a, b):
    """evaluate S_name (name 'S4' or 'S5') at a,b,x1..x_k mod p"""
    d=POLYS[name]; vals=[a,b]+list(xs)+[0]*(6-len(xs))
    tot=0
    for mon,c in d.items():
        t=int(c)
        for v,e in zip(vals,mon):
            if e: t=t*pow(v,e,p)
        tot+=t
    return tot%p
if __name__=='__main__':
    rng=random.Random(1)
    for bits in (12,16):
        p,a,b=rand_curve(bits,rng)
        for m,name in ((3,'S4'),(4,'S5')):
            ok=0
            for _ in range(5):
                Ps=[rand_point(p,a,b,rng) for _ in range(m)]
                R=None
                for P in Ps: R=add(R,P,p,a)
                ok+= semaev_eval(name,[P[0] for P in Ps]+[R[0]],p,a,b)==0
            bad=semaev_eval(name,[rand_point(p,a,b,rng)[0] for _ in range(m+1)],p,a,b)!=0
            print(bits,name,'true-sums vanish',ok,'/5','random nonzero',bad)
