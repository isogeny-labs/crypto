#!/usr/bin/env python3
"""CT75: source-local finite-ring torsion Hensel, digit enumeration, odd-prime test.

Checks public subgroup elements independently, without giving the lift their
scalar labels. Affine formulas only evaluate nonzero multiples < ell, where all
secant/tangent denominators are p-adic units. Not a global characteristic hop.
All checks are explicit under python -O.
"""
import json

FIXTURES = ((29, 3, (18, 5), 6), (29, 5, (5, 5), 6),
            (37, 7, (9, 4), 4), (59, 11, (19, 22), 3),
            (139, 13, (41, 9), 3))

def check(condition, message):
    if not condition:
        raise ArithmeticError(message)

def rhs(x):
    return x*x*x-x*x-10*x-20

def valid(q, modulus):
    x, y = q
    return (y*y+y-rhs(x)) % modulus == 0

def negative(q, modulus):
    return (q[0], (-q[1]-1) % modulus)

def add(a, b, modulus, p):
    x, y = a
    z, w = b
    if a == b:
        denominator = 2*y+1
        numerator = 3*x*x-2*x-10
    else:
        denominator = z-x
        numerator = w-y
    check(denominator % p != 0, 'nonunit affine group-law denominator')
    slope = numerator*pow(denominator, -1, modulus) % modulus
    X = (slope*slope+1-x-z) % modulus
    return X, (-slope*X-y+slope*x-1) % modulus

def multiply(k, q, modulus, p):
    check(k > 0, 'positive multiplication only')
    ans = None
    base = q
    while k:
        if k & 1:
            ans = base if ans is None else add(ans, base, modulus, p)
        k >>= 1
        if k:
            base = add(base, base, modulus, p)
    return ans

def torsion_test(q, ell, modulus, p):
    h = (ell-1)//2
    return multiply(h, q, modulus, p) == negative(multiply(h+1, q, modulus, p), modulus)

def ordinate_lift(x, old_y, place, p):
    error = old_y*old_y+old_y-rhs(x)
    check(error % place == 0, 'invalid old curve point')
    yy = old_y + place*(-(error//place)*pow(2*old_y+1, -1, p) % p)
    check(valid((x, yy), place*p), 'ordinate lift incorrect')
    return yy

def next_lift(q, ell, place, p):
    x, y = q
    target_modulus = place*p
    matches = []
    for digit in range(p):
        xx = x+digit*place
        yy = ordinate_lift(xx, y, place, p)
        if torsion_test((xx, yy), ell, target_modulus, p):
            matches.append((xx, yy))
    check(len(matches) == 1, 'nonunique torsion x-digit')
    return matches[0]

def main():
    for p, ell, generator, NMAX in FIXTURES:
        check(p not in (2, 11) and ell != p, 'bad reduction or torsion prime')
        check(valid(generator, p) and torsion_test(generator, ell, p, p), 'invalid source')
        # The scalar is used only to generate and later verify toy public test vectors.
        # Each lift below receives only the encoded affine point, not the scalar.
        public_inputs = [multiply(k, generator, p, p) for k in range(1, ell)]
        check(len(set(public_inputs)) == ell-1, 'duplicate public point')
        lifts = list(public_inputs)
        place = p
        for n in range(1, NMAX+1):
            if n > 1:
                lifts = [next_lift(q, ell, place, p) for q in lifts]
                place *= p
            base = lifts[0]
            check(valid(base, place) and torsion_test(base, ell, place, p), 'bad generator lift')
            for k, q in enumerate(lifts, 1):
                check(valid(q, place) and torsion_test(q, ell, place, p), 'bad public lift')
                check((q[0] % p, q[1] % p) == public_inputs[k-1], 'wrong source reduction')
                check(multiply(k, base, place, p) == q, 'encoded target DLP relation lost')
            naive_y = generator[1]
            old_place = p
            for _ in range(1, n):
                naive_y = ordinate_lift(generator[0], naive_y, old_place, p)
                old_place *= p
            naive = (generator[0], naive_y)
            check(valid(naive, place), 'naive curve lift invalid')
            naive_torsion = torsion_test(naive, ell, place, p)
            record = {
                'N': n, 'all_public_nonzero': ell-1, 'curve': True,
                'ell': ell, 'modulus': str(place),
                'naive_torsion': naive_torsion,
                'point': [str(a) for a in base],
                'second_multiple': [str(a) for a in lifts[1]],
                'source': [str(a) for a in generator],
                'source_prime': p,
                'verified_relations': ell-1,
            }
            print(json.dumps(record, sort_keys=True, separators=(',', ':')))

if __name__ == '__main__':
    main()
