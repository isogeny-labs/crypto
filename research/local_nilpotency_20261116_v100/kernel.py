from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic

def trim(a):
 a=list(a)
 while a and a[-1]==0:a.pop()
 return a
def add(a,b,f):return trim([f.add(a[i]if i<len(a)else 0,b[i]if i<len(b)else 0)for i in range(max(len(a),len(b)))])
def modulo(a,H,f):
 a=trim(a);assert H[-1]==1
 while len(a)>=len(H):
  c=a[-1];shift=len(a)-len(H)
  for i,h in enumerate(H):a[i+shift]=f.sub(a[i+shift],f.mul(c,h))
  a=trim(a)
 return a
def mul(a,b,H,f):
 out=[0]*(len(a)+len(b)-1)if a and b else[]
 for i,x in enumerate(a):
  for j,y in enumerate(b):out[i+j]=f.add(out[i+j],f.mul(x,y))
 return modulo(out,H,f)
def square(a,H,f):
 if f.p!=2:return mul(a,a,H,f)
 out=[0]*(2*len(a)-1)if a else[]
 for i,x in enumerate(a):out[2*i]=f.sq(x)
 return modulo(out,H,f)
def power(a,k,H,f):
 out=[1]
 while k:
  if k&1:out=mul(out,a,H,f)
  k>>=1
  if k:a=square(a,H,f)
 return out
def predicate(H,f,exponent=None):
 assert len(H)==6 and H[-1]==1
 L=add(power([0,1],f.q,H,f),[0,f.neg(1)],f);k=exponent if exponent is not None else(8 if f.p==2 else 5)
 return not power(L,k,H,f),L
def roots_product(roots,f):
 H=[1]
 for a in roots:
  out=[0]*(len(H)+1)
  for i,h in enumerate(H):out[i]=f.sub(out[i],f.mul(a,h));out[i+1]=f.add(out[i+1],h)
  H=out
 return H
def literal_split(H,f):
 H=list(H)
 for a in range(f.q):
  while len(H)>1:
   value=0
   for c in reversed(H):value=f.add(f.mul(value,a),c)
   if value:break
   Q=[0]*(len(H)-1);Q[-1]=H[-1]
   for i in range(len(H)-2,0,-1):Q[i-1]=f.add(H[i],f.mul(a,Q[i]))
   assert f.add(H[0],f.mul(a,Q[0]))==0;H=Q
 return len(H)==1
