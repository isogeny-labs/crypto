"""Construct equivariant trace multipliers in a Frobenius coordinate ring."""
import argparse
import json
from collections import Counter
from functools import cache
from itertools import combinations
from pathlib import Path
from random import Random
import sys
from time import perf_counter

from sympy import Poly, factor_list, symbols

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'lfd_lower_bound_20261111'))
from lfd_small import GF, rank_f2


def domains(width, degree):
    return [sum(1 << i for i in indices)
            for size in range(degree + 1) for indices in combinations(range(width), size)]


def xor_selected(words, mask):
    value = 0
    while mask:
        bit = mask & -mask
        mask ^= bit
        value ^= words[bit.bit_length() - 1]
    return value


def span(basis):
    points = [0]
    for word in basis:
        points += [point ^ word for point in points]
    return points


def kernel(columns):
    pivots = {}
    free = []
    for i, image in enumerate(columns):
        lift = 1 << i
        while image:
            j = image.bit_length() - 1
            if j not in pivots:
                pivots[j] = (image, lift)
                break
            old_image, old_lift = pivots[j]
            image ^= old_image
            lift ^= old_lift
        else:
            free.append(lift)
    return free


def row_kernel(rows, variables):
    """Return the dual basis to the quotient by the packed row span."""
    pivots = {}
    for row in rows:
        while row:
            j = row.bit_length() - 1
            if j not in pivots:
                pivots[j] = row
                break
            row ^= pivots[j]
    quotient = []
    dimension = 0
    for i in range(variables):
        if i in pivots:
            quotient.append(xor_selected(quotient, pivots[i] ^ (1 << i)))
        else:
            quotient.append(1 << dimension)
            dimension += 1
    basis = [0] * dimension
    for i, word in enumerate(quotient):
        while word:
            bit = word & -word
            word ^= bit
            basis[bit.bit_length() - 1] |= 1 << i
    return basis


def factors(n):
    z = symbols('z')
    k = (n - 1) // 2
    candidates = [Poly(p, z, modulus=2) for p, _ in factor_list(z**n + 1, z, modulus=2)[1]]
    out = [sum(1 << powers[0] for powers, coef in p.terms() if coef) for p in candidates if p.degree() == k]
    assert k % 2 and len(out) == 2
    return out


def geometry(n, g):
    field = GF(n)
    k = g.bit_length() - 1
    square = field.sq

    def annihilate(value):
        x = value
        output = 0
        for i in range(k + 1):
            if g >> i & 1:
                output ^= x
            x = square(x)
        return output

    nullspace = kernel(annihilate(1 << i) for i in range(n))
    assert len(nullspace) == k
    basis = [nullspace[0]]
    for _ in range(k - 1):
        basis.append(square(basis[-1]))
    assert rank_f2(basis) == k
    trace = field.tr
    assert not any(map(trace, basis))
    assert all(not trace(field.mul(a, b)) for a in basis for b in basis)
    return field, basis, square


class FrobeniusRing:
    def __init__(self, g, degree):
        self.k = g.bit_length() - 1
        self.degree = degree
        self.monomials = domains(self.k, degree + 1)
        self.slots = {m: i for i, m in enumerate(self.monomials)}
        self.generators = domains(self.k, degree)
        self.shift = [1 << (j + 1) for j in range(self.k - 1)] + [g ^ (1 << self.k)]
        # g(z)(z+1) annihilates U + F2.
        self.annihilator = g ^ (g << 1)

        @cache
        def multiply_variable(mask, j):
            bit = 1 << j
            if not (mask & bit):
                return 1 << self.slots[mask | bit]
            result = 0
            active = self.shift[j]
            while active:
                bit = active & -active
                active ^= bit
                result ^= multiply_variable(mask ^ (1 << j), bit.bit_length() - 1)
            return result

        self.multiply_variable = multiply_variable
        self.frobenius = []
        top = 1 << (self.k - 1)
        for m in self.monomials:
            if not (m & top):
                self.frobenius.append(1 << self.slots[m << 1])
            else:
                active = self.shift[-1]
                result = 0
                shifted = (m ^ top) << 1
                while active:
                    bit = active & -active
                    active ^= bit
                    result ^= multiply_variable(shifted, bit.bit_length() - 1)
                self.frobenius.append(result)
        self.product_shift = [multiply_variable(m, 1) if m.bit_count() <= degree else 0 for m in self.monomials]

    def family(self):
        width = len(self.monomials)
        variables = len(self.generators)
        image = [0] * width
        # Square sqrt(alpha)+s*alpha, then apply the annihilator of U+F2.
        for i in range(variables):
            column = (1 << i) ^ xor_selected(self.product_shift, self.frobenius[i])
            while column:
                bit = column & -column
                column ^= bit
                image[bit.bit_length() - 1] |= 1 << i
        effect = [0] * width
        active = self.annihilator
        while active:
            if active & 1:
                effect = [a ^ b for a, b in zip(effect, image)]
            active >>= 1
            if not active:
                break
            shifted = [0] * width
            for values, support in zip(image, self.frobenius):
                if not values:
                    continue
                while support:
                    bit = support & -support
                    support ^= bit
                    shifted[bit.bit_length() - 1] ^= values
            image = shifted
        return row_kernel(effect, variables)

    def multiply(self, left, right):
        output = 0
        while right:
            bit = right & -right
            right ^= bit
            term = self.monomials[bit.bit_length() - 1]
            product = left
            while term:
                var = term & -term
                term ^= var
                next_product = 0
                while product:
                    v = product & -product
                    product ^= v
                    next_product ^= self.multiply_variable(self.monomials[v.bit_length() - 1], var.bit_length() - 1)
                product = next_product
            output ^= product
        return output

    def scalar_multiple_span(self):
        n = 2 * self.k + 1
        conjugates = [1 << self.slots[1]]
        for _ in range(1, n):
            conjugates.append(xor_selected(self.frobenius, conjugates[-1]))
        rows = []
        scalar_dimensions = []
        for r in range(self.degree + 1):
            if r:
                alpha = 0
                for j in range(r):
                    term = conjugates[-j % n]
                    for i in range(1, r - j):
                        term = self.multiply(term, conjugates[i])
                    alpha ^= term
            else:
                alpha = 1 << self.slots[0]
            count = len(domains(self.k, self.degree - r))
            invariants = kernel((1 << i) ^ self.frobenius[i] for i in range(count))
            scalar_dimensions.append(len(invariants))
            rows.extend(self.multiply(alpha, invariant) for invariant in invariants)
        return rows, scalar_dimensions


def monomial_values(field, square, terms, k):
    slots = {m: i for i, m in enumerate(terms)}
    steps = [(slots[m ^ (m & -m)], (m & -m).bit_length() - 1) for m in terms[1:]]

    def evaluate(s):
        powers = [s]
        for _ in range(k - 1):
            powers.append(square(powers[-1]))
        values = [1]
        for previous, power in steps:
            values.append(field.mul(values[previous], powers[power]))
        return values

    return evaluate


def samples(field, basis, square, count):
    k = len(basis)
    if count >= (1 << k) - 1:
        seen = {0}
        representatives = []
        for s in span(basis):
            if s in seen:
                continue
            representatives.append(s)
            while s not in seen:
                seen.add(s)
                s = square(s)
        return representatives, True
    rng = Random(field.n)
    selectors = rng.sample(range(1, 1 << k), count)
    return [xor_selected(basis, selector) for selector in selectors], False


def field_annihilate(square, word, polynomial):
    result = 0
    while polynomial:
        if polynomial & 1:
            result ^= word
        polynomial >>= 1
        if polynomial:
            word = square(word)
    return result


def profile(n, degree, count):
    for g in factors(n):
        initial = perf_counter()
        field, basis, square = geometry(n, g)
        k = len(basis)
        ring = FrobeniusRing(g, degree)
        family = ring.family()
        scalar_rows, scalar_dimensions = ring.scalar_multiple_span()
        assert rank_f2([*family, *scalar_rows]) == len(family)
        scalar_rank = rank_f2(scalar_rows)
        setup = perf_counter() - initial
        start = perf_counter()
        d = len(ring.generators)
        independent = monomial_values(field, square, ring.generators, k)
        representatives, exhaustive = samples(field, basis, square, count)
        seen = Counter()
        for s in representatives:
            values = independent(s)
            evaluated = [xor_selected(values, f) for f in family]
            s2 = square(s)
            for alpha in evaluated:
                effect = alpha ^ field.mul(s2, square(alpha))
                assert not field_annihilate(square, effect, ring.annihilator)
            seen[rank_f2(evaluated)] += 1
        full = n - k + 1
        assert max(seen) <= full
        print(json.dumps(dict(n=n, g=g, k=k, multiplier_degree=degree, symbolic_columns=d,
                              symbolic_rows=len(ring.monomials), equivariant_dimension=len(family),
                              checked=len(representatives), exhaustive=exhaustive, ranks=seen,
                              full=full, scalar_dimensions=scalar_dimensions, scalar_multiple_dimension=scalar_rank,
                              span_certificate=scalar_rank == len(family), setup_s=setup, evaluation_s=perf_counter() - start)), flush=True)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('n', type=int, nargs='+', choices=(7, 23, 47))
    parser.add_argument('--degree', type=int, choices=(1, 2, 3), default=3)
    parser.add_argument('--samples', type=int, default=128)
    args = parser.parse_args()
    for n in args.n:
        profile(n, args.degree, (1 << ((n - 1) // 2)) - 1 if n <= 23 else args.samples)
