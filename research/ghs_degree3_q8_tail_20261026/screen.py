#!/usr/bin/env sage -python
"""Degree-three elliptic cover test on cataloged GF(8) cubic classes."""

import hashlib
import itertools
import json
import sys
from pathlib import Path

from sage.all import EllipticCurve, GF, HyperellipticCurve, PolynomialRing


ROOT = Path(__file__).resolve().parent
PARENT = ROOT.parent
PRIOR = PARENT / 'ghs_automorphism_screen_q16_20261026'
NEIGHBOR = PARENT / 'ghs_degree3_neighbor_screen_q16_20261026'
sys.path.insert(0, str(PRIOR))
sys.path.insert(0, str(NEIGHBOR))
from screen_degree2 import screen_model
from screen_degree3_pattern import (as_coboundary, assert_as_solution,
                                    branch_residue, curves, parse_model,
                                    positive_control, scalar_from_simple_zero,
                                    second_pattern)
from screen_neighbors import positive_control_five_support, rational_record


SOURCE = PARENT / 'ghs_binary_cubic_isogeny_20261026/results_q8.json'
CATALOG = ROOT / 'catalog_models_raw.json'
LABELS = {526: '3.8.a_a_n', 542: '3.8.a_a_bd'}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def first_pattern(h, f, roots, x, K, a2, a6):
    hits = []
    for indices in itertools.permutations(range(4)):
        r0, r1, r2, r3 = [roots[i] for i in indices]
        N = (x-r0)**2*(x-r1)
        D = (x-r2)**2*(x-r3)
        scalar = scalar_from_simple_zero(h, f, r0, r1, r2, r3, a6.sqrt())
        R = scalar*N/D
        difference = f/h**2+R+a2+a6/R**2
        t = as_coboundary(difference, roots, x, K)
        if t is not None:
            assert_as_solution(difference, t)
            hits.append({'pattern': 'four_support',
                         'branch_permutation': list(indices),
                         'R': rational_record(R), 't': rational_record(t)})
    return hits


def main():
    source = json.loads(SOURCE.read_text())
    catalog = json.loads(CATALOG.read_text())
    rows = {z['label']: z for z in catalog['rows']}
    assert set(rows) == set(LABELS.values())
    k = GF(8, 'a', modulus='conway')
    K = GF(512, 'w', modulus='conway')
    embed = k.embeddings(K)[0]
    Rk = PolynomialRing(k, 'x')
    xk = Rk.gen()
    ring = PolynomialRing(K, 'x')
    x = ring.gen()
    V, from_vector, to_vector = K.vector_space(map=True)
    F2 = GF(2)
    linear_cache = {}
    models = {}
    degree_two = []
    for order, label in LABELS.items():
        row = rows[label]
        lines = [line for line in curves(row['curves']) if line.startswith('y^2+')]
        assert len(lines) == int(row['hyp_count'])
        models[order] = []
        for model_index, line in enumerate(lines):
            h0, f0 = parse_model(line, Rk, k.gen(), xk)
            C = HyperellipticCurve(f0, h0)
            assert C.genus() == 3
            h = ring([embed(z) for z in h0.list()])
            f = ring([embed(z) for z in f0.list()])
            roots = sorted(h.roots(multiplicities=False),
                           key=lambda z: int(z.to_integer()))
            reason = None
            if h.degree() != 4 or len(roots) != 4:
                reason = 'not_four_finite_split_simple_roots'
            elif any(h.derivative()(r)==0 or branch_residue(h,f,r)==0
                     for r in roots):
                reason = 'branch_gate_failed'
            if reason is None:
                lift_result = screen_model(h,f,K,ring,x,V,from_vector,
                                           to_vector,F2,linear_cache)
                assert lift_result['status']=='checked'
                degree_two.append({'order':order,'model_index':model_index,
                                   'nonidentity_lifts':lift_result['nonidentity_lifts'],
                                   'branch_stabilizer_count':
                                   lift_result['branch_stabilizer_count']})
            models[order].append({'index': model_index, 'equation': line,
                                  'h': h, 'f': f, 'roots': roots,
                                  'skip': reason})

    controls = {
        'four_support': positive_control(K, x, K.from_integer(1), K.from_integer(9)),
        'five_support': positive_control_five_support(
            K, x, K.from_integer(1), K.from_integer(9))}
    records = []
    hits = []
    skips = []
    count_a = count_b = residue_pass = 0
    for order, label in LABELS.items():
        sources = [z for z in source['source_population'] if z['order']==order]
        assert len(sources) == (27 if order==526 else 18)
        assert all(z['direct_class']=='C' and z['largest_prime_subgroup']==
                   (263 if order==526 else 271) for z in sources)
        for src in sources:
            a2, a6 = K.from_integer(src['a']), K.from_integer(src['b'])
            E = EllipticCurve(K, [K.one(),a2,K.zero(),K.zero(),a6])
            assert E.cardinality()==order
            for model in models[order]:
                identity = {'order':order, 'catalog_label':label,
                            'source_a2':src['a'], 'source_a6':src['b'],
                            'model_index':model['index']}
                if model['skip'] is not None:
                    skips.append({**identity, 'reason':model['skip']})
                    continue
                h,f,roots = model['h'],model['f'],model['roots']
                first = first_pattern(h,f,roots,x,K,a2,a6)
                second_count, second_pass, second = second_pattern(
                    h,f,roots,x,K,a2,a6,model['index'])
                count_a += 24
                count_b += second_count
                residue_pass += second_pass
                for hit in first+second:
                    if 't' not in hit:
                        R = ring(hit['R_numerator'])/ring(hit['R_denominator'])
                        t = as_coboundary(f/h**2+R+a2+a6/R**2,
                                          roots,x,K)
                        assert t is not None
                        hit['t'] = rational_record(t)
                    hits.append({**identity, **hit})
                records.append({**identity,'four_support_checked':24,
                                'five_support_checked':second_count,
                                'five_support_residue_pass':second_pass,
                                'hits':len(first)+len(second)})
        print('ORDER',order,'sources',len(sources),
              'models',len(models[order]),'hits',len(hits),flush=True)
    output = {'schema':'ghs-q8-degree3-tail-screen/v1',
              'source_sha256':sha(SOURCE),
              'catalog_sha256':sha(CATALOG),
              'protocol_sha256':sha(ROOT/'PROTOCOL.md'),
              'script_sha256':sha(Path(__file__)),
              'parent_screen_sha256':sha(PRIOR/'screen_degree3_pattern.py'),
              'parent_neighbor_sha256':sha(NEIGHBOR/'screen_neighbors.py'),
              'source_counts':{str(n):(27 if n==526 else 18) for n in LABELS},
              'catalog_model_counts':{str(n):len(models[n]) for n in LABELS},
              'pairs_checked':len(records), 'pairs_skipped':len(skips),
              'four_support_checked':count_a,
              'five_support_checked':count_b,
              'five_support_residue_pass':residue_pass,
              'positive_controls':controls,
              'degree_two_screen':degree_two,
              'records':records,'skips':skips,'hits':hits,
              'model_equations':{str(n):[z['equation'] for z in models[n]]
                                 for n in LABELS}}
    (ROOT/'certificate.json').write_text(json.dumps(output,indent=2)+'\n')
    print('FINAL',len(records),'pairs',len(skips),'skips',
          count_a,count_b,'functions',len(hits),'hits',flush=True)


if __name__=='__main__':
    main()
