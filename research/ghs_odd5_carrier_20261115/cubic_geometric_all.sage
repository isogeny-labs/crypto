"""Symbolic geometric cubic quotient screen for all 84 odd-five models."""
import json
import time
from itertools import combinations
from pathlib import Path
from sage.all import GF, PolynomialRing, power_mod, gcd

P=Path(__file__).resolve().parent
L=GF(3**10,'b')
R0=PolynomialRing(L,'x')
x0=R0.gen()
V=PolynomialRing(L,'v')
v=V.gen()
R=PolynomialRing(V,'x')
x=R.gen()
U=PolynomialRing(V,'u')
u=U.gen()
TRACES={-26,-22,-14,-10,10,14,22,26}

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
screen=json.loads((P/'screen.json').read_text())
out=[]
for raw in screen['rows']:
    mask,_,_,_,_,_,trace=raw
    if trace not in TRACES:continue
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
assert len(out)==84
print(json.dumps({'field':'F_(3^10)','models':84,'pole_pairs':4620,
                  'nonconstant_gcd_pairs':sum(r['nonconstant_gcd_pairs'] for r in out),
                  'rows':out,'elapsed_sec':time.monotonic()-start},indent=2))
