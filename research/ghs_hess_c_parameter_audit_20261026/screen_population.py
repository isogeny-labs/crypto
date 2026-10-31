#!/usr/bin/env sage -python
"""Apply Hess's published cubic trace-zero factor criterion to the q8 census."""

import hashlib
import json
from collections import Counter
from pathlib import Path

from sage.all import GF

root = Path(__file__).resolve().parent
source = root.parent / 'ghs_binary_cubic_isogeny_20261026' / 'results_q8.json'
raw = source.read_bytes()
population = json.loads(raw)['source_population']
assert len(population) == 1022
K = GF(512, 'w', modulus='conway')
w = K.gen()
assert w**9 + w**4 + 1 == 0


def relative_trace(x):
    return x + x**8 + x**64


trace_zero = [x for x in K if x and relative_trace(x) == 0]
assert len(trace_zero) == 63
product_pairs = {}
for gamma in trace_zero:
    for beta in trace_zero:
        product_pairs.setdefault(gamma * beta, (gamma, beta))

direct_orders = {row['order'] for row in population
                 if row['direct_class'] == 'B'}
assert len(direct_orders) == 12
records = []
for row in population:
    b = K.from_integer(row['b'])
    sqrt_b = b**256
    pair = product_pairs.get(sqrt_b)
    if row['a'] == 0 and pair is not None:
        gamma, beta = pair
        assert gamma * beta == sqrt_b
        assert relative_trace(gamma) == relative_trace(beta) == 0
        assert gamma and beta
        records.append({'a2': row['a'], 'b': row['b'],
                        'order': row['order'],
                        'largest_prime_subgroup': row['largest_prime_subgroup'],
                        'direct_class': row['direct_class'],
                        'inside_direct_B_order_closure': row['order'] in direct_orders,
                        'full_degree_b': b**8 != b,
                        'gamma': int(gamma.to_integer()),
                        'beta': int(beta.to_integer()),
                        'sqrt_b': int(sqrt_b.to_integer())})

result = {'schema': 'ghs-hess-cubic-tracezero-population/v1',
          'source_sha256': hashlib.sha256(raw).hexdigest(),
          'field_polynomial': str(K.modulus()),
          'trace_zero_nonzero_count': len(trace_zero),
          'product_set_size': len(product_pairs),
          'population_count': len(population),
          'a2_zero_population': sum(row['a'] == 0 for row in population),
          'criterion_model_count': len(records),
          'criterion_order_count': len({r['order'] for r in records}),
          'criterion_inside_direct_order_closure': sum(
              r['inside_direct_B_order_closure'] for r in records),
          'criterion_outside_direct_order_closure': sum(
              not r['inside_direct_B_order_closure'] for r in records),
          'criterion_full_degree_count': sum(r['full_degree_b'] for r in records),
          'by_direct_class': dict(Counter(r['direct_class'] for r in records)),
          'records': records}
(root / 'population_screen.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps({k: v for k, v in result.items() if k != 'records'}, indent=2))
print('outside_first', [r for r in records if not r['inside_direct_B_order_closure']][:12])
