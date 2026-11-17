#!/usr/bin/env python3
"""Independent bit-field replay of the four triangle-cover certificates."""
import json
import hashlib
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "ghs_scalar3_frobenius_gate_20261027"))
from audit import check_point_pair, scale  # independent binary-curve group law

from census import inverse, mul, power, trace


def divide(a, b):
    return mul(a, inverse(b))


def evaluate(coeffs, x):
    out = 0
    for c in reversed(coeffs):
        out = mul(out,x) ^ c
    return out


def one(cert):
    N = cert["source_order"]
    a, b = cert["source_a"], cert["source_b"]
    check_point_pair(a, b, cert["subgroup_prime"], cert["source_G"], cert["source_Q"])
    t0,t1,t2 = cert["t_orbit"]
    A,A1,A2 = cert["A_orbit"]
    assert t1 == power(t0,16) and t2 == power(t1,16) and t0 == power(t2,16)
    assert A1 == power(A,16) and A2 == power(A1,16) and A == power(A2,16)
    delta = t0 ^ t1
    assert delta == cert["delta"] and t1 ^ t2 == power(delta,16)
    beta, gamma = divide(A,delta), divide(A1,delta)
    s, c, h = mul(beta,gamma), beta ^ gamma, cert["normalization"]["h"]
    assert cert["normalization"] == {"beta":beta,"gamma":gamma,"sqrt_b":s,"c":c,"h":h}
    assert mul(s,s) == b and trace(c) == trace(a)
    assert mul(h,h) ^ h == c ^ a
    gate = cert["family_gate"]
    lam = power(b, gate["exponent_d"])
    assert gate == {"exponent_d":1084,"lambda":lam,
                    "trace_lambda":trace(lam),"trace_a":trace(a)}
    assert trace(lam) == trace(c) == trace(a)
    assert A == mul(power(delta,241),lam)

    def f(x, i):
        if i == 0:
            return divide(A,x^t0) ^ divide(A1,x^t1)
        if i == 1:
            return divide(A1,x^t1) ^ divide(A2,x^t2)
        return divide(A2,x^t2) ^ divide(A,x^t0)

    unramified = 0
    for x in range(4096):
        if x in (t0,t1,t2):
            continue
        f0,f1,f2 = f(x,0),f(x,1),f(x,2)
        assert f0 ^ f1 ^ f2 == 0
        unramified += (2 if trace(f0)==0 else 0)*(2 if trace(f1)==0 else 0)
        if x != t0 and x != t1:
            X = mul(gamma, divide(x^t0,x^t1))
            if X:
                assert f0 == X ^ c ^ divide(s,X)
    poles = sum(2 if trace(f(x,i))==0 else 0
                for x,i in ((t0,1),(t1,2),(t2,0)))
    assert cert["F4096_fibers"] == {"unramified":unramified,
                                     "three_poles":poles,"infinity":4}
    assert unramified+poles+4 == cert["curve_point_counts"]["F4096"]
    assert unramified+poles+4 == 3*N-8194

    k = [x for x in range(4096) if power(x,16)==x]
    U = [y for y in range(4096) if y ^ power(y,16) ^ power(y,256)==0]
    assert len(k)==16 and len(U)==256
    lifts = {}
    for x in k:
        fx = f(x,0)
        ys = [y for y in U if mul(y,y)^y == fx]
        assert len(ys)==1
        lifts[str(x)] = ys[0]
    assert {str(k):v for k,v in cert["F16_affine_lifts"].items()} == lifts
    assert cert["curve_point_counts"]["F16"] == 17
    assert cert["curve_point_counts"]["F256"] == 257
    for key in ("G","Q"):
        px,py = cert["source_"+key]
        assert px and px != gamma
        x = divide(mul(px,t1)^mul(gamma,t0),px^gamma)
        y0 = divide(py^s^mul(px,h),px)
        f1 = f(x,1)
        lifts = [z for z in range(4096) if mul(z,z)^z == f1]
        assert mul(y0,y0)^y0 == f(x,0)
        doubled = scale((px,py),2,a)
        assert doubled is not None
        assert cert["pullback_fibers"][key] == {
            "x":x,"y0":y0,"f1":f1,"split_y1":lifts,
            "pushforward_twice":list(doubled)}

    path = HERE/f"descent_{N}.json"
    model = json.loads(path.read_text())
    assert model["source_order"] == N
    assert model["source_certificate_sha256"] == hashlib.sha256(
        (HERE/f"certificate_{N}.json").read_bytes()).hexdigest()
    alpha = model["alpha"]
    p = alpha ^ power(alpha,256)
    v = power(alpha,16) ^ power(alpha,256)
    assert (p,v) == (model["p"],model["v"])
    l = model["additive_quartic_coefficients_low_to_high"]
    assert l[0] == l[3] == 0 and l[4] == 1
    assert {x for x in range(4096) if evaluate(l,x)==0} == {0,p,v,p^v}
    assert all(power(z,16)==z for z in l)
    num = model["rhs_numerator_coefficients_low_to_high"]
    den = model["rhs_denominator_coefficients_low_to_high"]
    assert all(power(z,16)==z for z in num+den)
    for x in range(4096):
        if x in (t0,t1,t2):
            continue
        f0,f1 = f(x,0),f(x,1)
        p2,v2 = mul(p,p),mul(v,v)
        p4,v4 = mul(p2,p2),mul(v2,v2)
        expected = mul(p4,f0^mul(f0,f0)) ^ mul(v4,f1^mul(f1,f1)) ^ \
                   mul(l[2],mul(p2,f0)^mul(v2,f1))
        assert evaluate(den,x) != 0
        assert divide(evaluate(num,x),evaluate(den,x)) == expected
    k = [x for x in range(4096) if power(x,16)==x]
    model_lifts = {}
    for x in k:
        rhs = divide(evaluate(num,x),evaluate(den,x))
        ys = [u for u in k if evaluate(l,u)==rhs]
        assert len(ys)==1
        model_lifts[str(x)] = ys[0]
    assert model["F16_affine_lifts"] == model_lifts
    assert model["F16_total_count"] == 17
    print("PASS",N,"b",b,"r",cert["subgroup_prime"],"#C(F4096)",unramified+poles+4)


def main():
    census = json.loads((HERE/"census.json").read_text())
    assert census["summary"]["all_triangle_models"] == 4095
    for N in (3972,4006,4198,4222):
        one(json.loads((HERE/f"certificate_{N}.json").read_text()))


if __name__ == "__main__":
    main()
