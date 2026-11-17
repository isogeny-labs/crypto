#!/usr/bin/env sage -python
"""Projective smoothness by zero-dimensional singular-cone ideals."""

import hashlib
import json
from pathlib import Path

from sage.all import GF, PolynomialRing

ROOT=Path(__file__).resolve().parent
RAW=ROOT/'search_results.json'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    data=json.loads(RAW.read_text())
    k=GF(8,'a',modulus='conway')
    ring=PolynomialRing(k,names=('X','Y','Z'))
    X,Y,Z=ring.gens()
    monomials=[X**i*Y**j*Z**(4-i-j)
               for i in range(4,-1,-1) for j in range(4-i,-1,-1)]
    records=[]
    for w in data['witnesses']:
        F=sum((k.from_integer(c)*m for c,m in zip(w['coefficients'],monomials)),
              ring.zero())
        ideal=ring.ideal([F,F.derivative(X),F.derivative(Y),F.derivative(Z)])
        dimension=int(ideal.dimension())
        assert dimension==0
        records.append({'order':w['group_order'],
                        'singular_cone_dimension':dimension,
                        'groebner_basis_size':len(ideal.groebner_basis())})
        print('SMOOTH',w['group_order'],'singular-cone dimension',dimension,
              flush=True)
    output={'schema':'ghs-q8-quartic-smoothness-audit/v1',
            'search_sha256':sha(RAW),
            'script_sha256':sha(Path(__file__)),
            'records':records}
    (ROOT/'smoothness_certificate.json').write_text(json.dumps(output,indent=2)+'\n')


if __name__=='__main__':
    main()
