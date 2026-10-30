"""One-target relation, linear-algebra, and decomposition attack after transfer."""

import json
import random
import time
from pathlib import Path

root=Path(__file__).resolve().parent
witness=next(w for w in json.loads((root.parent/'ghs_quartic_new_classes_q8_20261026'/'search_results.json').read_text())['witnesses'] if w['group_order']==554)
start=time.perf_counter()
k=GF(8,'a',modulus='conway')
K=GF(512,'w',modulus='conway')
w=K.gen()
aK=w^73
assert aK^3+aK+1==0
R=PolynomialRing(K,names=('x','y','z'))
x,y,z=R.gens()
monomials=[(i,j,4-i-j) for i in range(4,-1,-1) for j in range(4-i,-1,-1)]
def embed(c):
    n=int(c.to_integer())
    return sum((aK^i for i in range(3) if n & (1<<i)),K(0))
F=sum(embed(k.from_integer(c))*x^i*y^j*z^l
      for c,(i,j,l) in zip(witness['coefficients'],monomials))
C=Curve(F)
assert C.is_smooth()
Kpoints=lambda coords:C(coords)
O_fiber=[Kpoints([w^54,w^2,1]),Kpoints([w^75,w^281,1])]
G_fiber=[Kpoints([w^164,w^2,1]),Kpoints([w^262,w^140,1])]
Q_fiber=[Kpoints([w^328,w^211,1]),Kpoints([w^109,1,0])]
def divisor(points):
    return sum((p.place().divisor() for p in points),O_fiber[0].place().divisor()*0)
J=C.jacobian(model='hess',base_div=3*O_fiber[0].place().divisor()).group()
setup_seconds=time.perf_counter()-start
print('setup_seconds',setup_seconds,flush=True)

map_start=time.perf_counter()
def frob(p): return C([v^8 for v in p])
def trace_fiber(fiber):
    numerator=fiber
    denominator=O_fiber
    result=J.zero()
    for _ in range(3):
        result+=J.point(divisor(numerator)-divisor(denominator))
        numerator=[frob(p) for p in numerator]
        denominator=[frob(p) for p in denominator]
    return result
TG=trace_fiber(G_fiber)
TQ=trace_fiber(Q_fiber)
assert not TG.is_zero() and (277*TG).is_zero()
assert not TQ.is_zero() and (277*TQ).is_zero()
map_seconds=time.perf_counter()-map_start
print('map_seconds',map_seconds,flush=True)

fb_start=time.perf_counter()
Rk=PolynomialRing(k,names=('X','Y','Z'))
X,Y,Z=Rk.gens()
Fk=sum(k.from_integer(c)*X^i*Y^j*Z^l
       for c,(i,j,l) in zip(witness['coefficients'],monomials))
Ck=Curve(Fk)
points=Ck.rational_points()
assert len(points)==9
lifted=[C([embed(p[0]),embed(p[1]),embed(p[2])]) for p in points]
P0=lifted[0]
factor_base=[2*J.point(p.place().divisor()-P0.place().divisor()) for p in lifted[1:]]
assert all((277*F).is_zero() for F in factor_base)
factor_base_seconds=time.perf_counter()-fb_start
print('factor_base_seconds',factor_base_seconds,flush=True)

walk_start=time.perf_counter()
r=int(277)
m=len(factor_base)
rand=random.Random(int(20261026))
zero=[int(0)]*m
state=J.zero()
vector=zero[:]
seen={str(state):(vector[:],state)}
relations=[]
decompositions={}
for i in range(1,2001):
    j=rand.randrange(m)
    state+=factor_base[j]
    vector[j]=int((vector[j]+1)%r)
    key=str(state)
    if key in seen:
        previous,previous_state=seen[key]
        assert state==previous_state
        relation=[int((vector[t]-previous[t])%r) for t in range(m)]
        if any(relation): relations.append(relation)
    else:
        seen[key]=(vector[:],state)
    if state==TG and 'G' not in decompositions: decompositions['G']=vector[:]
    if state==TQ and 'Q' not in decompositions: decompositions['Q']=vector[:]
    if i%100==0:
        print('walk_progress',i,'relations',len(relations),
              'decomposed',sorted(decompositions),flush=True)
    if i%25==0 and len(decompositions)==2 and len(relations)>=m-1:
        kernel=matrix(GF(r),relations).right_kernel()
        if kernel.dimension()==1:
            break
walk_seconds=time.perf_counter()-walk_start
print('walk_steps',i,'relation_count',len(relations),
      'decomposed',sorted(decompositions),'walk_seconds',walk_seconds,flush=True)

linear_start=time.perf_counter()
status='bounded_unknown'
answer=None
kernel_dimension=None
kernel_vector=None
if len(decompositions)==2 and len(relations)>=m-1:
    kernel=matrix(GF(r),relations).right_kernel()
    kernel_dimension=int(kernel.dimension())
    if kernel_dimension==1:
        logs=kernel.basis()[0]
        kernel_vector=[int(v) for v in logs]
        scoreG=sum((GF(r)(decompositions['G'][j])*logs[j] for j in range(m)),GF(r)(0))
        scoreQ=sum((GF(r)(decompositions['Q'][j])*logs[j] for j in range(m)),GF(r)(0))
        assert scoreG!=0
        answer=int(scoreQ/scoreG)
        status='solved'
linear_seconds=time.perf_counter()-linear_start

verify_start=time.perf_counter()
E=EllipticCurve(K,[K(1),K(1),K(0),K(0),K.from_integer(105)])
G=E([w^478,w^433])
Q=E([w^77,w^119])
assert E.cardinality()==554 and (277*G).is_zero()
if answer is not None: assert answer*G==Q and answer==7
source_verify_seconds=time.perf_counter()-verify_start
result={'schema':'ghs-q8-554-full-transfer-attack/v1','status':status,
        'source_b':int(105),'prime':int(r),'answer':answer,
        'walk_steps':int(i),'walk_additions':int(i),
        'relation_count':len(relations),'kernel_dimension':kernel_dimension,
        'factor_base_size':int(m),'factor_base_point_count':len(points),
        'decompositions':decompositions,'relations':relations,
        'kernel_vector':kernel_vector,
        'setup_seconds':float(setup_seconds),'map_seconds':float(map_seconds),
        'factor_base_seconds':float(factor_base_seconds),
        'walk_seconds':float(walk_seconds),'linear_seconds':float(linear_seconds),
        'source_verify_seconds':float(source_verify_seconds),
        'total_seconds':float(time.perf_counter()-start)}
print('status',status,'answer',answer,'total_seconds',result['total_seconds'],flush=True)
(root/'full_attack.json').write_text(json.dumps(result,indent=2)+'\n')
