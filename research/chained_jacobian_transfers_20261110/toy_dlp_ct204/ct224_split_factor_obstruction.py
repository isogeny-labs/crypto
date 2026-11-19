#!/usr/bin/env python3
"""CT224: exact Weil-polynomial splitting and rational-group CRT audit.

Checks algebraic consequences of CT221's frozen zeta polynomial. Does not
independently recount extension-field points or certify a computational map.
"""
import json
import math
import sys

P = 101
SOURCE_ORDER = 109
J_ORDER = 115593846
A_POLY = [1, 0, 296, 0, 29896, 0, 1030301]

def mul(a, b):
    out = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i+j] += x*y
    return out

def curve_count(p, a, b):
    squares = [0] * p
    for y in range(p):
        squares[(y*y) % p] += 1
    return 1 + sum(squares[(x*x*x+a*x+b) % p] for x in range(p))

def crt_projector(orders, index):
    n = math.prod(orders)
    h = n // orders[index]
    return h * pow(h, -1, orders[index]) % n

def quartic_is_irreducible_over_q(p, c):
    # X^4 + c X^2 + p^2. Any rational quadratic factorization
    # either has zero linear coefficients, or symmetric +/- a terms.
    delta = c*c - 4*p*p
    if delta >= 0 and math.isqrt(delta)**2 == delta:
        return False
    for b in (p, -p):
        v = 2*b-c
        if v >= 0 and math.isqrt(v)**2 == v:
            return False
    return True

def analyze(poly=A_POLY):
    e0 = [1, 0, P]
    b = [1, 0, 195, 0, P*P]
    if mul(e0, b) != poly:
        raise ValueError('Weil factorization failed')
    if not quartic_is_irreducible_over_q(P,195):
        raise ValueError('quartic factor is reducible')
    ne = curve_count(P,1,46)
    nc = 1 + sum(
        1 for u in range(P) for y in range(P)
        if y*y%P == (pow(u,9,P)+pow(u,3,P)+46)%P)
    ne0 = curve_count(P,0,1)
    nb = sum(b)
    orders = [ne, ne0, nb]
    if ne != SOURCE_ORDER or nc != ne or ne0 != 102 or nb != 10397:
        raise ValueError('rational point census disagrees with factorization')
    if math.prod(orders) != J_ORDER:
        raise ValueError('Jacobian order disagrees with factorization')
    if any(math.gcd(orders[i],orders[j]) != 1 for i in range(3) for j in range(i)):
        raise ValueError('group factors not pairwise coprime')
    projectors = [crt_projector(orders,i) for i in range(3)]
    if projectors[0] != 80597544 or sum(projectors)%J_ORDER != 1:
        raise ValueError('CRT identity mismatch')
    for i, e in enumerate(projectors):
        for j, n in enumerate(orders):
            if e % n != (1 if i==j else 0):
                raise ValueError('bad CRT projector residue')
        if e*e % J_ORDER != e:
            raise ValueError('non-idempotent CRT scalar')
    return {
      'schema':'ct224-f101-isogeny-factor-obstruction-v1',
      'pcc':'unverified', 'field_prime':P,
      'L_A_coefficients_ascending':poly,
      'L_E0_coefficients_ascending':e0,
      'L_B_coefficients_ascending':b,
      'B_frobenius_characteristic_coefficients_descending':[1,0,195,0,P*P],
      'B_characteristic_irreducible_over_Q':True,
      'E_rational_count':ne, 'C_rational_count':nc,
      'E0_rational_count':ne0, 'B_rational_order':nb,
      'J_rational_order':J_ORDER,
      'orders_E_E0_B':orders, 'crt_projectors_E_E0_B':projectors,
      'all_three_orders_pairwise_coprime':True,
      'scope':'isogeny decomposition over F101 conditional on CT221 zeta counts; Hom between distinct simple factors vanishes over F101; no solver cost claimed'}

def self_test():
    out = analyze()
    rejected = 0
    for bad in (
      [1,0,297,0,29896,0,1030301],
      [1,0,296,0,29897,0,1030301],
      [1,0,296,0,29896,0,1030302],
    ):
        try: analyze(bad)
        except ValueError: rejected += 1
    if rejected != 3:
        raise AssertionError('corrupted Weil coefficients not rejected')
    if all((out['crt_projectors_E_E0_B'][0]+1)%x == (1 if i==0 else 0)
           for i,x in enumerate(out['orders_E_E0_B'])):
        raise AssertionError('wrong projector passed')
    print('CT224_SELFTEST_PASS',out['orders_E_E0_B'],out['crt_projectors_E_E0_B'],rejected)

if __name__ == '__main__':
    if '--self-test' in sys.argv:
        self_test()
    else:
        print(json.dumps(analyze(),sort_keys=True,indent=2))
