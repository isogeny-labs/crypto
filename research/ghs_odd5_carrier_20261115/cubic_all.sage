"""All 84 selected-trace hyperelliptic models: exact cubic quotient search."""
import json
import time
from itertools import combinations
from pathlib import Path
from sage.all import GF, PolynomialRing, EllipticCurve, power_mod

P = Path(__file__).resolve().parent
F = GF(3)
R0 = PolynomialRing(F,'z')
z = R0.gen()
K = F.extension(z**5+2*z+1,'a')
R = PolynomialRing(K,'x')
x = R.gen()
TRACES = {-26,-22,-14,-10,10,14,22,26}

def polynomial(mask):
    f=x**11
    for i in range(11):
        f+=(mask%3)*x**i
        mask//=3
    assert mask==0 and f.is_squarefree()
    return f

def quadratic_divisors(f):
    factors=[p for p,e in f.factor() if e==1]
    assert sum(p.degree() for p in factors)==11
    out=[p for p in factors if p.degree()==2]
    out += [p*q for p,q in combinations(factors,2)
            if p.degree()==q.degree()==1]
    assert len({str(B) for B in out})==len(out)
    return out

start=time.monotonic()
screen=json.loads((P/'screen.json').read_text())
rows=[]
for raw in screen['rows']:
    mask,_,_,_,_,_,trace=raw
    if trace not in TRACES:
        continue
    f=polynomial(mask)
    maps=[]
    bcount=0
    trials=0
    for B in quadratic_divisors(f):
        bcount+=1
        C=f//B
        assert B*C==f
        A0=power_mod(C,3**9,B)
        assert (A0**3-C)%B==0
        for v in K:
            trials+=1
            A=A0+B*(x+v)
            assert A.is_monic() and A.degree()==3 and A.gcd(B)==1
            Q,rem=(C-A**3).quo_rem(B)
            assert rem==0
            c2=Q[6]
            Q-=c2*A**2
            c1=Q[5]
            Q-=c1*A*B
            c0=Q[4]
            if Q!=c0*B**2:
                continue
            E=EllipticCurve(K,[0,c2,0,c1,c0])
            if E.discriminant()==0:
                continue
            order=int(E.cardinality())
            assert order==244-trace,(mask,trace,order)
            maps.append(dict(A=str(A),B=str(B),c2=str(c2),c1=str(c1),
                             c0=str(c0),order=order))
    assert trials==243*bcount
    rows.append(dict(mask=mask,trace=trace,quadratic_divisors=bcount,
                     cubic_candidates=trials,maps=maps))
assert len(rows)==84
print(json.dumps(dict(field='F3[a]/(a^5+2*a+1)',selected_models=84,
                      total_cubic_candidates=sum(r['cubic_candidates'] for r in rows),
                      total_maps=sum(len(r['maps']) for r in rows),
                      rows=rows,elapsed_sec=time.monotonic()-start),indent=2))
