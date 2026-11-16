#!/usr/bin/env sage -python
"""Evaluate the two-common-pole source subgroup in the quartic Jacobian."""
import hashlib
import json
import time
from pathlib import Path

from sage.all import Curve, EllipticCurve, PolynomialRing

from triangle import K, code, dec, trace

HERE=Path(__file__).resolve().parent
source_path=HERE/"two_pole_4132.json"
canonical_path=HERE/"canonical_two_pole_4132.json"
source=json.loads(source_path.read_text())
data=json.loads(canonical_path.read_text())
q=16
r=source["subgroup_prime"]
A,B,s,p,v=map(dec,(source[k] for k in ("A","B","sqrt_b","p","v")))
E=EllipticCurve(K,[K(1),K(0),K(0),K(0),dec(source["source_b"])])
G=E(*map(dec,source["source_G"]))
Q=E(*map(dec,source["source_Q"]))
assert 7*G==Q and G!=E(0) and r*G==E(0)
assert A*B==s

R3=PolynomialRing(K,names=("X","Y","Z"))
X,Y,Z=R3.gens()
F=sum((dec(c)*X**e[0]*Y**e[1]*Z**e[2]
       for e,c in zip(data["quartic_exponents"],data["quartic_coefficients"])),K(0))
C=Curve(F)
assert C.is_smooth()

def eval_poly(exponents,coefficients,x,y):
    return sum((dec(c)*x**e[0]*y**e[1]
                for e,c in zip(exponents,coefficients)),K(0))

def split_fiber(P):
    if P==E(0) or P[0]==0: return None
    xx,yy=P[0],P[1]
    xp=xx/A
    y0=(yy+s)/xx
    f1=A**q*xp+B**q/xp
    if trace(f1)!=0: return None
    root=next(z for z in K if z*z+z==f1)
    out=[]
    for y1 in (root,root+1):
        u=p*y0+v*y1
        xyz=[eval_poly(ex,co,xp,u) for ex,co in
             zip(data["map_exponents"],data["map_coefficients"])]
        P2=C(xyz)
        assert P2 in C
        out.append(P2)
    assert out[0]!=out[1]
    return out

n=next(n for n in range(1,min(r-7,200))
       if all(split_fiber(m*G) is not None for m in (n,n+1,n+7)))
fibers={label:split_fiber(m*G) for label,m in
        (("R",n),("R_plus_G",n+1),("R_plus_Q",n+7))}
start=time.perf_counter()
base=3*fibers["R"][0].place().divisor()
J=C.jacobian(model="hess",base_div=base).group()
setup=time.perf_counter()-start
zero_div=fibers["R"][0].place().divisor()*0

def divisor(points):
    return sum((P.place().divisor() for P in points),zero_div)

def frob(points):
    return [C([z**q for z in P]) for P in points]

def traced_difference(numerator,denominator):
    out=J.zero()
    for _ in range(3):
        out+=J.point(divisor(numerator)-divisor(denominator))
        numerator,denominator=frob(numerator),frob(denominator)
    return out

TG=traced_difference(fibers["R_plus_G"],fibers["R"])
TQ=traced_difference(fibers["R_plus_Q"],fibers["R"])
mapping=time.perf_counter()-start-setup
checks={"G_nonzero":not TG.is_zero(),"G_order_r":(r*TG).is_zero(),
        "Q_nonzero":not TQ.is_zero(),"Q_order_r":(r*TQ).is_zero(),
        "Q_equals_7G":TQ==7*TG}
assert all(checks.values()),checks
out={"schema":"ghs-two-pole-evaluated-subgroup-map-v1",
     "script_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
     "source_sha256":hashlib.sha256(source_path.read_bytes()).hexdigest(),
     "canonical_sha256":hashlib.sha256(canonical_path.read_bytes()).hexdigest(),
     "source_b":source["source_b"],"source_order":4132,"subgroup_prime":r,
     "translated_base_scalar":n,
     "quartic_fibers":{key:[[code(z) for z in point] for point in row]
                       for key,row in fibers.items()},
     "checks":checks,"jacobian_setup_seconds":setup,
     "subgroup_map_seconds":mapping}
(HERE/"two_pole_map_4132.json").write_text(json.dumps(out,indent=2)+"\n")
print("PASS",checks,"setup",setup,"map",mapping)
