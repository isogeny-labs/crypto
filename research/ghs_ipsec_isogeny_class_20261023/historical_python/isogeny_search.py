#!/usr/bin/env python3
"""Bounded classical modular-polynomial search in characteristic two.

Searches j-invariants, not explicit isogeny maps.  A rational modular root is
only a candidate until a twist with the target group order is established.
The resulting exclusion screens both twists via their identical b coefficient.
Data: Andrew Sutherland's classical modular-polynomial files (MIT).
"""
import argparse
from collections import Counter, deque
from functools import lru_cache
import hashlib
import json
from pathlib import Path
import random
import re
import time

from analyze import F, IPSEC_B, IPSEC_ORDER, direct_info, support_bound


def trim(p):
    while p and p[-1] == 0:
        p.pop()
    return p


def add(a, b):
    c = a[:] + [0]*max(0, len(b)-len(a))
    for i, v in enumerate(b):
        c[i] ^= v
    return trim(c)


def monic(p):
    p = trim(p[:])
    if p and p[-1] != 1:
        scale = F.inv(p[-1])
        p = [F.mul(scale, v) for v in p]
    return p


def divmod_poly(a, b):
    a, b = trim(a[:]), trim(b[:])
    assert b
    quotient = [0]*max(0, len(a)-len(b)+1)
    lead_inverse = F.inv(b[-1]) if b[-1] != 1 else 1
    while a and len(a) >= len(b):
        shift = len(a)-len(b)
        c = F.mul(a[-1], lead_inverse) if lead_inverse != 1 else a[-1]
        quotient[shift] = c
        for i, v in enumerate(b):
            if v:
                a[i+shift] ^= c if v == 1 else F.mul(c, v)
        trim(a)
    return trim(quotient), a


def gcd(a, b):
    while b:
        _, r = divmod_poly(a, b)
        a, b = b, r
    return monic(a)


def sqmod(a, modulus):
    p = [0]*max(0, 2*len(a)-1)
    for i, v in enumerate(a):
        p[2*i] = F.sq(v)
    return divmod_poly(p, modulus)[1]


def evaluate(p, x):
    y = 0
    for c in reversed(p):
        y = F.mul(y, x) ^ c
    return y


def rational_roots(p):
    p = monic(p)
    x = [0, 1]
    xq = x
    for _ in range(155):
        xq = sqmod(xq, p)
    linear_part = gcd(p, add(xq, x))
    roots = []
    todo = [linear_part]
    rng = random.Random(0x2409)
    while todo:
        f = todo.pop()
        degree = len(f)-1
        if degree <= 0:
            continue
        if degree == 1:
            roots.append(f[0])
            continue
        if degree == 2:
            c = F.mul(f[0], F.inv(F.sq(f[1])))
            assert F.trace(c) == 0
            # Half-trace: odd absolute field degree.
            ht, v = 0, c
            for _ in range(78):
                ht ^= v
                v = F.sq(F.sq(v))
            assert F.sq(ht) ^ ht == c
            root = F.mul(f[1], ht)
            roots.extend([root, root ^ f[1]])
            continue
        for _ in range(100):
            h = [0, rng.getrandbits(155)]
            trace = h[:]
            for _ in range(154):
                h = sqmod(h, f)
                trace = add(trace, h)
            g = gcd(trace, f)
            if 1 < len(g) < len(f):
                quotient, remainder = divmod_poly(f, g)
                assert not remainder
                todo.extend([g, quotient])
                break
        else:
            raise RuntimeError('Trace splitting failed')
    assert len(roots) == len(linear_part)-1
    assert all(evaluate(p, root) == 0 for root in roots)
    return sorted(set(roots))


class ModularPolynomial:
    def __init__(self, level, path):
        self.level = level
        data = path.read_bytes()
        self.sha256 = hashlib.sha256(data).hexdigest()
        terms = []
        for line in data.decode('ascii').splitlines():
            match = re.fullmatch(r'\[(\d+),(\d+)\]\s+(-?\d+)', line.strip())
            assert match, line
            i, j, coefficient = map(int, match.groups())
            if coefficient & 1:
                terms.append((i, j))
                if i != j:
                    terms.append((j, i))
        self.terms = terms
        self.degree = max(i for i, _ in terms)
        assert self.degree == level+1

    def at(self, j):
        powers = [1]
        for _ in range(self.degree):
            powers.append(F.mul(powers[-1], j))
        coefficients = [0]*(self.degree+1)
        for i, k in self.terms:
            coefficients[k] ^= powers[i]
        return trim(coefficients)


def checks():
    # Verify root finding on polynomials constructed from known roots.
    for test_roots in [[1, 2], [0, 1, 2, 3], [0x7338f, 0x10006, 5, 9, 17]]:
        p = [1]
        for root in test_roots:
            q = [0]*(len(p)+1)
            for i, coefficient in enumerate(p):
                q[i] ^= F.mul(root, coefficient)
                q[i+1] ^= coefficient
            p = q
        assert rational_roots(p) == sorted(test_roots)


@lru_cache(maxsize=None)
def bound_for(order):
    return support_bound(order, 0)['necessary_genus_lower_bound']


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--max-nodes', type=int, default=40)
    parser.add_argument('--max-depth', type=int, default=4)
    args = parser.parse_args()
    start = time.perf_counter()
    checks()
    directory = Path(__file__).parent
    polynomials = [ModularPolynomial(level, directory/'sources'/f'phi_j_{level}.txt')
                   for level in [3, 5, 7, 11, 13]]
    initial = F.inv(IPSEC_B)
    queue = deque([initial])
    visited = {initial: {'depth': 0, 'parent': None, 'level': None}}
    records = []
    edges = []
    root_counts = Counter()
    while queue and len(records) < args.max_nodes:
        j = queue.popleft()
        b = F.inv(j)
        direct = direct_info(b, 5)
        bound = bound_for(int(direct['annihilator_hex'], 16))
        record = {'j_hex': hex(j), 'b_hex': hex(b), **visited[j],
                  'direct_F32_m': direct['m'],
                  'direct_F32_genus': direct['exact_genus'],
                  'F32_annihilator_hex': direct['annihilator_hex'],
                  'hess_F32_necessary_genus_lower_bound': bound}
        if record['parent'] is not None:
            record['parent'] = hex(record['parent'])
        neighbor_counts = {}
        if record['depth'] < args.max_depth:
            for polynomial in polynomials:
                roots = rational_roots(polynomial.at(j))
                neighbor_counts[str(polynomial.level)] = len(roots)
                root_counts[str(polynomial.level)] += len(roots)
                for child in roots:
                    assert child != 0  # ordinary component
                    # Check the reverse modular equation independently.
                    assert evaluate(polynomial.at(child), j) == 0
                    edges.append({'from': hex(j), 'to': hex(child),
                                  'level': polynomial.level})
                    if child not in visited:
                        visited[child] = {'depth': record['depth']+1,
                                          'parent': j, 'level': polynomial.level}
                        queue.append(child)
        record['rational_root_counts'] = neighbor_counts
        records.append(record)
        print(f'node {len(records)} depth={record["depth"]} m={direct["m"]} '
              f'Hess_bound={bound} neighbors={neighbor_counts}', flush=True)
    t = (1 << 155)+1-IPSEC_ORDER
    discriminant = t*t-4*(1 << 155)
    results = {'self_checks': 'passed', 'search_type': 'bounded modular j-graph',
               'scope': 'modular roots; explicit maps and twist order not constructed',
               'levels': [p.level for p in polynomials],
               'max_nodes': args.max_nodes, 'max_depth': args.max_depth,
               'nodes_screened': len(records), 'nodes_discovered': len(visited),
               'pending_nodes': len(queue),
               'curve_order_from_RFC2409': str(IPSEC_ORDER),
               'frobenius_trace': str(t),
               'discriminant': str(discriminant),
               'root_counts': dict(root_counts),
               'm_distribution': dict(Counter(r['direct_F32_m'] for r in records)),
               'hess_bound_distribution': dict(Counter(
                   r['hess_F32_necessary_genus_lower_bound'] for r in records)),
               'modular_polynomial_sha256': {str(p.level): p.sha256 for p in polynomials},
               'records': records, 'edges': edges,
               'elapsed_seconds': time.perf_counter()-start}
    (directory/'isogeny_results.json').write_text(json.dumps(results, indent=2)+'\n')
    print(json.dumps({k:v for k,v in results.items() if k not in ['records','edges']},
                     indent=2), flush=True)


if __name__ == '__main__':
    main()
