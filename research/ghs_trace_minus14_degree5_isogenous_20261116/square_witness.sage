#!/usr/bin/env sage
"""Independently falsify the twelve retained degree-five square identities."""
import hashlib
import json
from pathlib import Path
from sage.misc.sage_eval import sage_eval

HERE=Path(__file__).resolve().parent
MASKS=(9735,17371,21624,44227,64725,90937,92153,
       100118,124586,144548,156795,159256)
F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,name='a',modulus=z^5+2*z+1);a=K.gen()
R=PolynomialRing(K,'x');x=R.gen()
b=2*a^4+a^3+a^2+a+1
rows=[]
for mask in MASKS:
    receipt=json.loads((HERE/('cell_%d.json'%mask)).read_text())
    assert receipt['fmask']==mask and receipt['bmask']==202
    assert len(receipt['survivors'])==1 and receipt['hits']==[]
    s=receipt['survivors'][0]
    A=R(sage_eval(s['A'],locals={'a':a,'x':x}))
    B=R(sage_eval(s['B'],locals={'a':a,'x':x}))
    f=x^11+sum(K((mask//(3^i))%3)*x^i for i in range(11))
    assert A.degree()==5 and gcd(A,B)==1
    N=B*(A^3+A^2*B+b*B^3)
    P,rem=N.quo_rem(f)
    assert rem==0 and P.degree()==8
    lc=P[8];character=lc^121
    assert character in (K(1),K(2))
    if character==K(2):
        obstruction={'kind':'nonsquare-leading-coefficient',
                     'leading_coefficient':str(lc),
                     'quadratic_character':str(character)}
    else:
        c4=lc.sqrt()
        c3=P[7]/(2*c4)
        c2=(P[6]-c3^2)/(2*c4)
        c1=(P[5]-2*c3*c2)/(2*c4)
        c0=(P[4]-c2^2-2*c3*c1)/(2*c4)
        C=c4*x^4+c3*x^3+c2*x^2+c1*x+c0
        residual=P-C^2
        assert residual!=0 and residual.degree()<4
        obstruction={'kind':'lower-coefficient-contradiction',
                     'candidate_square_root':str(C),
                     'residual_degree':int(residual.degree()),
                     'residual_leading_coefficient':str(residual.leading_coefficient())}
    rows.append({'fmask':mask,'survivor_shape':s['shape'],
                 'quotient_degree':int(P.degree()),'obstruction':obstruction})
out={'schema':'ghs.trace-minus14.degree5-square-witness/v1',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'rows':rows,'cells':len(rows),'exact_square_hits':0}
path=HERE/'SQUARE_WITNESS.json'
assert not path.exists(),'refusing to overwrite witness'
path.write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({'cells':len(rows),'kinds':
                  {kind:sum(r['obstruction']['kind']==kind for r in rows)
                   for kind in ('nonsquare-leading-coefficient',
                                'lower-coefficient-contradiction')}},sort_keys=True))
