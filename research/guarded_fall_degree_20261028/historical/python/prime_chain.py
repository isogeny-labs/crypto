"""Generate small, rational prime-field three-summand Semaev fixtures.

Variables: x1,x2,x3,u,v. Inputs are three g_S membership equations,
f3(x1,x2,u), f3(u,x3,r), and v*(x1-x2)-1. The guard restricts the relation
to first summands with distinct abscissas. S contains only abscissas of
F_p-rational curve points. No field equation for u is needed in this model.

This is a toy fixture generator and exhaustive ground-truth check, not a
claim about unguarded decompositions, full factor bases, or secp256k1 cost.
"""

import argparse
import itertools
import json
from pathlib import Path


VARIABLES = ('x1', 'x2', 'x3', 'u', 'v')
ZERO = (0,)*5


def add(p, *polys):
    result = {}
    for poly in polys:
        for e, c in poly.items():
            result[e] = (result.get(e, 0) + c) % p
    return {e:c for e,c in result.items() if c}


def scale(p, c, poly):
    return {e:c*a % p for e,a in poly.items() if c*a % p}


def mul(p, left, right):
    result = {}
    for e, a in left.items():
        for f, b in right.items():
            g = tuple(x+y for x,y in zip(e,f))
            result[g] = (result.get(g,0) + a*b) % p
    return {e:c for e,c in result.items() if c}


def constant(p, c):
    return {ZERO:c % p} if c % p else {}


def variable(i):
    e = [0]*5
    e[i] = 1
    return {tuple(e):1}


def f3(p, a, b, x, y, z):
    # (x-y)^2 z^2 - 2((x+y)(xy+a)+2b)z
    #                + (xy-a)^2 - 4b(x+y)
    xy, plus, minus = mul(p,x,y), add(p,x,y), add(p,x,scale(p,-1,y))
    middle = add(p,mul(p,plus,add(p,xy,constant(p,a))),constant(p,2*b))
    tail = add(p,xy,constant(p,-a))
    return add(p,mul(p,mul(p,minus,minus),mul(p,z,z)),
               scale(p,-2,mul(p,middle,z)),mul(p,tail,tail),scale(p,-4*b,plus))


def f3_value(p, a, b, x, y, z):
    return ((x-y)**2*z*z - 2*((x+y)*(x*y+a)+2*b)*z
            + (x*y-a)**2 - 4*b*(x+y)) % p


def evaluate(poly, values, p):
    result = 0
    for e, c in poly.items():
        term = c
        for x, power in zip(values,e):
            term = term*pow(x,power,p) % p
        result = (result+term) % p
    return result


def curve_points(p, a, b):
    if p <= 3 or any(p % d == 0 for d in range(2,int(p**0.5)+1)):
        raise ValueError('toy generator requires a prime p > 3')
    if (4*a**3 + 27*b*b) % p == 0:
        raise ValueError('singular curve')
    return [(x,y) for x in range(p) for y in range(p)
            if (y*y-x*x*x-a*x-b) % p == 0]


def point_add(p, a, left, right):
    if left is None:
        return right
    if right is None:
        return left
    x,y = left
    xx,yy = right
    if x == xx and (y+yy) % p == 0:
        return None
    if x == xx:
        slope = (3*x*x+a)*pow(2*y,-1,p) % p
    else:
        slope = (yy-y)*pow(xx-x,-1,p) % p
    z = (slope*slope-x-xx) % p
    return z,(slope*(x-z)-y) % p


def negate(p, point):
    return None if point is None else (point[0],-point[1] % p)


def build_problem(p, a, b, base, target):
    x1,x2,x3,u,v = [variable(i) for i in range(5)]
    polys = []
    for x in (x1,x2,x3):
        membership = constant(p,1)
        for s in base:
            membership = mul(p,membership,add(p,x,constant(p,-s)))
        polys.append(membership)
    polys.extend((f3(p,a,b,x1,x2,u),f3(p,a,b,u,x3,constant(p,target[0])),
                  add(p,mul(p,v,add(p,x1,scale(p,-1,x2))),constant(p,-1))))
    terms = [[[c,list(e)] for e,c in sorted(poly.items(),reverse=True)] for poly in polys]
    return {'p':p, 'n':5, 'polynomials':terms,
            'variables':list(VARIABLES), 'curve':{'a':a,'b':b},
            'factor_base_abscissas':list(base), 'target':list(target),
            'restriction':'x1 != x2', 'implicit_field_equations':False},polys


def ground_truth(p, a, points, base):
    restricted = [point for point in points if point[0] in base]
    witnesses = {}
    for one,two,three in itertools.product(restricted,repeat=3):
        if one[0] == two[0]:
            continue
        intermediate = point_add(p,a,one,two)
        assert intermediate is not None
        result = point_add(p,a,intermediate,three)
        if result is not None:
            witnesses.setdefault(result,(one,two,three,intermediate))
    return witnesses


def polynomial_witness(p,a,b,base,r):
    for x1,x2,x3 in itertools.product(base,repeat=3):
        if x1 == x2:
            continue
        v = pow((x1-x2) % p,-1,p)
        for u in range(p):
            if (f3_value(p,a,b,x1,x2,u) == 0 and
                    f3_value(p,a,b,u,x3,r) == 0):
                return (x1,x2,x3,u,v)
    return None


def validate_pair_roots(p,a,b,points):
    count = 0
    for one,two in itertools.product(points,repeat=2):
        expected = set()
        for sign_two in (two,negate(p,two)):
            result = point_add(p,a,one,sign_two)
            if result is not None:
                expected.add(result[0])
        actual = {u for u in range(p) if f3_value(p,a,b,one[0],two[0],u) == 0}
        assert actual == expected, (p,one,two,actual,expected)
        count += 1
    return count


def generate(out):
    out.mkdir(parents=True,exist_ok=True)
    report = {'pair_root_checks':0, 'target_checks':0, 'fixtures':[],
              'scope':'guarded rational three-summand chain over prime fields'}
    for p in (17,19,101):
        a,b = 0,7
        points = curve_points(p,a,b)
        if p < 30:
            report['pair_root_checks'] += validate_pair_roots(p,a,b,points)
        abscissas = sorted({point[0] for point in points})
        for size in (2,3):
            base = abscissas[:size]
            truth = ground_truth(p,a,points,base)
            # Check every affine target abscissa, including 2-torsion targets.
            target_by_x = {}
            for point in points:
                target_by_x.setdefault(point[0],point)
            for r,target in target_by_x.items():
                witness = polynomial_witness(p,a,b,base,r)
                assert (witness is not None) == (target in truth)
                report['target_checks'] += 1
            for satisfiable in (False,True):
                target = next((point for point in points if (point in truth) == satisfiable),None)
                if target is None:
                    continue
                problem,polys = build_problem(p,a,b,base,target)
                witness = polynomial_witness(p,a,b,base,target[0])
                if witness is not None:
                    assert all(evaluate(poly,witness,p) == 0 for poly in polys)
                problem['ground_truth'] = {
                    'satisfiable':satisfiable,
                    'method':'exhaustive F_p-rational curve addition and chain enumeration',
                    'polynomial_witness':list(witness) if witness is not None else None,
                    'point_witness':truth.get(target)}
                name = f'p{p}_B{size}_{"sat" if satisfiable else "unsat"}.json'
                (out/name).write_text(json.dumps(problem,indent=2)+'\n')
                report['fixtures'].append({'file':name,'p':p,'B':size,
                                           'satisfiable':satisfiable,
                                           'target':list(target),
                                           'max_input_degree':max(sum(e) for poly in polys for e in poly)})
    report['validation'] = 'passed'
    (out/'manifest.json').write_text(json.dumps(report,indent=2)+'\n')
    return report


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out',type=Path,default=Path(__file__).with_name('prime-chain-fixtures'))
    args = parser.parse_args()
    print(json.dumps(generate(args.out),indent=2))
