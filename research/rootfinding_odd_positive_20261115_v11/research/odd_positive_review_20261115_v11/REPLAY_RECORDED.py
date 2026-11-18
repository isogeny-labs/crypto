"""Independent replay of recorded certificates only; no candidate/root search."""
import gzip
import hashlib
import io
import itertools
import json
from pathlib import Path
import sys
import time

START = time.monotonic()
CHECKS = 0


def require(value, message):
    global CHECKS
    CHECKS += 1
    if not value:
        raise AssertionError(message)
    if time.monotonic() - START > 2:
        raise TimeoutError('two-second recorded replay budget')


class Coordinates:
    def __init__(self, q):
        require(q in (9, 25), 'declared odd extension')
        self.p, self.m = (3, 1) if q == 9 else (5, 2)
        self.q = q

    def add(self, x, y):
        return ((x % self.p + y % self.p) % self.p +
                self.p * ((x // self.p + y // self.p) % self.p))

    def neg(self, x):
        return (-x % self.p) + self.p * (-(x // self.p) % self.p)

    def sub(self, x, y):
        return self.add(x, self.neg(y))

    def mul(self, x, y):
        # Independent coefficient convolution followed by t²=-m reduction.
        a, b, c, d = x % self.p, x // self.p, y % self.p, y // self.p
        convolution = [a * c, a * d + b * c, b * d]
        return ((convolution[0] - self.m * convolution[2]) % self.p +
                self.p * (convolution[1] % self.p))


def trim(a):
    a = list(a)
    while a and a[-1] == 0:
        a.pop()
    return a


def product(f, a, b):
    if not a or not b:
        return []
    c = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            c[i + j] = f.add(c[i + j], f.mul(x, y))
    return trim(c)


def difference(f, a, b):
    return trim([f.sub(a[i] if i < len(a) else 0, b[i] if i < len(b) else 0)
                 for i in range(max(len(a), len(b)))])


def monic_division(f, a, b):
    require(b and b[-1] == 1, 'monic certificate divisor')
    a = trim(a)
    q = [0] * max(0, len(a) - len(b) + 1)
    while len(a) >= len(b):
        i, c = len(a) - len(b), a[-1]
        q[i] = c
        for j, v in enumerate(b):
            a[i + j] = f.sub(a[i + j], f.mul(c, v))
        a = trim(a)
    return trim(q), a


def value(f, polynomial, x):
    result = 0
    for c in reversed(polynomial):
        result = f.add(f.mul(result, x), c)
    return result


def derivative(f, polynomial):
    return trim([f.mul(i % f.p, polynomial[i]) for i in range(1, len(polynomial))])


def replay(path):
    packed = path.read_bytes()
    require(len(packed) <= 16 * 1024, 'compressed evidence cap')
    with gzip.GzipFile(fileobj=io.BytesIO(packed)) as stream:
        payload = stream.read(128 * 1024 + 1)
    require(len(payload) <= 128 * 1024, 'raw evidence cap')
    raw = json.loads(payload)
    require(raw.get('error') is None, 'candidate worker error')
    out = raw['details']
    records = out['records']
    require(len(records) == 2, 'both frozen positive cells recorded')
    summaries = []
    for record in records:
        public = record['public']
        q = public['q']
        f = Coordinates(q)
        C = public['C']
        z, y = public['target']
        h, a, ya = public['h'], public['a'], public['ya']
        require(a != z and ya != 0, 'regular point guards')
        require(f.mul(y, y) == value(f, C, z), 'target on curve')
        require(f.mul(ya, ya) == value(f, C, a), 'selected source on curve')
        details = [dict(k=d['parameters'][0], r=d['parameters'][1], s=d['parameters'][2],
                        t=d['parameters'][3], P=d['P'], norm=d['N'], H=d['H'], Q=d['Q'],
                        domain=public['domain'], domain_polynomial=d['domain_polynomial'],
                        remainder=d['remainder'], cube_residue=d['cube_residue'],
                        supported=d['supported'], sources=d['source_orders'])
                   for d in record['candidates']]
        require(sorted(record['dense_parameters']) == sorted(record['derived_parameters']),
                'raw oracle/derived list equality')
        recorded_parameters = sorted([[d['k'], d['r'], d['s'], d['t']] for d in details])
        require(recorded_parameters == sorted(record['derived_parameters']),
                'all candidate certificates retained')
        require(sorted([v['parameters'] for v in record['dense_survivors']]) == recorded_parameters,
                'all dense survivor certificates retained')
        dense_quotients = {tuple(v['parameters']): v['Q'] for v in record['dense_survivors']}
        expected_ks = {k for k in range(q) if f.add(f.mul(h, a), k) != 0}
        require(len(details) == len(expected_ks) and {d['k'] for d in details} == expected_ks,
                'every regular k has its unique affine solution certificate')
        positive = 0
        known_seen = False
        for d in details:
            k, r, s = d['k'], d['r'], d['s']
            P, N, H, Q = d['P'], d['norm'], d['H'], d['Q']
            domain = d['domain']
            require(len(domain) == 2 and len(set(domain)) == 2 and a in domain and z not in domain,
                    'arbitrary disjoint two-point source domain')
            require(P == [d['t'], s, r, 1], 'parameter/polynomial certificate')
            require(dense_quotients[(k, r, s, P[0])] == Q, 'independent dense survivor quotient')
            require(r == f.add(record['affine_r'][0], f.mul(record['affine_r'][1], k)) and
                    s == f.add(record['affine_s'][0], f.mul(record['affine_s'][1], k)),
                    'all affine parameter certificates')
            qq = record['quotient_coefficients']
            predicted = [f.add(f.add(qq[0][j], f.mul(qq[1][j], k)),
                               f.mul(qq[2][j], f.mul(k, k))) for j in range(4)]
            require(Q == predicted, 'all quadratic quotient certificates')
            G = [k, h]
            ga = value(f, G, a)
            require(ga != 0 and value(f, P, z) == f.neg(f.mul(value(f, G, z), y)),
                    'target interpolation and common-factor guard')
            require(value(f, P, a) == f.neg(f.mul(ga, ya)), 'selected point interpolation')
            direct_n = difference(f, product(f, P, P), product(f, C, product(f, G, G)))
            require(direct_n == N, 'independent literal norm replay')
            require(value(f, derivative(f, direct_n), a) == 0, 'literal norm derivative')
            direct_h, rem = monic_division(f, direct_n, [f.neg(z), 1])
            require(not rem and direct_h == H, 'target norm quotient')
            double = product(f, [f.neg(a), 1], [f.neg(a), 1])
            direct_q, rem = monic_division(f, direct_h, double)
            require(not rem and direct_q == Q and len(Q) == 4, 'exact cubic quotient')
            L = product(f, [f.neg(domain[0]), 1], [f.neg(domain[1]), 1])
            require(L == d['domain_polynomial'], 'domain polynomial certificate')
            _, R = monic_division(f, L, Q)
            _, R3 = monic_division(f, product(f, product(f, R, R), R), Q)
            require(R == d['remainder'] and R3 == d['cube_residue'], 'every recorded support residue')
            bags = []
            for bag in itertools.combinations_with_replacement(domain, 3):
                poly = [1]
                for x in bag:
                    poly = product(f, poly, [f.neg(x), 1])
                if poly == Q:
                    bags.append(bag)
            require((not R3) == bool(bags) == d['supported'], 'independent cubic multiset support')
            wanted = []
            if bags:
                require(len(bags) == 1, 'unique canonical cubic bag')
                wanted = [list(v) for v in sorted(set(itertools.permutations(list(bags[0]) + [a, a])))]
                positive += 1
            require(d['sources'] == wanted, 'full canonical source ordering equality')
            known = (k, r, s, P[0]) == ((2, 7, 7, 5) if q == 9 else (2, 14, 11, 0))
            if known:
                require(d['supported'] and len(wanted) == 10, 'known positive norm/order certificate')
                known_seen = True
        require(known_seen and positive > 0, 'nonvacuous positive in each field')
        summaries.append(dict(q=q, candidate_certificates=len(details), positives=positive,
                              known_positive_verified=True, all_residues_and_orders_replayed=True))
    return dict(schema='recorded_certificate_replay_v1', status='passed',
                input_sha256=hashlib.sha256(packed).hexdigest(), fields=summaries,
                checks=CHECKS, seconds=time.monotonic() - START,
                new_candidate_or_root_search=False, original_root_count_exact=None,
                complete_global_root_search=False, qualified_speedup=None)


if __name__ == '__main__':
    print(json.dumps(replay(Path(sys.argv[1])), sort_keys=True))
