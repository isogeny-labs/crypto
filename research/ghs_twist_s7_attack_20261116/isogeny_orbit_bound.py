"""Exact Frobenius recurrence and a bounded s=7 isogeny screen."""
import hashlib
import json
import time
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, ZZ, is_prime, sage_eval

HERE=Path(__file__).resolve().parent
source=json.loads((HERE.parent/'ghs_twist_extension_20261116'/'twist_s7_transfer.json').read_text())
F3=GF(3)
Z=PolynomialRing(F3,'z')
z=Z.gen()
L=GF(3**35,'w',modulus=Z(sage_eval(source['field_modulus'],locals={'x':z},preparse=True)))
w=L.gen()
def v(cs):return sum((L(c)*w**i for i,c in enumerate(cs)),L(0))
a=v(source['a_embedding'])
E=EllipticCurve(L,[0,1,0,0,2*(a+a**3)])
U=[ZZ(0),ZZ(1)]
for n in range(2,8):U.append(-26*U[n-1]-243*U[n-2])
assert U==[0,1,-26,433,-4940,23221,596674,-21156227]
assert is_prime(abs(U[7]))
screen=[]
for ell in (2,3,5,7,13,29,41,43):
    t0=time.process_time_ns()
    maps=E.isogenies_prime_degree(ell)
    screen.append({'degree':ell,'rational_isogenies':len(maps),
                   'codomain_j_degrees':[int(phi.codomain().j_invariant().minimal_polynomial().degree()) for phi in maps],
                   'cpu_s':float((time.process_time_ns()-t0)/1e9)})
out={'schema':'ghs.odd5.twist-s7-isogeny-orbit-bound/v1',
     'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
     'source_j_degree':int(E.j_invariant().minimal_polynomial().degree()),
     'trace_over_F243':-26,'field_size_F243':243,'extension_degree':7,
     'lucas_U_0_to_7':[int(x) for x in U],
     'exceptional_factor':int(abs(U[7])),'exceptional_factor_prime':True,
     'screen':screen}
(HERE/'isogeny_orbit_bound.json').write_text(json.dumps(out,indent=2,sort_keys=True)+'\n')
print(json.dumps(out,sort_keys=True))
