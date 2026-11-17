#!/usr/bin/env python3
"""Generate exact Magma inputs from the frozen quartic witness certificate."""

import hashlib
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parent
SOURCE=ROOT.parent/'ghs_quartic_new_classes_q8_20261026/search_results.json'
ORDERS=(482,524,554,556)
EXPONENTS=[(i,j,4-i-j) for i in range(4,-1,-1)
           for j in range(4-i,-1,-1)]


def constant(n):
    terms=[part for bit,part in ((1,'1'),(2,'a'),(4,'a^2')) if n&bit]
    return '('+'+'.join(terms)+')' if terms else '0'


def monomial(i,j,k):
    pieces=[]
    for name,n in (('x',i),('y',j),('z',k)):
        if n:
            pieces.append(name if n==1 else f'{name}^{n}')
    return '*'.join(pieces)


def main():
    data=json.loads(SOURCE.read_text())
    rows={w['group_order']:w for w in data['witnesses']}
    assert set(ORDERS)<=set(rows)
    assert len(EXPONENTS)==15
    for order in ORDERS:
        w=rows[order]
        terms=[constant(c)+'*'+monomial(*exp)
               for c,exp in zip(w['coefficients'],EXPONENTS) if c]
        code='\n'.join([
            'K<a> := GF(8);',
            'print "field", DefiningPolynomial(K);',
            'P<x,y,z> := PolynomialRing(K, 3);',
            'F := '+' +\n     '.join(terms)+';',
            'C := Curve(ProjectiveSpace(P), F);',
            'print "rational_points", #RationalPoints(C);',
            'print "smooth", IsNonsingular(C);',
            'B, beta := AutomorphismGroupOfPlaneQuartic(C : explicit:=true);',
            'print "base_automorphism_order", #B;',
            'print "base_group_name", GroupName(B);',
            'for i in [1..Ngens(B)] do',
            '    print "base_generator_order", Order(B.i);',
            '    print "base_generator_matrix", beta(B.i);',
            'end for;',
            'A, phi := AutomorphismGroupOfPlaneQuartic(C : geometric:=true, explicit:=true);',
            'print "geometric_automorphism_order", #A;',
            'print "geometric_group_name", GroupName(A);',
            'for i in [1..Ngens(A)] do',
            '    print "geometric_generator_order", Order(A.i);',
            '    print "geometric_generator_matrix", phi(A.i);',
            'end for;',
            ''])
        path=ROOT/(str(order)+'.magma')
        path.write_text(code)
        print(order,hashlib.sha256(path.read_bytes()).hexdigest())


if __name__=='__main__':
    main()
