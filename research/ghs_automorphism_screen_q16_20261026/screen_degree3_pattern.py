#!/usr/bin/env sage -python
"""Exact AS test for the four-branch (2,1)/(2,1) degree-three pattern.

The simple zero determines the scalar, so all functions in this pattern are
tested. Other degree-three divisor patterns are outside the search. A
constructed genus-three degree-three cover is a positive control.
"""

import argparse
import hashlib
import itertools
import json
from pathlib import Path

from sage.all import GF, HyperellipticCurve, PolynomialRing

from screen_degree2 import curves, parse_model


ROOT = Path(__file__).resolve().parent
SNAPSHOT = ROOT / 'catalog_models_raw.json'
SOURCE = ROOT.parent / 'ghs_cubic_catalog_q16_20261026/certificate.json'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def pole_order_and_lead(value, root, x):
    numerator, denominator = value.numerator(), value.denominator()
    factor = x - root
    order = 0
    while denominator(root) == 0:
        denominator, remainder = denominator.quo_rem(factor)
        assert remainder == 0
        order += 1
    if order:
        assert numerator(root) != 0
        return order, numerator(root) / denominator(root)
    return 0, None


def reduce_poles(value, roots, x, allow_simple=False):
    """Remove even leading poles; return None on an unremovable odd pole."""
    shift = value.parent().zero()
    for root in roots:
        while True:
            order, lead = pole_order_and_lead(value, root, x)
            if order == 0 or (allow_simple and order == 1):
                break
            if order & 1:
                return None
            term = lead.sqrt() / (x - root) ** (order // 2)
            shift += term
            value += term**2 + term
    return value, shift


def as_coboundary(value, roots, x, K):
    original = value
    reduced = reduce_poles(value, roots, x)
    if reduced is None:
        return None
    value, shift = reduced
    if value.denominator().degree() != 0:
        return None
    polynomial = value.numerator() / value.denominator()[0]
    while polynomial.degree() > 0:
        degree = polynomial.degree()
        if degree & 1:
            return None
        term = polynomial[degree].sqrt() * x ** (degree // 2)
        shift += term
        polynomial += term**2 + term
    if int(polynomial[0].trace()) != 0:
        return None
    constant = next(z for z in K if z**2 + z == polynomial[0])
    shift += constant
    assert original == shift**2 + shift
    return shift


def assert_as_solution(value, shift):
    assert shift is not None
    assert value == shift**2 + shift


def branch_residue(h, f, root):
    slope = h.derivative()(root)
    assert slope != 0
    return f.derivative()(root)/slope**2 + f(root).sqrt()/slope


def scalar_from_simple_zero(h, f, r0, r1, r2, r3, b):
    residue = branch_residue(h, f, r1)
    assert residue != 0
    # The coefficient of (x-r1)^(-1) in b/R must match this residue.
    denominator_at_r1 = (r1-r2)**2 * (r1-r3)
    numerator_slope = (r1-r0)**2
    return b*denominator_at_r1/(residue*numerator_slope)


def second_pattern(h, f, roots, x, K, a2, a6, model_index):
    """One unbranched double zero/pole, four simple branch preimages."""
    b = a6.sqrt()
    residues = [branch_residue(h, f, root) for root in roots]
    checked = 0
    residue_pass = 0
    hits = []
    for single_index in range(4):
        single = roots[single_index]
        others = [roots[j] for j in range(4) if j != single_index]
        product_at_single = (single-others[0])*(single-others[1])*(single-others[2])
        derivative_at_others = [
            (r-others[(j+1)%3])*(r-others[(j+2)%3])
            for j, r in enumerate(others)]
        product_poly = (x-others[0])*(x-others[1])*(x-others[2])
        for hidden in itertools.chain((None,), K):
            if hidden is not None and hidden in roots:
                continue
            hidden_factor_at_single = K.one() if hidden is None else (single-hidden)**2
            hidden_factors = [K.one() if hidden is None else (r-hidden)**2
                              for r in others]
            for orientation in ('double_zero', 'double_pole'):
                checked += 1
                if orientation == 'double_zero':
                    scalar = b*product_at_single/(
                        residues[single_index]*hidden_factor_at_single)
                    assert scalar != 0
                    if any(scalar*hidden_factors[j]*(r-single)/derivative_at_others[j]
                           != residues[roots.index(r)]
                           for j, r in enumerate(others)):
                        continue
                    numerator = ((x-hidden)**2 if hidden is not None else 1)*(x-single)
                    denominator = product_poly
                else:
                    scalar = residues[single_index]*hidden_factor_at_single/product_at_single
                    assert scalar != 0
                    if any(b*hidden_factors[j]*(r-single)/(
                           scalar*derivative_at_others[j])
                           != residues[roots.index(r)]
                           for j, r in enumerate(others)):
                        continue
                    numerator = product_poly
                    denominator = ((x-hidden)**2 if hidden is not None else 1)*(x-single)
                residue_pass += 1
                R = scalar*numerator/denominator
                difference = f/h**2 + R + a2 + a6/R**2
                witness = as_coboundary(difference,
                                        roots if hidden is None else roots+[hidden], x, K)
                if witness is not None:
                    assert_as_solution(difference, witness)
                    hits.append({'model_index': model_index,
                                 'orientation': orientation,
                                 'single_branch_index': single_index,
                                 'hidden': 'infinity' if hidden is None else int(hidden.to_integer()),
                                 'scalar': int(scalar.to_integer()),
                                 'R_numerator': [int(z.to_integer()) for z in (scalar*numerator).list()],
                                 'R_denominator': [int(z.to_integer()) for z in denominator.list()]})
                    print('HIT', hits[-1], flush=True)
    return checked, residue_pass, hits


def check_local_residues(h, f, roots, x, K, a6):
    """Check the fast branch-residue formulas against rational Laurent leads."""
    representative, _ = reduce_poles(f/h**2, roots, x, allow_simple=True)
    assert all(pole_order_and_lead(representative, r, x)
               == (1, branch_residue(h, f, r)) for r in roots)
    single = roots[0]
    others = roots[1:]
    product = (x-others[0])*(x-others[1])*(x-others[2])
    finite_hidden = next(s for s in K if s not in roots)
    for hidden in (None, finite_hidden):
        factor = 1 if hidden is None else (x-hidden)**2
        for orientation in ('double_zero', 'double_pole'):
            if orientation == 'double_zero':
                numerator, denominator = factor*(x-single), product
            else:
                numerator, denominator = product, factor*(x-single)
            R = numerator/denominator
            target = R + a6.sqrt()/R
            for root in roots:
                order, lead = pole_order_and_lead(target, root, x)
                assert order == 1
                if numerator(root) == 0:
                    predicted = a6.sqrt()*denominator(root)/numerator.derivative()(root)
                else:
                    predicted = numerator(root)/denominator.derivative()(root)
                assert lead == predicted


def positive_control(K, x, a2, a6):
    roots = [K.zero(), K.one(), K.gen(), K.gen()**2]
    h = 1
    for root in roots:
        h *= x - root
    for scalar_int in range(1, K.order()):
        scalar = K.from_integer(scalar_int)
        numerator = scalar * (x-roots[0])**2 * (x-roots[1])
        denominator = (x-roots[2])**2 * (x-roots[3])
        R = numerator / denominator
        target = R + a2 + a6 / R**2
        reduced = reduce_poles(target, roots, x, allow_simple=True)
        if reduced is None:
            continue
        representative, shift = reduced
        if any(pole_order_and_lead(representative, root, x)[0] != 1
               for root in roots):
            continue
        f = h**2 * representative
        if f.denominator().degree() != 0:
            continue
        f = f.numerator() / f.denominator()[0]
        if f.degree() > 8 or HyperellipticCurve(f, h).genus() != 3:
            continue
        assert scalar_from_simple_zero(h, f, *roots, a6.sqrt()) == scalar
        test = f / h**2 + target
        witness = as_coboundary(test, roots, x, K)
        assert_as_solution(test, witness)
        # v=R*(y/h+witness) respects v^2+u*v=u^3+a2*u^2+a6.
        assert (R**2 * (f / h**2 + witness**2 + witness)
                == R**3 + a2*R**2 + a6)
        return {'scalar': scalar_int,
                'branch_roots': [int(r.to_integer()) for r in roots],
                'h_coefficients': [int(z.to_integer()) for z in h.list()],
                'f_coefficients': [int(z.to_integer()) for z in f.list()],
                'degree': 3}
    raise AssertionError('positive control not found')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--limit-models', type=int, default=4)
    args = parser.parse_args()
    assert 1 <= args.limit_models <= 4
    archive = json.loads(SNAPSHOT.read_text())
    source = json.loads(SOURCE.read_text())
    exact = next(z for z in source['orders'] if z['order'] == 3974)
    assert exact['catalog_label'] == '3.16.a_a_aet'
    assert source['example']['source_a'] == 32
    assert source['example']['source_b'] == 306
    assert source['example']['subgroup_prime'] == 1987
    row = next(z for z in archive['rows'] if z['label'] == exact['catalog_label'])
    k = GF(16, 'a', modulus='conway')
    K = GF(4096, 'w', modulus='conway')
    embed = k.embeddings(K)[0]
    Rk = PolynomialRing(k, 'x')
    xk = Rk.gen()
    ring = PolynomialRing(K, 'x')
    x = ring.gen()
    a2, a6 = K.from_integer(32), K.from_integer(306)
    control = positive_control(K, x, a2, a6)
    checked = 0
    hits = []
    per_model = []
    second_checked = 0
    second_residue_pass = 0
    for model_index, line in enumerate(curves(row['curves'])[:args.limit_models]):
        h0, f0 = parse_model(line, Rk, k.gen(), xk)
        h = ring([embed(z) for z in h0.list()])
        f = ring([embed(z) for z in f0.list()])
        roots = sorted(h.roots(multiplicities=False), key=lambda z: int(z.to_integer()))
        assert len(roots) == 4 and all(h.derivative()(r) != 0 for r in roots)
        check_local_residues(h, f, roots, x, K, a6)
        local = 0
        for indices in itertools.permutations(range(4)):
            r0, r1, r2, r3 = [roots[i] for i in indices]
            numerator = (x-r0)**2*(x-r1)
            denominator = (x-r2)**2*(x-r3)
            scalar = scalar_from_simple_zero(h, f, r0, r1, r2, r3, a6.sqrt())
            R = scalar * numerator / denominator
            difference = f/h**2 + R + a2 + a6/R**2
            witness = as_coboundary(difference, roots, x, K)
            checked += 1
            local += 1
            if witness is not None:
                assert_as_solution(difference, witness)
                hits.append({'model_index': model_index,
                             'permutation': list(indices),
                             'scalar': int(scalar.to_integer()),
                             'R_numerator': [int(z.to_integer()) for z in (scalar*numerator).list()],
                             'R_denominator': [int(z.to_integer()) for z in denominator.list()]})
                print('HIT', hits[-1], flush=True)
        per_model.append({'model_index': model_index, 'checked': local,
                          'model': line, 'branch_roots': [int(r.to_integer()) for r in roots]})
        local_second, local_residue_pass, local_hits = second_pattern(
            h, f, roots, x, K, a2, a6, model_index)
        second_checked += local_second
        second_residue_pass += local_residue_pass
        hits.extend(local_hits)
        per_model[-1].update({'second_pattern_checked': local_second,
                              'second_pattern_residue_pass': local_residue_pass})
        print('MODEL', model_index, 'pattern_A', local, 'pattern_B', local_second,
              'residue_pass', local_residue_pass, 'hits', len(hits), flush=True)
    output = {'schema': 'ghs-q16-degree3-four-branch-pattern/v1',
              'catalog_snapshot_sha256': sha(SNAPSHOT),
              'source_certificate_sha256': sha(SOURCE),
              'protocol_sha256': sha(ROOT / 'PROTOCOL.md'),
              'script_sha256': sha(Path(__file__)),
              'source_order': 3974, 'source_subgroup_order': 1987,
              'source_a2': 32, 'source_a6': 306,
              'models_checked': args.limit_models,
              'pattern_A_functions_checked': checked,
              'pattern_B_functions_checked': second_checked,
              'pattern_B_residue_pass': second_residue_pass,
              'local_residue_formula_checked': True,
              'positive_control': control,
              'per_model': per_model, 'hits': hits,
              'scope': 'All degree-three rational functions whose Artin-Schreier pullback has exactly four simple branch poles at the catalog branch points; pattern B has one unbranched double zero or pole.'}
    path = ROOT / ('degree3_pattern_%dmodels.json' % args.limit_models)
    path.write_text(json.dumps(output, indent=2)+'\n')
    print('FINAL', checked, 'pattern-A;', second_checked, 'pattern-B;',
          second_residue_pass, 'residue pass;', len(hits), 'hits', flush=True)


if __name__ == '__main__':
    main()
