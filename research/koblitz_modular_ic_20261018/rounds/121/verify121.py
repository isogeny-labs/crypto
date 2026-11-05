#!/usr/bin/env python3
"""Independent F4 table and support check of the third translation."""
import hashlib
import json
from pathlib import Path

here = Path(__file__).resolve().parent
p = json.loads((here/'protocol.json').read_text())
source_path = here.parent/'120/result.json'
assert hashlib.sha256(source_path.read_bytes()).hexdigest() == p['result120_sha256']
source = json.loads(source_path.read_text())
result = json.loads((here/'result.json').read_text())

def product(c, factor):
    # Explicit F4 multiplication tables for factors 1,a,a^2.
    return {1:(0,1,2,3),2:(0,2,3,1),3:(0,3,1,2)}[factor][c]

def exp_factor(beta, j):
    if j == 0: return 1
    if j in (1,4,16): return beta
    if j == 8: return product(beta,beta)
    raise AssertionError(j)

for key, m in p['place_edge_m'].items():
    for prior, current in zip(source['branches'][key], result['branches'][key]):
        beta = current['gamma']
        assert current['alpha'] == prior['alpha']
        old = {r['w_exponent']:r for r in prior['translated_rows']}
        constant = {}
        bounds = {}
        for j in (0,1,4,8,16):
            offset = m*j-8*m
            bounds[j] = old[j]['known_until_exclusive']+offset
            shifted = [[v+offset,c] for v,c in old[j]['nonzero_support_below_bound']]
            if j:
                actual = next(r for r in current['translated_rows'] if r['u_exponent']==j)
                assert actual['known_until_exclusive'] == bounds[j]
                assert actual['nonzero_support_below_bound'] == shifted
            for v,c in shifted:
                constant[v] = constant.get(v,0)^product(c,exp_factor(beta,j))
        bound = min(bounds.values())
        expected = [[v,c] for v,c in sorted(constant.items()) if c and v<bound]
        actual = current['translated_rows'][0]
        assert actual['known_until_exclusive']==bound
        assert actual['nonzero_support_below_bound']==expected
        assert actual['s_order']==expected[0][0]
    print(key,'independent F4 table supports match')
assert [[r['s_order'] for r in result['branches'][key][0]['translated_rows']]
        for key in ('y0_e1','y0_e3')] == [[6,2,11,0,24],[8,9,15,0,16]]
print('TRANSLATE121_INDEPENDENT_PASS')
