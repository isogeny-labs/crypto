#!/usr/bin/env sage
"""Exact finite inputs for a prime-to-23 isogeny existence argument."""
import hashlib
import json
from pathlib import Path

F2=GF(2)
T=PolynomialRing(F2,'z'); z=T.gen()
K=GF(2^7,name='a',modulus=z^7+z+1); a=K.gen()
E=EllipticCurve(K,[1,1,0,0,a^3])
assert E.cardinality()==138
R=PolynomialRing(ZZ,'X'); X=R.gen()
P=X^14+9*X^7+128
assert P(1)==138 and P.change_ring(GF(13)).is_irreducible()
D=P.discriminant()
assert D == -(2^42)*(7^14)*(431^7)
assert gcd(23,D)==1
P23=P.change_ring(GF(23))
assert gcd(P23,P23.derivative())==1

v_mask=0x8f; u_mask=0xc07a
counts=[]
for m in range(1,8):
    F=GF(2^m,name='w')
    def ev(mask,t):
        return sum(F((mask>>i)&1)*t^i for i in range(mask.bit_length()))
    n=1
    for x in F:
        vv=ev(v_mask,x); uu=ev(u_mask,x)
        if vv==0:
            n+=1
        elif (uu/vv^2).trace()==0:
            n+=2
    counts.append(n)
assert counts == [3,5,9,17,33,65,192]
source=Path(__file__).with_suffix('')
print(json.dumps({'schema':'ghs.w1-minus9.prime23-inputs/v1',
                  'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
                  'field':'F2[a]/(a^7+a+1)',
                  'source_equation':'y^2+x*y=x^3+x^2+a^3',
                  'source_order':E.cardinality(),
                  'carrier_v_mask':v_mask,'carrier_u_mask':u_mask,
                  'carrier_point_counts':counts,
                  'frobenius_polynomial':str(P),
                  'irreducible_mod_13':True,
                  'discriminant':str(D),
                  'discriminant_prime_to_23':True,
                  'squarefree_mod_23':True},sort_keys=True,default=int))
