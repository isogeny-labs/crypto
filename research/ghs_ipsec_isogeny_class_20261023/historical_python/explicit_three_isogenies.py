#!/usr/bin/env python3
"""Explicit normalized degree-three Velu maps for the IPsec curve.

For a kernel +/-Q with x(Q)=q, q^4+q^3+b=0.  The normalized target
is y^2+xy=x^3+a*x^2+(b+q+q^2).  The map is verified by an exact
polynomial identity, in addition to point and group-law checks.
"""
import json
from pathlib import Path
import random
from analyze import F, IPSEC_B, IPSEC_ORDER
from isogeny_search import add, trim, rational_roots, ModularPolynomial


def mul_poly(p, q):
    r = [0]*max(0, len(p)+len(q)-1)
    for i, a in enumerate(p):
        for j, b in enumerate(q):
            if a and b:
                r[i+j] ^= F.mul(a, b)
    return trim(r)


def sq_poly(p):
    r = [0]*max(0, 2*len(p)-1)
    for i, a in enumerate(p):
        r[2*i] = F.sq(a)
    return trim(r)


def scale(p, a):
    return [F.mul(v, a) for v in p]


def point_on_curve(point, b, a=0):
    if point is None:
        return True
    x, y = point
    return F.sq(y) ^ F.mul(x,y) == F.mul(F.sq(x),x) ^ F.mul(a,F.sq(x)) ^ b


def ec_add(p, q, a=0):
    if p is None:
        return q
    if q is None:
        return p
    x, y = p
    u, v = q
    if x == u:
        if y != v or x == 0:
            return None
        lam = x ^ F.mul(y, F.inv(x))
        w = F.sq(lam) ^ lam ^ a
        z = F.sq(x) ^ F.mul(lam ^ 1,w)
    else:
        lam = F.mul(y ^ v,F.inv(x ^ u))
        w = F.sq(lam) ^ lam ^ x ^ u ^ a
        z = F.mul(lam,x ^ w) ^ w ^ y
    return w,z


def scalar(point, n, a=0):
    result = None
    while n:
        if n & 1:
            result = ec_add(result,point,a)
        point = ec_add(point,point,a)
        n >>= 1
    return result


def sample_point(b, rng, a=0):
    while True:
        x = rng.getrandbits(155)
        if not x:
            continue
        rhs = x ^ a ^ F.mul(b,F.inv(F.sq(x)))
        if F.trace(rhs):
            continue
        v, ht = rhs, 0
        for _ in range(78):
            ht ^= v
            v = F.sq(F.sq(v))
        point = x,F.mul(x,ht)
        assert point_on_curve(point,b,a)
        return point


def image(point, q):
    if point is None:
        return None
    x,y = point
    d = x ^ q
    if not d:
        return None
    inv_d = F.inv(d)
    inv_d2 = F.sq(inv_d)
    inv_d3 = F.mul(inv_d,inv_d2)
    a = 1 ^ F.mul(q,inv_d2)
    b = F.mul(F.mul(q,F.sq(x)),inv_d3) ^ F.mul(F.pow(q,3),inv_d2) ^ q
    return F.mul(x,a), F.mul(y,a) ^ b


def verify_identity(q, b):
    # Substitute the map and the source equation, clear denominator (x+q)^6.
    d = [q,1]
    d2 = sq_poly(d)
    d3 = mul_poly(d2,d)
    d6 = sq_poly(d3)
    an = add(d2,[q])
    bn = add(add([0,0,q],scale(d,F.pow(q,3))),scale(d3,q))
    terms = [mul_poly(mul_poly(sq_poly(an),[b,0,0,1]),d2),
             sq_poly(bn),
             mul_poly(mul_poly(mul_poly([0,1],an),bn),d),
             mul_poly([0,0,0,1],mul_poly(sq_poly(an),an)),
             scale(d6,b ^ q ^ F.sq(q))]
    result = []
    for term in terms:
        result = add(result,term)
    assert result == [], result
    # y coefficient vanishes since x' = x*(1+q/(x+q)^2).


def main():
    directory = Path(__file__).parent
    modular = ModularPolynomial(3,directory/'sources/phi_j_3.txt')
    kernel_xs = rational_roots([IPSEC_B,0,0,1,1])
    target_js = rational_roots(modular.at(F.inv(IPSEC_B)))
    assert len(kernel_xs) == 2
    records = []
    for q in kernel_xs:
        new_b = IPSEC_B ^ q ^ F.sq(q)
        assert F.inv(new_b) in target_js
        verify_identity(q,IPSEC_B)
        rng = random.Random(2409)
        for _ in range(8):
            p, r = sample_point(IPSEC_B,rng),sample_point(IPSEC_B,rng)
            fp, fr = image(p,q),image(r,q)
            assert point_on_curve(fp,new_b) and point_on_curve(fr,new_b)
            assert image(ec_add(p,r),q) == ec_add(fp,fr)
            assert scalar(fp,IPSEC_ORDER) is None
        records.append({'kernel_x_hex': hex(q), 'target_b_hex': hex(new_b),
                        'target_j_hex': hex(F.inv(new_b)),
                        'rational_function_identity': 'exactly verified',
                        'group_law_point_checks': 8,
                        'target_order_annihilation_checks': 8})
    # Screened candidates: check the a=0 model against the stated group order.
    search = json.loads((directory/'isogeny_results.json').read_text())
    rng = random.Random(20261023)
    order_checks = []
    for record in search['records']:
        b = int(record['b_hex'],16)
        p = sample_point(b,rng)
        assert scalar(p,IPSEC_ORDER) is None
        assert scalar(p,12) is not None
        order_checks.append(record['b_hex'])
    results = {'source_b': hex(IPSEC_B), 'degree': 3,
               'map': {'D': 'x+q', 'A': '1+q/D^2',
                       'x_image': 'x*A',
                       'y_image': 'y*A+q*x^2/D^3+q^3/D^2+q',
                       'target_b': 'b+q+q^2'},
               'maps': records,
               'candidate_a0_models_order_checked': len(order_checks),
               'order_check_scope': 'sample-point annihilation, not independent point counting',
               'all_checks_passed': True}
    (directory/'explicit_isogeny_results.json').write_text(json.dumps(results,indent=2)+'\n')
    print(json.dumps(results,indent=2))


if __name__ == '__main__':
    main()
