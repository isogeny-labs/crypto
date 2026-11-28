from itertools import combinations
from research.image_cells_20261115_v67.algebra import BField,rem,trim,binary_tower,tower_value
from research.translation_orbits_20261115_v69.algebra import orbit_polynomial,literal_s3

def inv(a,f):
 if not a:raise ZeroDivisionError()
 u,v=a,f.m;left,right=1,0
 while u!=1:
  if not u:raise ValueError('invalid field modulus')
  shift=u.bit_length()-v.bit_length()
  if shift<0:u,v=v,u;left,right=right,left;shift=-shift
  u^=v<<shift;left^=right<<shift
 while left.bit_length()>f.n:left^=f.m<<(left.bit_length()-f.n-1)
 assert f.mul(a,left)==1
 return left

def monic(a,f):
 a=trim(a)
 if not a:return []
 k=inv(a[-1],f);return [f.mul(x,k)for x in a]
def gcd(a,b,f):
 a=trim(a);b=trim(b)
 while b:b=monic(b,f);a,b=b,rem(a,b,f)
 return monic(a,f)
def exact_quotient(a,b,f):
 a=trim(a);b=monic(b,f);q=[0]*max(0,len(a)-len(b)+1)
 while len(a)>=len(b):
  shift=len(a)-len(b);v=a[-1];q[shift]^=v
  for j,x in enumerate(b):a[shift+j]^=f.mul(v,x)
  a=trim(a)
 assert not a
 return trim(q)
def square_step(q,H,scalar,f):
 if not q:return []
 squared=[0]*(2*len(q)-1)
 for i,x in enumerate(q):squared[2*i]=f.sq(x)
 new=rem(squared,H,f)
 if len(new)<len(q):new +=[0]*(len(q)-len(new))
 for i,x in enumerate(q):new[i]^=f.mul(scalar,x)
 return trim(new)

def shift_roots(R,scalars,f):
 q=[0,1]
 for a in scalars:q=square_step(q,R,a,f)
 G=gcd(R,q,f)
 if len(G)<=1:return []
 residues=[rem([0,1],G,f)]
 for a in scalars:residues.append(square_step(residues[-1],G,a,f))
 roots=[]
 def split(H,level,offset):
  if len(H)<=1:return
  if len(H)==2:
   root=H[0];assert(root^offset)>>level==0;roots.append(root);return
  assert level>0
  p=rem(residues[level-1],H,f);constant=tower_value(offset,scalars[:level-1],f)
  if not p:p=[constant]
  else:p[0]^=constant
  left=gcd(H,p,f);right=exact_quotient(H,left,f);split(left,level-1,offset);split(right,level-1,offset^(1<<(level-1)))
 split(G,len(scalars),0);assert len(roots)==len(G)-1 and len(set(roots))==len(roots);return sorted(roots)

class Context:
 def __init__(self,n,mod):
  self.f=BField(n,mod);self.piv={}
  for i in range(n):
   v=self.f.sq(1<<i)^(1<<i);coordinate=1<<i
   while v:
    j=v.bit_length()-1
    if j in self.piv:a,b=self.piv[j];v^=a;coordinate^=b
    else:self.piv[j]=(v,coordinate);break
  assert len(self.piv)==n-1
 def artin_schreier(self,t):
  x=0
  while t:
   j=t.bit_length()-1
   if j not in self.piv:return []
   a,b=self.piv[j];t^=a;x^=b
  return [x,x^1]
 def roots(self,H):
  f=self.f
  if not H:
   if f.q>256:raise ValueError('free auxiliary exceeds output budget')
   return list(range(f.q))
  H=monic(H,f)
  if len(H)<=1:return []
  if len(H)==2:return [H[0]]
  assert len(H)==3
  c,b,_=H
  if not b:
   x=c
   for _ in range(f.n-1):x=f.sq(x)
   assert f.sq(x)==c;return [x]
  t=f.mul(c,inv(f.sq(b),f));return sorted(f.mul(b,x)for x in self.artin_schreier(t))
 def common(self,a,b,c,z,B):
  f=self.f;ab=f.mul(a,b);cz=f.mul(c,z);P=[f.sq(ab)^B,ab,f.sq(a)^f.sq(b)];Q=[f.sq(cz)^B,cz,f.sq(c)^f.sq(z)];return self.roots(gcd(P,Q,f))

def evaluate(R,t,f):
 value=0
 for a in R[::-1]:value=f.mul(value,t)^a
 return value

def gray_values(R,dimension,f):
 if not 0<=dimension<=21:raise ValueError('Gray allocation cap')
 size=1<<dimension;values=[0]*size;degree=max((i.bit_count()for i,c in enumerate(R)if c),default=0)
 for weight in range(min(degree,dimension)+1):
  for positions in combinations(range(dimension),weight):
   mask=sum(1<<j for j in positions);value=evaluate(R,mask,f)
   if mask:
    sub=(mask-1)&mask
    while True:
     value^=values[sub]
     if not sub:break
     sub=(sub-1)&mask
   values[mask]=value
 step=1
 while step<size:
  for base in range(0,size,2*step):
   for index in range(base+step,base+2*step):values[index]^=values[index-step]
  step*=2
 return values

def solve(instance,method):
 n=instance['n'];ctx=Context(n,instance['modulus']);f=ctx.f;dimension=instance['shift_dimension'];assert 0<=dimension<=n;beta=instance['beta'];z=instance['z'];B=instance['B'];R=orbit_polynomial(beta,z,instance.get('A',0),B,f);assert len(R)==13 and R[-1]==1
 if method=='gcd':shifts=shift_roots(R,binary_tower(f,dimension),f)
 elif method=='horner':shifts=[t for t in range(1<<dimension)if evaluate(R,t,f)==0]
 elif method=='gray':shifts=[i for i,v in enumerate(gray_values(R,dimension,f))if v==0]
 else:raise ValueError('method')
 tuples=[]
 for t in shifts:
  a,b,c=[v^t for v in beta]
  for u in ctx.common(a,b,c,z,B):
   assert not literal_s3(a,b,u,instance.get('A',0),B,f)and not literal_s3(u,c,z,instance.get('A',0),B,f);tuples.append([t,u])
 return {'complete':True,'orbit_polynomial':R,'candidate_shifts':shifts,'roots':sorted(tuples),'original_tuples':[[beta[0]^t,beta[1]^t,beta[2]^t,u]for t,u in sorted(tuples)],'source_family':'fixed ordered common-shift offsets;not independent-source global search'}
