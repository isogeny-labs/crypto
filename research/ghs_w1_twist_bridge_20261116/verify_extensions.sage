#!/usr/bin/env sage
"""Exact prime and discriminant screens for the W1 -9 base-change line."""
import hashlib
import json
from pathlib import Path

ts=[ZZ(2),ZZ(-9)]
for s in range(2,16):
    ts.append(-9*ts[-1]-128*ts[-2])
rows=[]
for s in (1,3,5,9,11,13,15):
    assert gcd(s,7)==1 and s%2==1
    n=ZZ(2)^(7*s)+1-ts[s]
    fac=list(n.factor())
    assert prod(p^e for p,e in fac)==n
    r=max(p for p,e in fac)
    assert r.is_prime(proof=True)
    delta=ts[s]^2-4*ZZ(2)^(7*s)
    assert gcd(r,2*7*delta)==1
    rows.append({'s':s,'q':2^s,'trace':ts[s],'source_order':n,
                 'factorization':[[p,e] for p,e in fac],
                 'largest_prime':r,'largest_prime_bits':r.nbits(),
                 'discriminant_factor_gcd':1})
source=Path(__file__).with_suffix('')
print(json.dumps({'schema':'ghs.w1-minus9.extension-primes/v1',
                  'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
                  'rows':rows},sort_keys=True,default=int))
