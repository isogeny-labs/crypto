#!/usr/bin/env python3
"""Independent F4-table, coefficient-by-coefficient replay through first N51 terms."""
import hashlib
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROUNDS = HERE.parent
P = json.loads((HERE / 'protocol.json').read_text())
OUT = json.loads((HERE / 'result.json').read_text())
TABLE = ((0, 0, 0, 0), (0, 1, 2, 3), (0, 2, 3, 1), (0, 3, 1, 2))


def times(a, b): return TABLE[a][b]


def f4pow(a, k):
    z = 1
    for _ in range(k): z = times(z, a)
    return z


def conv(a, b, degree):
    z = [0] * (degree + 1)
    for i, x in enumerate(a[:degree + 1]):
        if x:
            for j, y in enumerate(b[:degree - i + 1]):
                if y: z[i + j] ^= times(x, y)
    return z


def value(rows, w, degree):
    result = [0] * (degree + 1)
    powers = [[1] + [0] * degree]
    for _ in range(max(rows) if rows else 0): powers.append(conv(powers[-1], w, degree))
    for j, coeff in rows.items():
        z = conv(coeff, powers[j], degree)
        result = [x ^ y for x, y in zip(result, z)]
    return result


def translated_n(key, alpha, data, path):
    source = data[124]['branches'][key]
    coeff = [{v: 1 for v in row['nonzero_support_below_bound']} for row in source['coefficient_rows']]
    bounds = [row['known_until_exclusive'] for row in source['coefficient_rows']]
    for shift, beta in [(-source['lambda_pole_order'], alpha)] + path:
        new = [{} for _ in range(15)]
        cuts = [None] * 15
        for e in range(15):
            for j in range(e + 1):
                if math.comb(e, j) % 2 == 0: continue
                cut = bounds[e] + shift * e
                cuts[j] = cut if cuts[j] is None else min(cuts[j], cut)
                scale = f4pow(beta, e - j)
                for v, a in coeff[e].items():
                    at = v + shift * e
                    new[j][at] = new[j].get(at, 0) ^ times(a, scale)
        coeff = [{v: a for v, a in row.items() if a and v < cuts[j]}
                 for j, row in enumerate(new)]
        bounds = cuts
    return coeff, bounds


def normalize(rows, q, edge, degree):
    out = {}
    for j, series in rows.items():
        row = [0] * (degree + 1)
        for v, a in series.items():
            at = 7 * v + q * j - edge
            assert at >= 0, (j, v, at)
            if at <= degree: row[at] = a
        out[j] = row
    return out


data = {}
for label, expected in P['input_sha256'].items():
    path = ROUNDS / ('125/branch125.py' if label == '125_source' else label + '/result.json')
    assert hashlib.sha256(path.read_bytes()).hexdigest() == expected
    if label != '125_source': data[int(label)] = json.loads(path.read_text())
assert OUT['input_sha256'] == P['input_sha256']
for key, stage_number in P['terminal_stage'].items():
    for recorded in OUT['branches'][key]:
        alpha = recorded['alpha']
        previous = next(e for e in data[125]['branches'][key] if e['alpha'] == alpha)
        stage = next(e for e in data[stage_number]['branches'][key] if e['alpha'] == alpha)
        degree = recorded['first_nonzero_offset']
        assert degree is not None and degree < min(recorded['g_precision_t_exclusive'], recorded['n_precision_t_exclusive'])
        q = recorded['q']
        terminal = {j: {v: a for v, a in row['nonzero_support_below_bound']}
                    for j, row in zip((0, 1, 4, 8, 16), stage['translated_rows'])}
        g = normalize(terminal, q, recorded['g_edge'], degree)
        u = [0] * (degree + 1)
        w = [1] + u[1:]
        assert value(g, w, 0) == [0]
        for k in range(1, degree + 1):
            w = [1] + u[1:]
            u[k] = value(g, w, k)[k]
        w = [1] + u[1:]
        assert value(g, w, degree) == [0] * (degree + 1)
        path = [tuple(x) for x in previous['substitution_path']]
        n_coeff, n_bounds = translated_n(key, alpha, data, path)
        for j, row in enumerate(previous['final_coefficient_rows']):
            support = [[v, a] for v, a in sorted(n_coeff[j].items())]
            sha = hashlib.sha256(json.dumps(support, separators=(',', ':')).encode()).hexdigest()
            assert sha == row['support_sha256'] and n_bounds[j] == row['known_until_exclusive']
        n = normalize(dict(enumerate(n_coeff)), q, recorded['n_candidate_order'], degree)
        coeff = value(n, w, degree)
        assert coeff[:degree] == [0] * degree
        assert coeff[degree] == recorded['first_nonzero_F4'] != 0
        assert recorded['n51_normalized_order'] == recorded['n_candidate_order'] + degree
        print(key, alpha, 'order', recorded['n51_normalized_order'], 'independent lift PASS')
print('BRANCH126_INDEPENDENT_PASS')
