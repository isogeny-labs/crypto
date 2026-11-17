#!/usr/bin/env sage -python
"""Complete degree-three cover screen for 12 sources x 4 catalog models."""

import hashlib
import itertools
import json
import sys
from pathlib import Path

from sage.all import EllipticCurve, GF, HyperellipticCurve, PolynomialRing


ROOT = Path(__file__).resolve().parent
PARENT = ROOT.parent
PRIOR = PARENT / 'ghs_automorphism_screen_q16_20261026'
sys.path.insert(0, str(PRIOR))
from screen_degree3_pattern import (as_coboundary, assert_as_solution,
                                    branch_residue, curves, parse_model,
                                    pole_order_and_lead, positive_control,
                                    reduce_poles, scalar_from_simple_zero,
                                    second_pattern)

SOURCE = PARENT / 'ghs_binary_cubic_isogeny_q16_20261026/results.json'
CATALOG = PRIOR / 'catalog_models_raw.json'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def rational_record(value):
    return {'numerator': [int(z.to_integer()) for z in value.numerator().list()],
            'denominator': [int(z.to_integer()) for z in value.denominator().list()]}


def positive_control_five_support(K, x, a2, a6):
    roots = [K.zero(), K.one(), K.gen(), K.gen()**2]
    h = (x-roots[0])*(x-roots[1])*(x-roots[2])*(x-roots[3])
    denominator = (x-roots[1])*(x-roots[2])*(x-roots[3])
    A = (x-roots[0])/denominator
    b = a6.sqrt()
    for hidden in K:
        if hidden in roots:
            continue
        A0, A1 = A(hidden), A.derivative()(hidden)
        if A0 == 0 or A1 == 0:
            continue
        # Exactly cancels the reduced simple pole at the hidden double zero.
        scalar = b*A1**2/A0**3
        R = scalar*(x-hidden)**2*A
        target = R+a2+a6/R**2
        reduced = reduce_poles(target, roots+[hidden], x, allow_simple=True)
        if reduced is None:
            continue
        representative, _ = reduced
        if pole_order_and_lead(representative, hidden, x)[0] != 0:
            continue
        if any(pole_order_and_lead(representative, r, x)[0] != 1
               for r in roots):
            continue
        f = h**2*representative
        if f.denominator().degree() != 0:
            continue
        f = f.numerator()/f.denominator()[0]
        if f.degree() > 8 or HyperellipticCurve(f, h).genus() != 3:
            continue
        difference = f/h**2+target
        witness = as_coboundary(difference, roots+[hidden], x, K)
        assert_as_solution(difference, witness)
        _, _, hits = second_pattern(h, f, roots, x, K, a2, a6, -1)
        assert any(z['orientation']=='double_zero' and
                   z['hidden']==int(hidden.to_integer()) and
                   z['scalar']==int(scalar.to_integer()) for z in hits)
        return {'hidden': int(hidden.to_integer()),
                'scalar': int(scalar.to_integer()),
                'h_coefficients': [int(z.to_integer()) for z in h.list()],
                'f_coefficients': [int(z.to_integer()) for z in f.list()],
                'degree': 3, 'recovered_maps': len(hits)}
    raise AssertionError('five-support positive control not found')


def check_four_support(h, f, roots, x, K, a2, a6, model_index):
    count = 0
    hits = []
    for indices in itertools.permutations(range(4)):
        r0, r1, r2, r3 = [roots[i] for i in indices]
        numerator = (x-r0)**2*(x-r1)
        denominator = (x-r2)**2*(x-r3)
        scalar = scalar_from_simple_zero(h, f, r0, r1, r2, r3, a6.sqrt())
        R = scalar*numerator/denominator
        difference = f/h**2 + R + a2 + a6/R**2
        witness = as_coboundary(difference, roots, x, K)
        count += 1
        if witness is not None:
            assert_as_solution(difference, witness)
            hits.append({'model_index': model_index,
                         'pattern': 'four_support',
                         'branch_permutation': list(indices),
                         'R': rational_record(R),
                         't': rational_record(witness)})
    assert count == 24
    return count, hits


def main():
    source = json.loads(SOURCE.read_text())
    archive = json.loads(CATALOG.read_text())
    models = next(z for z in archive['rows'] if z['label'] == '3.16.a_a_aet')
    lines = curves(models['curves'])
    assert len(lines) == 4
    sources = [s for s in source['source_population'] if s['order'] == 3974]
    assert len(sources) == 12 and len({s['b'] for s in sources}) == 12
    assert all(s['a'] == 32 and s['largest_prime_subgroup'] == 1987
               for s in sources)

    k = GF(16, 'a', modulus='conway')
    K = GF(4096, 'w', modulus='conway')
    embed = k.embeddings(K)[0]
    Rk = PolynomialRing(k, 'x')
    xk = Rk.gen()
    ring = PolynomialRing(K, 'x')
    x = ring.gen()
    parsed = []
    for i, line in enumerate(lines):
        h0, f0 = parse_model(line, Rk, k.gen(), xk)
        h = ring([embed(z) for z in h0.list()])
        f = ring([embed(z) for z in f0.list()])
        roots = sorted(h.roots(multiplicities=False),
                       key=lambda z: int(z.to_integer()))
        assert len(roots) == 4
        assert all(branch_residue(h, f, r) != 0 for r in roots)
        parsed.append((i, line, h, f, roots))

    control = positive_control(K, x, K.from_integer(32), K.from_integer(306))
    control_five = positive_control_five_support(
        K, x, K.from_integer(32), K.from_integer(306))
    rows = []
    hits = []
    four_count = 0
    five_count = 0
    residue_pass = 0
    for source_row in sources:
        a2, a6 = K.from_integer(source_row['a']), K.from_integer(source_row['b'])
        E = EllipticCurve(K, [K.one(), a2, K.zero(), K.zero(), a6])
        assert E.cardinality() == 3974
        for i, line, h, f, roots in parsed:
            first_count, first_hits = check_four_support(h, f, roots, x, K,
                                                          a2, a6, i)
            second_count, second_pass, second_hits = second_pattern(
                h, f, roots, x, K, a2, a6, i)
            four_count += first_count
            five_count += second_count
            residue_pass += second_pass
            for hit in first_hits + second_hits:
                hit['source_a2'] = source_row['a']
                hit['source_a6'] = source_row['b']
                if 't' not in hit:
                    R = ring(hit['R_numerator']) / ring(hit['R_denominator'])
                    difference = f/h**2 + R + a2 + a6/R**2
                    t = as_coboundary(difference, roots, x, K)
                    assert_as_solution(difference, t)
                    hit['t'] = rational_record(t)
                hits.append(hit)
            rows.append({'source_a2': source_row['a'],
                         'source_a6': source_row['b'],
                         'model_index': i,
                         'four_support_checked': first_count,
                         'five_support_checked': second_count,
                         'five_support_residue_pass': second_pass,
                         'hits': len(first_hits) + len(second_hits)})
        print('SOURCE', source_row['b'], 'hits', len(hits), flush=True)
    assert four_count == 12*4*24
    assert five_count == 12*4*4*2*(4096-4+1)
    output = {'schema': 'ghs-q16-degree3-neighbor-screen/v1',
              'source_table_sha256': sha(SOURCE),
              'catalog_snapshot_sha256': sha(CATALOG),
              'parent_screen_sha256': sha(PRIOR / 'screen_degree3_pattern.py'),
              'protocol_sha256': sha(ROOT / 'PROTOCOL.md'),
              'script_sha256': sha(Path(__file__)),
              'source_order': 3974, 'source_subgroup_order': 1987,
              'source_models': len(sources), 'catalog_models': len(lines),
              'pairs_checked': len(rows),
              'four_support_checked': four_count,
              'five_support_checked': five_count,
              'five_support_residue_pass': residue_pass,
              'positive_control_four_support': control,
              'positive_control_five_support': control_five,
              'pairs': rows, 'hits': hits,
              'scope': 'Degree-three maps to twelve exact normalized order-3974 sources from four listed hyperelliptic models over K; other elliptic curves and other Jacobian models are not classified.'}
    (ROOT / 'certificate.json').write_text(json.dumps(output, indent=2)+'\n')
    print('FINAL', four_count, five_count, residue_pass, len(hits), flush=True)


if __name__ == '__main__':
    main()
