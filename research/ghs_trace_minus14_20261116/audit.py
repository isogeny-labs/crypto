#!/usr/bin/env python3
"""Structural receipt audit; not an independent algebraic elimination."""
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
NAMES = (
    'degree4_all', 'degree5_screen', 'degree6_screen', 'sparse',
    'linear_square', 'quadratic_square', 'cubic_square',
    'two_poles_linear', 'three_poles',
)
out = {}
for name in NAMES:
    source = (HERE / (name + '.sage')).read_bytes()
    receipt = json.loads((HERE / (name + '.json')).read_text())
    assert receipt['source_sha256'] == hashlib.sha256(source).hexdigest(), name
    assert receipt.get('trace', -14) == -14, name
    assert receipt.get('target_order', 258) == 258, name
    assert receipt.get('bmask', receipt.get('a6_mask', 11)) == 11, name
    assert receipt['hits'] == [], name
    assert receipt.get('censored', []) == [], name
    assert receipt.get('counts', {}) or name == 'degree4_all', name
    out[name] = {
        'source_sha256': receipt['source_sha256'],
        'hits': len(receipt['hits']),
        'counts': receipt['counts'],
        'branch_assignment_count': receipt.get('branch_assignment_count'),
    }

assert sum(v['assignments'] for v in out['degree4_all']['counts'].values()) == 59697
assert out['degree5_screen']['counts']['square_quadratic_nonbranch']['logical_assignments'] == 58801 * 243
assert out['degree6_screen']['branch_assignment_count'] == 14506074
assert out['sparse']['counts']['one']['branch_assignments'] == 243
assert out['linear_square']['counts']['H2']['logical_assignments'] == 58806
assert out['quadratic_square']['counts']['Hquadratic']['systems'] == 243
assert out['quadratic_square']['counts']['QiHquadratic']['systems'] == 405
assert out['cubic_square']['counts']['systems'] == 243
assert out['cubic_square']['counts']['censored_systems'] == 0
assert out['two_poles_linear']['counts']['logical_candidates'] == 7906140
assert out['three_poles']['counts']['logical_candidates'] == 643043610
print(json.dumps({'schema': 'ghs.trace-minus14.structural-audit/v1',
                  'scope': 'one exact carrier/source pair',
                  'checks': out}, sort_keys=True))
