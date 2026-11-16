from itertools import permutations,combinations_with_replacement
from research.positive_binary_charts_20261115_v53 import math as curve
from research.binary_norm_coverage_20261115_v57.oracle import Extension
from research.torsion_shifted_cubic_20261115_v58.worker import interpolate4
from research.torsion_shifted_cubic_20261115_v58.solver import coefficients,chart
from research.original_chain_lift_20261115_v55.solver import Lift
from research.native_shifted_cubic_20261115_v59.oracles import family
from rootfinder.binaryfield import BinaryField

def original_triples(f,B,roots,z):
 xs=[x for x,m in roots for _ in range(m)];assert len(xs)==3;L=Lift(f,B);out=set()
 for a,b,c in set(permutations(xs)):
  for u in L.roots(a,b):
   if not L.residual(u,c,z):out.add((a,b,c,u))
 return sorted(out)

def fixture(f,A,B,d,seed,distinct):
 gen=curve.Generator(f,A,B,seed,d);beta=curve.sqrt(f,B)
 for attempt in range(64):
  used=set();ps=[]
  for i in range(3 if distinct else 1):
   Q=gen.point(used);used.add(Q[0]);ps.append(Q)
  if not distinct:ps=ps*3
  S=curve.total(f,A,B,ps)
  if S is None or S[0]<1<<d or f.mul(beta,f.inv(S[0]))<1<<d:continue
  Q=(S[0],S[1]^S[0]);r,s,h=interpolate4(f,Extension(f),A,ps,Q);C=(1,)
  for x,y in ps:C=curve.polynomial.b._mul(C,(x,1),f)
  assert coefficients(f,A,B,*Q,h,r)==C
  from collections import Counter
  values=original_triples(f,B,sorted(Counter(x for x,y in ps).items()),Q[0]);assert values
  return {'field_degree':f.degree,'modulus':f.modulus,'A':A,'B':B,'d':d,'z':Q[0],'source_ordinate':Q[1],'cli_ordinate':Q[1]^Q[0],'translated_target_x':f.mul(beta,f.inv(Q[0])),'contract':'all finite original three-source S3-chain roots in declared source domain'}, {'source_points':ps,'r':r,'s':s,'h':h,'C':C,'original_tuples':values,'generation_attempt':attempt,'seed':seed,'original_variable_count':3*d+f.degree}
 raise RuntimeError('fixture generation cap')

def controls():
 count=systems=positives=chart_systems=0;inside_profiles=[]
 for n,mod,A,B in[(3,11,0,1),(3,11,1,3),(4,19,8,7),(4,19,1,15)]:
  f=BinaryField(n,mod);outside=[(x,v)for x in range(4,f.order)for v in range(f.order)if f.square(v)^f.mul(x,v)==f.mul(f.square(x),x)^f.mul(A,f.square(x))^B];z,y=outside[0]if outside else(0,curve.sqrt(f,B));allowed=set()
  if not outside:inside_profiles.append({'n':n,'A':A,'B':B,'reason':'no rational target outside declared source domain; coefficient-only control atx0'})
  for xs in combinations_with_replacement(range(4),3):
   C=(1,)
   for x in xs:C=curve.polynomial.b._mul(C,(x,1),f)
   allowed.add(C)
  expected=set()
  for h in range(f.order):
   for r in range(f.order):
    s=f.square(z)^f.mul(r,z)^f.mul(h,y);F=(s,r,1);b=curve.polynomial.b;N=b._add(b._add(b._mul(F,F,f),(0,)+tuple(f.mul(h,v)for v in F)),(f.mul(B,f.square(h)),0,f.mul(A,f.square(h)),f.square(h)));C,rem=b._divmod(N,(z,1),f);assert not rem;assert C==coefficients(f,A,B,z,y,h,r);count+=1
    if C in allowed:expected.add((h,r,C))
  if outside:
   got=chart(f,A,B,z,y,2);assert {(v['h'],v['r'],v['C'])for v in got['solutions']}==expected;chart_systems+=1
  systems+=1;positives+=len(expected)
 return {'profiles':systems,'complete_field_parameter_states':count,'supported_parameters':positives,'even_extensions_included':True,'chart_profiles':chart_systems,'inside_target_coefficient_profiles':inside_profiles}
