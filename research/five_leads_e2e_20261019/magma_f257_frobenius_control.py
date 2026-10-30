#!/usr/bin/env python3
"""Reject the invalid rational-3-torsion shortcut using the F257 control."""

import json
from pathlib import Path

HERE = Path(__file__).resolve().parent


def rank_mod3(matrix):
    rows = [[x % 3 for x in row] for row in matrix]
    rank = 0
    for col in range(len(rows[0])):
        pivot = next((i for i in range(rank, len(rows)) if rows[i][col]), None)
        if pivot is None:
            continue
        rows[rank], rows[pivot] = rows[pivot], rows[rank]
        inverse = pow(rows[rank][col], -1, 3)
        rows[rank] = [(x * inverse) % 3 for x in rows[rank]]
        for i in range(len(rows)):
            if i != rank:
                multiple = rows[i][col]
                rows[i] = [(x - multiple * y) % 3 for x, y in zip(rows[i], rows[rank])]
        rank += 1
    return rank


def euler_coefficients(expression):
    out = [0] * 7
    for term in expression.replace(' ', '').replace('-', '+-').split('+'):
        if not term:
            continue
        if '*x^' in term:
            coefficient, exponent = term.split('*x^')
            out[int(exponent)] += int(coefficient)
        elif '*x' in term:
            out[1] += int(term.split('*x')[0])
        else:
            out[0] += int(term)
    return out


def divides_mod3(dividend, divisor):
    work = [x % 3 for x in dividend]
    leading_inverse = pow(divisor[-1] % 3, -1, 3)
    for offset in range(len(work) - len(divisor), -1, -1):
        multiple = work[offset + len(divisor) - 1] * leading_inverse % 3
        for i, value in enumerate(divisor):
            work[offset + i] = (work[offset + i] - multiple * value) % 3
    return all(value == 0 for value in work)


def main():
    order_receipt = json.loads((HERE / 'correctness/magma-tian-f257-group-control-v1.json').read_text())
    kernel = json.loads((HERE / 'correctness/tian-genus3-f257-kernel-gate-v1.json').read_text())
    order = order_receipt['order']
    assert order_receipt['response']['success'] and order_receipt['curl_returncode'] == 0
    assert kernel['field_modulus_irreducible_by_rabin']
    assert kernel['generators_order_three'] == 3
    assert kernel['distinct_kernel_elements'] == 27 and kernel['frobenius_stable']
    assert order % 690212 == 0
    lines = order_receipt['response']['stdout'].splitlines()
    euler = lines[lines.index('EULER') + 1]
    euler_coeffs = euler_coefficients(euler)
    euler_at_one = sum(euler_coeffs)
    assert euler_at_one == order
    valuation = 0
    remaining = order
    while remaining % 3 == 0:
        remaining //= 3
        valuation += 1
    images = kernel['frobenius_generator_images']
    matrix = [[images[col][row] for col in range(3)] for row in range(3)]
    shifted = [[(matrix[i][j] - (i == j)) % 3 for j in range(3)] for i in range(3)]
    fixed_dimension = 3 - rank_mod3(shifted)
    trace = sum(matrix[i][i] for i in range(3))
    principal_minors = sum(matrix[i][i] * matrix[j][j] - matrix[i][j] * matrix[j][i] for i in range(3) for j in range(i + 1, 3))
    determinant = (matrix[0][0] * (matrix[1][1] * matrix[2][2] - matrix[1][2] * matrix[2][1])
                   - matrix[0][1] * (matrix[1][0] * matrix[2][2] - matrix[1][2] * matrix[2][0])
                   + matrix[0][2] * (matrix[1][0] * matrix[2][1] - matrix[1][1] * matrix[2][0]))
    kernel_charpoly = [(-determinant) % 3, principal_minors % 3, (-trace) % 3, 1]
    assert divides_mod3(list(reversed(euler_coeffs)), kernel_charpoly)
    assert valuation == 1 and fixed_dimension == 1
    print(json.dumps({'schema': 'magma-f257-frobenius-control/v1',
                      'jacobian_order': order, 'euler_factor_at_one': euler_at_one,
                      'source_point_order': 690212,
                      'jacobian_3_adic_valuation': valuation,
                      'stable_kernel_size': kernel['distinct_kernel_elements'],
                      'frobenius_matrix_mod3_rows': matrix,
                      'kernel_charpoly_mod3_ascending': kernel_charpoly,
                      'kernel_charpoly_divides_jacobian_charpoly_mod3': True,
                      'kernel_fixed_dimension': fixed_dimension,
                      'rational_rank3_filter_would_reject_positive_control': True,
                      'odd_kernel_isotropy_checked': False}, sort_keys=True))


if __name__ == '__main__':
    main()
