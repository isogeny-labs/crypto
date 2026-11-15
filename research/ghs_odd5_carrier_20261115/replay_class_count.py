#!/usr/bin/env sage -python
"""Independently recount all 484 ordinary F243 source classes by character sums."""
import collections
import hashlib
import json
from pathlib import Path
from sage.all import GF, PolynomialRing
from sage.version import version as sage_version

P = Path(__file__).resolve().parent
input_bytes = (P / 'class_count.json').read_bytes()
expected = json.loads(input_bytes)
R = PolynomialRing(GF(3), 'x')
x = R.gen()
K = GF(3**5, 'a', modulus=x**5+2*x+1)
elements = list(K)
square_values = {z*z for z in elements}
assert len(square_values) == 122

def from_mask(mask):
    return K(R([(mask // 3**i) % 3 for i in range(5)]))

histogram = collections.Counter()
selected = []
rows = []
for a2 in (1, 2):
    for mask in range(1, 243):
        b = from_mask(mask)
        # For each x there is one y if rhs=0, two if rhs is a nonzero square.
        count = 1
        for u in elements:
            v = u*u*u + a2*u*u + b
            count += 1 if v == 0 else (2 if v in square_values else 0)
        trace = 244-count
        rows.append([a2, mask, trace, count])
        histogram[trace] += 1
        if trace in expected['selected_traces']:
            selected.append([a2, mask, trace, count])

assert len(rows) == 484
assert {str(t): histogram[t] for t in sorted(histogram)} == expected['trace_histogram']
assert selected == expected['selected_rows']
assert len(selected) == expected['selected_class_count'] == 100
result = {
    'schema': 'ghs-odd5-full-source-replay/v1',
    'sage_version': sage_version,
    'script_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    'class_count_sha256': hashlib.sha256(input_bytes).hexdigest(),
    'counted_classes': len(rows),
    'x_rhs_evaluations': len(rows)*len(elements),
    'trace_histogram': {str(t): histogram[t] for t in sorted(histogram)},
    'selected_class_count': len(selected),
    'all_484_rows_match': True,
}
target = P / 'class_replay.json'
if target.exists():
    raise SystemExit('refusing to overwrite immutable replay')
target.write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps({'counted_classes': len(rows), 'selected_class_count': len(selected), 'all_484_rows_match': True}))
