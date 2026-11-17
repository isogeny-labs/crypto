#!/usr/bin/env sage -python
"""Evaluate source G,Q in the actual quartic Jacobian via split translated fibers."""
import hashlib
import json
import sys
import time
from pathlib import Path

from sage.all import Curve, EllipticCurve, PolynomialRing

from triangle import K, code, dec, trace

HERE = Path(__file__).resolve().parent
ORDERS = (3972,4006,4198,4222)
QBASE = 16


def one(order):
    cert_path = HERE/f"certificate_{order}.json"
    data_path = HERE/f"canonical_data_{order}.json"
    cert = json.loads(cert_path.read_text())
    data = json.loads(data_path.read_text())
    assert cert["source_order"] == data["source_order"] == order
    a,b,r = cert["source_a"],cert["source_b"],cert["subgroup_prime"]
    E = EllipticCurve(K,[K(1),dec(a),K(0),K(0),dec(b)])
    G = E(dec(cert["source_G"][0]),dec(cert["source_G"][1]))
    assert G != E(0) and r*G == E(0) and 7*G == E(*map(dec,cert["source_Q"]))
    t0,t1,t2 = map(dec,cert["t_orbit"])
    A,A1,A2 = map(dec,cert["A_orbit"])
    gamma = dec(cert["normalization"]["gamma"])
    s = dec(cert["normalization"]["sqrt_b"])
    h = dec(cert["normalization"]["h"])
    alpha = t0
    p = alpha+alpha**(QBASE*QBASE)
    v = alpha**QBASE+alpha**(QBASE*QBASE)

    R3 = PolynomialRing(K,names=("X","Y","Z"))
    X,Y,Z = R3.gens()
    F = sum((dec(c)*X**e[0]*Y**e[1]*Z**e[2]
             for e,c in zip(data["quartic_exponents"],data["quartic_coefficients"])),K(0))
    C = Curve(F)
    assert C.is_smooth()

    def eval_poly(exponents,coefficients,x,y):
        return sum((dec(c)*x**e[0]*y**e[1]
                    for e,c in zip(exponents,coefficients)),K(0))

    def split_fiber(P):
        if P == E(0):
            return None
        ex,ey = P[0],P[1]
        if ex == 0 or ex == gamma:
            return None
        xp = (ex*t1+gamma*t0)/(ex+gamma)
        yp = (ey+s+ex*h)/ex
        f1 = A1/(xp+t1)+A2/(xp+t2)
        if trace(f1) != 0:
            return None
        root = next(z for z in K if z*z+z == f1)
        out = []
        for y1 in (root,root+1):
            u = p*yp+v*y1
            xyz = [eval_poly(ex,co,xp,u) for ex,co in
                   zip(data["map_exponents"],data["map_coefficients"])]
            point = C(xyz)
            assert point in C
            out.append(point)
        assert out[0] != out[1]
        return out

    n = next(n for n in range(1,min(r-7,200))
             if all(split_fiber(m*G) is not None for m in (n,n+1,n+7)))
    fibers = {label:split_fiber(m*G) for label,m in
              (("R",n),("R_plus_G",n+1),("R_plus_Q",n+7))}
    assert all(len(row)==2 for row in fibers.values())

    start = time.perf_counter()
    base = 3*fibers["R"][0].place().divisor()
    J = C.jacobian(model="hess",base_div=base).group()
    setup_seconds = time.perf_counter()-start
    zero_div = fibers["R"][0].place().divisor()*0

    def divisor(points):
        return sum((P.place().divisor() for P in points),zero_div)

    def frob(points):
        return [C([z**QBASE for z in P]) for P in points]

    def traced_difference(numerator,denominator):
        out = J.zero()
        for _ in range(3):
            out += J.point(divisor(numerator)-divisor(denominator))
            numerator,denominator = frob(numerator),frob(denominator)
        return out

    TG = traced_difference(fibers["R_plus_G"],fibers["R"])
    TQ = traced_difference(fibers["R_plus_Q"],fibers["R"])
    map_seconds = time.perf_counter()-start-setup_seconds
    checks = {"G_nonzero":not TG.is_zero(),"G_order_r":(r*TG).is_zero(),
              "Q_nonzero":not TQ.is_zero(),"Q_order_r":(r*TQ).is_zero(),
              "Q_equals_7G":TQ == 7*TG}
    assert all(checks.values()),checks
    output = {
        "schema":"ghs-twisted-v4-evaluated-subgroup-map-v1",
        "script_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "source_certificate_sha256":hashlib.sha256(cert_path.read_bytes()).hexdigest(),
        "canonical_data_sha256":hashlib.sha256(data_path.read_bytes()).hexdigest(),
        "source_order":order,"source_b":b,"subgroup_prime":r,
        "translated_base_scalar":n,
        "quartic_fibers":{key:[[code(z) for z in point] for point in row]
                          for key,row in fibers.items()},
        "checks":checks,"jacobian_setup_seconds":setup_seconds,
        "subgroup_map_seconds":map_seconds,
    }
    (HERE/f"subgroup_map_{order}.json").write_text(json.dumps(output,indent=2)+"\n")
    print("PASS",order,"r",r,"n",n,"setup",setup_seconds,
          "map",map_seconds,flush=True)


def main():
    orders = ORDERS if len(sys.argv)==1 else tuple(map(int,sys.argv[1:]))
    assert set(orders).issubset(ORDERS)
    for order in orders:
        one(order)


if __name__ == "__main__":
    main()
