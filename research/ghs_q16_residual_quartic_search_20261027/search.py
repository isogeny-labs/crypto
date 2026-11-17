#!/usr/bin/env sage -python
"""Frozen 30,000-draw GF(16) quartic screen for residual cubic classes."""

from collections import Counter
from hashlib import sha256
import json
from pathlib import Path
import random
import sys
import time

from sage.all import Curve, GF


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
PARENT = ROOT/'ghs_binary_cubic_isogeny_q16_20261026'
CLOSURE = ROOT/'ghs_cubic_hess_closure_q16_20261027/certificate.json'
PRIOR = PARENT/'verification.json'
sys.path.insert(0, str(PARENT))
from screen import count_by_roots, count_n1, equation, point_monomials


SEED = 2026102716
DRAW_COUNT = 30000


def digest(path):
    return sha256(path.read_bytes()).hexdigest()


def main():
    closure = json.loads(CLOSURE.read_text())
    prior = json.loads(PRIOR.read_text())
    prior_orders = {int(z['group_order']) for z in prior['checked']}
    targets = [z for z in closure['orders']
               if not z['in_hess_isogeny_closure']
               and z['catalog_has_jacobian'] == 1
               and z['largest_prime'] > 1900
               and z['order'] == 2 * z['largest_prime']]
    orders = {int(z['order']): z for z in targets}
    assert len(orders) == 13
    assert sorted(orders) == [3974,3986,3998,4006,4022,4034,4054,
                              4078,4126,4166,4174,4178,4198]

    k = GF(16, 'a', modulus='conway')
    L2 = GF(256, 'u', modulus='conway')
    K = GF(4096, 'w', modulus='conway')
    e2 = k.embeddings(L2)[0]
    e3 = k.embeddings(K)[0]
    matrix1 = point_monomials(k)
    rng = random.Random(SEED)
    tally = Counter()
    full_order_frequency = Counter()
    witnesses = {}
    started = time.monotonic()
    for index in range(DRAW_COUNT):
        coeff = [1] + [rng.randrange(16) for _ in range(14)]
        tally['drawn'] += 1
        if count_n1(coeff, matrix1, k) != 17:
            continue
        tally['N1_17'] += 1
        if count_by_roots(coeff, k, L2, e2) != 257:
            continue
        tally['N2_257'] += 1
        f = equation(coeff, k)
        if not Curve(f).is_smooth():
            continue
        tally['smooth'] += 1
        N3 = count_by_roots(coeff, k, K, e3)
        if (N3-4097) % 3:
            continue
        tally['integral_c3'] += 1
        c3 = (N3-4097)//3
        order = 4097+c3
        full_order_frequency[order] += 1
        if order not in orders:
            continue
        tally['target_match'] += 1
        if order in witnesses:
            continue
        row = orders[order]
        witnesses[order] = {
            'draw_index': index,
            'coefficients': coeff,
            'point_counts': [17,257,N3],
            'weil_polynomial_coefficients': [1,0,0,c3,0,0,4096],
            'source_order': order,
            'source_models': row['source_models'],
            'subgroup_prime': row['largest_prime'],
            'catalog_label': row['catalog_label'],
            'seen_order_in_prior_screen': order in prior_orders,
        }
        print('WITNESS',order,'draw',index,'prior_order',order in prior_orders,
              flush=True)
        if index % 1000 == 0:
            print('PROGRESS',index,'witnesses',len(witnesses),flush=True)
    output = {
        'schema': 'ghs-q16-residual-quartic-search/v1',
        'protocol_sha256': digest(HERE/'PROTOCOL.md'),
        'script_sha256': digest(HERE/'search.py'),
        'closure_sha256': digest(CLOSURE),
        'prior_verification_sha256': digest(PRIOR),
        'parent_screen_sha256': digest(PARENT/'screen.py'),
        'seed': SEED, 'draw_count': DRAW_COUNT,
        'field_moduli': {'k':str(k.modulus()), 'L2':str(L2.modulus()),
                         'K':str(K.modulus())},
        'target_orders': sorted(orders),
        'prior_witness_orders': sorted(prior_orders),
        'tally': dict(tally),
        'full_order_frequency': {str(n):m for n,m in sorted(full_order_frequency.items())},
        'witnesses': [witnesses[n] for n in sorted(witnesses)],
        'elapsed_seconds_diagnostic': time.monotonic()-started,
    }
    (HERE/'results.json').write_text(json.dumps(output,indent=2)+'\n')
    print('FINAL',json.dumps({'tally':dict(tally),
                              'witness_orders':sorted(witnesses)}),flush=True)


if __name__=='__main__':
    main()
