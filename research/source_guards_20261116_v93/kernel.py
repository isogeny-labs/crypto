from research.separator_images_20261116_v91.kernel import Solver
from research.twist_auxiliary_20261116_v86.algebra import Field

def polynomial(n,mod,d,k,u0,r,gx,gy,B=1):
 f=Field(2,mod);dx=d-r;assert 0<=r<=d and u0&((1<<k)-1)==0
 def plus(*args):
  out={}
  for a in args:
   for m,c in a.items():
    out[m]=out.get(m,0)^c
    if not out[m]:del out[m]
  return out
 def mul(a,b):
  out={}
  for m,c in a.items():
   for h,v in b.items():
    key=m|h;out[key]=out.get(key,0)^f.mul(c,v)
    if not out[key]:del out[key]
  if len(out)>100000:raise OverflowError('term cap')
  return out
 def square(a):return {m:f.sq(c)for m,c in a.items()}
 X={1<<j:1<<(r+j)for j in range(dx)};Y={1<<(dx+j):1<<(r+j)for j in range(dx)};U={1<<(2*dx+i):1<<i for i in range(k)}
 if gx:X[0]=gx
 if gy:Y[0]=gy
 if u0:U[0]=u0
 xy=mul(X,Y);xu=mul(X,U);yu=mul(Y,U)
 return plus(square(xy),square(xu),square(yu),mul(xy,U),{0:B})

def certificate(poly,n):
 constant=poly.get(0,0);cols=[v for m,v in poly.items()if m];rows=Solver.prepare(cols)[0]
 if Solver.particular(constant,rows)is not None:return {'empty':False,'rank':len(rows),'dual':None}
 def reduce(v):
  for p in sorted(rows,reverse=True):
   if v>>p&1:v^=rows[p][0]
  return v
 rem=reduce(constant);bit=(rem&-rem).bit_length()-1;w=sum(((reduce(1<<j)>>bit)&1)<<j for j in range(n));assert all((w&c).bit_count()%2==0 for c in cols)and(w&constant).bit_count()%2==1
 return {'empty':True,'rank':len(rows),'dual':w}

def evaluate(poly,assignment):
 out=0
 for m,c in poly.items():
  if assignment&m==m:out^=c
 return out
