"""Deterministic planted Semaev decomposition systems over GF(2^n), Weil-descended.

Generates the Boolean (GF(2), x_i^2 = x_i) equations whose zeros include a
planted decomposition R = P_1 + ... + P_m of a target point into factor-base
points, on the binary curve E: y^2 + xy = x^3 + a x^2 + b over GF(2^n). The
factor base is every point whose abscissa lies in the polynomial-basis subspace
V = <1, z, ..., z^(l-1)> (as in src/cryptanalysis/ic_corpus.rs and Trimoska's
benchmarks).

Variants (variable order x0 > x1 > ... is the frozen manifest order):

``plain``            Unsymmetrised chained S3, any m >= 3 (m*l + (m-2)*n vars):
                     S3(X1,X2,U1), S3(U1,X3,U2), ..., S3(U_{m-2},X_m,x_R). Cubic.
``symmetrised``      m = 3 only, b = 1: x-vars (3l) plus e-vars (6l-3) with the
                     correspondence e_k = sigma_k(X1,X2,X3) (degree <= 3) and the
                     n descended symmetrised S4 equations (quadratic in e).
``symmetrised-e``    m = 3 only, b = 1: the e-only quadratic S4 system (6l-3 vars);
                     abscissae are recovered as the factor-base roots of
                     T^3 + e1 T^2 + e2 T + e3.

There is no binary symmetrised S5 in this repository; ``symmetrised`` with m = 4
raises NotImplementedError rather than improvising one.

S3 is the formula of src/cryptanalysis/binary_semaev.rs and the symmetrised S4
(b = 1) is the twelve-term f3 of src/cryptanalysis/binary_semaev_s4.rs. The unit
tests check both on actual curve points, and every generated instance is
checked to vanish at its planted assignment before it is returned.

Randomness is a SHA-256 counter stream keyed by (generator version, m, n, l, a,
b, variant-independent seed, index), so a planted decomposition is the same for
every variant of one (m, n, l, a, b, seed, index) cell.
"""

import argparse
import hashlib
import itertools
import json
from pathlib import Path
import sys

GENERATOR = "groebner_compare.semaev/v1"
VARIANTS = ("plain", "symmetrised", "symmetrised-e")


# ---------------------------------------------------------------- GF(2)[z]

def clmul(a, b):
    result = 0
    while b:
        if b & 1:
            result ^= a
        a <<= 1
        b >>= 1
    return result


def polymod(a, f):
    degree = f.bit_length() - 1
    while a.bit_length() - 1 >= degree:
        a ^= f << (a.bit_length() - 1 - degree)
    return a


def polygcd(a, b):
    while b:
        a, b = b, polymod(a, b)
    return a


def is_irreducible(f):
    """Ben-Or: gcd(z^(2^i) - z, f) = 1 for every i <= deg(f)/2."""
    n = f.bit_length() - 1
    if n < 1 or not f & 1:
        return n == 1
    power = 2  # z
    for _ in range(n // 2):
        power = polymod(clmul(power, power), f)
        if polygcd(power ^ 2, f) != 1:
            return False
    return True


def irreducible(n):
    """Same search order as koblitz_index_calculus::find_irreducible_sparse."""
    if not 2 <= n <= 127:
        raise ValueError("field degree must lie in 2..127")
    lows = {1}
    for i in range(1, n):
        lows.add(1 | 1 << i)
        for j in range(i + 1, n):
            lows.add(1 | 1 << i | 1 << j)
            for k in range(j + 1, n):
                lows.add(1 | 1 << i | 1 << j | 1 << k)
    for low in sorted(lows):
        if is_irreducible(1 << n | low):
            return 1 << n | low
    raise ValueError("no sparse irreducible found")


class Field:
    def __init__(self, n, modulus=None):
        self.n = n
        self.modulus = modulus if modulus is not None else irreducible(n)
        if self.modulus.bit_length() - 1 != n or not is_irreducible(self.modulus):
            raise ValueError("modulus must be irreducible of degree n")
        self.low = [t for t in range(n) if self.modulus >> t & 1]

    def mul(self, a, b):
        return polymod(clmul(a, b), self.modulus)

    def sq(self, a):
        return self.mul(a, a)

    def pow(self, a, e):
        result = 1
        while e:
            if e & 1:
                result = self.mul(result, a)
            a = self.sq(a)
            e >>= 1
        return result

    def inv(self, a):
        if not a:
            raise ZeroDivisionError("inverse of zero")
        return self.pow(a, (1 << self.n) - 2)

    def trace(self, a):
        t, s = 0, a
        for _ in range(self.n):
            t ^= s
            s = self.sq(s)
        return t

    def solve_artin_schreier(self, t):
        """Some w with w^2 + w = t, or None. Plain GF(2) linear algebra."""
        pivots = {}
        for i in range(self.n):
            image = self.sq(1 << i) ^ (1 << i)
            combination = 1 << i
            while image:
                top = image.bit_length() - 1
                if top not in pivots:
                    pivots[top] = (image, combination)
                    break
                image ^= pivots[top][0]
                combination ^= pivots[top][1]
        w = 0
        while t:
            top = t.bit_length() - 1
            if top not in pivots:
                return None
            t ^= pivots[top][0]
            w ^= pivots[top][1]
        return w


# ---------------------------------------------------------------- curve

class Curve:
    """y^2 + xy = x^3 + a x^2 + b over GF(2^n); None is the point at infinity."""

    def __init__(self, field, a, b):
        if not b:
            raise ValueError("b = 0 is singular")
        self.F, self.a, self.b = field, a, b

    def on_curve(self, point):
        if point is None:
            return True
        F, (x, y) = self.F, point
        return (F.sq(y) ^ F.mul(x, y)) == (F.mul(F.sq(x), x) ^ F.mul(self.a, F.sq(x)) ^ self.b)

    def neg(self, point):
        return None if point is None else (point[0], point[0] ^ point[1])

    def add(self, p, q):
        if p is None:
            return q
        if q is None:
            return p
        F = self.F
        (x1, y1), (x2, y2) = p, q
        if x1 == x2:
            if y2 == x1 ^ y1:
                return None
            if not x1:
                return None
            lam = x1 ^ F.mul(y1, F.inv(x1))
            x3 = F.sq(lam) ^ lam ^ self.a
            return (x3, F.sq(x1) ^ F.mul(lam ^ 1, x3))
        lam = F.mul(y1 ^ y2, F.inv(x1 ^ x2))
        x3 = F.sq(lam) ^ lam ^ x1 ^ x2 ^ self.a
        return (x3, F.mul(lam, x1 ^ x3) ^ x3 ^ y1)

    def lift(self, x):
        """All points with abscissa x (0, 1 or 2 of them)."""
        F = self.F
        if not x:
            return [(0, F.pow(self.b, 1 << (F.n - 1)))]
        c = F.mul(F.sq(x), x) ^ F.mul(self.a, F.sq(x)) ^ self.b
        w = F.solve_artin_schreier(F.mul(c, F.inv(F.sq(x))))
        if w is None:
            return []
        y = F.mul(x, w)
        return [(x, y), (x, y ^ x)]

    def order(self, limit_bits=22):
        """Point count by exhaustive trace test; None above 2^limit_bits."""
        F = self.F
        if F.n > limit_bits:
            return None
        count = 2  # infinity and the x = 0 point
        for x in range(1, 1 << F.n):
            c = F.mul(F.sq(x), x) ^ F.mul(self.a, F.sq(x)) ^ self.b
            if F.trace(F.mul(c, F.inv(F.sq(x)))) == 0:
                count += 2
        return count


def s3_value(F, x1, x2, x3, b):
    """S3 = (x1+x2)^2 x3^2 + x1 x2 x3 + (x1 x2)^2 + b over GF(2^n)."""
    p = F.mul(x1, x2)
    return F.mul(F.sq(x1 ^ x2), F.sq(x3)) ^ F.mul(p, x3) ^ F.sq(p) ^ b


def symmetric3(F, xs):
    x1, x2, x3 = xs
    return (x1 ^ x2 ^ x3, F.mul(x1, x2) ^ F.mul(x1, x3) ^ F.mul(x2, x3),
            F.mul(F.mul(x1, x2), x3))


def s4_symmetrised_value(F, e1, e2, e3, xr):
    """Twelve-term symmetrised S4 for b = 1 (binary_semaev_s4.rs)."""
    sq, mul = F.sq, F.mul
    e12, e22, e32 = sq(e1), sq(e2), sq(e3)
    xr2 = sq(xr)
    xr3 = mul(xr2, xr)
    xr4 = sq(xr2)
    return (xr4 ^ sq(e12) ^ sq(e32) ^ mul(sq(e22), xr4) ^ mul(mul(e32, e3), xr)
            ^ mul(mul(e3, e22), xr3) ^ mul(mul(e3, e12), xr) ^ mul(e3, xr3)
            ^ mul(mul(e12, e32), xr2) ^ mul(e32, xr4) ^ e32 ^ mul(e22, xr2))


# ------------------------------------------------ symbolic Weil descent (ANF)
# A symbolic field element is a list of n Boolean polynomials (sets of
# monomial masks, bit i = x_i, mask 0 = 1): the polynomial-basis coordinates.

def bmul(p, q):
    result = set()
    for a in p:
        for b in q:
            result ^= {a | b}
    return result


class Symbolic:
    def __init__(self, field):
        self.F = field
        self.n = field.n

    def const(self, c):
        return [{0} if c >> d & 1 else set() for d in range(self.n)]

    def variables(self, offset, length):
        if length > self.n:
            raise ValueError("symbolic element longer than the field degree")
        return [{1 << (offset + d)} if d < length else set() for d in range(self.n)]

    def add(self, *items):
        result = [set() for _ in range(self.n)]
        for item in items:
            for d in range(self.n):
                result[d] ^= item[d]
        return result

    def reduce(self, wide):
        for k in range(len(wide) - 1, self.n - 1, -1):
            if wide[k]:
                for t in self.F.low:
                    wide[k - self.n + t] ^= wide[k]
                wide[k] = set()
        return wide[:self.n]

    def mul(self, left, right):
        wide = [set() for _ in range(2 * self.n - 1)]
        for i, p in enumerate(left):
            if p:
                for j, q in enumerate(right):
                    if q:
                        wide[i + j] ^= bmul(p, q)
        return self.reduce(wide)

    def square(self, element):
        # Frobenius is relocation: c^2 = c for Boolean c, no cross terms.
        wide = [set() for _ in range(2 * self.n - 1)]
        for d, p in enumerate(element):
            wide[2 * d] = set(p)
        return self.reduce(wide)

    def s3(self, x1, x2, x3, b):
        p = self.mul(x1, x2)
        return self.add(self.mul(self.square(self.add(x1, x2)), self.square(x3)),
                        self.mul(p, x3), self.square(p), self.const(b))

    def s4_symmetrised(self, e1, e2, e3, xr):
        F, c = self.F, self.const
        e12, e22, e32 = self.square(e1), self.square(e2), self.square(e3)
        xr2 = F.sq(xr)
        xr3, xr4 = F.mul(xr2, xr), F.sq(xr2)
        return self.add(c(xr4), self.square(e12), self.square(e32),
                        self.mul(self.square(e22), c(xr4)),
                        self.mul(self.mul(e32, e3), c(xr)),
                        self.mul(self.mul(e3, e22), c(xr3)),
                        self.mul(self.mul(e3, e12), c(xr)),
                        self.mul(e3, c(xr3)), self.mul(self.mul(e12, e32), c(xr2)),
                        self.mul(e32, c(xr4)), e32, self.mul(e22, c(xr2)))


# ---------------------------------------------------------------- randomness

class Stream:
    """Deterministic SHA-256 counter stream (independent of Python's random)."""

    def __init__(self, *key):
        self.key = json.dumps(key, sort_keys=True, separators=(",", ":")).encode()
        self.counter = 0

    def below(self, bound):
        self.counter += 1
        raw = hashlib.sha256(self.key + self.counter.to_bytes(8, "big")).digest()
        return int.from_bytes(raw, "big") % bound


# ---------------------------------------------------------------- generator

def e_lengths(l):
    return (l, 2 * l - 1, 3 * l - 2)


def layout(variant, m, n, l):
    """Variable offsets and total count; also validates the cell."""
    if variant not in VARIANTS:
        raise ValueError(f"variant must be one of {VARIANTS}")
    if m < 3:
        raise ValueError("m must be >= 3")
    if not 1 <= l < n:
        raise ValueError("require 1 <= l < n")
    if variant == "plain":
        return {"x": [i * l for i in range(m)], "u": [m * l + k * n for k in range(m - 2)],
                "nvars": m * l + (m - 2) * n}
    if m != 3:
        raise NotImplementedError("no binary symmetrised S5 (m = 4) in this repository; "
                                  "only the plain chained-S3 variant exists for m = 4")
    if 3 * l - 2 > n:
        raise ValueError("symmetrised variants need 3l - 2 <= n (unreduced e3)")
    lengths = e_lengths(l)
    base = 3 * l if variant == "symmetrised" else 0
    e = [base, base + lengths[0], base + lengths[0] + lengths[1]]
    out = {"e": e, "e_lengths": list(lengths), "nvars": base + sum(lengths)}
    if variant == "symmetrised":
        out["x"] = [0, l, 2 * l]
    return out


def setup(n, a, b, modulus=None):
    field = Field(n, modulus)
    return field, Curve(field, a, b)


def plant(curve, m, l, stream, attempts=4096):
    """m points with distinct nonzero abscissae in V; x_R nonzero and outside V."""
    candidates = [x for x in range(1, 1 << l) if curve.lift(x)]
    if len(candidates) < m:
        raise ValueError(f"only {len(candidates)} liftable factor-base abscissae; need {m}")
    for _ in range(attempts):
        pool = list(candidates)
        xs = [pool.pop(stream.below(len(pool))) for _ in range(m)]
        points = [curve.lift(x)[stream.below(2)] for x in xs]
        partial, partials = None, []
        for point in points:
            partial = curve.add(partial, point)
            partials.append(partial)
        if any(p is None for p in partials):
            continue
        target = partials[-1]
        if target[0] == 0 or target[0] < (1 << l):
            continue
        return points, partials, target
    raise ValueError("no nondegenerate planting found")


def generate(m, n, l, *, variant="plain", a=1, b=1, seed=1, index=0, modulus=None):
    """Return (instance, planted). The planted answer is NOT inside the instance."""
    shape = layout(variant, m, n, l)
    if variant != "plain" and b != 1:
        raise ValueError("the repository's symmetrised S4 is specialised to b = 1")
    field, curve = setup(n, a, b, modulus)
    stream = Stream(GENERATOR, "plant", m, n, l, a, b, field.modulus, seed, index)
    points, partials, target = plant(curve, m, l, stream)
    xr = target[0]
    sym = Symbolic(field)
    xs = [p[0] for p in points]
    planted = 0
    equations = []
    if variant == "plain":
        X = [sym.variables(o, l) for o in shape["x"]]
        U = [sym.variables(o, n) for o in shape["u"]]
        chain = X[:1] + U  # left operand of each link
        rights = X[1:]
        outs = U + [sym.const(xr)]
        for left, right, out in zip(chain, rights, outs):
            equations.extend(sym.s3(left, right, out, b))
        for o, x in zip(shape["x"], xs):
            planted |= x << o
        for o, p in zip(shape["u"], partials[1:-1]):
            planted |= p[0] << o
    else:
        E = [sym.variables(o, k) for o, k in zip(shape["e"], shape["e_lengths"])]
        equations.extend(sym.s4_symmetrised(*E, xr))
        if variant == "symmetrised":
            equations.extend(correspondence(shape, l))
            for o, x in zip(shape["x"], xs):
                planted |= x << o
        # Unreduced elementary symmetric polynomials of elements of V.
        values = unreduced_symmetric(xs)
        for o, v in zip(shape["e"], values):
            planted |= v << o
    rows = [sorted(p) for p in equations]
    nvars = shape["nvars"]
    if not all(sum((mask & planted) == mask for mask in row) % 2 == 0 for row in rows):
        raise AssertionError("generated system does not vanish at the planted assignment")
    meta = {"generator": GENERATOR, "variant": variant, "m": m, "n": n, "l": l,
            "a": a, "b": b, "seed": seed, "index": index, "modulus": field.modulus,
            "x_R": xr, "layout": shape, "curve_order": curve.order(),
            "factor_base": "abscissa in <1,z,...,z^(l-1)> (polynomial basis)",
            "equations_sha256": hashlib.sha256(json.dumps(rows, separators=(",", ":"))
                                               .encode()).hexdigest()}
    instance = {"id": f"semaev-m{m}-n{n}-l{l}-{variant}-a{a}-s{seed}-i{index}",
                "equations": rows, "semaev": meta}
    answer = {"assignment": planted, "abscissae": xs,
              "points": [list(p) for p in points], "target": list(target)}
    return instance, answer


def unreduced_symmetric(xs):
    x1, x2, x3 = xs
    return (x1 ^ x2 ^ x3, clmul(x1, x2) ^ clmul(x1, x3) ^ clmul(x2, x3),
            clmul(clmul(x1, x2), x3))


def correspondence(shape, l):
    """e_{k,d} + sigma_{k,d}(X) for the three unreduced symmetric functions."""
    x = shape["x"]
    e1, e2, e3 = shape["e"]
    bit = lambda i, p: 1 << (x[i] + p)
    rows = []
    for d in range(l):
        rows.append({1 << (e1 + d), bit(0, d), bit(1, d), bit(2, d)})
    for d in range(2 * l - 1):
        row = {1 << (e2 + d)}
        for i, j in ((0, 1), (0, 2), (1, 2)):
            for p in range(max(0, d - l + 1), min(l - 1, d) + 1):
                row ^= {bit(i, p) | bit(j, d - p)}
        rows.append(row)
    for d in range(3 * l - 2):
        row = {1 << (e3 + d)}
        for p in range(l):
            for q in range(l):
                r = d - p - q
                if 0 <= r < l:
                    row ^= {bit(0, p) | bit(1, q) | bit(2, r)}
        rows.append(row)
    return rows


# ------------------------------------------------- decoding and re-addition

def field_value(assignment, offset, length):
    return (assignment >> offset) & ((1 << length) - 1)


def decode_abscissae(meta, assignment):
    """Candidate factor-base abscissa tuples encoded by a Boolean solution."""
    shape, l = meta["layout"], meta["l"]
    if "x" in shape:
        return [[field_value(assignment, o, l) for o in shape["x"]]]
    field = Field(meta["n"], meta["modulus"])
    e = [field_value(assignment, o, k) for o, k in zip(shape["e"], shape["e_lengths"])]
    roots = [t for t in range(1 << l)
             if (field.mul(field.sq(t), t) ^ field.mul(e[0], field.sq(t))
                 ^ field.mul(e[1], t) ^ e[2]) == 0]
    found = []
    for triple in itertools.combinations_with_replacement(roots, 3):
        if list(unreduced_symmetric(triple)) == e:
            found.append(list(triple))
    return found


def readdition(meta, assignment):
    """Group re-addition: some signs give P_1 + ... + P_m = +-R, P_i in the FB.

    Returns the verified (abscissae, sign-pattern) or None. Uses only curve
    arithmetic; no Semaev polynomial is evaluated here.
    """
    field, curve = setup(meta["n"], meta["a"], meta["b"], meta["modulus"])
    xr, l = meta["x_R"], meta["l"]
    for xs in decode_abscissae(meta, assignment):
        if len(xs) != meta["m"] or any(not 0 <= x < (1 << l) for x in xs):
            continue
        lifts = [curve.lift(x) for x in xs]
        if any(not lift for lift in lifts):
            continue
        for choice in itertools.product(*lifts):
            total = None
            for point in choice:
                total = curve.add(total, point)
            if total is not None and total[0] == xr and curve.on_curve(total):
                return {"abscissae": xs, "points": [list(p) for p in choice],
                        "sum": list(total)}
    return None


# ---------------------------------------------------------------- exports

def to_msolve(equations, nvars, *, field_equations=True):
    names = [f"x{i}" for i in range(nvars)]

    def monomial(mask):
        return "*".join(names[i] for i in range(nvars) if mask >> i & 1) or "1"

    polys = ["+".join(monomial(m) for m in row) for row in equations if row]
    if field_equations:
        polys += [f"{v}^2+{v}" for v in names]
    return ",".join(names) + "\n2\n" + ",\n".join(polys) + "\n"


def to_dimacs_xor(equations, nvars):
    """CryptoMiniSat DIMACS: Tseitin AND gate per nonlinear monomial, XOR rows."""
    nonlinear = sorted({m for row in equations for m in row if m.bit_count() > 1})
    gate = {mono: nvars + i + 1 for i, mono in enumerate(nonlinear)}
    clauses = []
    for mono, aux in gate.items():
        factors = [i + 1 for i in range(nvars) if mono >> i & 1]
        clauses.extend(f"-{aux} {f} 0" for f in factors)
        clauses.append(" ".join([str(aux)] + [f"-{f}" for f in factors]) + " 0")
    xors = 0
    for row in equations:
        literals = [gate[m] if m in gate else m.bit_length() for m in row if m]
        rhs = 0 in row  # XOR(literals) must equal the constant term
        if not literals:
            if rhs:
                clauses.append("0")  # unsatisfiable empty clause
            continue
        if not rhs:
            literals[0] = -literals[0]
        clauses.append("x" + " ".join(map(str, literals)) + " 0")
        xors += 1
    header = f"p cnf {nvars + len(nonlinear)} {len(clauses)}"
    return "\n".join([header] + clauses) + "\n", {"tseitin_gates": len(nonlinear),
                                                    "xor_rows": xors}


# ---------------------------------------------------------------- manifests

def default_backends(python=None):
    py = python or sys.executable
    ext = [py, "-m", "groebner_compare.external_worker"]
    return [
        {"id": "msolve-f4", "mode": "cold", "command": ext + ["msolve"],
         "provenance": "msolve on PATH or MSOLVE_BINARY; -g 2 F4 reduced basis only "
                       "(its GF(2) parametrisation output is not used)"},
        {"id": "cryptominisat5-xor", "mode": "cold", "command": ext + ["cryptominisat5"],
         "provenance": "cryptominisat5 CLI on PATH or CRYPTOMINISAT_BINARY; native XOR rows"},
        {"id": "magma-f4", "mode": "cold", "command": ext + ["magma"],
         "provenance": "licensed Magma only; reports unavailable otherwise"},
        {"id": "groebner-julia-learn_apply", "mode": "learn_apply",
         "command": ["julia", "--project=.groebner-julia", "groebner_compare/worker.jl"],
         "provenance": "existing Groebner.jl learn/apply trace (fixed-schedule F4)"},
        {"id": "groebner-julia-warm", "mode": "warm",
         "command": ["julia", "--project=.groebner-julia", "groebner_compare/worker.jl"],
         "provenance": "existing Groebner.jl ordinary solve; warm control for learn_apply"},
        {"id": "repository-boolean-f4", "mode": "warm",
         "command": [py, "-m", "groebner_compare.research_worker", "block-f4"],
         "provenance": "groebner_compare.experiments.BooleanF4 prototype (no blocks)"},
        {"id": "sympy-reference", "mode": "cold",
         "command": [py, "-m", "groebner_compare.worker", "sympy"],
         "provenance": "SymPy groebner modulus=2; correctness reference"},
        {"id": "m5gb", "mode": "cold",
         "command": [py, "-m", "groebner_compare.research_worker", "m5gb"],
         "provenance": "upstream M5GB bridge via M5GB_BINARY"},
    ]


def build_manifest(m, n, l, *, variant="plain", a=1, b=1, seed=1, count=3,
                   backends=None, watchdog=120, python=None):
    instances, planted = [], {}
    for index in range(count):
        instance, answer = generate(m, n, l, variant=variant, a=a, b=b, seed=seed,
                                    index=index)
        instances.append(instance)
        planted[instance["id"]] = answer["assignment"]
    nvars = instances[0]["semaev"]["layout"]["nvars"]
    py = python or sys.executable
    return {"schema": 1,
            "encoding": f"{GENERATOR}:{variant}:m{m}:n{n}:l{l}:a{a}:b{b}",
            "certificate": "exhaustive" if nvars <= 12 else "witness-replay",
            "ring": {"field": "GF(2)", "quotient": "x_i^2+x_i", "order": "degrevlex",
                     "nvars": nvars, "variables": [f"x{i}" for i in range(nvars)]},
            "watchdog_seconds": watchdog, "threads": 1,
            "instances": instances,
            "planted_answers": planted,
            "witness_verifier": {"command": [py, "-m", "groebner_compare.semaev_verifier"],
                                 "provenance": "regenerate by seed, then group re-addition"},
            "backends": backends if backends is not None else default_backends(py)}


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--m", type=int, required=True)
    parser.add_argument("--n", type=int, required=True)
    parser.add_argument("--l", type=int, required=True)
    parser.add_argument("--variant", choices=VARIANTS, default="plain")
    parser.add_argument("--a", type=int, default=1)
    parser.add_argument("--b", type=int, default=1)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--count", type=int, default=3)
    parser.add_argument("--watchdog", type=float, default=120)
    parser.add_argument("--backends", help="comma-separated backend ids to keep")
    parser.add_argument("--output", type=Path, required=True, help="manifest JSON path")
    parser.add_argument("--export-dir", type=Path,
                        help="also write <id>.ms and <id>.dimacs per instance")
    args = parser.parse_args()
    manifest = build_manifest(args.m, args.n, args.l, variant=args.variant, a=args.a,
                              b=args.b, seed=args.seed, count=args.count,
                              watchdog=args.watchdog)
    if args.backends:
        keep = args.backends.split(",")
        manifest["backends"] = [b for b in manifest["backends"] if b["id"] in keep]
    if args.output.exists():
        raise SystemExit(f"refusing to overwrite {args.output}")
    args.output.write_text(json.dumps(manifest, indent=1) + "\n")
    if args.export_dir:
        args.export_dir.mkdir(parents=True, exist_ok=False)
        nvars = manifest["ring"]["nvars"]
        for instance in manifest["instances"]:
            (args.export_dir / f"{instance['id']}.ms").write_text(
                to_msolve(instance["equations"], nvars))
            (args.export_dir / f"{instance['id']}.dimacs").write_text(
                to_dimacs_xor(instance["equations"], nvars)[0])


if __name__ == "__main__":
    main()
