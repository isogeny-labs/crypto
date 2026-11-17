#!/usr/bin/env sage -python
"""Algebraically exhaustive degree-three map screen for order-4126 sources."""

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
ONE_SOURCE = HERE / 'certificate.json'
sys.path.insert(0, str(PARENT))
sys.path.insert(0, str(NEIGHBOR))

from screen_degree3_pattern import (
    as_coboundary, assert_as_solution, branch_residue, curves, parse_model,
    positive_control, scalar_from_simple_zero,
)
from screen_neighbors import positive_control_five_support, rational_record


def digest(path):
    return sha256(path.read_bytes()).hexdigest()


def four_support(h, f, roots, residues, x, K, a2, a6):
    tested, residue_pass, hits = 0, 0, []
    b = a6.sqrt()
    for indices in itertools.permutations(range(4)):
        tested += 1
        r0, r1, r2, r3 = [roots[i] for i in indices]
        scalar = scalar_from_simple_zero(h, f, r0, r1, r2, r3, b)
        # At r3, R has a simple pole and b/R² has no pole.
        necessary = scalar*(r3-r0)**2*(r3-r1)/(r3-r2)**2
        if necessary != residues[indices[3]]:
            continue
        residue_pass += 1
        R = scalar*(x-r0)**2*(x-r1)/((x-r2)**2*(x-r3))
        difference = f/h**2 + R + a2 + a6/R**2
        witness = as_coboundary(difference, roots, x, K)
        if witness is not None:
            assert_as_solution(difference, witness)
            hits.append({'pattern': 'four_support', 'permutation': list(indices),
                         'R': rational_record(R), 't': rational_record(witness)})
    return tested, residue_pass, hits


def five_support(h, f, roots, residues, x, K, a2, a6):
    b = a6.sqrt()
    covered = 0
    residue_pass = 0
    finite_candidates = 0
    hits = []
    for single_index, single in enumerate(roots):
        others = [roots[j] for j in range(4) if j != single_index]
        p = (single-others[0])*(single-others[1])*(single-others[2])
        a_values = []
        for j, r in enumerate(others):
            derivative = (r-others[(j+1)%3])*(r-others[(j+2)%3])
            a_values.append((r-single)/derivative)
        assert all(z != 0 for z in a_values)
        c_values = [residues[roots.index(r)]*residues[single_index]/
                    (b*p*a_values[j]) for j, r in enumerate(others)]
        # Infinity corresponds to (r-h)^2/(s-h)^2 = 1 for all r.
        hidden_values = [None] if all(c == 1 for c in c_values) else []
        c = c_values[0]
        if c != 1:
            H = (others[0]**2 + c*single**2)/(c+1)
            hidden = H.sqrt()
            if hidden not in roots:
                finite_candidates += 1
                if all(c_values[j] == (r**2+H)/(single**2+H)
                       for j, r in enumerate(others)):
                    hidden_values.append(hidden)
        for orientation in ('double_zero', 'double_pole'):
            covered += 4096 - 4 + 1
            for hidden in hidden_values:
                square_at_single = K.one() if hidden is None else (single-hidden)**2
                product = (x-others[0])*(x-others[1])*(x-others[2])
                factor = 1 if hidden is None else (x-hidden)**2
                if orientation == 'double_zero':
                    scalar = b*p/(residues[single_index]*square_at_single)
                    R = scalar*factor*(x-single)/product
                else:
                    scalar = residues[single_index]*square_at_single/p
                    R = scalar*product/(factor*(x-single))
                residue_pass += 1
                difference = f/h**2 + R + a2 + a6/R**2
                extra = [] if hidden is None else [hidden]
                witness = as_coboundary(difference, roots+extra, x, K)
                if witness is not None:
                    assert_as_solution(difference, witness)
                    hits.append({
                        'pattern': 'five_support',
                        'orientation': orientation,
                        'single_branch_index': single_index,
                        'hidden': 'infinity' if hidden is None else
                                  int(hidden.to_integer()),
                        'R': rational_record(R), 't': rational_record(witness),
                    })
    assert covered == 4 * 2 * (4096-4+1)
    return covered, finite_candidates, residue_pass, hits


def model_from_control(control, RK):
    h = RK([RK.base_ring().from_integer(z) for z in control['h_coefficients']])
    f = RK([RK.base_ring().from_integer(z) for z in control['f_coefficients']])
    roots = sorted(h.roots(multiplicities=False),
                   key=lambda z: int(z.to_integer()))
    residues = [branch_residue(h, f, r) for r in roots]
    return h, f, roots, residues


def main():
    catalog = json.loads(CATALOG.read_text())
    closure = json.loads(CLOSURE.read_text())
    earlier = json.loads(ONE_SOURCE.read_text())
    row = next(z for z in catalog['rows'] if z['label'] == '3.16.a_a_bd')
    lines = curves(row['curves'])
    assert len(lines) == int(row['hyp_count']) == 140
    source_row = next(z for z in closure['orders'] if z['order'] == 4126)
    source_b = source_row['by_a']['32']['outside']
    assert len(source_b) == 108 and source_row['by_a']['0']['outside'] == []
    assert earlier['models_checked'] == 140 and earlier['hits'] == []

    k = GF(16, 'a', modulus='conway')
    K = GF(4096, 'w', modulus='conway')
    embed = k.embeddings(K)[0]
    Rk, RK = PolynomialRing(k, 'x'), PolynomialRing(K, 'x')
    xk, x = Rk.gen(), RK.gen()
    a2 = K.from_integer(32)
    parsed = []
    for i, line in enumerate(lines):
        h0, f0 = parse_model(line, Rk, k.gen(), xk)
        assert HyperellipticCurve(f0, h0).genus() == 3
        h = RK([embed(z) for z in h0.list()])
        f = RK([embed(z) for z in f0.list()])
        roots = sorted(h.roots(multiplicities=False),
                       key=lambda z: int(z.to_integer()))
        assert h.degree() == len(roots) == 4
        residues = [branch_residue(h, f, r) for r in roots]
        assert all(z != 0 for z in residues)
        parsed.append((i, h, f, roots, residues))

    four_control = positive_control(K, x, a2, K.from_integer(63))
    five_control = positive_control_five_support(
        K, x, a2, K.from_integer(63))
    h, f, roots, residues = model_from_control(four_control, RK)
    ftest = four_support(h, f, roots, residues, x, K, a2, K.from_integer(63))
    assert ftest[2] and any(hit['pattern']=='four_support' for hit in ftest[2])
    h, f, roots, residues = model_from_control(five_control, RK)
    stest = five_support(h, f, roots, residues, x, K, a2, K.from_integer(63))
    assert any(hit['hidden']==five_control['hidden'] for hit in stest[3])

    per_source, hits = [], []
    totals = {'pairs': 0, 'four_assignments_covered': 0,
              'four_residue_pass': 0, 'five_assignments_covered': 0,
              'five_finite_candidates': 0, 'five_residue_pass': 0}
    for b_code in source_b:
        a6 = K.from_integer(b_code)
        E = EllipticCurve(K, [K.one(), a2, K.zero(), K.zero(), a6])
        assert E.cardinality() == 4126
        row_counts = {key: 0 for key in totals}
        for model_index, h, f, roots, residues in parsed:
            n4, r4, local4 = four_support(
                h, f, roots, residues, x, K, a2, a6)
            n5, f5, r5, local5 = five_support(
                h, f, roots, residues, x, K, a2, a6)
            values = {'pairs': 1, 'four_assignments_covered': n4,
                      'four_residue_pass': r4,
                      'five_assignments_covered': n5,
                      'five_finite_candidates': f5,
                      'five_residue_pass': r5}
            for key, value in values.items():
                totals[key] += value
                row_counts[key] += value
            for hit in local4 + local5:
                hits.append({'source_a2': 32, 'source_a6': b_code,
                             'model_index': model_index, **hit})
        per_source.append({'a2': 32, 'a6': b_code, **row_counts,
                           'hits': sum(z['source_a6']==b_code for z in hits)})
        if b_code == 63:
            assert row_counts['four_assignments_covered'] == earlier['four_support_checked']
            assert row_counts['five_assignments_covered'] == earlier['five_support_checked']
            assert row_counts['five_residue_pass'] == earlier['five_support_residue_pass']
            assert per_source[-1]['hits'] == 0
        print('SOURCE', b_code, 'four_residue', row_counts['four_residue_pass'],
              'five_residue', row_counts['five_residue_pass'],
              'hits', per_source[-1]['hits'], flush=True)

    assert totals['pairs'] == 108 * 140
    assert totals['four_assignments_covered'] == 108 * 140 * 24
    assert totals['five_assignments_covered'] == 108 * 140 * 32744
    output = {
        'schema': 'ghs-q16-4126-degree3-all-sources/v1',
        'protocol_sha256': digest(HERE/'ALL_SOURCES_PROTOCOL.md'),
        'script_sha256': digest(HERE/'fast_screen.py'),
        'catalog_sha256': digest(CATALOG),
        'closure_sha256': digest(CLOSURE),
        'one_source_certificate_sha256': digest(ONE_SOURCE),
        'parent_enumerator_sha256': digest(PARENT/'screen_degree3_pattern.py'),
        'parent_neighbor_sha256': digest(NEIGHBOR/'screen_neighbors.py'),
        'source_a2': 32, 'source_a6_codes': source_b,
        'catalog_label': '3.16.a_a_bd',
        'catalog_models': 140,
        'controls': {'four_support': four_control,
                     'five_support': five_control,
                     'fast_four_hits': len(ftest[2]),
                     'fast_five_hits': len(stest[3])},
        'totals': totals,
        'per_source': per_source,
        'hits': hits,
    }
    (HERE/'all_sources_certificate.json').write_text(
        json.dumps(output, indent=2)+'\n')
    print('FINAL', totals, 'hits', len(hits), flush=True)


if __name__ == '__main__':
    main()
