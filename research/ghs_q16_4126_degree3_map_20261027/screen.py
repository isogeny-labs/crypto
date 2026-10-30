#!/usr/bin/env sage -python
"""Degree-three map screen for the 140 listed order-4126 hyperelliptic curves."""

import argparse
from hashlib import sha256
import itertools
import json
from pathlib import Path
import sys

from sage.all import EllipticCurve, GF, HyperellipticCurve, PolynomialRing


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
PARENT = ROOT / 'ghs_automorphism_screen_q16_20261026'
NEIGHBOR = ROOT / 'ghs_degree3_neighbor_screen_q16_20261026'
CATALOG = PARENT / 'catalog_models_raw.json'
CLOSURE = ROOT / 'ghs_cubic_hess_closure_q16_20261027/certificate.json'
sys.path.insert(0, str(PARENT))
sys.path.insert(0, str(NEIGHBOR))

from screen_degree3_pattern import (
    as_coboundary, assert_as_solution, branch_residue, curves,
    parse_model, positive_control, scalar_from_simple_zero,
    second_pattern,
)
from screen_neighbors import positive_control_five_support, rational_record


def digest(path):
    return sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--limit-models', type=int, default=140)
    args = parser.parse_args()
    assert 1 <= args.limit_models <= 140

    catalog = json.loads(CATALOG.read_text())
    closure = json.loads(CLOSURE.read_text())
    class_row = next(x for x in catalog['rows'] if x['label'] == '3.16.a_a_bd')
    lines = curves(class_row['curves'])
    assert int(class_row['hyp_count']) == len(lines) == 140
    source_row = next(x for x in closure['orders'] if x['order'] == 4126)
    assert source_row['source_models'] == 108
    assert not source_row['in_hess_isogeny_closure']
    assert 63 in source_row['by_a']['32']['outside']

    k = GF(16, 'a', modulus='conway')
    K = GF(4096, 'w', modulus='conway')
    embed = k.embeddings(K)[0]
    Rk = PolynomialRing(k, 'x')
    RK = PolynomialRing(K, 'x')
    xk, x = Rk.gen(), RK.gen()
    a2, a6 = K.from_integer(32), K.from_integer(63)
    E = EllipticCurve(K, [K.one(), a2, K.zero(), K.zero(), a6])
    G = E(K.from_integer(62), K.from_integer(1499))
    Q = E(K.from_integer(2633), K.from_integer(1798))
    assert E.cardinality() == 4126
    assert 2063 * G == E(0) and G != E(0)
    assert Q == 7 * G

    controls = {
        'four_support': positive_control(K, x, a2, a6),
        'five_support': positive_control_five_support(K, x, a2, a6),
    }
    rows, hits = [], []
    first_total = second_total = residue_total = 0
    for model_index, line in enumerate(lines[:args.limit_models]):
        h0, f0 = parse_model(line, Rk, k.gen(), xk)
        assert HyperellipticCurve(f0, h0).genus() == 3
        h = RK([embed(z) for z in h0.list()])
        f = RK([embed(z) for z in f0.list()])
        roots = sorted(h.roots(multiplicities=False),
                       key=lambda z: int(z.to_integer()))
        assert h.degree() == 4 and len(roots) == 4
        assert all(h.derivative()(r) != 0 and branch_residue(h, f, r) != 0
                   for r in roots)

        first_hits = []
        for indices in itertools.permutations(range(4)):
            r0, r1, r2, r3 = [roots[i] for i in indices]
            numerator = (x-r0)**2 * (x-r1)
            denominator = (x-r2)**2 * (x-r3)
            scalar = scalar_from_simple_zero(
                h, f, r0, r1, r2, r3, a6.sqrt())
            R = scalar*numerator/denominator
            difference = f/h**2 + R + a2 + a6/R**2
            witness = as_coboundary(difference, roots, x, K)
            first_total += 1
            if witness is not None:
                assert_as_solution(difference, witness)
                first_hits.append({
                    'model_index': model_index,
                    'pattern': 'four_support',
                    'permutation': list(indices),
                    'R': rational_record(R),
                    't': rational_record(witness),
                })

        second_count, residue_count, second_hits = second_pattern(
            h, f, roots, x, K, a2, a6, model_index)
        second_total += second_count
        residue_total += residue_count
        for hit in second_hits:
            R = RK(hit['R_numerator']) / RK(hit['R_denominator'])
            hidden = hit['hidden']
            extra = [] if hidden == 'infinity' else [K.from_integer(hidden)]
            difference = f/h**2 + R + a2 + a6/R**2
            witness = as_coboundary(difference, roots + extra, x, K)
            assert_as_solution(difference, witness)
            hit['t'] = rational_record(witness)
            hit['pattern'] = 'five_support'
        hits.extend(first_hits + second_hits)
        rows.append({
            'model_index': model_index,
            'model_equation': line,
            'branch_roots': [int(z.to_integer()) for z in roots],
            'four_support_checked': 24,
            'five_support_checked': second_count,
            'five_support_residue_pass': residue_count,
            'hits': len(first_hits) + len(second_hits),
        })
        print('MODEL', model_index, 'five_checked', second_count,
              'residue_pass', residue_count, 'hits', rows[-1]['hits'],
              flush=True)

    assert first_total == args.limit_models * 24
    assert second_total == args.limit_models * 4 * 2 * (4096-4+1)
    result = {
        'schema': 'ghs-q16-4126-degree3-map/v1',
        'protocol_sha256': digest(HERE / 'PROTOCOL.md'),
        'script_sha256': digest(HERE / 'screen.py'),
        'catalog_sha256': digest(CATALOG),
        'closure_sha256': digest(CLOSURE),
        'parent_enumerator_sha256': digest(PARENT / 'screen_degree3_pattern.py'),
        'parent_neighbor_sha256': digest(NEIGHBOR / 'screen_neighbors.py'),
        'source': {'a2': 32, 'a6': 63, 'order': 4126,
                   'subgroup_order': 2063, 'G': [62, 1499],
                   'Q': [2633, 1798], 'log': 7},
        'catalog_label': '3.16.a_a_bd',
        'catalog_models_total': 140,
        'models_checked': args.limit_models,
        'four_support_checked': first_total,
        'five_support_checked': second_total,
        'five_support_residue_pass': residue_total,
        'positive_controls': controls,
        'per_model': rows,
        'hits': hits,
    }
    suffix = '' if args.limit_models == 140 else f'-first{args.limit_models}'
    (HERE / f'certificate{suffix}.json').write_text(
        json.dumps(result, indent=2) + '\n')
    print('FINAL', args.limit_models, first_total, second_total,
          residue_total, len(hits), flush=True)


if __name__ == '__main__':
    main()
