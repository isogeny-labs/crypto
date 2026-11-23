#!/usr/bin/env python3
"""Independent bit-plane check of F4 translated constants and hulls."""
import hashlib
import json
from pathlib import Path

here = Path(__file__).resolve().parent
protocol = json.loads((here/'protocol.json').read_text())
source_path = here.parent/'118/result.json'
assert hashlib.sha256(source_path.read_bytes()).hexdigest() == protocol['result118_sha256']
source = json.loads(source_path.read_text())
result = json.loads((here/'result.json').read_text())
assert result['result118_sha256'] == protocol['result118_sha256']

for key in protocol['places']:
    rows = {r['lambda_exponent']: r for r in source['branches'][key]['coefficient_rows']}
    k = protocol['edge_k'][key]
    base = rows[0]['s_order']
    sets = {j: {v-k*j-base for v in r['nonzero_support_below_bound']} for j, r in rows.items()}
    bound = min(r['known_until_exclusive']-k*j-base for j, r in rows.items())
    # For alpha: 1,a,a,a^2,a at exponents 0,1,4,8,16.
    # Represent F4=F2[a] as two independently accumulated bit planes.
    low = sets[0] ^ sets[8]
    high = sets[1] ^ sets[4] ^ sets[8] ^ sets[16]
    alpha_support = [[v, (int(v in low) | (int(v in high)<<1))]
                     for v in sorted(low | high) if v < bound]
    # Frobenius a -> a^2: low+high*a maps to (low+high)+high*a.
    conjugate_support = [[v, a ^ (a>>1)] for v, a in alpha_support]
    for entry, expected in zip(result['branches'][key], (alpha_support, conjugate_support)):
        got = entry['translated_rows'][0]
        assert got['known_until_exclusive'] == bound
        assert got['nonzero_support_below_bound'] == expected
        assert got['s_order'] == expected[0][0]
        for j in (1, 4, 8, 16):
            row = next(r for r in entry['translated_rows'] if r['z_exponent'] == j)
            assert row['nonzero_support_below_bound'] == [[v, 1] for v in sorted(sets[j])]
    print(key, 'independent F4 bit planes match')
assert [[r['s_order'] for r in result['branches'][key][0]['translated_rows']]
        for key in protocol['places']] == [
            [8, 23, 23, 0, 0], [8, 23, 23, 0, 0],
            [28, 23, 23, 0, 0], [16, 23, 23, 0, 0]]
print('TRANSLATE119_INDEPENDENT_PASS')
