#!/usr/bin/env sage -python
"""Check the sparse-branch quotient obstruction across the archived catalog."""

import collections
import hashlib
import json
import re
from pathlib import Path

from sage.all import GF, HyperellipticCurve, PolynomialRing, sage_eval


ROOT = Path(__file__).resolve().parent
SNAPSHOT = ROOT / 'catalog_models_raw.json'
SOURCE = ROOT.parent / 'ghs_cubic_catalog_q16_20261026/certificate.json'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def curve_lines(raw):
    if raw in ('{}', '\\N'):
        return []
    assert raw.startswith('{') and raw.endswith('}')
    return raw[1:-1].split(',')


def parse_model(line, ring, a, x):
    match = re.fullmatch(r'y\^2\+\((.+)\)\*y=(.+)', line)
    assert match is not None, line
    h = ring(sage_eval(match.group(1), locals={'a': a, 'x': x}))
    f = ring(sage_eval(match.group(2), locals={'a': a, 'x': x}))
    return h, f


def main():
    archive = json.loads(SNAPSHOT.read_text())
    source = json.loads(SOURCE.read_text())
    source_by_label = {row['catalog_label']: row for row in source['orders']}
    assert len(archive['rows']) == len(source_by_label) == 128
    assert {row['label'] for row in archive['rows']} == set(source_by_label)
    k = GF(16, 'a', modulus='conway')
    K = GF(4096, 'w', modulus='conway')
    embed = k.embeddings(K)[0]
    Rk = PolynomialRing(k, 'x')
    xk = Rk.gen()
    R = PolynomialRing(K, 'x')
    x = R.gen()
    cuberoots = [c for c in K if c**3 == 1]
    assert len(cuberoots) == 3
    counts = collections.Counter()
    per_class = []
    positive_control = None

    for row in archive['rows']:
        order = source_by_label[row['label']]
        family = 'direct' if order['direct_B_order'] else 'outside'
        lines = curve_lines(row['curves'])
        assert len(lines) == int(row['hyp_count'])
        if order['catalog_has_jacobian'] == 0:
            assert not lines
        own = collections.Counter()
        for idx, line in enumerate(lines):
            h0, f0 = parse_model(line, Rk, k.gen(), xk)
            counts['all_models'] += 1
            own['all_models'] += 1
            counts[family + '_models'] += 1
            sparse = (h0.degree() == 4 and h0[4] != 0 and h0[1] != 0 and
                      h0[0] == h0[2] == h0[3] == 0)
            if not sparse:
                continue
            counts[family + '_sparse_h'] += 1
            own['sparse_h'] += 1
            assert f0.degree() <= 8
            assert all(f0[i] == 0 for i in range(4, 8))
            gamma0 = f0[3] == 0
            if gamma0:
                counts[family + '_sparse_gamma0'] += 1
                own['sparse_gamma0'] += 1
            else:
                counts[family + '_sparse_gamma_nonzero'] += 1
                own['sparse_gamma_nonzero'] += 1
            curve = HyperellipticCurve(f0, h0)
            assert curve.genus() == 3
            h = R([embed(v) for v in h0.list()])
            f = R([embed(v) for v in f0.list()])
            roots = [r for r, multiplicity in h.roots() if multiplicity == 1]
            assert len(roots) == 4
            beta = h[1]
            branch = [f.derivative()(r)/beta**2 + f(r).sqrt()/beta for r in roots]
            assert all(v != 0 for v in branch)
            if not gamma0:
                gamma = f[3]
                for c in cuberoots:
                    for d in roots:
                        assert h(c*x+d) == c*h
                        rhs = f(c*x+d) + c**2*f
                        if c == 1 and d == 0:
                            assert rhs == 0
                        elif c != 1:
                            assert rhs.degree() == 3 and rhs[3] == gamma*(1+c**2)
                        else:
                            assert rhs.degree() == 2 and rhs[2] == gamma*d
                counts['verified_no_geometric_degree2_models'] += (family == 'outside')
            if row['label'] == '3.16.a_a_aex' and idx == 0:
                assert gamma0
                translations = [d for d in roots if d != 0 and f(x+d) == f]
                assert len(translations) == 3
                positive_control = {'label': row['label'], 'model': line,
                                    'translation_count': len(translations)}
        if own:
            per_class.append({'label': row['label'], 'order': order['order'],
                              'direct_B_order': order['direct_B_order'],
                              **dict(own)})
        print(f"{row['label']} {family} models={len(lines)} sparse={own['sparse_h']}", flush=True)

    assert positive_control is not None
    assert counts['all_models'] == 10664
    assert counts['direct_models'] == 3516 and counts['outside_models'] == 7148
    assert counts['direct_sparse_h'] == 456 and counts['outside_sparse_h'] == 876
    assert counts['direct_sparse_gamma0'] == 104
    assert counts['outside_sparse_gamma0'] == 0
    assert counts['verified_no_geometric_degree2_models'] == 876
    output = {'schema': 'ghs-q16-sparse-quotient-obstruction/v1',
              'catalog_snapshot_sha256': sha(SNAPSHOT),
              'source_certificate_sha256': sha(SOURCE),
              'protocol_sha256': sha(ROOT / 'PROTOCOL.md'),
              'script_sha256': sha(Path(__file__)),
              'counts': dict(sorted(counts.items())),
              'positive_control': positive_control,
              'per_class': per_class,
              'conclusion': '876 catalog-listed outside-direct models satisfy the geometric degree-two quotient obstruction',
              'limit': 'does not exclude other models, higher-degree maps, or an attack through an isogeny'}
    (ROOT / 'sparse_certificate.json').write_text(json.dumps(output, indent=2) + '\n')
    print(json.dumps(output['counts'], sort_keys=True), flush=True)


if __name__ == '__main__':
    main()
