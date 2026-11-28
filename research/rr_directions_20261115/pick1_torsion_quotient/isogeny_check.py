"""Numeric check behind the torsion-quotient argument on y^2+xy=x^3+1 / GF(2^m).

Claims checked exhaustively over all affine points:
  (a) x(P+T) = 1/x(P) for T=(0,1), P not in {O,T}.
  (b) t(P) = x(P) + 1/x(P) is constant on {P, P+T}.
  (c) x(P + tau P) (= x(taubar(-P)), taubar = -1-tau, ker = <T>) is an
      affine F_2-linear function of t(P):  x(P+tauP) = t(P) + 1.
So the quotient coordinate t is, up to an affine F_2-linear map, the x-coordinate
on E/<T> = image of taubar, which is E itself.
"""
import sys
sys.path.insert(0, '/home/user/crypto/ecc2k130/codegen')
import field, curves

for m in (5, 9, 11):
    onb = field.Onb(m)
    E = curves.Curve(onb)
    one = onb.one()
    T = (0, one)
    assert E.onCurve(T) and E.dbl(T) is None
    pts = []
    for xi in range(1, 1 << m):
        x = onb.fromCoords(xi)
        P = E.pointFromX(x)
        if P is None:
            continue
        pts.append(P)
        pts.append(E.neg(P))
    ok_a = ok_b = ok_c = 0
    affine_const = None
    for P in pts:
        x, y = P
        xinv = onb.pow(x, (1 << m) - 2)
        Q = E.add(P, T)
        assert Q is not None and Q[0] == xinv
        ok_a += 1
        t = onb.add(x, xinv)
        tQ = onb.add(Q[0], onb.pow(Q[0], (1 << m) - 2))
        assert tQ == t
        ok_b += 1
        S = E.add(P, E.frob(P))
        assert S is not None, "P + tau P = O only for P in ker"
        d = onb.add(S[0], t)
        if affine_const is None:
            affine_const = d
        assert d == affine_const, (m, onb.toCoords(d), onb.toCoords(affine_const))
        ok_c += 1
    print("m=%d: %d affine points; x(P+T)=1/x: %d ok; t invariant: %d ok; "
          "x(P+tauP) = t + c with c = %s (coords %d): %d ok"
          % (m, len(pts), ok_a, ok_b, 'one' if affine_const == one else hex(affine_const),
             onb.toCoords(affine_const), ok_c))
