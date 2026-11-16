class Field:
 def __init__(self,p,mod):self.p=p;self.mod=mod;self.n=len(mod)-1;self.q=p**self.n;self.mask=sum(c<<i for i,c in enumerate(mod))if p==2 else None
 def add(self,a,b):
  if self.p==2:return a^b
  if self.n==1:return(a+b)%self.p
  return sum(((a//self.p**i+b//self.p**i)%self.p)*self.p**i for i in range(self.n))
 def neg(self,a):
  if self.p==2:return a
  return sum((-(a//self.p**i)%self.p)*self.p**i for i in range(self.n))
 def sub(self,a,b):return self.add(a,self.neg(b))
 def mul(self,a,b):
  if self.n==1:return a*b%self.p
  if self.p==2:
   v=0
   while b:
    if b&1:v^=a
    a<<=1;b>>=1
   for j in range(v.bit_length()-1,self.n-1,-1):
    if v>>j&1:v^=self.mask<<(j-self.n)
   return v
  p=self.p;n=self.n;c=[0]*(2*n-1)
  for i in range(n):
   for j in range(n):c[i+j]=(c[i+j]+(a//p**i%p)*(b//p**j%p))%p
  for i in range(2*n-2,n-1,-1):
   for j in range(n):c[i-n+j]=(c[i-n+j]-c[i]*self.mod[j])%p
  return sum(c[i]*p**i for i in range(n))
 def sq(self,a):return self.mul(a,a)
 def power(self,a,n):
  v=1
  while n:
   if n&1:v=self.mul(v,a)
   n>>=1
   if n:a=self.sq(a)
  return v
 def inv(self,a):
  if not a:raise ZeroDivisionError('zero')
  return self.power(a,self.q-2)
 def trace(self,a):
  if self.p!=2:raise ValueError('binary absolute trace required')
  t=0
  for _ in range(self.n):t^=a;a=self.sq(a)
  assert t in [0,1];return t

def short_points(f,A,B):
 out={}
 for x in range(f.q):
  rhs=f.add(f.mul(f.sq(x),x),f.add(f.mul(A,f.sq(x)if f.p==2 else x),B));ys=[y for y in range(f.q)if f.add(f.sq(y),f.mul(x,y)if f.p==2 else 0)==rhs];out[x]=ys
 return out

def s3(x,y,z,A,B,f):
 if f.p==2:return f.sq(f.mul(x,y))^f.sq(f.mul(x,z))^f.sq(f.mul(y,z))^f.mul(f.mul(x,y),z)^B
 xy=f.mul(x,y);s=f.add(x,y);scale=lambda a,k:f.mul(a,k%f.p)
 return f.sub(f.add(f.sub(f.mul(f.sq(f.sub(x,y)),f.sq(z)),f.mul(scale(f.add(f.mul(s,f.add(xy,A)),scale(B,2)),2),z)),f.sq(f.sub(xy,A))),scale(f.mul(B,s),4))
