#!/usr/bin/env python3
"""CT225: geometric-Hom obstruction for the F_101 cubic-cover split factors.

Exact Weil-polynomial and modular matrix arithmetic. The final statement
that ordinary elliptic endomorphism Q-algebras are isogeny invariants over
an algebraic closure uses standard elliptic-curve theory, not a computation.
Does NOT count a complete ECDLP solve or imply a speedup.
"""
import json
import math
import sys

P = 101
R = 109
E_TRACE = -7
E0_TRACE = 0
B_COEFF = 195


def require(cond, message):
    if not cond:
        raise ValueError(message)


def squarefree(n):
    """Signed squarefree part of a nonzero integer (trial division)."""
    require(n != 0, 'zero discriminant has no quadratic CM field')
    sign = -1 if n < 0 else 1
    x = abs(n)
    result = sign
    for d in range(2, math.isqrt(x) + 1):
        k = 0
        while x % d == 0:
            k += 1
            x //= d
        if k % 2:
            result *= d
    return result * x


def matmul(a, b, mod):
    n = len(a)
    return [[sum(a[i][k]*b[k][j] for k in range(n)) % mod
             for j in range(n)] for i in range(n)]


def determinant_mod(a, mod):
    # All pivots are invertible because 109 is prime.
    v = [row[:] for row in a]
    total = 1
    n = len(v)
    for j in range(n):
        pivot = next((i for i in range(j, n) if v[i][j] % mod), None)
        if pivot is None:
            return 0
        if pivot != j:
            v[j], v[pivot] = v[pivot], v[j]
            total = (-total) % mod
        q = v[j][j] % mod
        total = total * q % mod
        inv = pow(q, -1, mod)
        for i in range(j+1, n):
            scale = v[i][j] * inv % mod
            for k in range(j, n):
                v[i][k] = (v[i][k] - scale*v[j][k]) % mod
    return total


def first_rational_109(ascending_coefficients, limit):
    """First n with #A(F_{101^n}) divisible by 109, and modular residues.

    The Frobenius companion matrix has charpoly X^d + ... + a0.
    #A(F_{q^n}) == det(1-Frob^n) by the Weil formula.
    """
    d = len(ascending_coefficients)-1
    require(ascending_coefficients[-1] == 1, 'nonmonic Frobenius')
    companion = [[0] * d for _ in range(d)]
    for j in range(d-1):
        companion[j+1][j] = 1
    for i in range(d):
        companion[i][-1] = -ascending_coefficients[i]
    power = [[int(i == j) for j in range(d)] for i in range(d)]
    first = None
    residues = []
    for n in range(1, limit + 1):
        power = matmul(power, companion, R)
        m = [[(int(i == j)-power[i][j]) % R for j in range(d)] for i in range(d)]
        residue = determinant_mod(m, R)
        if n <= 8:
            residues.append(residue)
        if residue == 0 and first is None:
            first = n
    return first, residues


def analyze(e_trace=E_TRACE, e0_trace=E0_TRACE, b_coefficient=B_COEFF):
    require((e_trace, e0_trace, b_coefficient) == (-7, 0, 195),
            'model differs from CT221/CT224: do not re-use their counts')
    require(P == 101 and R == P+1-e_trace, 'source order mismatch')
    require(b_coefficient*b_coefficient - 4*P*P == -2779,
            'surface quartic discriminant mismatch')
    # P_B(X)=X^4+195X^2+101^2. Squaring each Frobenius root
    # yields (Y^2+195Y+101^2)^2, a square of an elliptic Weil polynomial.
    d_trace = -b_coefficient
    e_trace2 = e_trace*e_trace-2*P
    e0_trace2 = e0_trace*e0_trace-2*P
    q2 = P*P
    disc_e = e_trace2*e_trace2 - 4*q2
    disc_d = d_trace*d_trace - 4*q2
    require((e_trace2, e0_trace2, d_trace) == (-153, -202, -195),
            'degree-two Frobenius traces disagree')
    require(disc_e < 0 and disc_d < 0, 'not imaginary quadratic')
    field_e = squarefree(disc_e)
    field_d = squarefree(disc_d)
    require((field_e, field_d) == (-355, -2779), 'CM fields disagree')
    require(math.gcd(e_trace2, P) == 1 and math.gcd(d_trace, P) == 1,
            'ordinary status not established')
    require(e0_trace == 0 and P > 3, 'supersingular E0 premise failed')
    require(field_e != field_d, 'ordinary E and D can become isogenous')
    # Independent residue computation by powers of companion matrices,
    # not by reusing the trace recurrence or the source group law.
    limit = 1320
    e_hit, e_res = first_rational_109([P, -e_trace, 1], limit)
    e0_hit, e0_res = first_rational_109([P, -e0_trace, 1], limit)
    b_hit, b_res = first_rational_109([P*P, 0, b_coefficient, 0, 1], limit)
    require(e_hit == 1 and e0_hit == 24 and b_hit == 1320,
            'unexpected first 109-primary overlap degree')
    return {
      'schema': 'ct225-geometric-hom-obstruction-v1', 'pcc': 'unverified',
      'characteristic': P, 'source_subgroup_order': R,
      'E_trace_Fp': e_trace, 'E0_trace_Fp': e0_trace,
      'B_Frob_polynomial_descending': [1,0,b_coefficient,0,P*P],
      'E_trace_Fp2': e_trace2, 'E0_trace_Fp2': e0_trace2,
      'B_Fp2_elliptic_factor_trace': d_trace,
      'B_Fp2_isogeny_type': 'D^2 by Frobenius polynomial (Y^2+195Y+10201)^2',
      'E_ordinary_CM_discriminant': disc_e,
      'D_ordinary_CM_discriminant': disc_d,
      'E_CM_squarefree_radicand': field_e,
      'D_CM_squarefree_radicand': field_d,
      'E0_supersingular': True, 'E_and_D_ordinary': True,
      'Hom_over_algebraic_closure_E_to_E0_and_B': 'zero (ordinary/supersingular; distinct CM fields)',
      'bounded_modular_109_census_limit': limit,
      'first_degree_order_divisible_by_109': {'E':e_hit,'E0':e0_hit,'B':b_hit},
      'first_eight_group_order_residues_mod_109': {'E':e_res,'E0':e0_res,'B':b_res},
      'qualification': 'geometric Hom obstruction on complementary factors; abstract finite group homomorphisms can exist in extensions; full J remains live; no cold-cost test',
    }


def self_test():
    v = analyze()
    require(v['first_degree_order_divisible_by_109'] == {'E':1,'E0':24,'B':1320},
            'bad first-hit audit')
    rejected = 0
    for args in ({'e_trace':-6},{'e0_trace':1},{'b_coefficient':196}):
        try:
            analyze(**args)
        except ValueError:
            rejected += 1
    require(rejected == 3, 'mutated input escaped model lock')
    require(squarefree(-17395) == -355 and squarefree(-2779) == -2779,
            'squarefree-part controls failed')
    print('CT225_SELFTEST_PASS', v['E_CM_squarefree_radicand'],
          v['D_CM_squarefree_radicand'],
          v['first_degree_order_divisible_by_109'], rejected)


if __name__ == '__main__':
    if '--self-test' in sys.argv:
        self_test()
    else:
        print(json.dumps(analyze(), indent=2, sort_keys=True))
