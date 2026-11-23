"""Lead B: dual-rail gates on the repository's ECC2K-130 S'4 SAT instance.

Nothing in the repository is modified.  The repository generator
(research/nagao_relations/solver_16/weil.py: buildS4, System.cmsClauses) is
imported and run unchanged; this module adds

  railify()      a CNF+XOR post-pass that recognises the Tseitin AND gates in
                 the generator's output and adds the complementary rails
  buildLinearLeaf()  the chained "linear leaf" system (S3(x1,xR,u) = 0 and
                 S3(x2,x3,u) = 0, the second affine in (s,p) once u is known),
                 built with the repository's own Descent/Monomials/System
  CMS            pycryptosat wrapper reading the per-call conflict count from
                 the solver's verbose line (same method as followup-subcube)
  FastField / pairDecision   an independent F_2^131 arithmetic and the pair
                 enumeration decision used as ground truth

Every number produced with this is a pilot assertion, [pcc:unverified].
"""
import ctypes
import os
import re
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
sys.path.insert(0, str(ROOT / 'research/nagao_relations/solver_16'))
import weil      # noqa: E402  (also puts ecc2k130/codegen on sys.path)
import curves    # noqa: E402
import field     # noqa: E402

N = 131


def repoField():
    poly, terms = curves.findIrreduciblePoly(N)
    return field.Pb(N, poly), poly


# ------------------------------------------------------- independent field

class FastField:
    """Own GF(2^131) arithmetic (4-bit window multiply, Euclid inverse)."""

    def __init__(self, n, poly):
        self.n = n
        self.poly = poly

    def red(self, x):
        n, poly = self.n, self.poly
        bl = x.bit_length()
        while bl > n:
            x ^= poly << (bl - 1 - n)
            bl = x.bit_length()
        return x

    def mul(self, a, b):
        tab = [0, a, a << 1, a ^ (a << 1)]
        tab += [t ^ (a << 2) for t in tab]
        tab += [t ^ (a << 3) for t in tab]
        r = 0
        sh = 0
        while b:
            r ^= tab[b & 15] << sh
            b >>= 4
            sh += 4
        return self.red(r)

    def inv(self, a):
        # extended Euclid on polynomials
        u, v = a, self.poly
        g1, g2 = 1, 0
        while u != 1:
            j = u.bit_length() - v.bit_length()
            if j < 0:
                u, v = v, u
                g1, g2 = g2, g1
                j = -j
            u ^= v << j
            g1 ^= g2 << j
        return self.red(g1)

    def tr(self, a):
        t = a
        acc = a
        for _ in range(self.n - 1):
            t = self.mul(t, t)
            acc ^= t
        return acc

    def htr(self, a):
        t = a
        acc = a
        for _ in range((self.n - 1) // 2):
            t = self.mul(t, t)
            t = self.mul(t, t)
            acc ^= t
        return acc


class FastCurve:
    """y^2 + xy = x^3 + 1."""

    def __init__(self, F):
        self.F = F

    def on(self, P):
        F = self.F
        x, y = P
        return F.mul(y, y) ^ F.mul(x, y) == F.mul(F.mul(x, x), x) ^ 1

    def lift(self, x):
        """A point with abscissa x != 0, or None.  y = x z, z^2+z = x + 1/x^2."""
        F = self.F
        if x == 0:
            return None
        ix = F.inv(x)
        c = x ^ F.mul(ix, ix)
        if F.tr(c):
            return None
        z = F.htr(c)
        P = (x, F.mul(x, z))
        assert self.on(P)
        return P

    @staticmethod
    def neg(P):
        return (P[0], P[0] ^ P[1])

    def add(self, P, Q):
        F = self.F
        if P is None:
            return Q
        if Q is None:
            return P
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if y1 != y2 or x1 == 0:
                return None
            lam = x1 ^ F.mul(y1, F.inv(x1))
            x3 = F.mul(lam, lam) ^ lam
            return (x3, F.mul(x1, x1) ^ F.mul(lam ^ 1, x3))
        dx = x1 ^ x2
        lam = F.mul(y1 ^ y2, F.inv(dx))
        x3 = F.mul(lam, lam) ^ lam ^ dx
        return (x3, F.mul(lam, x1 ^ x3) ^ x3 ^ y1)


def liftsToTarget(C, xs, R):
    """Own-arithmetic check: abscissae lift and some signs sum to R exactly."""
    pts = [C.lift(x) for x in xs]
    if any(p is None for p in pts):
        return False
    for m in range(8):
        acc = None
        for i in range(3):
            acc = C.add(acc, C.neg(pts[i]) if (m >> i) & 1 else pts[i])
        if acc == R:
            return True
    return False


def pairDecision(C, d, R, basePts):
    """Pair enumeration: all unordered pairs of factor-base abscissae and all
    signs; is x(R - s1 P_i - s2 P_j) a nonzero element of V that lifts?
    Returns the sorted list of abscissa triples found (empty = non-decomposable)."""
    F = C.F
    xsSet = set(p[0] for p in basePts)
    found = set()
    Q = []
    for p in basePts:
        Q.append((C.add(R, C.neg(p)), C.add(R, p)))
    A = len(basePts)
    for i in range(A):
        for q in Q[i]:
            if q is None:
                continue
            x1, y1 = q
            for j in range(i + 1, A):
                x2, y2 = basePts[j]
                dx = x1 ^ x2
                if dx == 0:
                    continue
                idx = F.inv(dx)
                for yy in (y2, y2 ^ x2):
                    lam = F.mul(y1 ^ yy, idx)
                    x3 = F.mul(lam, lam) ^ lam ^ dx
                    if x3 >> d == 0 and x3 in xsSet:
                        found.add(tuple(sorted((basePts[i][0], x2, x3))))
    return sorted(found)


# ---------------------------------------------------------- the rail pass

def findGates(clauses):
    """AND gates in a Tseitin CNF: a clause [v, -a, -b, ..] together with the
    binaries [-v, a], [-v, b], ..  Returns {v: (a, b, ..)} (inputs sorted)."""
    binary = set()
    for c in clauses:
        if len(c) == 2 and c[0] < 0 and c[1] > 0:
            binary.add((-c[0], c[1]))
    gates = {}
    for c in clauses:
        if len(c) >= 3 and c[0] > 0 and all(l < 0 for l in c[1:]):
            v = c[0]
            ins = tuple(sorted(-l for l in c[1:]))
            if all((v, a) in binary for a in ins):
                gates[v] = ins
    return gates


def railify(nv, clauses, xors, selectors=None):
    """Dual-rail post-pass on a CNF+XOR instance.

    For every AND gate t = a & rest (rest = the AND of the other inputs) and
    every input a (or only inputs in `selectors`), add r = (not a) & rest and
    the XOR clause t + r + rest = 0.  Whichever way a is set, one rail is a
    unit and the XOR ties the other to `rest` inside the Gauss-Jordan matrix.
    For a 3-input gate `rest` is the 2-input gate over the other two inputs,
    reused when the instance has it and created (plain) otherwise.
    Returns (nv, clauses, xors, info); the input lists are not modified."""
    gates = findGates(clauses)
    byKey = {}
    for v, ins in gates.items():
        byKey[ins] = v
    clauses = [list(c) for c in clauses]
    xors = [(list(l), r) for l, r in xors]
    created = 0
    rails = 0
    for t in sorted(gates):
        ins = gates[t]
        for a in ins:
            if selectors is not None and a not in selectors:
                continue
            rest = tuple(x for x in ins if x != a)
            if len(rest) == 1:
                rv = rest[0]
            else:
                rv = byKey.get(rest)
                if rv is None:
                    nv += 1
                    rv = nv
                    byKey[rest] = rv
                    created += 1
                    for u in rest:
                        clauses.append([-rv, u])
                    clauses.append([rv] + [-u for u in rest])
            nv += 1
            r = nv
            rails += 1
            clauses.append([-r, -a])
            clauses.append([-r, rv])
            clauses.append([r, a, -rv])
            xors.append(([t, r, rv], False))
    return nv, clauses, xors, {'gates': len(gates), 'rails': rails, 'created_rest_gates': created}


# ------------------------------------------------- the linear-leaf system

def buildLinearLeaf(pb, d, xR):
    """Chained form, built with the repository's Descent/Monomials/System.

    Variables: x1, x2, x3 (3d bits, same positions as buildS4), s = x2 + x3
    (d bits), p = x2 x3 unreduced (2d - 1 bits), u (n bits).
      S3(x1, xR, u) = (x1 xR + (x1 + xR) u)^2 + xR x1 u + 1 = 0
      S3(x2, x3, u) = p^2 + (u s)^2 + u p + 1 = 0
    With u known the second is F_2-affine in the bits of (s, p)."""
    m = pb.m
    nx = 3 * d
    sV = list(range(nx + 1, nx + d + 1))
    pV = list(range(nx + d + 1, nx + d + 2 * d))
    uV = list(range(nx + 3 * d, nx + 3 * d + m))
    nvars = nx + 3 * d - 1 + m
    assert uV[-1] == nvars
    mono = weil.Monomials(nvars)
    D = weil.Descent(pb, mono)
    x = [D.fromVars(range(1 + blk * d, 1 + (blk + 1) * d)) for blk in range(3)]
    eqs = []
    for k in range(d):
        eqs.append(weil.varPoly(sV[k]) ^ x[1][k] ^ x[2][k])
    p23 = D.mulUnreduced(x[1], x[2], d, d)
    for k in range(2 * d - 1):
        eqs.append(weil.varPoly(pV[k]) ^ p23[k])
    s = D.fromVars(sV)
    p = D.fromCoefficients([weil.varPoly(t) for t in pV])
    u = D.fromVars(uV)
    x1u = D.mul(x[0], u)
    inner = D.add(D.add(D.mulConst(x[0], xR), x1u), D.mulConst(u, xR))
    eA = D.addConst(D.add(D.square(inner), D.mulConst(x1u, xR)), 1)
    us = D.mul(u, s)
    up = D.mul(u, p)
    eB = D.addConst(D.add(D.add(D.square(p), D.square(us)), up), 1)
    eqs.extend(eA)
    eqs.extend(eB)
    layout = {'x_blocks': [[1 + blk * d, (blk + 1) * d] for blk in range(3)],
              's': [sV[0], sV[-1]], 'p': [pV[0], pV[-1]], 'u': [uV[0], uV[-1]]}
    sysm = weil.System('ll', nvars, eqs, mono, layout)
    sysm.descent = D
    return sysm


# ---------------------------------------------------------------- solver

_CAP = None
_LIBC = ctypes.CDLL(None)
SOLVE_RE = re.compile(r"c \[solve\] \d+ ret: (\S+) confl: (\d+)")


def captureStdout():
    """Send fd 1 (CMS verbose lines) to a temp file; Python prints go to stderr."""
    global _CAP
    if _CAP is None:
        _CAP = tempfile.TemporaryFile(mode='w+b')
        sys.stdout.flush()
        os.dup2(_CAP.fileno(), 1)
        sys.stdout = sys.stderr


def _capPos():
    _LIBC.fflush(None)
    return os.lseek(_CAP.fileno(), 0, os.SEEK_END)


def _capRead(pos):
    end = _capPos()
    data = os.pread(_CAP.fileno(), end - pos, pos).decode('latin1')
    if end > (1 << 26):
        os.ftruncate(_CAP.fileno(), 0)
        os.lseek(_CAP.fileno(), 0, os.SEEK_SET)
    return data


class CMS:
    def __init__(self, rendered, opts=None, timeLimit=None):
        import pycryptosat
        captureStdout()
        nv, clauses, xors = rendered[:3]
        kw = {'verbose': 1, 'threads': 1}
        if opts:
            kw['options'] = dict(opts)
        if timeLimit is not None:
            kw['time_limit'] = float(timeLimit)
        self.s = pycryptosat.Solver(**kw)
        self.s.add_clauses(clauses)
        for vs, rhs in xors:
            if vs:
                self.s.add_xor_clause(vs, rhs)
            elif rhs:
                self.s.add_clause([])
        self.lastOut = ''

    def solve(self, assumptions=(), conflLimit=None):
        """-> (True/False/None, model, conflicts spent in this call)."""
        pos = _capPos()
        kw = {}
        if conflLimit is not None:
            kw['confl_limit'] = int(conflLimit)
        sat, model = self.s.solve(list(assumptions), **kw)
        data = _capRead(pos)
        self.lastOut = data
        m = SOLVE_RE.findall(data)
        assert m, data[-400:]
        return sat, model, int(m[-1][1])


def modelXs(model, d):
    xs = []
    for blk in range(3):
        v = 0
        for k in range(d):
            if model[1 + blk * d + k]:
                v |= 1 << k
        xs.append(v)
    return xs
