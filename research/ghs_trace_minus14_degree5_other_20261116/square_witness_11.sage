#!/usr/bin/env sage
"""Independent square obstructions for the two mask-11 carrier orbit reps."""
import hashlib
import json
from pathlib import Path
from sage.misc.sage_eval import sage_eval

HERE=Path(__file__).resolve().parent
F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,name='a',modulus=z^5+2*z+1);a=K.gen()
R=PolynomialRing(K,'x');x=R.gen();b=a^2+2
inputs=((9735,HERE.parent/'ghs_trace_minus14_20261116/degree5_screen.json'),
        (17371,HERE/'cell_11_17371.json'))
rows=[]
for mask,path in inputs:
    d=json.loads(path.read_text())
    assert d['hits']==[]
    f=x^11+sum(K((mask//(3^i))%3)*x^i for i in range(11))
    for j,s in enumerate(d['survivors']):
        A=R(sage_eval(s['A'],locals={'a':a,'x':x}))
        B=R(sage_eval(s['B'],locals={'a':a,'x':x}))
        assert A.degree()==5 and gcd(A,B)==1
        P,rem=(B*(A^3+A^2*B+b*B^3)).quo_rem(f)
        assert rem==0 and P.degree()==8
        character=P[8]^121
        assert character in (K(1),K(2))
        if character==K(2):
            obstruction={'kind':'nonsquare-leading-coefficient',
                         'leading_coefficient':str(P[8]),
                         'quadratic_character':str(character)}
        else:
            c4=P[8].sqrt()
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
        rows.append({'bmask':11,'fmask':mask,'survivor_index':j,
                     'shape':s['shape'],'obstruction':obstruction})
out={'schema':'ghs.trace-minus14.degree5-mask11-square-witness/v1',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'candidate_count':len(rows),'exact_square_hits':0,'rows':rows}
path=HERE/'SQUARE_WITNESS_11.json'
assert not path.exists(),'refusing to overwrite witness'
path.write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({'candidate_count':len(rows),'fmask_counts':
                  {str(m):sum(r['fmask']==m for r in rows) for m,_ in inputs}},sort_keys=True))
