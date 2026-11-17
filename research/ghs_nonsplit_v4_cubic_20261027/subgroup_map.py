#!/usr/bin/env sage -python
"""Evaluate the nonsplit-pole quotient correspondence in the q16 quartic Jacobian."""
import hashlib
import json
import time
from pathlib import Path

from sage.all import Curve, EllipticCurve, PolynomialRing

from nonsplit import L, dec, code, q

HERE=Path(__file__).resolve().parent
source_path=HERE/"nonsplit_4054.json"
canonical_path=HERE/"canonical_4054.json"
preimage_path=HERE/"preimages_4054.json"
quotient_path=HERE/"quotient_map_4054.json"
source=json.loads(source_path.read_text())
canonical=json.loads(canonical_path.read_text())
preimages=json.loads(preimage_path.read_text())
quotient=json.loads(quotient_path.read_text())
K=L.subfield(12,"w")
conv=lambda n:K(dec(n))
E=EllipticCurve(K,[K(1),conv(source["source_a"]),K(0),K(0),conv(source["source_b"])])
G=E(*map(conv,source["source_G"]))
Q=E(*map(conv,source["source_Q"]))
r=source["subgroup_prime"]
assert G!=E(0) and r*G==E(0) and Q==7*G

R3=PolynomialRing(K,names=("X","Y","Z"))
X,Y,Z=R3.gens()
F=sum((conv(c)*X**e[0]*Y**e[1]*Z**e[2]
       for e,c in zip(canonical["quartic_exponents"],
                      canonical["quartic_coefficients"])),K(0))
C=Curve(F)
assert C.is_smooth()

def eval_poly(exponents,coefficients,x,y,z=K(1)):
    return sum((conv(c)*x**e[0]*y**e[1]*z**(e[2] if len(e)==3 else 0)
                for e,c in zip(exponents,coefficients)),K(0))

iso_r,iso_s,iso_t,iso_u=map(conv,quotient["E0_to_E_isomorphism_rst_u"])
A,t=map(dec,(source["residue_A"],source["pole_t"]))
p,v=map(conv,(source["descent_p"],source["descent_v"]))
Abar=dec(source["residue_Aq3"])
tf=t**q
assert t+tf==1

def source_image(x,y0):
    values=[eval_poly(ex,co,x,y0) for ex,co in
            zip(quotient["to_E0_exponents"],quotient["to_E0_coefficients"])]
    assert values[2]!=0
    x0,y=values[0]/values[2],values[1]/values[2]
    return E(iso_u**2*x0+iso_r,iso_u**3*y+iso_s*iso_u**2*x0+iso_t)

def quartic_fiber(label,m):
    xx,y0,y1a,y1b=map(conv,preimages["fibers"][label])
    f0=A/(L(xx)+t)+Abar/(L(xx)+tf)
    f1=A**q/(L(xx)+tf)+Abar**q/(L(xx)+t)
    assert L(y0*y0+y0)==f0
    assert L(y1a*y1a+y1a)==f1 and y1b==y1a+1
    assert source_image(xx,y0)==m*G
    points=[]
    for y1 in (y1a,y1b):
        u=p*y0+v*y1
        xyz=[eval_poly(ex,co,xx,u) for ex,co in
             zip(canonical["map_exponents"],canonical["map_coefficients"])]
        P=C(xyz)
        assert P in C
        points.append(P)
    assert points[0]!=points[1]
    return points

fibers={label:quartic_fiber(label,m) for label,m in
        (("R",1),("R_plus_G",2),("R_plus_Q",8))}
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
out={"schema":"ghs-nonsplit-evaluated-subgroup-map-v1",
     "script_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
     "source_sha256":hashlib.sha256(source_path.read_bytes()).hexdigest(),
     "canonical_sha256":hashlib.sha256(canonical_path.read_bytes()).hexdigest(),
     "preimage_sha256":hashlib.sha256(preimage_path.read_bytes()).hexdigest(),
     "quotient_sha256":hashlib.sha256(quotient_path.read_bytes()).hexdigest(),
     "source_order":4054,"subgroup_prime":r,"source_G":source["source_G"],
     "source_Q":source["source_Q"],"translated_base_scalar":1,
     "quartic_fibers":{key:[[code(L(z)) for z in P] for P in row]
                       for key,row in fibers.items()},
     "checks":checks,"jacobian_setup_seconds":setup,
     "subgroup_map_seconds":mapping}
(HERE/"subgroup_map_4054.json").write_text(json.dumps(out,indent=2)+"\n")
print("PASS",checks,"setup",setup,"map",mapping)
