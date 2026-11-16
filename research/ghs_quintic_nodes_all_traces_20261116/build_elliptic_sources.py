#!/usr/bin/env sage -python
"""Find and independently count an exact elliptic source for each eligible class."""
from pathlib import Path
from sage.all import GF, EllipticCurve
import hashlib
import json

here = Path(__file__).parent
analysis_path = here / 'ANALYSIS.json'
analysis = json.loads(analysis_path.read_text())
wanted = set(analysis['eligible_unmodeled_traces'])
K = GF(3**5, name='u')
u = K.gen()
elements = list(K)
assert len(elements) == 243
records = {}

def trace_by_character(a2, a4, a6):
    total = 0
    for x in elements:
        v = x**3 + a2*x*x + a4*x + a6
        total += 0 if v == 0 else (1 if v**121 == 1 else -1)
    return -total

for a2 in (K(1), K(2)):
    for a6 in elements:
        if a6 == 0:
            continue  # singular for this subfamily
        t = trace_by_character(a2, K(0), a6)
        if t in wanted and t not in records:
            E = EllipticCurve(K, [0, a2, 0, 0, a6])
            assert E.cardinality() == 244-t
            records[t] = {'trace_t': t, 'ainvs': [str(v) for v in E.a_invariants()],
                          'order': int(E.cardinality()), 'supersingular': bool(E.is_supersingular())}

# The ordinary short-form family omits the supersingular trace -27 class.
if -27 in wanted:
    a2, a4, a6 = K(0), K(2), u**4
    assert trace_by_character(a2, a4, a6) == -27
    E = EllipticCurve(K, [0, a2, 0, a4, a6])
    assert E.cardinality() == 271 and E.is_supersingular()
    records[-27] = {'trace_t': -27, 'ainvs': [str(v) for v in E.a_invariants()],
                    'order': 271, 'supersingular': True}

assert set(records) == wanted
print(json.dumps({'schema': 'ghs.quintic-nodes-elliptic-sources/v1',
                  'source_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  'input_sha256': hashlib.sha256(analysis_path.read_bytes()).hexdigest(),
                  'field_modulus': str(K.modulus()), 'sources': [records[t] for t in sorted(records)]},
                 sort_keys=True))
