#!/usr/bin/env python3
"""Source-bound degree-7 Hensel lift and N51 valuation at x=1."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

P = 40
ROOT = Path(__file__).resolve().parents[1]
PATHS = {
    'donor51': ROOT/'51/runs/base23-struct51-03/marked23-triangular51.sing',
    'audit102': ROOT/'102/audit102.py', 'result107': ROOT/'107/result.json',
    'result108': ROOT/'108/result.json', 'result109': ROOT/'109/result.json',
    'norm109': ROOT/'109/norm109.py',
}


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def add(a, b):
    return [x ^ y for x, y in zip(a, b)]


def mul(a, b, f4mul):
    out = [0] * P
    for i, x in enumerate(a):
        if not x: continue
        for j, y in enumerate(b[:P-i]):
            if y: out[i+j] ^= f4mul(x, y)
    return out


def power(a, n, f4mul):
    out = [1] + [0] * (P-1)
    for _ in range(n): out = mul(out, a, f4mul)
    return out


def g_polynomial(rows):
    terms = {}
    precision_bounds = []
    for row in rows:
        j = row['u_exponent']
        precision_bounds.append(7*32+23*j-184)
        a = [0] * P
        for e in row['one_support']:
            k = 7*e+23*j-184
            if 0 <= k < P: a[k] ^= 1
        for e in row['alpha_support']:
            k = 7*e+23*j-184
            if 0 <= k < P: a[k] ^= 2
        terms[j] = a
    assert min(precision_bounds) >= P
    assert terms[1][0] == terms[8][0] == 1
    assert not any(terms[j][0] for j in (0,4,16))
    return terms, min(precision_bounds)


def evaluate(terms, v, f4mul):
    out = [0]*P
    for j, a in terms.items(): out = add(out, mul(a, power(v, j, f4mul), f4mul))
    return out


def lift_v(terms, f4mul):
    v = [1]+[0]*(P-1)
    assert evaluate(terms, v, f4mul)[0] == 0
    for k in range(1, P):
        v[k] = evaluate(terms, v, f4mul)[k]
    assert evaluate(terms, v, f4mul) == [0]*P
    return v


def n_polynomial(parsed, z_exponents, shift, norm):
    z = sum(1 << i for i in z_exponents)
    t = norm.mul(2, z); x = 1 ^ t; y = z ^ shift
    coefficients = {j: {} for j in range(15)}
    bounds = []
    for le, ye, num, den in parsed:
        offset, bits, until, _, _ = norm.rational_series(num, den, x, y, ye)
        for j in range(le+1):
            if j & ~le: continue
            bounds.append(7*until+23*j)
            scalar = norm.f4pow(2, le-j)
            for k in range(bits.bit_length()):
                if bits >> k & 1:
                    e = offset+k
                    coefficients[j][e] = coefficients[j].get(e, 0) ^ scalar
    known = min(bounds)
    assert known > P
    terms = {}
    for j, c in coefficients.items():
        a = [0]*P
        for e, scalar in c.items():
            k = 7*e+23*j
            if scalar and 0 <= k < P: a[k] ^= scalar
            if scalar and k < 0: raise ValueError('negative N weighted order')
        terms[j] = a
    return terms, known


def main():
    output = Path(sys.argv[1]); protocol = json.loads((Path(__file__).with_name('protocol.json')).read_text())
    for name, path in PATHS.items():
        if hashlib.sha256(path.read_bytes()).hexdigest() != protocol['input_sha256'][name]:
            raise ValueError('source hash mismatch: '+name)
    norm = load('norm109', PATHS['norm109']); audit = load('audit102', PATHS['audit102'])
    parsed = norm.parse_terms(PATHS['donor51'].read_text(), audit)
    base = json.loads(PATHS['result108'].read_text())
    g = json.loads(PATHS['result107'].read_text())
    prior = json.loads(PATHS['result109'].read_text())
    out = {'precision_r_exclusive': P, 'source_sha256': protocol['input_sha256'], 'term_count': len(parsed)}
    for key, shift in (('y0',0), ('y1',1)):
        assert prior[key]['degree_seven']['leading_F4_sum'] == 0
        gt, gb = g_polynomial(g[key]['translated_rows'])
        v = lift_v(gt, norm.f4mul)
        nt, nb = n_polynomial(parsed, base[key]['z_exponents'], shift, norm)
        n = evaluate(nt, v, norm.f4mul)
        assert n[0] == 0
        nz = [(i,a) for i,a in enumerate(n) if a]
        out[key] = {'g_precision_bound_r': gb, 'n_precision_bound_r': nb,
                    'v_nonzero': [(i,a) for i,a in enumerate(v) if a],
                    'n_nonzero': nz, 'n_valuation': nz[0][0] if nz else None}
    output.mkdir(parents=True, exist_ok=True)
    (output/'result.json').write_text(json.dumps(out,indent=2,sort_keys=True)+'\n')
    print('LIFT110_ALL_CHECKS_PASS')

if __name__ == '__main__': main()
