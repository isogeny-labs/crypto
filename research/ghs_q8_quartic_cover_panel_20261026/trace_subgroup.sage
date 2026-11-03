"""Evaluate quotient pullback and GF(512)/GF(8) trace on an order-277 class."""

import json
import time
from pathlib import Path

root=Path(__file__).resolve().parent
witness=next(w for w in json.loads((root.parent/'ghs_quartic_new_classes_q8_20261026'/'search_results.json').read_text())['witnesses'] if w['group_order']==554)
K=GF(512,'w',modulus='conway')
w=K.gen()
a=w^73
R=PolynomialRing(K,names=('x','y','z'))
x,y,z=R.gens()
exps=[(i,j,4-i-j) for i in range(4,-1,-1) for j in range(4-i,-1,-1)]
F=sum(sum((a^i for i in range(3) if c & (1<<i)),K(0))*x^i*y^j*z^k
      for c,(i,j,k) in zip(witness['coefficients'],exps))
C=Curve(F)
assert C.is_smooth()
origin=[C([w^54,w^2,1]),C([w^75,w^281,1])]
generator_fiber=[C([w^164,w^2,1]),C([w^262,w^140,1])]
target_fiber=[C([w^328,w^211,1]),C([w^109,1,0])]

def divisor(points):
    return sum((p.place().divisor() for p in points),origin[0].place().divisor()*0)

def frob(p):
    return C([v^8 for v in p])

t0=time.perf_counter()
base=3*origin[0].place().divisor()
J=C.jacobian(model='hess',base_div=base).group()
setup_seconds=time.perf_counter()-t0
print('jacobian_setup_seconds',setup_seconds,flush=True)
t1=time.perf_counter()
generator_classes=[]
target_classes=[]
for i in range(3):
    generator_classes.append(J.point(divisor(generator_fiber)-divisor(origin)))
    target_classes.append(J.point(divisor(target_fiber)-divisor(origin)))
    generator_fiber=[frob(p) for p in generator_fiber]
    target_fiber=[frob(p) for p in target_fiber]
    origin=[frob(p) for p in origin]
pullback_seconds=time.perf_counter()-t1
print('pullback_and_frobenius_seconds',pullback_seconds,flush=True)
direct=generator_classes[0]
traced=sum(generator_classes,J.zero())
target_traced=sum(target_classes,J.zero())
flags={
  'pullback_nonzero':bool(not direct.is_zero()),
  'pullback_277_zero':bool((277*direct).is_zero()),
  'trace_nonzero':bool(not traced.is_zero()),
  'trace_277_zero':bool((277*traced).is_zero()),
  'trace_target_equals_7G':bool(target_traced==7*traced),
  'trace_target_nonzero':bool(not target_traced.is_zero()),
}
total_seconds=time.perf_counter()-t0
for key,value in flags.items(): print(key,value,flush=True)
print('total_seconds',total_seconds,flush=True)
(root/'trace_subgroup.json').write_text(json.dumps({
  'schema':'ghs-q8-quartic-trace-subgroup/v1',
  'source_b':int(105),'prime':int(277),
  'generator_x_exponent':int(478),'generator_y_exponent':int(433),
  'target_x_exponent':int(77),'target_y_exponent':int(119),
  **flags,
  'jacobian_setup_seconds':float(setup_seconds),
  'pullback_and_frobenius_seconds':float(pullback_seconds),
  'total_seconds':float(total_seconds),
},indent=2)+'\n')
