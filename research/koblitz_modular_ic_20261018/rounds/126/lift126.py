#!/usr/bin/env python3
"""SHA-bound F4 Hensel lift of the four marked23 tame-seven branches."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROUNDS = HERE.parent


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def f4mul(a, b):
    z = 0
    if b & 1: z ^= a
    if b & 2: z ^= a << 1
    if z & 4: z ^= 7
    return z


def bits_from_terms(terms, cutoff):
    lo = hi = 0
    for exp, coeff in terms:
        assert exp >= 0 or coeff == 0, (exp, coeff)
        if 0 <= exp < cutoff:
            if coeff & 1: lo ^= 1 << exp
            if coeff & 2: hi ^= 1 << exp
    return lo, hi


def add(a, b):
    return a[0] ^ b[0], a[1] ^ b[1]


def binary_mul(a, b, mask):
    out = 0
    while b:
        low = b & -b
        out ^= a << (low.bit_length() - 1)
        b ^= low
    return out & mask


def mul(a, b, mask):
    p00 = binary_mul(a[0], b[0], mask)
    p11 = binary_mul(a[1], b[1], mask)
    return p00 ^ p11, binary_mul(a[0] ^ a[1], b[0] ^ b[1], mask) ^ p00


def power(a, n, mask):
    out = (1, 0)
    while n:
        if n & 1: out = mul(out, a, mask)
        a = mul(a, a, mask)
        n >>= 1
    return out


def scalar(a, n):
    return ((a[0] >> n) & 1) | (((a[1] >> n) & 1) << 1)


def support_digest(a, cutoff):
    pairs = [[n, scalar(a, n)] for n in range(cutoff) if scalar(a, n)]
    return hashlib.sha256(json.dumps(pairs, separators=(',', ':')).encode()).hexdigest(), pairs


def normalized_rows(rows, q, edge, cutoff, terminal=False):
    result = []
    bounds = []
    for j, row in enumerate(rows):
        if terminal:
            j = (0, 1, 4, 8, 16)[j]
            terms = row['nonzero_support_below_bound']
        else:
            terms = row[1].items()
        bound = 7 * (row['known_until_exclusive'] if terminal else row[0]) + q * j - edge
        bounds.append(bound)
        result.append((j, bits_from_terms(((7 * v + q * j - edge, a) for v, a in terms), cutoff)))
    assert min(bounds) >= cutoff, min(bounds)
    return result, min(bounds)


def evaluate(rows, w, mask):
    out = (0, 0)
    for j, a in rows:
        if a != (0, 0): out = add(out, mul(a, power(w, j, mask), mask))
    return out


def main(dest):
    protocol = json.loads((HERE / 'protocol.json').read_text())
    cutoff = protocol['precision_t_exclusive']
    mask = (1 << cutoff) - 1
    data = {}
    for label, expected in protocol['input_sha256'].items():
        path = ROUNDS / ('125/branch125.py' if label == '125_source' else label + '/result.json')
        assert digest(path) == expected, label
        if label != '125_source': data[int(label)] = json.loads(path.read_text())
    spec = importlib.util.spec_from_file_location('branch125_pinned', ROUNDS / '125/branch125.py')
    old = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(old)
    result = {'protocol_id': protocol['id'], 'input_sha256': protocol['input_sha256'],
              'precision_t_exclusive': cutoff, 'branches': {}}
    for key, stage_number in protocol['terminal_stage'].items():
        entries = []
        for previous in data[125]['branches'][key]:
            alpha = previous['alpha']
            stage = next(e for e in data[stage_number]['branches'][key] if e['alpha'] == alpha)
            local = stage['translated_rows']
            q = local[1]['s_order'] - local[3]['s_order']
            edge = 7 * local[1]['s_order'] + q
            assert previous['tame_seven']['correction_order_in_normalized_parameter'] == q
            g_rows, g_bound = normalized_rows(local, q, edge, cutoff, terminal=True)
            table = dict(g_rows)
            assert scalar(table[1], 0) == 1
            assert scalar(evaluate(g_rows, (1, 0), mask), 0) == 0
            a1 = table[1]
            inverse = (1, 0)
            for _ in range(cutoff.bit_length()):
                inverse = mul(a1, mul(inverse, inverse, mask), mask)
            assert mul(a1, inverse, mask) == (1, 0)
            u = (0, 0)
            for _ in range(cutoff.bit_length() + 2):
                w = add((1, 0), u)
                residual = evaluate(g_rows, w, mask)
                update = mul(residual, inverse, mask)
                next_u = add(u, update)
                if next_u == u: break
                u = next_u
            else: raise AssertionError('Hensel iteration did not converge')
            w = add((1, 0), u)
            assert evaluate(g_rows, w, mask) == (0, 0)
            assert scalar(w, 0) == 1
            source = data[124]['branches'][key]
            poly = old.translate(old.source_poly(source['coefficient_rows']),
                                 -source['lambda_pole_order'], alpha)
            for shift, beta in previous['substitution_path']:
                poly = old.translate(poly, shift, beta)
            for row, recorded in zip(old.summary(poly), previous['final_coefficient_rows']):
                assert row == recorded, (key, alpha, row['correction_exponent'])
            n_edge = previous['tame_seven']['candidate_normalized_order']
            n_rows, n_bound = normalized_rows(poly, q, n_edge, cutoff)
            n_value = evaluate(n_rows, w, mask)
            n_sha, n_support = support_digest(n_value, cutoff)
            u_sha, u_support = support_digest(u, cutoff)
            first = n_support[0] if n_support else None
            entries.append({'alpha': alpha, 'q': q, 'g_edge': edge,
                            'g_precision_t_exclusive': g_bound,
                            'n_candidate_order': n_edge,
                            'n_precision_t_exclusive': n_bound,
                            'u_support_sha256': u_sha, 'u_first_support': u_support[:12],
                            'n_support_sha256': n_sha, 'n_first_support': n_support[:12],
                            'first_nonzero_offset': first[0] if first else None,
                            'first_nonzero_F4': first[1] if first else None,
                            'n51_normalized_order': n_edge + first[0] if first else None,
                            'n51_order_lower_bound': n_edge + cutoff if not first else None})
        assert entries[0]['first_nonzero_offset'] == entries[1]['first_nonzero_offset']
        if entries[0]['first_nonzero_F4'] is not None:
            assert f4mul(entries[0]['first_nonzero_F4'], entries[0]['first_nonzero_F4']) == entries[1]['first_nonzero_F4']
        result['branches'][key] = entries
    dest.mkdir(parents=True, exist_ok=True)
    (dest / 'result.json').write_text(json.dumps(result, indent=2, sort_keys=True) + '\n')
    print('BRANCH126_HENSEL_AND_N51_PASS')


if __name__ == '__main__':
    main(Path(sys.argv[1]))
