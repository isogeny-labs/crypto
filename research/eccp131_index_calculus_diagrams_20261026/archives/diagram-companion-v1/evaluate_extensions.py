"""Evaluate ECCp-131 index-calculus factor bases in F_(p^2) and F_(p^3).

Uses the published prime-field challenge P,Q, without assuming log_P(Q).
Validates the parameters, constructs both extension fields, verifies Frobenius
projections, tests two-/three-term Semaev relations, and probes a fixed small
factor base against actual challenge targets alpha*P+beta*Q.

Small controls reduce a,b modulo smaller primes; these are different curves,
not reductions of the challenge DLP. Their known logs generate test instances
and are used only to verify the output of relation collection/elimination.
The relation search is exhaustive table enumeration, not a Groebner solver.
No full-size discrete logarithm is computed.
"""

import argparse
import itertools
import json
import math
from pathlib import Path
import random
import time

from sympy import isprime


class PrimeField:
    def __init__(self, p):
        self.p = self.order = p
        self.degree = 1
        self.zero, self.one = 0, 1

    def c(self, value):
        return value % self.p

    def add(self, a, b):
        return (a + b) % self.p

    def sub(self, a, b):
        return (a - b) % self.p

    def neg(self, a):
        return -a % self.p

    def mul(self, a, b):
        return a * b % self.p

    def square(self, a):
        return a * a % self.p

    def inv(self, a):
        return pow(a, -1, self.p)

    def div(self, a, b):
        return a * pow(b, -1, self.p) % self.p

    def power(self, a, exponent):
        return pow(a, exponent, self.p)

    def random(self, rng):
        return rng.randrange(self.p)

    def sqrt(self, a):
        return square_root(self, a)


def trim(values):
    while values and values[-1] == 0:
        values.pop()
    return values


def poly_sub(a, b, p):
    return trim([((a[i] if i < len(a) else 0) - (b[i] if i < len(b) else 0)) % p for i in range(max(len(a), len(b)))])


def poly_mul(a, b, p):
    if not a or not b:
        return []
    result = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            result[i + j] = (result[i + j] + x * y) % p
    return trim(result)


def poly_divmod(a, b, p):
    a, b = trim(list(a)), trim(list(b))
    if not b:
        raise ZeroDivisionError
    quotient = [0] * max(0, len(a) - len(b) + 1)
    inverse = pow(b[-1], -1, p)
    while len(a) >= len(b):
        shift, coefficient = len(a) - len(b), a[-1] * inverse % p
        quotient[shift] = coefficient
        for j, value in enumerate(b):
            a[j + shift] = (a[j + shift] - coefficient * value) % p
        trim(a)
    return trim(quotient), a


class ExtensionField:
    def __init__(self, p, modulus):
        assert modulus[-1] == 1
        self.p, self.modulus = p, [x % p for x in modulus]
        self.degree = len(modulus) - 1
        self.order = p ** self.degree
        self.zero = (0,) * self.degree
        self.one = (1,) + (0,) * (self.degree - 1)
        self.w = (0, 1) + (0,) * (self.degree - 2)
        self._frobenius_basis = None

    def c(self, value):
        return (value % self.p,) + (0,) * (self.degree - 1)

    def add(self, a, b):
        return tuple((x + y) % self.p for x, y in zip(a, b))

    def sub(self, a, b):
        return tuple((x - y) % self.p for x, y in zip(a, b))

    def neg(self, a):
        return tuple(-x % self.p for x in a)

    def mul(self, a, b):
        result = [0] * (2 * self.degree - 1)
        for i, x in enumerate(a):
            for j, y in enumerate(b):
                result[i + j] += x * y
        for i in range(len(result) - 1, self.degree - 1, -1):
            coefficient = result[i] % self.p
            for j in range(self.degree):
                result[i - self.degree + j] -= coefficient * self.modulus[j]
        return tuple(x % self.p for x in result[:self.degree])

    def square(self, a):
        return self.mul(a, a)

    def inv(self, a):
        if a == self.zero:
            raise ZeroDivisionError
        r0, r1, t0, t1 = list(self.modulus), trim(list(a)), [], [1]
        while r1:
            quotient, remainder = poly_divmod(r0, r1, self.p)
            r0, r1 = r1, remainder
            t0, t1 = t1, poly_sub(t0, poly_mul(quotient, t1, self.p), self.p)
        assert len(r0) == 1
        scale = pow(r0[0], -1, self.p)
        _, result = poly_divmod([(x * scale) % self.p for x in t0], self.modulus, self.p)
        return tuple(result + [0] * (self.degree - len(result)))

    def div(self, a, b):
        return self.mul(a, self.inv(b))

    def power(self, a, exponent):
        result = self.one
        while exponent:
            if exponent & 1:
                result = self.mul(result, a)
            exponent >>= 1
            if exponent:
                a = self.square(a)
        return result

    def random(self, rng):
        return tuple(rng.randrange(self.p) for _ in range(self.degree))

    def sqrt(self, a):
        return square_root(self, a)

    def is_base(self, a):
        return all(x == 0 for x in a[1:])

    def check_irreducible(self):
        # For degrees 2 and 3, absence of a linear factor is sufficient.
        difference = trim(list(self.sub(self.power(self.w, self.p), self.w)))
        first, second = list(self.modulus), difference
        while second:
            first, second = second, poly_divmod(first, second, self.p)[1]
        if len(first) != 1:
            return False
        assert self.power(self.w, self.order) == self.w
        return True

    def frobenius(self, a):
        if self._frobenius_basis is None:
            self._frobenius_basis = [self.power(self.power(self.w, i), self.p) for i in range(self.degree)]
        return tuple(sum(a[i] * self._frobenius_basis[i][j] for i in range(self.degree)) % self.p for j in range(self.degree))


def square_root(field, value):
    if value == field.zero:
        return field.zero
    q = field.order
    if field.power(value, (q - 1) // 2) != field.one:
        return None
    if q % 4 == 3:
        root = field.power(value, (q + 1) // 4)
    else:
        odd, exponent = q - 1, 0
        while odd % 2 == 0:
            odd //= 2
            exponent += 1
        if not hasattr(field, "_non_square"):
            rng = random.Random(q + 17)
            while True:
                z = field.random(rng)
                if z != field.zero and field.power(z, (q - 1) // 2) == field.neg(field.one):
                    field._non_square = z
                    break
        c = field.power(field._non_square, odd)
        root = field.power(value, (odd + 1) // 2)
        t = field.power(value, odd)
        remaining = exponent
        while t != field.one:
            i, current = 0, t
            while current != field.one:
                current = field.square(current)
                i += 1
                assert i < remaining
            b = field.power(c, 1 << (remaining - i - 1))
            root = field.mul(root, b)
            c = field.square(b)
            t = field.mul(t, c)
            remaining = i
    assert field.square(root) == value
    return min(root, field.neg(root))


class Curve:
    def __init__(self, field, a, b):
        self.f, self.a, self.b = field, field.c(a), field.c(b)
        self.additions = 0

    def rhs(self, x):
        f = self.f
        return f.add(f.add(f.mul(f.square(x), x), f.mul(self.a, x)), self.b)

    def contains(self, point):
        return point is None or self.f.square(point[1]) == self.rhs(point[0])

    def neg(self, point):
        return None if point is None else (point[0], self.f.neg(point[1]))

    def add(self, first, second):
        self.additions += 1
        if first is None:
            return second
        if second is None:
            return first
        f, (x1, y1), (x2, y2) = self.f, first, second
        if x1 == x2:
            if f.add(y1, y2) == f.zero:
                return None
            slope = f.div(f.add(f.mul(f.c(3), f.square(x1)), self.a), f.mul(f.c(2), y1))
        else:
            slope = f.div(f.sub(y2, y1), f.sub(x2, x1))
        x3 = f.sub(f.sub(f.square(slope), x1), x2)
        y3 = f.sub(f.mul(slope, f.sub(x1, x3)), y1)
        return x3, y3

    def scale(self, scalar, point):
        if scalar < 0:
            return self.scale(-scalar, self.neg(point))
        result = None
        while scalar:
            if scalar & 1:
                result = self.add(result, point)
            scalar >>= 1
            if scalar:
                point = self.add(point, point)
        return result

    def point_from_x(self, x):
        y = self.f.sqrt(self.rhs(x))
        return None if y is None else (x, y)

    def random_point(self, rng):
        while True:
            point = self.point_from_x(self.f.random(rng))
            if point is not None:
                return point


def embed(field, point):
    return None if point is None else tuple(field.c(x) for x in point)


def conjugate_point(field, point):
    return None if point is None else tuple(field.frobenius(x) for x in point)


def s3_coefficients(curve, x, y):
    f, a, b = curve.f, curve.a, curve.b
    total, product = f.add(x, y), f.mul(x, y)
    return (
        f.square(f.sub(x, y)),
        f.neg(f.mul(f.c(2), f.add(f.mul(total, f.add(product, a)), f.mul(f.c(2), b)))),
        f.sub(f.square(f.sub(product, a)), f.mul(f.c(4), f.mul(b, total))),
    )


def semaev3(curve, x, y, z):
    f = curve.f
    a, b, c = s3_coefficients(curve, x, y)
    return f.add(f.add(f.mul(a, f.square(z)), f.mul(b, z)), c)


def semaev4(curve, x, y, z, target):
    f = curve.f
    a, b, c = s3_coefficients(curve, x, y)
    d, e, g = s3_coefficients(curve, z, target)
    first = f.sub(f.mul(a, g), f.mul(c, d))
    second = f.sub(f.mul(a, e), f.mul(b, d))
    third = f.sub(f.mul(b, g), f.mul(c, e))
    return f.sub(f.square(first), f.mul(second, third))


def make_extensions(p):
    quadratic = ExtensionField(p, [1, 0, 1])
    assert quadratic.check_irreducible()
    for constant in range(1, 100):
        cubic = ExtensionField(p, [constant, 1, 0, 1])
        if cubic.check_irreducible():
            return quadratic, cubic
    raise AssertionError("No cubic modulus found")


def json_point(point):
    def element(x):
        return hex(x) if isinstance(x, int) else [hex(y) for y in x]
    return None if point is None else [element(x) for x in point]


def nonrational_line_probe(base, ambient, n, width, targets):
    """Test x=w+t, including the traces of its points and its raw short sums.

    This line escapes the base subgroup. Its Frobenius orbits yield known
    trace relations, but do not give known logarithms of the projected points.
    """
    start, field = time.perf_counter(), ambient.f
    points, projected, trace_checks = [], [], 0
    for t in range(width):
        point = ambient.point_from_x(field.add(field.w, field.c(t)))
        if point is None:
            continue
        points.append(point)
        orbit, current, trace = [], point, None
        for _ in range(field.degree):
            orbit.append(current)
            trace = ambient.add(trace, current)
            current = conjugate_point(field, current)
        assert current == point
        assert trace is None or all(field.is_base(value) for value in trace)
        projection = ambient.scale(pow(field.degree, -1, n), trace)
        assert ambient.scale(field.degree, projection) == trace
        if projection is not None:
            base_projection = tuple(value[0] for value in projection)
            assert base.contains(base_projection)
            projected.append(base_projection)
        if trace is not None:
            polynomial = semaev3(ambient, orbit[0][0], orbit[1][0], trace[0]) if field.degree == 2 else semaev4(ambient, *[point[0] for point in orbit], trace[0])
            assert polynomial == field.zero
            trace_checks += 1
    rational_sums, zero_sums, signed_tuples, rational_targets = 0, 0, 0, set()
    for terms in itertools.combinations(points, field.degree):
        for tail in itertools.product((1, -1), repeat=field.degree-1):
            total = terms[0]
            for sign, point in zip(tail, terms[1:]):
                total = ambient.add(total, point if sign == 1 else ambient.neg(point))
            signed_tuples += 2
            if total is None:
                zero_sums += 2
            elif all(field.is_base(value) for value in total):
                rational_sums += 2
                base_total = tuple(value[0] for value in total)
                rational_targets.add(base_total)
                rational_targets.add(base.neg(base_total))
    projected_x = {point[0] for point in projected}
    return {
        "degree": field.degree, "x_line": "w+t for integer t in [0,width)",
        "width": width, "liftable_x_count": len(points),
        "projected_nonzero_points": len(projected), "distinct_projected_x": len(projected_x),
        "trace_relations_verified_by_group_law_and_Semaev": trace_checks,
        "signed_distinct_x_tuples_tested": signed_tuples,
        "nonidentity_rational_sums": rational_sums, "identity_sums": zero_sums,
        "distinct_rational_targets_reached": len(rational_targets),
        "challenge_target_hits": sum(target in rational_targets for target in targets),
        "projection_formula": "degree^(-1) * sum_{i=0}^{degree-1} Frobenius_p^i(point), modulo n",
        "sample_projected_factor_base_points": [json_point(point) for point in projected[:3]],
        "seconds": time.perf_counter()-start,
    }


def full_size(parameters, probe_width=32, probe_targets=32):
    start = time.perf_counter()
    p, a, b, n = (int(parameters[key], 16) for key in ("p", "a", "b", "n"))
    P, Q = (tuple(int(x, 16) for x in parameters[key]) for key in ("P", "Q"))
    assert isprime(p) and isprime(n) and parameters["h"] == 1
    assert (4 * a**3 + 27 * b**2) % p
    base = Curve(PrimeField(p), a, b)
    assert base.contains(P) and base.contains(Q)
    assert base.scale(n, P) is None and base.scale(n, Q) is None
    t = p + 1 - n
    twist_order = 2 * (p + 1) - n
    N2, N3 = n * twist_order, p**3 + 1 - t**3 + 3 * p * t
    trace_zero_order = N3 // n
    assert math.gcd(n, twist_order) == math.gcd(n, trace_zero_order) == 1
    assert all(pow(p, degree, n) != 1 for degree in (1, 2, 3))
    f2, f3 = make_extensions(p)
    e2, e3 = Curve(f2, a, b), Curve(f3, a, b)
    for ext_curve in (e2, e3):
        assert ext_curve.contains(embed(ext_curve.f, P)) and ext_curve.contains(embed(ext_curve.f, Q))
    rng = random.Random(20261021)

    # A genuine challenge target, with no use of its unknown discrete logarithm.
    alpha, beta = rng.randrange(1, n), rng.randrange(1, n)
    R = base.add(base.scale(alpha, P), base.scale(beta, Q))
    assert R is not None

    twist = Curve(PrimeField(p), a, -b)
    Btw = twist.random_point(rng)
    # For d=-1: standard twist X=-x_shared and y_original=i*Y_twist.
    B = (f2.c(-Btw[0]), (0, Btw[1]))
    assert e2.contains(B) and e2.scale(twist_order, B) is None
    generic2 = e2.add(embed(f2, R), B)
    conjugate = conjugate_point(f2, generic2)
    A2 = e2.scale(pow(2, -1, n), e2.add(generic2, conjugate))
    B2 = e2.scale(pow(2, -1, twist_order), e2.add(generic2, e2.neg(conjugate)))
    assert A2 == embed(f2, R) and B2 == B
    shared_B_x = -Btw[0] % p
    assert semaev3(e2, f2.c(R[0]), f2.c(shared_B_x), generic2[0]) == f2.zero

    generic3 = e3.random_point(rng)
    while all(f3.is_base(x) for x in generic3):
        generic3 = e3.random_point(rng)
    conjugate1 = conjugate_point(f3, generic3)
    conjugate2 = conjugate_point(f3, conjugate1)
    trace = e3.add(e3.add(generic3, conjugate1), conjugate2)
    assert trace is None or all(f3.is_base(x) for x in trace)
    A3 = e3.scale(pow(3, -1, n), trace)
    B3 = e3.add(generic3, e3.neg(A3))
    trace_B3 = e3.add(e3.add(B3, conjugate_point(f3, B3)), conjugate_point(f3, conjugate_point(f3, B3)))
    assert B3 is not None and trace_B3 is None
    assert not all(f3.is_base(x) for x in B3)
    assert e3.scale(trace_zero_order, B3) is None
    assert e3.scale(N3, generic3) is None

    # Construct relations at full size, but do not call these fixed-base collection.
    U = base.random_point(rng)
    V = base.add(R, base.neg(U))
    while V is None or V[0] == U[0]:
        U = base.random_point(rng)
        V = base.add(R, base.neg(U))
    assert semaev3(base, U[0], V[0], R[0]) == 0
    rational2_coefficients = semaev3(e2, f2.c(U[0]), f2.c(V[0]), f2.c(R[0]))
    assert rational2_coefficients == f2.zero
    U3, V3 = base.random_point(rng), base.random_point(rng)
    W3 = base.add(R, base.neg(base.add(U3, V3)))
    while W3 is None or len({U3[0], V3[0], W3[0]}) < 3:
        U3, V3 = base.random_point(rng), base.random_point(rng)
        W3 = base.add(R, base.neg(base.add(U3, V3)))
    rational3_coefficients = semaev4(e3, *[f3.c(point[0]) for point in (U3, V3, W3, R)])
    assert rational3_coefficients == f3.zero

    values = list(range(probe_width))
    e_points = [point for x in values if (point := base.point_from_x(x)) is not None]
    assert all(point[1] != 0 for point in e_points)  # Prime odd curve order.
    twist_x_count = probe_width - len(e_points)
    # Check the odd-degree square-class assertion in actual F_(p^3) arithmetic.
    for x in values:
        rhs = base.rhs(x)
        assert (f3.sqrt(f3.c(rhs)) is not None) == (base.f.sqrt(rhs) is not None)

    tables = {}
    table_metrics = {}
    for terms in (2, 3):
        table_start, table, signed_tuples = time.perf_counter(), set(), 0
        for points in itertools.combinations(e_points, terms):
            for signs_tail in itertools.product((1, -1), repeat=terms - 1):
                total = points[0]
                for sign, point in zip(signs_tail, points[1:]):
                    total = base.add(total, point if sign == 1 else base.neg(point))
                if total is not None:
                    table.add(total)
                    table.add(base.neg(total))
                signed_tuples += 2
        tables[terms] = table
        table_metrics[terms] = {"signed_distinct_x_tuples": signed_tuples, "distinct_target_points": len(table), "table_seconds": time.perf_counter() - table_start}
    hits, target_points = {2: 0, 3: 0}, []
    for _ in range(probe_targets):
        aa, bb = rng.randrange(1, n), rng.randrange(1, n)
        target = base.add(base.scale(aa, P), base.scale(bb, Q))
        target_points.append(target)
        for terms in (2, 3):
            hits[terms] += target in tables[terms]
    # Non-base coefficients vanish for every base target, not just for a root.
    nonroot2 = semaev3(e2, f2.c(2), f2.c(3), f2.c(R[0]))
    nonroot3 = semaev4(e3, f3.c(2), f3.c(3), f3.c(4), f3.c(R[0]))
    assert f2.is_base(nonroot2) and f3.is_base(nonroot3)
    cubic_probe = 0
    for points in itertools.islice(itertools.combinations(e_points, 3), 32):
        assert semaev4(e3, *[f3.c(point[0]) for point in points], B3[0]) != f3.zero
        cubic_probe += 1

    nonrational_probes = [nonrational_line_probe(base, ambient, n, probe_width, target_points) for ambient in (e2, e3)]

    return {
        "challenge": parameters["name"], "source": parameters["source"],
        "p": p, "n": n, "trace": t, "prime_p": True, "prime_n": True,
        "challenge_points_on_curve_and_killed_by_n": True,
        "field_size_bits": {"p": p.bit_length(), "p2": (p*p).bit_length(), "p3": (p**3).bit_length()},
        "log2_field_sizes": {"p": math.log2(p), "p2": 2*math.log2(p), "p3": 3*math.log2(p)},
        "extension_moduli_ascending": {"p2": f2.modulus, "p3": f3.modulus},
        "curve_orders": {"base": n, "twist": twist_order, "p2": N2, "p3": N3, "cubic_trace_zero": trace_zero_order},
        "gcd_n_twist_order": math.gcd(n, twist_order), "gcd_n_cubic_trace_zero_order": math.gcd(n, trace_zero_order),
        "p_power_mod_n": {str(degree): pow(p, degree, n) for degree in (1,2,3)},
        "rho_sqrt_n_log2_ignoring_constants": math.log2(n)/2,
        "generic_nonsubfield_cubic_IC_log2_p_to_4_over_3": math.log2(p)*4/3,
        "generic_cubic_IC_bound_applicable_to_this_factor_base": False,
        "full_x_factor_base_sizes_modulo_sign": {"p2": p, "p3": (n-1)//2},
        "quadratic_generic_decomposition": {"target": json_point(generic2), "base_component": json_point(A2), "twist_component": json_point(B2), "verified": True},
        "cubic_generic_decomposition": {"target": json_point(generic3), "base_component": json_point(A3), "trace_zero_component": json_point(B3), "verified": True},
        "challenge_target": {"alpha": alpha, "beta": beta, "R": json_point(R)},
        "constructed_rational_relations": {"two_term_points": [json_point(point) for point in (U,V)], "three_term_points": [json_point(point) for point in (U3,V3,W3)], "quadratic_S3_coefficients": rational2_coefficients, "cubic_S4_coefficients": rational3_coefficients},
        "fixed_factor_base_probe": {"x_interval_width": probe_width, "E_x_count": len(e_points), "twist_x_count": twist_x_count, "challenge_targets": probe_targets, "two_term_hits": hits[2], "three_term_hits": hits[3], "tables": table_metrics},
        "cubic_trace_zero_target_nonzero_S4_checks": cubic_probe,
        "nonrational_line_probes": nonrational_probes,
        "seconds": time.perf_counter() - start,
    }


class Elimination:
    def __init__(self, modulus, variables):
        self.modulus, self.variables, self.pivots = modulus, variables, {}

    def add(self, coefficients, rhs):
        p = self.modulus
        row = [x % p for x in coefficients] + [rhs % p]
        for column in range(self.variables):
            if not row[column]:
                continue
            if column in self.pivots:
                multiplier, pivot = row[column], self.pivots[column]
                row = [(x-multiplier*y) % p for x,y in zip(row,pivot)]
            else:
                inverse = pow(row[column], -1, p)
                row = [x*inverse % p for x in row]
                self.pivots[column] = row
                return row[-1] if column == self.variables-1 else None
        assert row[-1] == 0
        return None


def toy_index_calculus(parameters, p, degree):
    start = time.perf_counter()
    a, b = int(parameters["a"],16)%p, int(parameters["b"],16)%p
    curve = Curve(PrimeField(p),a,b)
    all_points = [point for x in range(p) if (point:=curve.point_from_x(x)) is not None]
    n = 1 + sum(1 if point[1]==0 else 2 for point in all_points)
    assert isprime(n)
    ext = make_extensions(p)[degree-2]
    ambient = Curve(ext,a,b)
    # Use the same P,Q and target-randomization stream for both extensions.
    rng = random.Random(20261021+p)
    P = curve.random_point(rng)
    secret = rng.randrange(1,n)
    Q = curve.scale(secret,P)
    values = list(range(min(32,p)))
    fb = [point for x in values if (point:=curve.point_from_x(x)) is not None]
    table, tuples = {}, 0
    for indices in itertools.combinations(range(len(fb)),degree):
        for tail in itertools.product((1,-1),repeat=degree-1):
            signs = (1,)+tail
            total = None
            for index, sign in zip(indices,signs):
                total = curve.add(total,fb[index] if sign==1 else curve.neg(fb[index]))
            if total is None:
                continue
            table.setdefault(total,[]).append((indices,signs))
            table.setdefault(curve.neg(total),[]).append((indices,tuple(-sign for sign in signs)))
            tuples += 2
    table_seconds = time.perf_counter()-start
    elimination = Elimination(n,len(fb)+1)
    relations, attempts, recovered = 0, 0, None
    for attempts in range(1,20001):
        alpha,beta = rng.randrange(n),rng.randrange(n)
        R = curve.add(curve.scale(alpha,P),curve.scale(beta,Q))
        if R is None:
            continue
        for indices,signs in table.get(R,[]):
            point_sum = None
            row = [0]*(len(fb)+1)
            xs=[]
            for index,sign in zip(indices,signs):
                row[index]=sign
                point=embed(ext,fb[index])
                point_sum=ambient.add(point_sum,point if sign==1 else ambient.neg(point))
                xs.append(point[0])
            assert point_sum==embed(ext,R)
            polynomial=semaev3(ambient,*xs,ext.c(R[0])) if degree==2 else semaev4(ambient,*xs,ext.c(R[0]))
            assert polynomial==ext.zero
            row[-1]=-beta
            relations+=1
            recovered=elimination.add(row,alpha)
            if recovered is not None:
                break
        if recovered is not None:
            break
    assert recovered==secret and curve.scale(recovered,P)==Q
    return {
        "p":p,"degree":degree,"a":a,"b":b,"group_order":n,
        "P":json_point(P),"Q":json_point(Q),"recovered_log":recovered,
        "factor_base_x_interval_width":len(values),"E_factor_base_x_count":len(fb),
        "twist_x_not_used":len(values)-len(fb) if degree==2 else 0,
        "relation_terms":degree,"signed_distinct_x_tuples":tuples,
        "distinct_table_targets":len(table),"attempts":attempts,"relations":relations,
        "matrix_rank":len(elimination.pivots),"table_build_seconds":table_seconds,
        "verified_in_extension_group_and_by_Semaev":True,"verified_log":True,
        "seconds":time.perf_counter()-start,
    }


def main():
    directory = Path(__file__).resolve().parent
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output",default=str(directory/"results.json"))
    parser.add_argument("--probe-width",type=int,default=32)
    parser.add_argument("--probe-targets",type=int,default=32)
    args=parser.parse_args()
    parameters=json.loads((directory/"parameters.json").read_text())
    results={"full_size":full_size(parameters,args.probe_width,args.probe_targets),"small_controls":[]}
    print(json.dumps({"full_size":{key:results["full_size"][key] for key in ("field_size_bits","gcd_n_twist_order","gcd_n_cubic_trace_zero_order","p_power_mod_n","fixed_factor_base_probe","seconds")}}),flush=True)
    for p in (571,811,1051):
        for degree in (2,3):
            run=toy_index_calculus(parameters,p,degree)
            results["small_controls"].append(run)
            print(json.dumps(run),flush=True)
    Path(args.output).write_text(json.dumps(results,indent=2)+"\n")
    print("Wrote "+args.output)


if __name__=="__main__":
    main()
