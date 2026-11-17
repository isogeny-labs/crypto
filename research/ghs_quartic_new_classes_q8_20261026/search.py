#!/usr/bin/env sage -python
"""Frozen extension of the exact GF(8) plane-quartic class screen."""

import hashlib
import json
import random
import sys
import time
from pathlib import Path

from sage.all import Curve, GF

ROOT = Path(__file__).resolve().parent
PRIOR = ROOT.parent / 'ghs_binary_cubic_isogeny_20261026'
sys.path.insert(0, str(PRIOR))
from screen import count_zero, point_monomials, quartic
from screen_q8 import point_count_by_roots

SOURCE = PRIOR / 'results_q8.json'
TARGETS = (478, 482, 524, 548, 554, 556)
SEED = 2026102642
DRAW_COUNT = 20000


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    start = time.monotonic()
    prior = json.loads(SOURCE.read_text())
    by_order = {}
    for row in prior['source_population']:
        by_order.setdefault(row['order'], []).append(row)
    assert len(prior['source_population']) == 1022
    for order in TARGETS:
        rows = by_order[order]
        assert not any(row['direct_class'] == 'B' for row in rows)
        assert max(row['largest_prime_subgroup'] for row in rows) >= 127
        assert min(row['cofactor'] for row in rows) <= 4
        assert order not in {row['group_order'] for row in prior['witnesses']}
    k = GF(8, 'a', modulus='conway')
    k2 = GF(64, 'u', modulus='conway')
    K = GF(512, 'w', modulus='conway')
    e2 = k.embeddings(k2)[0]
    e3 = k.embeddings(K)[0]
    mat1 = point_monomials(k)
    mat2 = point_monomials(k2)
    rng = random.Random(SEED)
    tallies = {'drawn': 0, 'N1': 0, 'N2': 0, 'smooth': 0,
               'integral_trace': 0, 'source_match': 0,
               'target_match': 0}
    witnesses = {}
    for i in range(DRAW_COUNT):
        coeffs = [1] + [rng.randrange(8) for _ in range(14)]
        tallies['drawn'] += 1
        if count_zero(coeffs, mat1, lambda a: a, k) != 9:
            continue
        tallies['N1'] += 1
        if count_zero(coeffs, mat2, e2, k) != 65:
            continue
        tallies['N2'] += 1
        F = quartic(coeffs, k, lambda a: a, k)
        if not Curve(F).is_smooth():
            continue
        tallies['smooth'] += 1
        N3 = point_count_by_roots(coeffs, k, K, e3)
        if (513-N3) % 3:
            continue
        tallies['integral_trace'] += 1
        t = (513-N3)//3
        order = 513-t
        if order not in by_order:
            continue
        tallies['source_match'] += 1
        if order not in TARGETS:
            continue
        tallies['target_match'] += 1
        if order not in witnesses:
            witnesses[order] = {'draw_index': i,
                                'coefficients': coeffs,
                                'equation': str(F),
                                'N1_N2_N3': [9, 65, N3],
                                'weil_polynomial_coefficients':
                                [1, 0, 0, -t, 0, 0, 512],
                                'group_order': order,
                                'sources': by_order[order]}
            print('WITNESS', order, 'draw', i, flush=True)
        if i % 1000 == 0:
            print('PROGRESS', i, tallies, flush=True)
    result = {'schema': 'ghs-q8-new-quartic-classes/v1',
              'source_sha256': sha(SOURCE),
              'protocol_sha256': sha(ROOT/'PROTOCOL.md'),
              'script_sha256': sha(Path(__file__)),
              'seed': SEED, 'draw_count': DRAW_COUNT,
              'targets': list(TARGETS),
              'tallies': tallies,
              'witnesses': [witnesses[n] for n in sorted(witnesses)],
              'elapsed_seconds_diagnostic': time.monotonic()-start}
    (ROOT/'search_results.json').write_text(json.dumps(result, indent=2,
                                                       sort_keys=True)+'\n')
    print('FINAL', tallies, 'orders', sorted(witnesses), flush=True)


if __name__ == '__main__':
    main()
