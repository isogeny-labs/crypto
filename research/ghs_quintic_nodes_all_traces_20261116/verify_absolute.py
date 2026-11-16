#!/usr/bin/env sage -python
"""Rule out every proper geometric factor orbit of an F3-irreducible sextic."""
from pathlib import Path
from sage.all import GF, PolynomialRing
import hashlib
import json

here = Path(__file__).parent
input_path = here / 'FULL.jsonl'
rows = [json.loads(line) for line in input_path.read_text().splitlines()]
carriers = [r for r in rows if r['status'] == 'genus5_carrier']
R = PolynomialRing(GF(3**6, name='w'), names=('x', 'y', 'z'))
checks = []
for r in carriers:
    factors = R(r['sextic']).factor()
    assert len(factors) == 1 and factors[0][1] == 1
    checks.append([r['case'], r['index'], r['trace_t']])

print(json.dumps({
    'schema': 'ghs.quintic-nodes-absolute-replay/v1',
    'source_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    'input_sha256': hashlib.sha256(input_path.read_bytes()).hexdigest(),
    'checked': len(checks),
    'checks': checks,
}, sort_keys=True))
