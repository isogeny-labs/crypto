"""Boolean polynomial systems for the Riemann–Roch (Nagao) point decomposition
on the Koblitz curve  E : y^2 + x y = x^3 + 1  over GF(2^m), polynomial basis.

Two presentations of "P1 + P2 + P3 = R with x(P_i) in a subspace V":

* ``incidence`` (Boolean degree 2): unknown bits are the V-coordinates of the
  x_i (ell each), the y_i (m each), the function coefficients a, b (m each) of
  f = x^2 + a x + c + b y in L(4O) with c pinned by f(-R) = 0, and three
  distinctness witnesses w_ij with (x_i + x_j) w_ij = 1 (m each).  Equations:
  the three curve equations, the three incidences f(P_i) = 0, the three
  witness equations.  Without the witnesses the system has ~ 3 * |V|^2
  spurious solutions with P_i = P_j (a repeated evaluation row is not a
  double zero); with them, every solution is an ordered genuine decomposition.
  b = 0 is impossible once the four abscissas are distinct (a quadratic in x
  cannot vanish at four points), and x_i = x_R is impossible because x_R is
  drawn outside V.

* ``norm`` (Boolean degree 3): unknowns are the V-coordinates and a, b.
  N(f) = (x^2 + a x + c)^2 + b x (x^2 + a x + c) + b^2 (x^3 + 1) is matched,
  coefficient by coefficient, with (X + x_R)(X + x_1)(X + x_2)(X + x_3).
  The divisor is encoded exactly, so repeated abscissas mean genuine repeated
  summands and no witnesses are needed.

Both are produced from the same symbolic field arithmetic: an element is a
list of m Boolean polynomials (coordinates in the polynomial basis), a Boolean
polynomial is a set of monomials, a monomial an int bitmask of variable
indices.  ``selftest`` evaluates the symbolic arithmetic against the concrete
field at random points.

Variable layout (indices into the Boolean system):
  u_{i,k}  i<3, k<ell        : 3*ell       (x_i = sum_k u_{i,k} v_k)
  then per arm as documented in ``layout``.

No type hints, camelCase identifiers, no itertools (project convention of the
sibling codegen; kept here).
"""

import json
import random


# ---------------------------------------------------------------- concrete field

class Gf2m:
    """GF(2^m) with a fixed irreducible polynomial, ints as bit vectors."""

    def __init__(self, m):
        self.m = m
        self.poly = irreduciblePoly(m)
        self.one = 1
        self.zero = 0
        # reduction table: X^k mod poly for k in [m, 2m-2]
        self.red = []
        v = 1 << m
        for k in range(m, 2 * m - 1):
            self.red.append(self._reduceOne(v))
            v = self._reduceOne(v << 1)

    def _reduceOne(self, v):
        m = self.m
        while v >> m:
            shift = v.bit_length() - 1 - m
            v ^= self.poly << shift
        return v

    def add(self, a, b):
        return a ^ b

    def mul(self, a, b):
        r = 0
        while b:
            if b & 1:
                r ^= a
            b >>= 1
            a <<= 1
            if a >> self.m:
                a ^= self.poly
        return r

    def sqr(self, a):
        return self.mul(a, a)

    def inv(self, a):
        if a == 0:
            raise ZeroDivisionError
        return self.pow(a, (1 << self.m) - 2)

    def pow(self, a, e):
        r = 1
        while e:
            if e & 1:
                r = self.mul(r, a)
            a = self.mul(a, a)
            e >>= 1
        return r

    def trace(self, a):
        t = a
        s = a
        for _ in range(self.m - 1):
            s = self.sqr(s)
            t ^= s
        return t

    def halfTrace(self, a):
        """m odd: h^2 + h = a when Tr(a) = 0."""
        h = a
        s = a
        for _ in range((self.m - 1) // 2):
            s = self.sqr(self.sqr(s))
            h ^= s
        return h


def irreduciblePoly(m):
    """Lowest-weight irreducible of degree m (trinomial, else pentanomial)."""
    for p in range((1 << m) + 1, 1 << (m + 1), 2):
        if bin(p).count('1') in (3, 5) and _isIrreducible(p, m):
            return p
    raise ValueError('no low-weight irreducible found')


def _isIrreducible(p, m):
    # X^(2^k) mod p for k=1..m; irreducible iff X^(2^m) = X and gcd(X^(2^k)-X, p)=1 for k<=m/2
    def mulmod(a, b):
        r = 0
        while b:
            if b & 1:
                r ^= a
            b >>= 1
            a <<= 1
            if a >> m:
                a ^= p
        return r

    def gcd(a, b):
        while b:
            while a.bit_length() >= b.bit_length() and a:
                a ^= b << (a.bit_length() - b.bit_length())
            a, b = b, a
        return a

    x = 2
    for k in range(1, m // 2 + 1):
        x = mulmod(x, x)
        if gcd(p, x ^ 2) != 1:
            return False
    return True


class Curve:
    """y^2 + xy = x^3 + 1, affine, None is the point at infinity."""

    def __init__(self, f):
        self.f = f

    def onCurve(self, p):
        if p is None:
            return True
        x, y = p
        f = self.f
        return f.add(f.sqr(y), f.mul(x, y)) == f.add(f.mul(f.sqr(x), x), 1)

    def neg(self, p):
        if p is None:
            return None
        return (p[0], p[0] ^ p[1])

    def dbl(self, p):
        if p is None or p[0] == 0:
            return None
        f = self.f
        x, y = p
        lam = f.add(x, f.mul(y, f.inv(x)))
        x3 = f.add(f.sqr(lam), lam)
        y3 = f.add(f.sqr(x), f.mul(f.add(lam, 1), x3))
        return (x3, y3)

    def add(self, p, q):
        if p is None:
            return q
        if q is None:
            return p
        if p[0] == q[0]:
            return self.dbl(p) if p[1] == q[1] else None
        f = self.f
        x1, y1 = p
        x2, y2 = q
        d = f.add(x1, x2)
        lam = f.mul(f.add(y1, y2), f.inv(d))
        x3 = f.add(f.add(f.sqr(lam), lam), d)
        y3 = f.add(f.add(f.mul(lam, f.add(x1, x3)), x3), y1)
        return (x3, y3)

    def mul(self, p, k):
        r = None
        while k:
            if k & 1:
                r = self.add(r, p)
            p = self.dbl(p)
            k >>= 1
        return r

    def pointFromX(self, x):
        f = self.f
        if x == 0:
            return (0, 1)
        c = f.add(x, f.inv(f.sqr(x)))
        if f.trace(c):
            return None
        y = f.mul(x, f.halfTrace(c))
        p = (x, y)
        assert self.onCurve(p)
        return p

    def order(self):
        """#E(GF(2^m)) for y^2+xy=x^3+1 by the Lucas recurrence (t_1 = -1)."""
        m = self.f.m
        v0, v1 = 2, -1
        for _ in range(m - 1):
            v0, v1 = v1, -v1 - 2 * v0
        return (1 << m) + 1 - v1


# -------------------------------------------------------- symbolic arithmetic

def polyMul(p, q):
    r = set()
    for a in p:
        for b in q:
            r ^= {a | b}
    return r


def polyAdd(p, q):
    return p ^ q


def polyDegree(p):
    return max((bin(t).count('1') for t in p), default=0)


class SymField:
    """Symbolic GF(2^m): an element is a list of m Boolean polynomials."""

    def __init__(self, f):
        self.f = f
        self.m = f.m

    def const(self, c):
        return [({0} if (c >> i) & 1 else set()) for i in range(self.m)]

    def zero(self):
        return [set() for _ in range(self.m)]

    def fromBits(self, varStart):
        return [{1 << (varStart + i)} for i in range(self.m)]

    def linear(self, basis, varStart):
        """sum_k u_k basis[k] with u_k the Boolean variables varStart+k."""
        e = self.zero()
        for k, v in enumerate(basis):
            for i in range(self.m):
                if (v >> i) & 1:
                    e[i] ^= {1 << (varStart + k)}
        return e

    def add(self, a, b):
        return [a[i] ^ b[i] for i in range(self.m)]

    def mul(self, a, b):
        m = self.m
        prod = [set() for _ in range(2 * m - 1)]
        for i in range(m):
            if not a[i]:
                continue
            for j in range(m):
                if b[j]:
                    prod[i + j] ^= polyMul(a[i], b[j])
        return self._reduce(prod)

    def sqr(self, a):
        m = self.m
        prod = [set() for _ in range(2 * m - 1)]
        for i in range(m):
            prod[2 * i] = set(a[i])
        return self._reduce(prod)

    def _reduce(self, prod):
        m = self.m
        out = prod[:m]
        for k in range(m, 2 * m - 1):
            if prod[k]:
                r = self.f.red[k - m]
                for i in range(m):
                    if (r >> i) & 1:
                        out[i] = out[i] ^ prod[k]
        return out

    def evaluate(self, e, assignment):
        """Concrete field element from a bit assignment (int bitmask of true variables)."""
        v = 0
        for i in range(self.m):
            bit = 0
            for t in e[i]:
                if t & assignment == t:
                    bit ^= 1
            v |= bit << i
        return v


# ------------------------------------------------------------------ systems

def randomSubspace(f, ell, rng, exclude=()):
    """ell independent elements; the span avoids 0 only trivially."""
    while True:
        basis = [rng.randrange(1, 1 << f.m) for _ in range(ell)]
        span = {0}
        for v in basis:
            span |= {s ^ v for s in span}
        if len(span) == 1 << ell and not any(x in span for x in exclude):
            return basis, span


def spanOf(basis):
    span = {0}
    for v in basis:
        span |= {s ^ v for s in span}
    return span


def buildSystem(arm, f, basis, target):
    """Return (polys, nVars, layout) for the given arm."""
    m = f.m
    ell = len(basis)
    S = SymField(f)
    xr, yr = target
    layout = {'u': (0, 3 * ell)}
    nxt = 3 * ell
    xs = [S.linear(basis, i * ell) for i in range(3)]
    polys = []
    if arm == 'incidence':
        ys = []
        for i in range(3):
            ys.append(S.fromBits(nxt))
            layout['y%d' % i] = (nxt, nxt + m)
            nxt += m
        a = S.fromBits(nxt); layout['a'] = (nxt, nxt + m); nxt += m
        b = S.fromBits(nxt); layout['b'] = (nxt, nxt + m); nxt += m
        ws = {}
        for (i, j) in ((0, 1), (0, 2), (1, 2)):
            ws[(i, j)] = S.fromBits(nxt)
            layout['w%d%d' % (i, j)] = (nxt, nxt + m)
            nxt += m
        # c = xr^2 + a xr + b (xr + yr)   (so f(-R) = 0, -R = (xr, xr + yr))
        c = S.add(S.add(S.const(f.sqr(xr)), S.mul(a, S.const(xr))), S.mul(b, S.const(xr ^ yr)))
        one = S.const(1)
        for i in range(3):
            x, y = xs[i], ys[i]
            curve = S.add(S.add(S.sqr(y), S.mul(x, y)), S.add(S.mul(S.sqr(x), x), one))
            inc = S.add(S.add(S.sqr(x), S.mul(a, x)), S.add(c, S.mul(b, y)))
            polys += curve + inc
        for (i, j), w in ws.items():
            polys += S.add(S.mul(S.add(xs[i], xs[j]), w), one)
    elif arm == 'norm':
        a = S.fromBits(nxt); layout['a'] = (nxt, nxt + m); nxt += m
        b = S.fromBits(nxt); layout['b'] = (nxt, nxt + m); nxt += m
        c = S.add(S.add(S.const(f.sqr(xr)), S.mul(a, S.const(xr))), S.mul(b, S.const(xr ^ yr)))
        # (X+x0)(X+x1)(X+x2)(X+xr) elementary symmetric functions
        u = S.add(xs[0], xs[1])
        v = S.mul(xs[0], xs[1])
        w = S.add(xs[2], S.const(xr))
        z = S.mul(xs[2], S.const(xr))
        e1 = S.add(u, w)
        e2 = S.add(S.add(v, z), S.mul(u, w))
        e3 = S.add(S.mul(u, z), S.mul(v, w))
        e4 = S.mul(v, z)
        # N(f) = A^2 + b X A + b^2 (X^3 + 1), A = X^2 + a X + c :
        #   X^4 + (b + b^2) X^3 + (a^2 + a b) X^2 + b c X + (c^2 + b^2)
        n1 = S.add(b, S.sqr(b))
        n2 = S.add(S.sqr(a), S.mul(a, b))
        n3 = S.mul(b, c)
        n4 = S.add(S.sqr(c), S.sqr(b))
        polys += S.add(e1, n1) + S.add(e2, n2) + S.add(e3, n3) + S.add(e4, n4)
    else:
        raise ValueError(arm)
    polys = [p for p in polys if p]
    return polys, nxt, layout


def decodeField(f, assignment, rng_):
    lo, hi = rng_
    v = 0
    for i in range(hi - lo):
        if (assignment >> (lo + i)) & 1:
            v |= 1 << i
    return v


def decodeSolution(arm, f, basis, target, layout, assignment, curve):
    """Independent check of a Boolean solution. Returns dict or None."""
    ell = len(basis)
    xs = []
    for i in range(3):
        x = 0
        for k in range(ell):
            if (assignment >> (i * ell + k)) & 1:
                x ^= basis[k]
        xs.append(x)
    if arm == 'incidence':
        pts = [(xs[i], decodeField(f, assignment, layout['y%d' % i])) for i in range(3)]
        if not all(curve.onCurve(p) for p in pts):
            return None
        s = curve.add(curve.add(pts[0], pts[1]), pts[2])
        return {'xs': xs, 'points': pts, 'sum_ok': s == target}
    # norm: x's determine the points up to sign; some sign pattern must sum to R
    cands = []
    for x in xs:
        p = curve.pointFromX(x)
        if p is None:
            return {'xs': xs, 'sum_ok': False, 'reason': 'abscissa off curve'}
        cands.append(p)
    for signs in range(8):
        pts = [cands[i] if not (signs >> i) & 1 else curve.neg(cands[i]) for i in range(3)]
        if curve.add(curve.add(pts[0], pts[1]), pts[2]) == target:
            return {'xs': xs, 'points': pts, 'sum_ok': True}
    return {'xs': xs, 'sum_ok': False}


def countDecompositions(curve, span, target):
    """Ordered triples of affine factor-base points (x in span \\ {0}) summing to target."""
    pts = []
    for x in span:
        if x == 0:
            continue
        p = curve.pointFromX(x)
        if p is not None:
            pts.append(p)
            pts.append(curve.neg(p))
    byPoint = {p: 1 for p in pts}
    n = 0
    for p in pts:
        for q in pts:
            s = curve.add(p, q)
            need = curve.add(target, curve.neg(s))
            if need is not None and need in byPoint:
                n += 1
    return n, len(pts)


def exportJson(polys, nVars):
    return json.dumps({'n_vars': nVars, 'polys': [
        [[i for i in range(nVars) if (t >> i) & 1] for t in sorted(p)] for p in polys]})


# ---------------------------------------------------------------- self-test

def selftest(m=7, ell=3, seed=1):
    rng = random.Random(seed)
    f = Gf2m(m)
    S = SymField(f)
    # symbolic arithmetic against the concrete field
    for _ in range(50):
        a = S.fromBits(0)
        b = S.fromBits(m)
        asg = rng.randrange(1 << (2 * m))
        av, bv = asg & ((1 << m) - 1), asg >> m
        assert S.evaluate(S.mul(a, b), asg) == f.mul(av, bv)
        assert S.evaluate(S.sqr(a), asg) == f.sqr(av)
        assert S.evaluate(S.add(a, b), asg) == av ^ bv
    curve = Curve(f)
    order = curve.order()
    # order check by brute force at small m
    cnt = 1 + sum(2 for x in range(1, 1 << m) if curve.pointFromX(x) is not None) + 1
    assert cnt == order, (cnt, order)
    # planted instance: every arm must be satisfied by the plant and its witnesses
    for arm in ('incidence', 'norm'):
        for trial in range(5):
            basis, span = randomSubspace(f, ell, rng)
            pts = []
            for x in sorted(span - {0}):
                p = curve.pointFromX(x)
                if p is not None:
                    pts.append(p if rng.random() < 0.5 else curve.neg(p))
            if len(pts) < 3:
                continue
            pts = rng.sample(pts, 3)
            target = curve.add(curve.add(pts[0], pts[1]), pts[2])
            if target is None or target[0] in span:
                continue
            polys, nVars, layout = buildSystem(arm, f, basis, target)
            # assignment from the plant
            asg = 0
            for i, p in enumerate(pts):
                coords = _coords(basis, p[0])
                for k in range(ell):
                    if (coords >> k) & 1:
                        asg |= 1 << (i * ell + k)
            # a, b from the two incidence conditions f(P1) = f(P2) = 0 minus f(P3)... solve linear 2x2
            ab = _solveAB(f, pts, target)
            assert ab is not None
            a, b = ab
            asg |= a << layout['a'][0]
            asg |= b << layout['b'][0]
            if arm == 'incidence':
                for i, p in enumerate(pts):
                    asg |= p[1] << layout['y%d' % i][0]
                for (i, j) in ((0, 1), (0, 2), (1, 2)):
                    asg |= f.inv(pts[i][0] ^ pts[j][0]) << layout['w%d%d' % (i, j)][0]
            bad = [p for p in polys if _evalPoly(p, asg)]
            assert not bad, (arm, len(bad))
            dec = decodeSolution(arm, f, basis, target, layout, asg, curve)
            assert dec and dec['sum_ok']
    print('selftest ok: m=%d ell=%d order=%d' % (m, ell, order))


def _coords(basis, x):
    # solve x = sum u_k basis_k over GF(2) by brute force (small ell)
    ell = len(basis)
    for u in range(1 << ell):
        s = 0
        for k in range(ell):
            if (u >> k) & 1:
                s ^= basis[k]
        if s == x:
            return u
    raise ValueError('x not in span')


def _solveAB(f, pts, target):
    """a, b with f(P_i) = 0 for i = 1, 2 given c pinned by f(-R) = 0; check P_3."""
    xr, yr = target
    # f(P) = x^2 + a x + c + b y, c = xr^2 + a xr + b (xr + yr)
    #      = (x^2 + xr^2) + a (x + xr) + b (y + xr + yr)
    rows = []
    for (x, y) in pts:
        rows.append((x ^ xr, y ^ xr ^ yr, f.sqr(x) ^ f.sqr(xr)))
    (a1, b1, r1), (a2, b2, r2) = rows[0], rows[1]
    det = f.add(f.mul(a1, b2), f.mul(a2, b1))
    if det == 0:
        return None
    di = f.inv(det)
    a = f.mul(f.add(f.mul(r1, b2), f.mul(r2, b1)), di)
    b = f.mul(f.add(f.mul(a1, r2), f.mul(a2, r1)), di)
    for (ax, bx, rx) in rows:
        if f.add(f.add(f.mul(a, ax), f.mul(b, bx)), rx) != 0:
            return None
    return a, b


def _evalPoly(p, asg):
    v = 0
    for t in p:
        if t & asg == t:
            v ^= 1
    return v


if __name__ == '__main__':
    import sys
    m = int(sys.argv[1]) if len(sys.argv) > 1 else 7
    selftest(m=m, ell=max(2, (m + 2) // 3))
