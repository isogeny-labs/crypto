#!/usr/bin/env sage
"""Certify an explicit degree-14 genus-five cover to the mask-167 negative curve."""
import hashlib
import json
import time
from pathlib import Path

t0=time.process_time()
HERE=Path(__file__).resolve().parent
COVER=HERE.parent/'ghs_odd5_carrier_20261115'/'degree7_quadratic_square_30.json'
cover=json.loads(COVER.read_text())
assert cover['bmask']==30 and len(cover['hits'])==1
hit=cover['hits'][0]
K=GF(3^5,'a',modulus=x^5+2*x+1)
a=K.gen()
R=PolynomialRing(K,'x');x=R.gen()
F=FractionField(R)
A=R(sage_eval(hit['A'],locals={'x':x,'a':a},preparse=True))
B=R(sage_eval(hit['B'],locals={'x':x,'a':a},preparse=True))
C=R(sage_eval(hit['C'],locals={'x':x,'a':a},preparse=True))
f=x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1
b=a+a^3
assert f*C^2==B*(A^3+2*A^2*B+b*B^3)
u=F(2*A)/F(B)
vcoeff=F(C)/F(B^2)
assert 2*F(f)*vcoeff^2==u^3+u^2+2*b

# Degree-two forward isogeny E30^- -> E2^- and x-shift E2^- -> E167^-.
c=-a^3-a^2+1
shift=2*a^3+2*a^2+1
gx=(u^2+c*u+c)/(u+c)-shift
gm=u^2+(a^3+a^2-1)*u+(a^4-a^3+a+1)
gd=u^2+(a^3+a^2-1)*u+(a^4+a^3-a^2+a-1)
gycoeff=vcoeff*gm/gd
b167=a^4+2*a+1
assert 2*F(f)*gycoeff^2==gx^3+gx^2+b167
assert max(gx.numerator().degree(),gx.denominator().degree())==14
assert gx.numerator().gcd(gx.denominator())==1

def poly_vector(p):
    return [[int(t) for t in v.polynomial().list()] for v in p.list()]
out={'schema':'ghs.odd5.mask167-degree14-cover/v1','status':'PASS',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'base_cover_receipt_sha256':hashlib.sha256(COVER.read_bytes()).hexdigest(),
     'field':'F3[a]/(a^5+2a+1)',
     'carrier':'y^2=2*(x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1)',
     'target':'v^2=u^3+u^2+a^4+2*a+1',
     'degree':14,'separable':True,
     'x_numerator':poly_vector(gx.numerator()),
     'x_denominator':poly_vector(gx.denominator()),
     'y_coefficient_numerator':poly_vector(gycoeff.numerator()),
     'y_coefficient_denominator':poly_vector(gycoeff.denominator()),
     'identity_verified':True,'cpu_seconds':time.process_time()-t0}
(HERE/'compose_cover.json').write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({'status':'PASS','degree':14,'x_num_degree':gx.numerator().degree(),
                  'x_den_degree':gx.denominator().degree(),
                  'y_num_degree':gycoeff.numerator().degree(),
                  'y_den_degree':gycoeff.denominator().degree()},sort_keys=True,default=int))
