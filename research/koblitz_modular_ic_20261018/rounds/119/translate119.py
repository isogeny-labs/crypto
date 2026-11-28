#!/usr/bin/env python3
"""Precision-aware F4 translation of the four R118 local lambda equations."""
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent/'118/result.json'
ES = (0, 1, 4, 8, 16)

def fmul(a, b):
    z = 0
    for i in (0, 1):
        if b >> i & 1:
            z ^= a << i
    if z & 4:
        z ^= 0b111   # a^2+a+1 = 0
    return z

def fpow(a, n):
    z = 1
    for _ in range(n):
        z = fmul(z, a)
    return z

def hull(rows):
    points = [(r['z_exponent'], r['s_order']) for r in rows]
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
    return [{'z_exponent': j, 's_order': v} for j, v in out]

def translated(branch, k, alpha):
    source = {r['lambda_exponent']: r for r in branch['coefficient_rows']}
    base = source[0]['s_order']
    assert base is not None
    normalized = {}
    for j in ES:
        r = source[j]
        shift = k*j+base
        support = [v-shift for v in r['nonzero_support_below_bound']]
        bound = r['known_until_exclusive']-shift
        assert min(support, default=0) >= 0 and bound > 0
        normalized[j] = (support, bound)
    constant = {}
    for j, (support, _) in normalized.items():
        factor = fpow(alpha, j)
        for v in support:
            constant[v] = constant.get(v, 0)^factor
    rows = []
    for j in ES:
        if j == 0:
            bound = min(b for _, b in normalized.values())
            support = [[v, a] for v, a in sorted(constant.items()) if a and v < bound]
        else:
            support = [[v, 1] for v in normalized[j][0]]
            bound = normalized[j][1]
        rows.append({'z_exponent': j, 's_order': support[0][0] if support else None,
                     's_order_lower_bound': support[0][0] if support else bound,
                     'known_until_exclusive': bound,
                     'nonzero_support_below_bound': support})
    return {'alpha': alpha, 'edge_k': k, 'edge_base_order': base,
            'translated_rows': rows, 'lower_hull_if_exact': hull(rows)}

def main():
    protocol = json.loads((HERE/'protocol.json').read_text())
    assert hashlib.sha256(SOURCE.read_bytes()).hexdigest() == protocol['result118_sha256']
    source = json.loads(SOURCE.read_text())
    out = {'protocol_id': protocol['id'], 'result118_sha256': protocol['result118_sha256'], 'branches': {}}
    for key in protocol['places']:
        entries = [translated(source['branches'][key], protocol['edge_k'][key], a) for a in protocol['roots']]
        for left, right in zip(entries[0]['translated_rows'], entries[1]['translated_rows']):
            assert left['s_order'] == right['s_order']
            assert left['known_until_exclusive'] == right['known_until_exclusive']
            assert [[v, fmul(a, a)] for v, a in left['nonzero_support_below_bound']] == right['nonzero_support_below_bound']
        out['branches'][key] = entries
    dest = Path(sys.argv[1])
    dest.mkdir(parents=True, exist_ok=True)
    (dest/'result.json').write_text(json.dumps(out, indent=2, sort_keys=True)+'\n')
    print('TRANSLATE119_ALL_CHECKS_PASS')

if __name__ == '__main__':
    main()
