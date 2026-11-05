#!/usr/bin/env python3
"""Exact unit-part precision for the four marked23 x=0 G51 expansions."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
HERE = Path(__file__).resolve().parent
PATHS = {
    'donor51': ROOT/'51/runs/base23-struct51-03/marked23-triangular51.sing',
    'parser105': ROOT/'105/jets105.py', 'result98': ROOT/'98/result.json',
    'result116': ROOT/'116/result.json', 'source117': ROOT/'117/coeff117.py',
    'result117': ROOT/'117/result.json',
}
P = 640
ES = (0, 1, 4, 8, 16)

def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

def evaluate(gterms, base, old, arithmetic):
    x = sum(1 << i for i in base['x_exponents'])
    y = sum(1 << i for i in base['y_exponents'])
    e = base['x_order']
    assert arithmetic.order(x) == e
    unit = x >> e
    unit_exact = unit == 1 and x == 1 << e
    unit_known = P if unit_exact else P-e
    max_x = max(max(t[3]) for t in gterms)
    xp = arithmetic.powers(x, max_x)
    yp = arithmetic.powers(y, 6)
    coeff = {j: {} for j in ES}
    bounds = {j: P for j in ES}
    stats = []
    for le, ye, den, exps in gterms:
        a = 0
        for xe in exps:
            a ^= xp[xe]
        a = arithmetic.mul(a, yp[ye])
        n = arithmetic.order(a)
        d = e * den
        assert d < P
        if n is None:
            until = P-d
        else:
            width = P-n if den == 0 else min(P-n, unit_known)
            assert width > 0
            q = arithmetic.powers(unit, den)[den] & ((1 << width)-1)
            ratio = arithmetic.mul(a >> n, arithmetic.inverse(q, width), width)
            offset = n-d
            until = offset+width
            for k in range(ratio.bit_length()):
                if ratio >> k & 1:
                    v = offset+k
                    coeff[le][v] = coeff[le].get(v, 0)^1
        bounds[le] = min(bounds[le], until)
        stats.append({'lambda_exponent': le, 'y_exponent': ye,
                      'denominator_exponent': den, 'numerator_order': n,
                      'denominator_order': d, 'known_until_exclusive': until})
    rows = []
    for j in ES:
        support = sorted(v for v, c in coeff[j].items() if c and v < bounds[j])
        row = {'lambda_exponent': j, 's_order': support[0] if support else None,
               's_order_lower_bound': support[0] if support else bounds[j],
               'known_until_exclusive': bounds[j], 'nonzero_support_below_bound': support}
        prior = next(r for r in old['coefficient_rows'] if r['lambda_exponent'] == j)
        assert row['known_until_exclusive'] >= prior['known_until_exclusive'], (j, row['known_until_exclusive'], prior['known_until_exclusive'])
        assert [v for v in support if v < prior['known_until_exclusive']] == prior['nonzero_support_below_bound']
        if prior['s_order'] is not None:
            assert row['s_order'] == prior['s_order']
        rows.append(row)
    return {'coefficient_rows': rows, 'newton_lower_hull_if_exact': arithmetic.hull(rows),
            'unit_exact': unit_exact, 'unit_known_exclusive': unit_known,
            'term_stats': stats}

def main():
    protocol = json.loads((HERE/'protocol.json').read_text())
    for name, path in PATHS.items():
        assert hashlib.sha256(path.read_bytes()).hexdigest() == protocol['inputs_sha256'][name], name
    old = json.loads(PATHS['result117'].read_text())
    base = json.loads(PATHS['result116'].read_text())
    assert base['precision_exclusive'] == P
    assert json.loads(PATHS['result98'].read_text())['maximum_exponent'] == 161
    _, g = load('jets105_r118', PATHS['parser105']).parse_source(PATHS['donor51'].read_text())
    assert len(g) == 29
    arithmetic = load('coeff117_r118', PATHS['source117'])
    result = {'protocol_id': protocol['id'], 'inputs_sha256': protocol['inputs_sha256'],
              'precision_base_exclusive': P, 'g_term_count': len(g), 'branches': {}}
    for key, branch in base['branches'].items():
        result['branches'][key] = evaluate(g, branch, old['branches'][key], arithmetic)
    out = Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)
    (out/'result.json').write_text(json.dumps(result, indent=2, sort_keys=True)+'\n')
    print('UNIT118_ALL_CHECKS_PASS')

if __name__ == '__main__':
    main()
