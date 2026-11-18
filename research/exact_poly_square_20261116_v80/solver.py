from research.image_cells_20261115_v67.algebra import pmul,rem,trim,evaluate
from research.translation_orbits_20261115_v69.algebra import orbit_polynomial,literal_s3,quad_coeffs
from research.prime_orbit_roots_20261116_v71.field import Prime

def monic(a,f):
 a=trim(a)
 if not a:return []
 k=f.inv(a[-1]);return [f.mul(x,k)for x in a]
def gcd(a,b,f):
 a=trim(a);b=trim(b)
 while b:b=monic(b,f);a,b=b,rem(a,b,f)
 return monic(a,f)
def quotient(a,b,f):
 a=trim(a);b=monic(b,f);out=[0]*max(0,len(a)-len(b)+1)
 while len(a)>=len(b):
  j=len(a)-len(b);x=a[-1];out[j]=x
  for i,y in enumerate(b):a[j+i]=f.sub(a[j+i],f.mul(x,y))
  a=trim(a)
 assert not a
 return trim(out)
from .kernel import power

def field_roots(R,f,max_attempts=96):
 q=power([0,1],f.p,R,f)
 if len(q)<2:q +=[0]*(2-len(q))
 q[1]=f.sub(q[1],1);G=gcd(R,q,f);degree=len(G)-1;roots=[];stack=[G];state=71;attempts=0
 while stack:
  H=stack.pop()
  if len(H)<=1:continue
  if len(H)==2:roots.append(f.neg(H[0]));continue
  factor=None
  while attempts<max_attempts:
   state=(state*6364136223846793005+1442695040888963407)&((1<<256)-1);a=[state%f.p,1];attempts+=1;factor=gcd(H,a,f)
   if 1<len(factor)<len(H):break
   v=power(a,(f.p-1)//2,H,f)
   if not v:v=[0]
   v[0]=f.sub(v[0],1);factor=gcd(H,v,f)
   if 1<len(factor)<len(H):break
   factor=None
  if factor is None:raise RuntimeError('bounded splitting unresolved')
  stack.extend([factor,quotient(H,factor,f)])
 assert len(roots)==degree and len(set(roots))==degree
 assert all(evaluate(R,x,f)==0 for x in roots)
 return sorted(roots)

def common(a,b,c,z,A,B,f):
 P=[v[0]if v else 0 for v in quad_coeffs([a],[b],A,B,f)];Q=[v[0]if v else 0 for v in quad_coeffs([c],[z],A,B,f)];H=gcd(P[::-1],Q[::-1],f)
 if not H:
  if f.p>256:raise RuntimeError('free auxiliary exceeds output budget')
  return list(range(f.p))
 if len(H)<=1:return []
 if len(H)==2:return [f.neg(H[0])]
 assert len(H)==3;c,b,_=H;disc=f.sub(f.sq(b),f.scale(c,4));s=f.sqrt(disc)
 if s is None:return []
 inv2=f.inv(2);return sorted({f.mul(f.sub(s,b),inv2),f.mul(f.sub(f.neg(s),b),inv2)})

def difference_shifts(R,lo,hi,f):
 degree=len(R)-1;values=[evaluate(R,lo+i,f)for i in range(degree+1)];delta=[]
 while values:
  delta.append(values[0]);values=[f.sub(values[i+1],values[i])for i in range(len(values)-1)]
 roots=[]
 for t in range(lo,hi+1):
  if delta[0]==0:roots.append(t)
  for i in range(degree):delta[i]=f.add(delta[i],delta[i+1])
 return roots

def solve(inst,method='factor'):
 f=Prime(inst['p']);beta=inst['beta'];A=inst['A'];B=inst['B'];z=inst['z'];lo=inst['shift_low'];hi=inst['shift_high'];assert 0<=lo<=hi<f.p;R=orbit_polynomial(beta,z,A,B,f);assert len(R)==13 and R[-1]==1
 if method=='factor':shifts=[t for t in field_roots(R,f)if lo<=t<=hi]
 elif method=='difference':shifts=difference_shifts(R,lo,hi,f)
 else:raise ValueError('method')
 roots=[]
 for t in shifts:
  a,b,c=[f.add(x,t)for x in beta]
  for u in common(a,b,c,z,A,B,f):
   assert literal_s3(a,b,u,A,B,f)==0 and literal_s3(u,c,z,A,B,f)==0;roots.append([t,u])
 return {'complete':True,'orbit_polynomial':R,'candidate_shifts':shifts,'roots':sorted(roots),'source_family':'fixed ordered offsets with exact canonical interval;not independent-source solving'}
