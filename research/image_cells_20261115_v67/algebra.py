"""Division-free finite-algebra image polynomials; exact finite-domain support."""
from itertools import permutations,combinations
from functools import lru_cache
from research.vieta_guard_20261115_v66.oracle import F as Binary
class BField(Binary):
 p=2
 def add(self,a,b):return a^b
 def neg(self,a):return a

def trim(a):
 a=list(a)
 while a and not a[-1]:a.pop()
 return a

def pmul(a,b,f):
 if not a or not b:return []
 out=[0]*(len(a)+len(b)-1)
 for i,x in enumerate(a):
  for j,y in enumerate(b):
   if x and y:out[i+j]=f.add(out[i+j],f.mul(x,y))
 return trim(out)
def rem(a,H,f):
 if H[-1]!=1:raise ValueError('monic modulus required')
 a=trim(a)
 while len(a)>=len(H):
  shift=len(a)-len(H);lead=a[-1]
  for j,c in enumerate(H):a[shift+j]=f.add(a[shift+j],f.neg(f.mul(lead,c)))
  a=trim(a)
 return a

def roots_poly(xs,f):
 out=[1]
 for x in xs:out=pmul(out,[f.neg(x),1],f)
 return out

def evaluate(a,x,f):
 out=0
 for c in a[::-1]:out=f.add(f.mul(out,x),c)
 return out

def supported(H,domain,f):
 work=list(H);roots=[]
 for x in domain:
  multiplicity=0
  while len(work)>1 and evaluate(work,x,f)==0:
   q=[0]*(len(work)-1);q[-1]=work[-1]
   for j in range(len(q)-2,-1,-1):q[j]=f.add(work[j+1],f.mul(x,q[j+1]))
   work=q;multiplicity+=1
  if multiplicity:roots.append([x,multiplicity])
 return {'supported':len(work)==1,'roots':roots,'remaining_degree':len(work)-1}

@lru_cache(None)
def terms(k):
 return [(pi,sum(pi[i]>pi[j]for i in range(k)for j in range(i+1,k))%2)for pi in permutations(range(k))]
def determinant(M,indices,f):
 out=0;k=len(indices)
 for pi,odd in terms(k):
  value=1
  for i,j in enumerate(pi):
   value=f.mul(value,M[indices[i]][indices[j]])
   if not value:break
  out=f.add(out,f.neg(value)if odd else value)
 return out

def image(H,q,f):
 m=len(H)-1;cols=[(rem([0]*j+list(q),H,f)+[0]*m)[:m]for j in range(m)];M=[[cols[j][i]for j in range(m)]for i in range(m)];B=[0]*m+[1]
 for k in range(1,m+1):
  value=0
  for indices in combinations(range(m),k):value=f.add(value,determinant(M,indices,f))
  B[m-k]=f.neg(value)if k%2 else value
 return B

def compose_mod(B,q,H,f):
 out=[]
 for c in B[::-1]:
  out=rem(pmul(out,q,f),H,f)
  if not out:out=[c]
  else:out[0]=f.add(out[0],c)
  out=trim(out)
 return out

def binary_tower(f,dimension):
 scalars=[]
 for i in range(dimension):
  value=1<<i
  for a in scalars:value=f.sq(value)^f.mul(a,value)
  assert value;scalars.append(value)
 return scalars

def tower_value(value,scalars,f):
 for a in scalars:value=f.sq(value)^f.mul(a,value)
 return value

def tower_mod(H,scalars,f):
 q=[0,1]
 for a in scalars:
  squared=[0]*(2*len(q)-1)
  for i,x in enumerate(q):squared[2*i]=f.sq(x)
  new=rem(squared,H,f)
  if len(new)<len(q):new +=[0]*(len(q)-len(new))
  for i,x in enumerate(q):new[i]^=f.mul(a,x)
  q=trim(new)
 return q
