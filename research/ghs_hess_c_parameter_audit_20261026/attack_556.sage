"""Certified Hess transfer to the canonical quartic and relation attack."""

import json
import random
import time
from pathlib import Path

root = Path(__file__).resolve().parent
start = time.perf_counter()
k = GF(8, 'a', modulus='conway')
K = GF(512, 'w', modulus='conway')
w = K.gen()
A = w**73
assert A**3 + A + 1 == 0
R = PolynomialRing(K, names=('x','y','z'))
x,y,z = R.gens()
F = (x**4+A*x**2*y**2+A*x**3*z+A**4*x*y**2*z+A**6*y**3*z
     +A**6*x**2*z**2+A**6*x*y*z**2+A**3*y**2*z**2+x*z**3
     +A**2*y*z**3+z**4)
C = Curve(F)
assert C.is_smooth()
P0 = C([0,1,0])
J = C.jacobian(model='hess', base_div=3*P0.place().divisor()).group()
setup_seconds = time.perf_counter()-start
print('setup_seconds',setup_seconds,flush=True)

map_start = time.perf_counter()
point_specs = [
    (lambda v:v**3+v**2+A**5*v+A**4,164,'G1'),
    (lambda v:v**3+v**2+A**6*v+A,205,'G2'),
    (lambda v:v**3+A*v**2+A**4*v+A**3,253,'Q1'),
    (lambda v:v**3+A*v**2+A**5*v+A,217,'Q2'),
]
orbits = {}
for polynomial, exponent, name in point_specs:
    roots = [v for v in K if polynomial(v)==0]
    assert len(roots)==3
    points = [C([v,v**exponent,1]) for v in roots]
    assert all(F(v,v**exponent,1)==0 for v in roots)
    assert {str(p) for p in points} == {str(C([p[0]**8,p[1]**8,p[2]**8])) for p in points}
    orbits[name] = points
base_place = P0.place().divisor()
def divisor(points):
    return sum((p.place().divisor() for p in points),6*base_place*0)
TG = J.point(divisor(orbits['G1']+orbits['G2'])-6*base_place)
TQ = J.point(divisor(orbits['Q1']+orbits['Q2'])-6*base_place)
assert not TG.is_zero() and (139*TG).is_zero()
assert not TQ.is_zero() and (TQ-7*TG).is_zero()
map_seconds = time.perf_counter()-map_start
print('map_seconds',map_seconds,'map_order',139,flush=True)

fb_start = time.perf_counter()
rk = PolynomialRing(k,names=('X','Y','Z'))
X,Y,Z = rk.gens()
a = k.gen()
Fk = (X**4+a*X**2*Y**2+a*X**3*Z+a**4*X*Y**2*Z+a**6*Y**3*Z
      +a**6*X**2*Z**2+a**6*X*Y*Z**2+a**3*Y**2*Z**2+X*Z**3
      +a**2*Y*Z**3+Z**4)
Ck = Curve(Fk)
points = Ck.rational_points()
assert len(points)==9
def embed(c):
    n=int(c.to_integer())
    return sum((A**i for i in range(3) if n & (1<<i)),K(0))
lifted = [C([embed(p[0]),embed(p[1]),embed(p[2])]) for p in points]
fb = [4*J.point(p.place().divisor()-base_place) for p in lifted if p!=P0]
assert len(fb)==8 and all((139*v).is_zero() for v in fb)
factor_base_seconds = time.perf_counter()-fb_start
print('factor_base_seconds',factor_base_seconds,flush=True)

walk_start = time.perf_counter()
r=139
m=len(fb)
rng=random.Random(int(20261026))
state=J.zero()
vector=[0]*m
seen={str(state):(vector[:],state)}
relations=[]
decomp={}
max_steps=2000
for i in range(1,max_steps+1):
    j=rng.randrange(m)
    state+=fb[j]
    vector[j]=(vector[j]+1)%r
    key=str(state)
    if key in seen:
        previous,previous_state=seen[key]
        assert state==previous_state
        rel=[(vector[t]-previous[t])%r for t in range(m)]
        if any(rel): relations.append(rel)
    else:
        seen[key]=(vector[:],state)
    if state==TG and 'G' not in decomp: decomp['G']=vector[:]
    if state==TQ and 'Q' not in decomp: decomp['Q']=vector[:]
    if i%100==0:
        print('walk_progress',i,'relations',len(relations),'decomposed',sorted(decomp),flush=True)
    if i%25==0 and len(decomp)==2 and len(relations)>=m-1:
        if matrix(GF(r),relations).right_kernel().dimension()==1: break
walk_seconds=time.perf_counter()-walk_start
print('walk_seconds',walk_seconds,'walk_steps',i,flush=True)

linear_start=time.perf_counter()
answer=None
kernel_dimension=None
kernel_vector=None
if len(decomp)==2 and len(relations)>=m-1:
    kernel=matrix(GF(r),relations).right_kernel()
    kernel_dimension=int(kernel.dimension())
    if kernel_dimension==1:
        logs=kernel.basis()[0]
        kernel_vector=[int(v) for v in logs]
        g=sum((GF(r)(decomp['G'][j])*logs[j] for j in range(m)),GF(r)(0))
        q=sum((GF(r)(decomp['Q'][j])*logs[j] for j in range(m)),GF(r)(0))
        assert g!=0
        answer=int(q/g)
linear_seconds=time.perf_counter()-linear_start

verify_start=time.perf_counter()
E=EllipticCurve(K,[K(1),K(0),K(0),K(0),K.from_integer(32)])
G=E([K.from_integer(255),K.from_integer(59)])
Q=E([K.from_integer(206),K.from_integer(467)])
assert E.cardinality()==556 and (139*G).is_zero()
if answer is not None:
    assert answer*G==Q and answer==7
source_verify_seconds=time.perf_counter()-verify_start
result={'schema':'ghs-hess-c-556-full-transfer-attack/v1',
        'status':'solved' if answer is not None else 'bounded_unknown',
        'source_b':32,'prime':r,'answer':answer,'walk_steps':i,
        'relation_count':len(relations),'factor_base_size':m,
        'decompositions':decomp,'relations':relations,
        'kernel_dimension':kernel_dimension,'kernel_vector':kernel_vector,
        'setup_seconds':setup_seconds,'map_seconds':map_seconds,
        'factor_base_seconds':factor_base_seconds,'walk_seconds':walk_seconds,
        'linear_seconds':linear_seconds,'source_verify_seconds':source_verify_seconds,
        'total_seconds':time.perf_counter()-start}
(root/'attack_556.json').write_text(json.dumps(result,indent=2,default=int)+'\n')
print('status',result['status'],'answer',answer,'total_seconds',result['total_seconds'],flush=True)
