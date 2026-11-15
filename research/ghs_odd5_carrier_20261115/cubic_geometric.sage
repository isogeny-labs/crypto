"""Geometric cubic elliptic quotient criterion over the branch splitting field."""
import json
import time
from itertools import combinations
from sage.all import GF, PolynomialRing, power_mod, gcd

F=GF(3)
L=GF(3**10,'b')
R0=PolynomialRing(L,'x')
x0=R0.gen()
V=PolynomialRing(L,'v')
v=V.gen()
R=PolynomialRing(V,'x')
x=R.gen()
U=PolynomialRing(V,'u')
u=U.gen()
ROWS=[(-26,28967),(26,35518)]

def poly(mask):
    f=x0**11
    for i in range(11):
        f+=(mask%3)*x0**i
        mask//=3
    assert mask==0 and f.is_squarefree()
    return f

def criterion(f,B):
    C=f//B
    assert B*C==f
    A0=power_mod(C,3**9,B)
    assert (A0**3-C)%B==0
    b=R(B);c=R(C);a0=R(A0)
    A=a0+b*(x+v)
    Q,rem=(c-A**3).quo_rem(b)
    assert rem==0
    c2=Q[6]
    Q-=c2*A**2
    c1=Q[5]
    Q-=c1*A*b
    c0=Q[4]
    residual=Q-c0*b**2
    G=V(0)
    for i in range(4):
        G=gcd(G,residual[i])
    # A zero residual means every v works; represent that separately.
    if G==0:
        return {'gcd_zero':True,'gcd_degree':None,'smooth_factor_degree':None}
    if G.degree()==0:
        return {'gcd_zero':False,'gcd_degree':0,'smooth_factor_degree':0}
    H=u**3+c2*u**2+c1*u+c0
    disc=H.discriminant()
    smooth_degree=sum(p.degree() for p,e in G.factor() if disc%p != 0)
    return {'gcd_zero':False,'gcd_degree':int(G.degree()),
            'smooth_factor_degree':int(smooth_degree),
            'gcd':str(G),'discriminant':str(disc)}

start=time.monotonic()
out=[]
for trace,mask in ROWS:
    f=poly(mask)
    roots=f.roots(L)
    assert len(roots)==11 and all(e==1 for _,e in roots)
    hits=[]
    for (r,_),(s,_) in combinations(roots,2):
        B=(x0-r)*(x0-s)
        result=criterion(f,B)
        if result['gcd_zero'] or result['gcd_degree']:
            hits.append({'B':str(B),**result})
    out.append({'trace':trace,'mask':mask,'pole_pairs':55,
                'nonconstant_gcd_pairs':len(hits),'pairs':hits})
print(json.dumps({'field':'F_(3^10)','rows':out,
                  'elapsed_sec':time.monotonic()-start},indent=2))
