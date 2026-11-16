#!/usr/bin/env sage -python
"""Find K-rational lifts of branch-set permutations for promising models."""

import argparse
import collections
import hashlib
import itertools
import json
import re
from pathlib import Path

from sage.all import GF, HyperellipticCurve, PolynomialRing, matrix, sage_eval, vector


ROOT = Path(__file__).resolve().parent
SNAPSHOT = ROOT / 'catalog_models_raw.json'
SOURCE = ROOT.parent / 'ghs_cubic_catalog_q16_20261026/certificate.json'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def curves(raw):
    if raw in ('{}', '\\N'):
        return []
    assert raw.startswith('{') and raw.endswith('}')
    return raw[1:-1].split(',')


def parse_model(line, ring, a, x):
    match = re.fullmatch(r'y\^2\+\((.+)\)\*y=(.+)', line)
    assert match is not None
    h = ring(sage_eval(match.group(1), locals={'a': a, 'x': x}))
    f = ring(sage_eval(match.group(2), locals={'a': a, 'x': x}))
    return h, f


def bitvec(poly, n, to_vector, F2):
    return vector(F2, [int(v) for i in range(n) for v in to_vector(poly[i])])


def screen_model(h, f, K, R, x, V, from_vector, to_vector, F2, linear_cache):
    roots = [r for r, multiplicity in h.roots() if multiplicity == 1]
    if len(roots) != 4:
        return {'status': 'roots_not_split_in_K', 'root_count': len(roots)}
    derivative = h.derivative()
    branch = [f.derivative()(r)/derivative(r)**2 +
              f(r).sqrt()/derivative(r) for r in roots]
    if any(v == 0 for v in branch):
        return {'status': 'branch_gate_failed'}
    lifted = []
    stabilizer = 0
    dim = K.degree()
    for perm in itertools.permutations(range(4)):
        M = matrix(K, [[roots[i], K.one(), roots[i]*roots[perm[i]],
                        roots[perm[i]]] for i in range(3)])
        assert M.rank() == 3
        A, B, C, D = M.right_kernel().basis()[0]
        if A*D+B*C == 0 or C*roots[3]+D == 0:
            continue
        if (A*roots[3]+B)/(C*roots[3]+D) != roots[perm[3]]:
            continue
        stabilizer += 1
        if perm == tuple(range(4)):
            continue
        num, den = A*x+B, C*x+D
        H = sum(h[i]*num**i*den**(4-i) for i in range(5))
        e = H[4]/h[4]
        assert e != 0 and H == e*h
        F = sum(f[i]*num**i*den**(8-i) for i in range(9))
        rhs = F + e**2*f
        cache_key = (tuple(int(h[i].to_integer()) for i in range(5)),
                     int(e.to_integer()))
        if cache_key not in linear_cache:
            columns = []
            for j in range(5):
                for bit in range(dim):
                    u = from_vector(V.gen(bit))*x**j
                    columns.append(bitvec(u**2+e*h*u, 9, to_vector, F2))
            L = matrix(F2, columns).transpose()
            linear_cache[cache_key] = (L, L.left_kernel().basis_matrix())
        L, left_kernel = linear_cache[cache_key]
        b = bitvec(rhs, 9, to_vector, F2)
        if any(left_kernel*b):
            continue
        v = L.solve_right(b)
        u = sum(from_vector(V(v[j*dim:(j+1)*dim]))*x**j for j in range(5))
        assert u**2+e*h*u == rhs
        lifted.append({'branch_permutation': list(perm),
                       'mobius_abcd': [int(z.to_integer()) for z in (A,B,C,D)],
                       'y_multiplier': int(e.to_integer()),
                       'y_shift_coefficients': [int(u[i].to_integer()) for i in range(5)]})
    return {'status': 'checked', 'branch_stabilizer_count': stabilizer,
            'nonidentity_lifts': lifted}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--all-nonsparse', action='store_true')
    args = parser.parse_args()
    mode = 'all_nonsparse' if args.all_nonsparse else 'gamma0_nonsparse'
    archive = json.loads(SNAPSHOT.read_text())
    source_by_label = {z['catalog_label']: z for z in json.loads(SOURCE.read_text())['orders']}
    k = GF(16, 'a', modulus='conway')
    K = GF(4096, 'w', modulus='conway')
    embed = k.embeddings(K)[0]
    Rk = PolynomialRing(k, 'x')
    xk = Rk.gen()
    R = PolynomialRing(K, 'x')
    x = R.gen()
    V, from_vector, to_vector = K.vector_space(map=True)
    F2 = GF(2)
    counts = collections.Counter()
    results = []
    per_class = collections.defaultdict(collections.Counter)
    control = None
    linear_cache = {}
    for row in archive['rows']:
        prior = source_by_label[row['label']]
        for idx, line in enumerate(curves(row['curves'])):
            if prior['direct_B_order'] and not (row['label'] == '3.16.a_a_aex' and idx == 0):
                continue
            h0, f0 = parse_model(line, Rk, k.gen(), xk)
            sparse = (h0.degree()==4 and h0[4]!=0 and h0[1]!=0 and
                      h0[0]==h0[2]==h0[3]==0)
            if prior['direct_B_order']:
                is_control = True
            else:
                is_control = False
                if sparse or (not args.all_nonsparse and f0[3] != 0):
                    continue
                counts['outside_' + mode + '_selected'] += 1
                per_class[row['label']]['selected'] += 1
            if h0.degree() != 4 or HyperellipticCurve(f0,h0).genus() != 3:
                counts['model_gate_skip'] += 1
                continue
            assert f0.degree() <= 8
            h = R([embed(v) for v in h0.list()])
            f = R([embed(v) for v in f0.list()])
            result = screen_model(h,f,K,R,x,V,from_vector,to_vector,F2,linear_cache)
            if result['status'] == 'checked':
                counts['models_checked'] += 1
                if result['nonidentity_lifts']:
                    counts['models_with_lift'] += 1
            else:
                counts[result['status']] += 1
            record = {'label': row['label'], 'order': prior['order'],
                      'model_index': idx, 'model': line, **result}
            if is_control:
                control = record
            else:
                per_class[row['label']][result['status']] += 1
                if result['status'] == 'checked' and result['nonidentity_lifts']:
                    per_class[row['label']]['with_lift'] += 1
                if not args.all_nonsparse or result['status'] != 'checked' or result['nonidentity_lifts']:
                    results.append(record)
                if result['status'] == 'checked' and result['nonidentity_lifts']:
                    print('POSITIVE', row['label'], idx, line, flush=True)
            if not is_control and counts['outside_' + mode + '_selected'] % (250 if args.all_nonsparse else 25) == 0:
                print('screened', counts['outside_' + mode + '_selected'], dict(counts), flush=True)
    assert counts['outside_' + mode + '_selected'] == (6272 if args.all_nonsparse else 358)
    assert control is not None and control['status'] == 'checked'
    assert len(control['nonidentity_lifts']) >= 3
    output = {'schema': 'ghs-q16-degree2-' + mode + '-screen/v1',
              'catalog_snapshot_sha256': sha(SNAPSHOT),
              'source_certificate_sha256': sha(SOURCE),
              'protocol_sha256': sha(ROOT / 'PROTOCOL.md'),
              'script_sha256': sha(Path(__file__)),
              'counts': dict(sorted(counts.items())),
              'linear_systems_cached': len(linear_cache),
              'direct_B_positive_control': control,
              'per_class_summary': {label: dict(value) for label, value in sorted(per_class.items())},
              'outside_results': results,
              'limit': 'K-rational degree-two lift screen of selected nonsparse listed models only'}
    name = 'degree2_all_nonsparse_certificate.json' if args.all_nonsparse else 'degree2_gamma0_certificate.json'
    (ROOT / name).write_text(json.dumps(output,indent=2)+'\n')
    print('FINAL', json.dumps(output['counts'],sort_keys=True), flush=True)


if __name__ == '__main__':
    main()
