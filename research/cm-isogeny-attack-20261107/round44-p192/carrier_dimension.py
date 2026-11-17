from pathlib import Path
import json
r=int('FFFFFFFFFFFFFFFFFFFFFFFF99DEF836146BC9B1B4D22831',16)
mu=4072180873419819102403428347769737935154921989474241554654
C=[[0,1,0,0],[0,0,1,0],[0,0,0,1],[-1,-1,-1,-1]]
I=[[int(i==j) for j in range(4)] for i in range(4)]
def mm(A,B):return [[sum(A[i][k]*B[k][j] for k in range(4)) for j in range(4)] for i in range(4)]
powers=[I]
for _ in range(5):powers.append(mm(powers[-1],C))
assert powers[5]==I and C!=I
v=[pow(mu,j,r) for j in range(4)]
assert [sum(C[i][j]*v[j] for j in range(4))%r for i in range(4)]==[mu*z%r for z in v]
assert all(sum(powers[j][i][k] for j in range(5))==0 for i in range(4) for k in range(4))
out={'all_checks_pass':True,'companion_matrix':C,'powers':powers,'marked_eigenvector':list(map(str,v)),'order':5,'minimum_dimension':4,'proof':'For imaginary quadratic K, K intersects Q(zeta5) in Q, since the unique quadratic subfield of Q(zeta5) is real Q(sqrt5). Thus Phi5 remains irreducible over K. A nonidentity matrix A over K with A^5=I has Phi5 in its minimal polynomial, forcing dimension>=4. The integer companion realizes equality.','scope':'Global order-five origin-fixing homomorphism on E^d. Does not exclude a map of order five only on the marked subgroup, nonlinear encodings, or shared arithmetic.','standard_four_coordinate_cost':{'raw_bytes':240,'ordinary_bytes':96,'baseline_walkers_per_cached':'5/2','equal_storage_ideal_work_ratio_lower_bound_squared':'5/4','assumption':'Four generic coordinatewise additions; deriving fifth point and representative is optimistically free.'}}
Path('carrier-dimension.json').write_text(json.dumps(out,indent=2)+'\n');print('CARRIER_ORDER_AND_EIGENVECTOR_PASS')
