#!/usr/bin/env python3
"""Second precision-aware F4 translation of marked23 repeated edges."""
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent/'119/result.json'
ES = (0, 1, 4, 8, 16)

def mul(a, b):
    z = 0
    for i in (0, 1):
        if b >> i & 1:
            z ^= a << i
    if z & 4:
        z ^= 7
    return z

def power(a, n):
    z = 1
    for _ in range(n):
        z = mul(z, a)
    return z

def order_row(j, values, bound):
    support = [[v, a] for v, a in sorted(values.items()) if a and v < bound]
    return {'w_exponent': j, 's_order': support[0][0] if support else None,
            's_order_lower_bound': support[0][0] if support else bound,
            'known_until_exclusive': bound, 'nonzero_support_below_bound': support}

def hull(rows):
    points = [(r['w_exponent'], r['s_order']) for r in rows]
    if any(v is None for _, v in points):
        return None
    out = []
    for p in points:
        while len(out) >= 2:
            a, b = out[-2:]
            if (b[0]-a[0])*(p[1]-b[1])-(b[1]-a[1])*(p[0]-b[0]) <= 0:
                out.pop()
            else:
                break
        out.append(p)
    return [{'w_exponent': j, 's_order': v} for j, v in out]

def translate(entry, m):
    old = {r['z_exponent']: r for r in entry['translated_rows']}
    assert old[0]['s_order'] == 8*m and old[8]['s_order'] == 0
    constant_lead = old[0]['nonzero_support_below_bound'][0][1]
    beta = next(a for a in (1, 2, 3) if power(a, 8) == constant_lead)
    normalized = {}
    for j in ES:
        shift = m*j-8*m
        values = {v+shift: a for v, a in old[j]['nonzero_support_below_bound']}
        bound = old[j]['known_until_exclusive']+shift
        assert min(values, default=0) >= 0 and bound > 0
        normalized[j] = values, bound
    c = {}
    for j, (values, _) in normalized.items():
        factor = power(beta, j)
        for v, a in values.items():
            c[v] = c.get(v, 0)^mul(a, factor)
    rows = [order_row(0, c, min(bound for _, bound in normalized.values()))]
    rows += [order_row(j, *normalized[j]) for j in ES[1:]]
    return {'alpha': entry['alpha'], 'beta': beta, 'edge_m': m,
            'translated_rows': rows, 'lower_hull_if_exact': hull(rows)}

def main():
    protocol = json.loads((HERE/'protocol.json').read_text())
    assert hashlib.sha256(SOURCE.read_bytes()).hexdigest() == protocol['result119_sha256']
    previous = json.loads(SOURCE.read_text())
    result = {'protocol_id': protocol['id'], 'result119_sha256': protocol['result119_sha256'], 'branches': {}}
    for key, m in protocol['place_edge_m'].items():
        entries = [translate(entry, m) for entry in previous['branches'][key]]
        assert entries[0]['beta'] != entries[1]['beta']
        for a, b in zip(entries[0]['translated_rows'], entries[1]['translated_rows']):
            assert a['s_order'] == b['s_order']
            assert a['known_until_exclusive'] == b['known_until_exclusive']
            assert [[v, c^(c>>1)] for v, c in a['nonzero_support_below_bound']] == b['nonzero_support_below_bound']
        result['branches'][key] = entries
    out = Path(sys.argv[1]); out.mkdir(parents=True, exist_ok=True)
    (out/'result.json').write_text(json.dumps(result, sort_keys=True, indent=2)+'\n')
    print('TRANSLATE120_ALL_CHECKS_PASS')

if __name__ == '__main__':
    main()
