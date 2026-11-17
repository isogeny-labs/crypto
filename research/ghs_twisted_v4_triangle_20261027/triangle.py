#!/usr/bin/env sage -python
"""Exact Frobenius-cycled triangle covers in four residual binary classes."""
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, is_prime

HERE = Path(__file__).resolve().parent
R = PolynomialRing(GF(2), "w")
w = R.gen()
K = GF(2**12, modulus=w**12+w**7+w**6+w**5+w**3+w+1, name="w")
u = K.gen()


def dec(n):
    return sum(((n >> i) & 1) * u**i for i in range(12))


def code(z):
    return sum(int(v) << i for i, v in enumerate(z.polynomial().list()))


def trace(z):
    return sum(z**(2**i) for i in range(12))


def one_case(a_code, b_code, N, r):
    q = 16
    a, b = dec(a_code), dec(b_code)
    E = EllipticCurve(K, [K(1), a, K(0), K(0), b])
    assert E.cardinality() == N and is_prime(r)
    G = None
    for xcode in range(4096):
        for point in sorted(E.lift_x(dec(xcode), all=True), key=lambda p: code(p[1])):
            candidate = (N//r)*point
            if candidate != E(0) and r*candidate == E(0):
                G = candidate
                break
        if G is not None:
            break
    assert G is not None
    Q = 7*G
    assert Q != E(0) and r*Q == E(0)

    delta = dec(14)
    assert delta+delta**q+delta**(q*q) == 0 and delta != 0
    t0 = next(dec(i) for i in range(4096) if dec(i)+dec(i)**q == delta)
    t1, t2 = t0**q, t0**(q*q)
    assert len({t0,t1,t2}) == 3
    A = (b.sqrt()*delta**2)**pow(q+1,-1,4095)
    d = pow(2*(q+1),-1,4095)
    lam = b**d
    assert A == delta**(q*q-q+1)*lam
    beta, gamma = A/delta, A**q/delta
    s, c = beta*gamma, beta+gamma
    assert s*s == b and trace(c) == trace(a)
    assert trace(c) == trace(lam)
    h = next(dec(i) for i in range(4096) if dec(i)**2+dec(i) == c+a)

    P = PolynomialRing(K, "x")
    x = P.gen()
    F = P.fraction_field()
    f0 = A/(x+t0) + A**q/(x+t1)
    f1 = A**q/(x+t1) + A**(q*q)/(x+t2)
    f2 = A**(q*q)/(x+t2) + A/(x+t0)
    assert f0+f1+f2 == 0
    assert t2**q == t0 and (A**(q*q))**q == A

    # The X-coordinate of the quotient E0 is gamma*(x+t0)/(x+t1).
    U = (x+t0)/(x+t1)
    X = gamma*U
    assert f0 == X+c+s/X
    # Given y0^2+y0=f0, Y=X*y0+s+X*h obeys the target equation.
    assert (X**2*f0 + s**2 + X*s + X**2*(h*h+h)) == (X**3+a*X**2+b)

    def pullback_fiber(point):
        """The two-point degree-two divisor above a named elliptic point."""
        ex, ey = point[0], point[1]
        assert ex != 0 and ex != gamma
        xp = (ex*t1+gamma*t0)/(ex+gamma)
        yp = (ey+s+ex*h)/ex
        assert yp**2+yp == A/(xp+t0)+A**q/(xp+t1)
        f1p = A**q/(xp+t1)+A**(q*q)/(xp+t2)
        lifts = sorted((z for z in K if z*z+z == f1p), key=code)
        assert len(lifts) in (0,2)
        doubled = 2*point
        assert doubled != E(0)
        return {"x":code(xp), "y0":code(yp), "f1":code(f1p),
                "split_y1":list(map(code,lifts)),
                "pushforward_twice": [code(doubled[0]),code(doubled[1])]}

    def value(num, pole):
        return A/(num+t0)+A**q/(num+t1) if pole == 0 else (
            A**q/(num+t1)+A**(q*q)/(num+t2) if pole == 1 else
            A**(q*q)/(num+t2)+A/(num+t0))

    finite_unramified = 0
    for xi in K:
        if xi in (t0, t1, t2):
            continue
        n0 = 2 if trace(value(xi, 0)) == 0 else 0
        n1 = 2 if trace(value(xi, 1)) == 0 else 0
        finite_unramified += n0*n1
    at_poles = sum(2 if trace(value(pole, regular)) == 0 else 0
                   for pole, regular in ((t0, 1), (t1, 2), (t2, 0)))
    at_infinity = 4
    count_K = finite_unramified+at_poles+at_infinity
    assert count_K == 3*N-2*(4096+1)

    # Over k and k^2 the Galois-cycled trace-zero AS variable has one
    # solution above each base point; the three poles form a cubic orbit.
    k_codes = [i for i in range(4096) if dec(i)**q == dec(i)]
    assert len(k_codes) == q
    U_codes = [i for i in range(4096) if dec(i)+dec(i)**q+dec(i)**(q*q) == 0]
    assert len(U_codes) == q*q
    above_k = {}
    for xi_code in k_codes:
        fx = value(dec(xi_code), 0)
        ys = [ycode for ycode in U_codes if dec(ycode)**2+dec(ycode) == fx]
        assert len(ys) == 1
        above_k[xi_code] = ys[0]
    assert len(above_k)+1 == q+1

    output = {
        "schema": "ghs-frobenius-cycled-v4-triangle-v2",
        "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "field_modulus": "w^12+w^7+w^6+w^5+w^3+w+1",
        "base_q": q, "source_a": a_code, "source_b": b_code,
        "source_order": N, "subgroup_prime": r,
        "source_G": [code(G[0]),code(G[1])],
        "source_Q": [code(Q[0]),code(Q[1])],
        "delta": code(delta), "t_orbit": [code(t0),code(t1),code(t2)],
        "A_orbit": [code(A),code(A**q),code(A**(q*q))],
        "normalization": {"beta":code(beta), "gamma":code(gamma),
                          "sqrt_b":code(s), "c":code(c), "h":code(h)},
        "family_gate": {"exponent_d":d,"lambda":code(lam),
                         "trace_lambda":int(trace(lam)),"trace_a":int(trace(a))},
        "cover": {"f0":str(f0), "f1":str(f1), "f2":str(f2),
                  "relation":"f0+f1+f2=0",
                  "descent":"sigma(x)=x, sigma(y0)=y1, sigma(y1)=y0+y1"},
        "curve_point_counts": {"F16":17,"F256":257,"F4096":count_K},
        "F4096_fibers": {"unramified":finite_unramified,"three_poles":at_poles,
                            "infinity":at_infinity},
        "F16_affine_lifts": above_k,
        "jacobian_order":N,
        "pullback_fibers": {"G":pullback_fiber(G),"Q":pullback_fiber(Q)},
        "subgroup_map":"sum of the three Galois-conjugate degree-two pullbacks; quotient projection is [2] on each elliptic factor",
    }
    (HERE/f"certificate_{N}.json").write_text(json.dumps(output,indent=2)+"\n")
    print({"source_order":N,"curve_counts":output["curve_point_counts"],
           "fibers":output["F4096_fibers"]})


def main():
    for row in ((32,116,4198,2099),(32,853,4222,2111),
                (0,51,3972,331),(32,198,4006,2003)):
        one_case(*row)


if __name__ == "__main__":
    main()
