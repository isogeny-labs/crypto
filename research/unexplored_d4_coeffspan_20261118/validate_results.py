#!/usr/bin/env python3
"""Independent reconstruction of logged S3 coefficient claims and sample summaries."""
import hashlib, json, statistics, sys
from sympy import Poly, symbols, expand
from sympy.ntheory.primetest import isprime
p=2147483647
x,y,c,A,B=symbols('x y c A B')
formal=Poly(expand((x-y)**2*c**2 - 2*((x+y)*(x*y+B)+2*A*x*y)*c+(x*y-B)**2),x,y)
monos=[(i,j) for i in range(3) for j in range(3)]
assert isprime(p)
for fn in sys.argv[1:]:
    d=json.load(open(fn));assert len(d['samples'])==d['cases']==96
    for s in d['samples']:
        val={A:s['A'],B:s['B'],c:s['xR']}
        coeff=[int(formal.coeff_monomial(x**i*y**j).subs(val))%p for i,j in monos]
        assert hashlib.sha256(json.dumps(coeff).encode()).hexdigest()==s['actual_coefficient_sha256']
        assert sum(q*pow(s['a'],i,p)*pow(s['b'],j,p) for q,(i,j) in zip(coeff,monos))%p==0
        ratio=(s['actual_norm2']/s['symmetric_null_norm2'])**0.5
        assert abs(ratio-s['norm_ratio'])<1e-12
    r=[s['norm_ratio'] for s in d['samples']]
    assert abs(statistics.median(r)-d['median_norm_ratio'])<1e-12
    assert sum(t<.5 for t in r)==d['under_half_cases']
    assert sum(s['first_vector_integer_vanish'] for s in d['samples'])==d['first_vector_integer_vanish_count']
    print(fn,'VALIDATED',d['cases'],'median',d['median_norm_ratio'],'valid actual roots',d['first_vector_integer_vanish_count'])
