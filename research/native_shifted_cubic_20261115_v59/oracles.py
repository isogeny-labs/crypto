from collections import Counter
from research.positive_binary_charts_20261115_v53 import math as curve
from research.binary_norm_coverage_20261115_v57.oracle import Extension
from research.torsion_shifted_cubic_20261115_v58.worker import interpolate4
from research.torsion_shifted_cubic_20261115_v58.solver import coefficients
from research.original_chain_lift_20261115_v55.solver import Lift
from research.norm_block_reuse_20261115_v36 import backend as linear

def fixture(f,A,d,seed,distinct):
 gen=curve.Generator(f,A,1,seed,d)
 for attempt in range(64):
  used=set();ps=[]
  for i in range(3 if distinct else 1):
   P=gen.point(used);used.add(P[0]);ps.append(P)
  if not distinct:ps=ps*3
  S=curve.total(f,A,1,ps)
  if S is None or S[0]<1<<d:continue
  Qp=(S[0],S[1]^S[0]);Q=curve.add(f,A,1,Qp,(0,1))
  if Q is None or Q[0]<1<<d:continue
  r,s,h=interpolate4(f,Extension(f),A,ps,Qp);C=(1,)
  for x,y in ps:C=curve.polynomial.b._mul(C,(x,1),f)
  assert coefficients(f,A,1,*Qp,h,r)==C
  counts=Counter(x for x,y in ps);counts[1]+=2;tuples=Lift(f,1).bag(sorted(counts.items()),Q[0])if A==1 else[]
  if A==1:assert tuples
  instance={'field_degree':f.degree,'modulus':f.modulus,'A':A,'B':1,'d':d,'z':Qp[0],'source_ordinate':Qp[1],'cli_ordinate':Qp[1]^Qp[0],'original_target':Q,'contract':'complete cubic support chart at shifted target; main h/k search separate'}
  witness={'source_points':ps,'r':r,'s':s,'h':h,'C':C,'original_tuples':tuples,'generation_attempt':attempt,'seed':seed,'original_variable_count':5*d+3*f.degree}
  return instance,witness
 raise RuntimeError('fixture generation cap')

def gold_projector(d):
 width=3*d-2;basis=linear.canonical_basis([(1<<(2*i+j))^(1<<(i+2*j))for i in range(d)for j in range(i+1,d)]);pivots={v.bit_length()-1 for v in basis};free=[i for i in range(width)if i not in pivots]
 def project(v):
  v=linear.apply_residue(v,basis);return((v>>width)<<len(free))|sum(((v>>i)&1)<<j for j,i in enumerate(free))
 return project

def family(f,instance,h):
 z=instance['z'];A=instance['A'];y=instance['source_ordinate'];d=instance['d'];P=gold_projector(d);w2=2*d-1;e1=f.square(h)^h^z;e20=f.mul(A,f.square(h))^f.mul(z,e1);e30=f.mul(f.square(h),y^f.mul(A,z)^f.square(z))^f.mul(f.square(z),z);cols=[]
 for i in range(f.degree):
  v=1<<i;q=f.square(v);delta=q^f.mul(h,v);cols.append((delta>>w2)|(P(f.mul(e1,delta)^f.mul(z,q))<<83))
 rhs=(e20>>w2)|(P(f.mul(e1,e20)^e30)<<83)
 return linear.solve(linear.rows_from(cols,rhs,166),83)
