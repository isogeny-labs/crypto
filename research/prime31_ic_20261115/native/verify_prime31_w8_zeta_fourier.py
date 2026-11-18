#!/usr/bin/env python3
"""Independent original-curve BigInt verifier for the zeta-Walsh 8-point sign evaluator.

The existing verifier recomputes all 8192 Gaussian determinants and actual group sums.
This checker separately derives the complementary-zeta coefficients, without importing
the candidate Rust evaluator or trusting its operation counts.
"""
import itertools
import json
import sys
from pathlib import Path
from verify_prime31_w8_sign_fourier import P, verify

BITS = range(8)
F = tuple(itertools.combinations(BITS, 4))

def subset_fourier(points, target):
    xs = [pt[0] for pt in points]
    ys = [pt[1] for pt in points]
    r, s = target
    terms = {}
    for U in F:
        S = tuple(i for i in BITS if i not in U)
        delta_s = 1
        delta_u = 1
        for i, j in itertools.combinations(S, 2):
            delta_s = delta_s * (xs[j] - xs[i]) % P
        for i, j in itertools.combinations(U, 2):
            delta_u = delta_u * (xs[j] - xs[i]) % P
        a = 1
        for i in S:
            a = a * (xs[i] - r) % P
        terms[U] = (-1 if sum(S) % 2 else 1) * delta_s * delta_u * a % P

    coeff = {}
    for k in range(5):
        for T in itertools.combinations(BITS, k):
            upper = sum(value for U, value in terms.items() if set(T).issubset(U))
            y = 1
            for i in T:
                y = y * ys[i] % P
            coeff[T] = upper * y * pow(s, 4-k, P) % P

    # Independent evaluation directly in the sign cube, without a Walsh kernel.
    vals = []
    for signmask in range(256):
        z = 0
        for T, c in coeff.items():
            parity = sum(bool(signmask & (1 << i)) for i in T) & 1
            z += -c if parity else c
        vals.append(z % P)
    return vals

def main(doc):
    verify(doc)  # independent exact 8192 Gaussian residues AND EC relation truth
    assert len(doc['records']) == 32
    totals = dict(mul=0, inv=0, add=0)
    for row in doc['records']:
        vals = subset_fourier(row['points'], row['target'])
        checksum = sum((i+1)*val for i, val in enumerate(vals)) % P
        assert checksum == row['residue_checksum'], (row['fixture'], row['arm'], 'checksum')
        assert [vals[i] for i in (0,1,63,127,255)] == row['sample_residues']
        assert [i for i, d in enumerate(vals) if d == 0] == row['zero_sign_masks']
        for key in totals:
            v = row['zeta_fp_'+key]
            assert isinstance(v,int) and v>=0
            totals[key] += v
        assert row['zeta_fp_inv'] == 0
        assert row['zeta_fp_mul'] == 1400
        assert row['fourier_fp_mul'] == 5600
        assert int(row['zeta_ns']) >= 0
    for key in totals:
        assert doc['zeta_total_fp_'+key] == totals[key]
    assert totals['mul'] == 44800
    assert totals['inv'] == 0
    assert doc['zeta_total_fp_mul'] < doc['fourier_total_fp_mul']
    assert int(doc['zeta_total_ns']) == sum(int(row['zeta_ns']) for row in doc['records'])
    print('PASS independent complementary-zeta residues on all 8192 signs')
    print('Fp multiplications old/new:', doc['fourier_total_fp_mul'], totals['mul'])
    print('Fp inversions new:', totals['inv'])

if __name__ == '__main__':
    if len(sys.argv) != 2:
        raise SystemExit('usage: verify_prime31_w8_zeta_fourier.py result.json')
    main(json.loads(Path(sys.argv[1]).read_text()))
