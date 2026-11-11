#!/usr/bin/env python3
"""CT25: independent geometric cross-norm regression for F101 split genus-2 targets.

No imports from CT20-CT24; standard-library only. Verifies actual closed fibers
in F_101^2, both elliptic quotient norms, Fp-Frobenius descent, and the
CM-field obstruction to geometric cross-factor homomorphisms.

Run: python3 -S research/chained_jacobian_transfers_20261110/ct25_absolute_cross_factor.py
Do not run with python -O: assertions are acceptance gates.
Scientific standing: producer-run only, [pcc:unverified].
"""
from math import isqrt

P = 101
D = 2  # nonsquare in F101; Fp2 = Fp[w] / (w^2-2)
CHARTS = (
    ("B+A", 92, 44, 33, 7),
    ("B+B", 26, 42, 68, 21),
)
SRC109 = (1, 46)
SRC113 = (44, 21)
RECIP109 = (26, 58)


def ff(a, b=0):
    return (a % P, b % P)


def plus(a, b):
    return ff(a[0] + b[0], a[1] + b[1])


def neg(a):
    return ff(-a[0], -a[1])


def minus(a, b):
    return plus(a, neg(b))


def times(a, b):
    return ff(a[0] * b[0] + D * a[1] * b[1],
              a[0] * b[1] + a[1] * b[0])


def fpow(a, n):
    assert n >= 0
    v = ff(1)
    while n:
        if n & 1:
            v = times(v, a)
        a = times(a, a)
        n >>= 1
    return v


def finv(a):
    den = (a[0] * a[0] - D * a[1] * a[1]) % P
    assert den, 'denominator is zero'
    z = pow(den, -1, P)
    return ff(a[0] * z, -a[1] * z)


def divide(a, b):
    return times(a, finv(b))


def cub(a):
    return fpow(a, 3)


def curve_rhs(x, a2, a4, a6):
    return plus(plus(cub(x), times(ff(a2), fpow(x, 2))),
                plus(times(ff(a4), x), ff(a6)))


def on_curve(Q, a2, a4, a6):
    return Q is None or fpow(Q[1], 2) == curve_rhs(Q[0], a2, a4, a6)


def ec_add(Q, R, a2, a4, a6):
    assert on_curve(Q, a2, a4, a6) and on_curve(R, a2, a4, a6)
    if Q is None:
        return R
    if R is None:
        return Q
    x, y = Q
    u, v = R
    if x == u and plus(y, v) == ff(0):
        return None
    if x == u:
        assert y == v and y != ff(0)
        m = divide(plus(plus(times(ff(3), fpow(x, 2)), times(ff(2 * a2), x)),
                        ff(a4)), times(ff(2), y))
    else:
        m = divide(minus(v, y), minus(u, x))
    xx = minus(minus(minus(fpow(m, 2), ff(a2)), x), u)
    yy = minus(times(m, minus(x, xx)), y)
    T = (xx, yy)
    assert on_curve(T, a2, a4, a6)
    return T


def short_points(a, b):
    squares = {}
    for y in range(P):
        squares.setdefault(y*y % P, []).append(y)
    return [None] + [(x, y) for x in range(P)
                     for y in squares.get((x**3+a*x+b) % P, ())]


def sqrt_base(x):
    return next((y for y in range(P) if y*y % P == x % P), None)


def sqrt_in_fp2(x):
    x %= P
    y = sqrt_base(x)
    if y is not None:
        return ff(y)
    # D=2 is nonsquare, so x/D is square for nonsquare x.
    z = sqrt_base(x * pow(D, -1, P))
    assert z is not None
    q = ff(0, z)
    assert fpow(q, 2) == ff(x)
    return q


def short_coeffs(a2, a4, a6):
    h = a2 * pow(3, -1, P) % P
    return ((a4 - a2*h) % P,
            (a6 - a4*h + 2*h*h*h) % P)


def sqfree_part(n):
    assert n != 0
    sign = -1 if n < 0 else 1
    n = abs(n)
    ans = 1
    d = 2
    while d*d <= n:
        multiplicity = 0
        while n % d == 0:
            n //= d
            multiplicity ^= 1
        if multiplicity:
            ans *= d
        d += 1
    return sign * ans * n


def verify_chart(name, A, B, b, scale):
    s2 = scale*scale % P
    s3 = s2*scale % P
    long_rec = (B, b*A % P, b*b % P)
    long_even = (A, B, b)
    rec_short = short_coeffs(*long_rec)
    even_short = short_coeffs(*long_even)
    assert (rec_short[0]*pow(scale, 4, P) % P,
            rec_short[1]*pow(scale, 6, P) % P) == RECIP109
    assert len(short_points(*rec_short)) == 109
    assert len(short_points(*even_short)) == 113
    assert len(short_points(*SRC109)) == 109
    assert len(short_points(*SRC113)) == 113
    assert pow(D, (P-1)//2, P) == P-1
    base_y = sqrt_base(b)
    assert base_y and base_y != 0
    base_points = ((ff(0), ff(base_y)), (ff(0), ff(-base_y)))
    assert ec_add(*base_points, *long_even) is None
    # Reference divisor D0 has pi_rec images O+O and pi_even images summing O.
    kinds = {'split': 0, 'nonsplit': 0, 'ramified_infinity': 0, 'zero': 0}
    checked = 0
    for T in short_points(*RECIP109):
        if T is None:
            kinds['zero'] += 1
            checked += 1
            continue
        X, Y = T
        U = (X*pow(s2, -1, P) - B*pow(3, -1, P)) % P
        V = Y*pow(s3, -1, P) % P
        Q = (ff(U), ff(V))
        assert on_curve(Q, *long_rec)
        norm = ec_add(Q, Q, *long_rec)
        assert on_curve(norm, *long_rec)
        if U == 0:
            # Q=(0,+/-b) has a double fiber at infinity on C.
            # pi_even sends both genus-2 infinities to E_even origin.
            assert V in (b, -b % P)
            kinds['ramified_infinity'] += 1
            checked += 1
            continue
        r = b * pow(U, -1, P) % P
        slope = V * pow(U, -1, P) % P
        t = sqrt_in_fp2(r)
        y = times(ff(slope), t)
        r2 = ff(r)
        assert fpow(t, 2) == r2
        assert fpow(y, 2) == plus(plus(cub(r2), times(ff(A), fpow(r2, 2))),
                                   plus(times(ff(B), r2), ff(b)))
        # Reciprocal quotient maps both geometric fiber points to Q.
        rec_images = []
        for tx, yy in ((t, y), (neg(t), neg(y))):
            u = divide(ff(b), fpow(tx, 2))
            v = divide(times(ff(b), yy), fpow(tx, 3))
            assert (u, v) == Q
            rec_images.append((u, v))
        assert ec_add(*rec_images, *long_rec) == norm
        # Even quotient maps the same two points to inverses, hence 0.
        even_Q = (r2, y)
        even_R = (r2, neg(y))
        assert on_curve(even_Q, *long_even)
        assert on_curve(even_R, *long_even)
        assert ec_add(even_Q, even_R, *long_even) is None
        # Closed degree-2 divisor invariant under Frobenius.
        tfr = fpow(t, P)
        yfr = fpow(y, P)
        assert (tfr, yfr) in ((t, y), (neg(t), neg(y)))
        if t[1] == 0:
            assert tfr == t
            kinds['split'] += 1
        else:
            assert tfr == neg(t)
            kinds['nonsplit'] += 1
        checked += 1
    assert checked == 109
    assert kinds == {'split': 58, 'nonsplit': 48, 'ramified_infinity': 2, 'zero': 1}, kinds
    # Negative control: an incorrect even-quotient coefficient rejects a
    # nonzero fiber (prevents a self-fulfilling norm cancellation).
    assert any(not on_curve((ff(r), ff(0)), A, (B+1) % P, b)
               for r in range(1, P))
    print(name, '109/109 points; reciprocal norm [2]; even cross norm 0;',
          'Fp2 Frobenius descent;', kinds)


def main():
    traces = []
    for label, curve, n in [('E109', SRC109, 109),
                             ('E113', SRC113, 113),
                             ('E109-target', RECIP109, 109)]:
        assert len(short_points(*curve)) == n
        t = P+1-n
        delta = t*t-4*P
        assert t % P != 0, 'ordinarity is required'
        traces.append((label, t, delta, sqfree_part(delta)))
    assert traces[0][1:] == traces[2][1:]
    assert traces[0][3] == -355
    assert traces[1][3] == -283
    assert traces[0][3] != traces[1][3]
    print('Ordinary elliptic factors:', traces,
          'different End^0 CM fields -> geometric Hom=0')
    for row in CHARTS:
        verify_chart(*row)
    print('PASS CT25: absolute cross-factor obstruction and 2x109 '
          'closed-fiber quotient tests [pcc:unverified]')


if __name__ == '__main__':
    main()
