#!/usr/bin/env sage
"""Exact rational two-isogeny and selected order-43 subgroup image."""
import hashlib
import json
from pathlib import Path

F3=GF(3); Z=PolynomialRing(F3,'z'); z=Z.gen()
K=GF(3^5,name='a',modulus=z^5+2*z+1); a=K.gen()
E=EllipticCurve(K,[0,1,0,0,a^2+2])
N=EllipticCurve(K,[0,1,0,0,2*a^4+a^3+a^2+a+1])
assert E.cardinality()==N.cardinality()==258
isos=E.isogenies_prime_degree(2)
assert len(isos)==1
phi=isos[0]; D=phi.codomain()
assert D.is_isomorphic(N)
nu=D.isomorphism_to(N)
G=E.gens()[0]
assert G.order()==258
P=6*G; Q=7*P
assert P.order()==43 and Q.order()==43
P2=nu(phi(P)); Q2=nu(phi(Q))
assert P2.order()==43 and Q2==7*P2
maps=phi.rational_maps()
out={'schema':'ghs.trace-minus14.two-isogeny-witness/v1',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'field':'F3[a]/(a^5+2a+1)',
     'source_ainvs':[str(t) for t in E.ainvs()],
     'codomain_ainvs':[str(t) for t in D.ainvs()],
     'normal_codomain_ainvs':[str(t) for t in N.ainvs()],
     'degree':int(phi.degree()),'source_order':int(E.cardinality()),
     'normal_codomain_order':int(N.cardinality()),
     'isomorphism':str(nu),
     'isogeny_rational_maps':[str(t) for t in maps],
     'source_generator':str(G),'source_P43':str(P),'source_Q43':str(Q),
     'codomain_P43':str(P2),'codomain_Q43':str(Q2),
     'Q_scalar':7,'subgroup_image_order':int(P2.order())}
path=Path(__file__).resolve().parent/'ISOGENY.json'
assert not path.exists(), 'refusing to overwrite receipt'
path.write_text(json.dumps(out,indent=2,sort_keys=True)+'\n')
print(json.dumps({'degree':out['degree'],'subgroup_image_order':out['subgroup_image_order'],
                  'source_order':out['source_order']},sort_keys=True))
