#!/usr/bin/env python3
"""Reproducible finite-field and Hess support checks; Python 3, no dependencies.

All bit strings use polynomial basis, least significant bit = constant term.
The generalized genus bound is NECESSARY, not a constructed descent.
"""
import json
from pathlib import Path
import random
import time


def prem(a, b):
    while a.bit_length() >= b.bit_length():
        a ^= b << (a.bit_length() - b.bit_length())
    return a


def pmul(a, b):
    c = 0
    while b:
        if b & 1:
            c ^= a
        a <<= 1
        b >>= 1
    return c


def pgcd(a, b):
    while b:
        a, b = b, prem(a, b)
    return a


def polystr(p, var='T'):
    return ' + '.join('1' if i == 0 else var if i == 1 else f'{var}^{i}'
                      for i in reversed(range(p.bit_length())) if (p >> i) & 1) or '0'


class BinaryField:
    def __init__(self, degree, modulus):
        self.n = degree
        self.modulus = modulus

    def mul(self, a, b):
        c = pmul(a, b)
        return prem(c, self.modulus)

    def sq(self, a):
        c = 0
        while a:
            t = a & -a
            c |= 1 << (2 * (t.bit_length() - 1))
            a ^= t
        return prem(c, self.modulus)

    def frob(self, a, power):
        for _ in range(power % self.n):
            a = self.sq(a)
        return a

    def pow(self, a, exponent):
        b = 1
        while exponent:
            if exponent & 1:
                b = self.mul(b, a)
            a = self.sq(a)
            exponent >>= 1
        return b

    def inv(self, a):
        if not a:
            raise ZeroDivisionError
        u, v, g, h = a, self.modulus, 1, 0
        while u != 1:
            if not u:
                raise ValueError('Non-field modulus')
            shift = u.bit_length() - v.bit_length()
            if shift < 0:
                u, v, g, h = v, u, h, g
                shift = -shift
            u ^= v << shift
            g ^= h << shift
        return prem(g, self.modulus)

    def trace(self, a):
        result = 0
        for _ in range(self.n):
            result ^= a
            a = self.sq(a)
        assert result in (0, 1)
        return result


F = BinaryField(155, (1 << 155) | (1 << 62) | 1)
F32 = BinaryField(5, 0x25)
FACTORS31 = [3, 0x25, 0x29, 0x2f, 0x37, 0x3b, 0x3d]
IPSEC_B = 0x7338f
JMS_B = (1 << 16) | (1 << 2) | 2
IPSEC_ORDER = int('0800000000000000000057db5698537193aef944', 16)


def rank(vectors):
    rows = {}
    for v in vectors:
        while v:
            p = v.bit_length() - 1
            if p in rows:
                v ^= rows[p]
            else:
                rows[p] = v
                break
    return len(rows)


def orbit(a, step, n):
    values = []
    for _ in range(n):
        values.append(a)
        a = F.frob(a, step)
    assert a == values[0]
    return values


def annihilator(values):
    rows = {}
    for i, v in enumerate(values):
        relation = 1 << i
        while v:
            p = v.bit_length() - 1
            if p in rows:
                w, c = rows[p]
                v ^= w
                relation ^= c
            else:
                rows[p] = (v, relation)
                break
        if not v:
            return relation
    raise ValueError('No recurrence')


def eval_sigma(poly, a, step=5):
    result = 0
    while poly:
        if poly & 1:
            result ^= a
        a = F.frob(a, step)
        poly >>= 1
    return result


def direct_info(b, step):
    n = 155 // step
    values = orbit(b, step, n)
    # Simultaneous square-root is F2-linear and fixes 1, so replacing
    # sqrt(b_i) with b_i preserves this augmented rank exactly.
    m = rank([(a << 1) | 1 for a in values])
    order = annihilator(values + [values[0]])
    assert eval_sigma(order, b, step) == 0
    relative_trace = 0
    for v in values:
        relative_trace ^= v
    return {'subfield_degree': step, 'extension_degree': n, 'm': m,
            'annihilator_hex': hex(order), 'annihilator': polystr(order),
            'annihilator_degree': order.bit_length() - 1,
            'relative_trace_hex': hex(relative_trace),
            'exact_genus': (1 << (m-1)) - (1 if relative_trace == 0 else 0),
            'genus_alternatives': [(1 << (m-1))-1, 1 << (m-1)]}


def roots31(poly):
    roots = set()
    for exponent in range(31):
        zeta = F32.pow(2, exponent)
        value = 0
        for i in reversed(range(poly.bit_length())):
            value = F32.mul(value, zeta) ^ ((poly >> i) & 1)
        if not value:
            roots.add(exponent)
    assert len(roots) == poly.bit_length() - 1
    return roots


FACTOR_ROOTS = [roots31(p) for p in FACTORS31]
SUPPORTS = []
for mask in range(1, 128):
    support = set()
    for i, roots in enumerate(FACTOR_ROOTS):
        if (mask >> i) & 1:
            support |= roots
    SUPPORTS.append((mask, support))


def support_of(order):
    support = set()
    factors = []
    residual = order
    for p, roots in zip(FACTORS31, FACTOR_ROOTS):
        if prem(order, p) == 0:
            factors.append(polystr(p))
            support |= roots
            # Exact division over F2.
            quotient = 0
            while residual.bit_length() >= p.bit_length():
                shift = residual.bit_length() - p.bit_length()
                quotient ^= 1 << shift
                residual ^= p << shift
            assert residual == 0
            residual = quotient
    assert residual == 1
    return support, factors


def embedded_f32():
    generator = F.pow(2, ((1 << 155)-1)//31)
    assert generator != 1 and F.pow(generator, 31) == 1
    candidates = []
    for i in range(1, 31):
        candidate = F.pow(generator, i)
        if F.pow(candidate, 5) ^ F.sq(candidate) ^ 1 == 0:
            candidates.append(candidate)
    zeta = min(candidates)
    embedded = []
    for x in range(32):
        value = 0
        for i in reversed(range(5)):
            value = F.mul(value, zeta) ^ ((x >> i) & 1)
        embedded.append(value)
    assert all(F.frob(v, 5) == v for v in embedded)
    return zeta, embedded


ZETA, EMBEDDED_F32 = embedded_f32()


def spectral_support(b):
    """Compute actual F32 eigencomponents, not their F2 conjugacy closure."""
    values = orbit(b, 5, 31)
    components = []
    for j in range(31):
        projection = 0
        for i, v in enumerate(values):
            coefficient = EMBEDDED_F32[F32.pow(2, (-i*j) % 31)]
            projection ^= F.mul(coefficient, v)
        if projection:
            assert F.frob(projection, 5) == F.mul(
                projection, EMBEDDED_F32[F32.pow(2, j)])
            components.append((j, projection))
    reconstruction = 0
    for _, component in components:
        reconstruction ^= component
    assert reconstruction == b
    return {j for j, _ in components}


def support_bound(order, trace_a=0, actual_support=None):
    """Exhaust all possible exact F2 Frobenius-annihilator factor supports.

    Over F32, sigma is diagonalizable: F32 contains all 31st roots of 1.
    A product of eigenvectors of exponents i,j has exponent i+j mod31.
    Therefore supp(sqrt(b)) must lie in supp(alpha)+supp(beta).
    Squaring permutes exponents by doubling.  The alpha/beta admissible
    sumsets are doubling-stable, so membership may be tested on b directly.
    This necessary condition does NOT ensure a rank-one product decomposition.
    """
    closure, factors = support_of(order)
    target = closure if actual_support is None else actual_support
    assert target <= closure
    assert {((e * (1 << i)) % 31) for e in target for i in range(5)} == closure
    best = None
    examples = []
    smaller_excluded = 0
    considered = 0
    for i, (mask_a, a) in enumerate(SUPPORTS):
        for mask_b, b in SUPPORTS[i:]:
            if trace_a and 0 not in a and 0 not in b:
                continue
            considered += 1
            union = a | b | ({0} if trace_a else set())
            t, r, s = len(union), len(a), len(b)
            genus = (1 << t) - (1 << (t-r)) - (1 << (t-s)) + 1
            sumset = {(x+y) % 31 for x in a for y in b}
            if not target <= sumset:
                smaller_excluded += 1
                continue
            if best is None or genus < best:
                best = genus
                examples = []
            if genus == best:
                examples.append({'alpha_factor_mask': mask_a,
                                 'beta_factor_mask': mask_b,
                                 'r': r, 's': s, 't': t,
                                 'sumset_size': len(sumset)})
    return {'necessary_genus_lower_bound': best,
            'bound_scope': 'Hess two-factor construction, fixed curve, descent to F32',
            'is_constructed_descent': False,
            'target_support': sorted(target), 'target_factors': factors,
            'F2_annihilator_support_closure': sorted(closure),
            'unordered_support_pairs_checked': considered,
            'pairs_excluded_by_support': smaller_excluded,
            'minimizing_support_pairs': examples}


def selfcheck():
    assert pgcd(pmul(FACTORS31[0], FACTORS31[1]), FACTORS31[0]) == FACTORS31[0]
    product = 1
    for p in FACTORS31:
        product = pmul(product, p)
    assert product == (1 << 31) | 1
    # Rabin irreducibility test for degree 155, with prime divisors 5,31.
    z = 2
    zpow = z
    for _ in range(155):
        zpow = F.sq(zpow)
    assert zpow == z
    for divisor in [5, 31]:
        zpow = z
        for _ in range(155 // divisor):
            zpow = F.sq(zpow)
        assert pgcd(zpow ^ z, F.modulus) == 1
    rng = random.Random(20261023)
    for _ in range(16):
        a, b = rng.getrandbits(155), rng.getrandbits(155)
        assert F.mul(a, a) == F.sq(a)
        assert F.mul(a, F.inv(a)) == 1
        assert F.sq(F.frob(a, 154)) == a
        assert F.sq(a ^ b) == F.sq(a) ^ F.sq(b)
    assert set.union(*FACTOR_ROOTS) == set(range(31))


def main():
    start = time.perf_counter()
    selfcheck()
    results = {'field_modulus': polystr(F.modulus, 'z'),
               'self_checks': 'passed', 'curves': {}}
    for name, a, b in [('ipsec', 0, IPSEC_B), ('jms_vulnerable', 1, JMS_B)]:
        info = {'a': a, 'b_hex': hex(b), 'b': polystr(b, 'z'),
                'absolute_trace_a': F.trace(a),
                'direct_ghs': [direct_info(b, step) for step in [5, 31, 1]]}
        info['hess_F32_support_bound'] = support_bound(
            int(info['direct_ghs'][0]['annihilator_hex'], 16), F.trace(a),
            spectral_support(b))
        results['curves'][name] = info
    ip = results['curves']['ipsec']['direct_ghs'][0]
    jms = results['curves']['jms_vulnerable']['direct_ghs'][0]
    # Independent cross-check against the annihilator explicitly in JMS §6.
    quotient = 0
    residual = (1 << 31) | 1
    while residual.bit_length() >= 0x29.bit_length():
        shift = residual.bit_length() - 0x29.bit_length()
        quotient ^= 1 << shift
        residual ^= 0x29 << shift
    assert residual == 0 and int(ip['annihilator_hex'], 16) == quotient
    assert jms['m'] == 6
    results['elapsed_seconds'] = time.perf_counter()-start
    output = Path(__file__).with_name('results.json')
    output.write_text(json.dumps(results, indent=2)+'\n')
    print(json.dumps(results, indent=2), flush=True)


if __name__ == '__main__':
    main()
