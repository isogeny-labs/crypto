"""Prepared generic exact affine-fiber block reduction; not executed."""
class Field:
 def __init__(self,p,degree=1,nu=2):
  assert p>2 and p%2 and degree in (1,2)
  if degree==2:assert pow(nu,(p-1)//2,p)==p-1
  self.p=p;self.degree=degree;self.nu=nu;self.order=p**degree
 def add(self,a,b):
  p=self.p
  return (a+b)%p if self.degree==1 else ((a%p+b%p)%p)+p*((a//p+b//p)%p)
 def neg(self,a):return (-a)%self.p if self.degree==1 else (-a%self.p)+self.p*((-(a//self.p))%self.p)
 def sub(self,a,b):return self.add(a,self.neg(b))
 def mul(self,a,b):
  p=self.p
  if self.degree==1:return a*b%p
  x,y=a%p,a//p;u,v=b%p,b//p
  return ((x*u+self.nu*y*v)%p)+p*((x*v+y*u)%p)
 def inv(self,a):
  assert a
  p=self.p
  if self.degree==1:return pow(a,p-2,p)
  x,y=a%p,a//p;norm=(x*x-self.nu*y*y)%p;assert norm
  c=pow(norm,p-2,p);return x*c%p+p*((-y*c)%p)
def dot(F,a,b):
 out=0
 for x,y in zip(a,b):out=F.add(out,F.mul(x,y))
 return out
def lin(F,B,c,n):
 out=[0]*n
 for v,a in zip(B,c):
  for i,x in enumerate(v):out[i]=F.add(out[i],F.mul(a,x))
 return out
def solve(F,M,rhs,n):
 A=[list(v)+[r]for v,r in zip(M,rhs)];piv=[];r=0
 for j in range(n):
  k=next((k for k in range(r,len(A))if A[k][j]),None)
  if k is None:continue
  A[r],A[k]=A[k],A[r];a=F.inv(A[r][j]);A[r]=[F.mul(a,x)for x in A[r]]
  for k in range(len(A)):
   if k==r or not A[k][j]:continue
   a=A[k][j];A[k]=[F.sub(x,F.mul(a,y))for x,y in zip(A[k],A[r])]
  piv.append(j);r+=1
 for row in A:
  if not any(row[:n])and row[n]:return None
 part=[0]*n
 for i,j in enumerate(piv):part[j]=A[i][n]
 basis=[]
 for j in range(n):
  if j in piv:continue
  v=[0]*n;v[j]=1
  for i,k in enumerate(piv):v[k]=F.neg(A[i][j])
  basis.append(v)
 assert all(dot(F,row,part)==c for row,c in zip(M,rhs))
 assert all(not dot(F,row,v)for row in M for v in basis)
 return dict(particular=part,basis=basis,rank=len(piv))
def block(F,M,rhs,K,U):
 """K is any certified constant-image subspace; U is a full complement."""
 m=len(M);n=len(M[0])if m else len(K[0])if K else len(U[0])if U else 0
 check=solve(F,list(K)+list(U),[0]*(len(K)+len(U)),n)
 assert check is not None and check['rank']==n and len(K)+len(U)==n
 E=[[dot(F,row,v)for v in K]for row in M]
 Et=[[row[j]for row in E]for j in range(len(K))]
 Q=solve(F,Et,[0]*len(K),m)['basis']
 MU=[[dot(F,row,v)for v in U]for row in M]
 red=[[dot(F,q,[row[j]for row in MU])for j in range(len(U))]for q in Q]
 rb=[dot(F,q,rhs)for q in Q];rs=solve(F,red,rb,len(U))
 if rs is None:return None
 part_u=lin(F,U,rs['particular'],n)
 delta=[F.sub(c,dot(F,row,part_u))for row,c in zip(M,rhs)]
 vs=solve(F,E,delta,len(K));assert vs is not None
 part=[F.add(a,b)for a,b in zip(part_u,lin(F,K,vs['particular'],n))]
 basis=[lin(F,K,v,n)for v in vs['basis']]
 for z in rs['basis']:
  u=lin(F,U,z,n);delta=[F.neg(dot(F,row,u))for row in M];v=solve(F,E,delta,len(K));assert v is not None
  basis.append([F.add(a,b)for a,b in zip(u,lin(F,K,v['particular'],n))])
 assert all(dot(F,row,part)==c for row,c in zip(M,rhs))
 assert all(not dot(F,row,v)for row in M for v in basis)
 independent=solve(F,basis,[0]*len(basis),n);assert independent['rank']==len(basis)
 return dict(particular=part,basis=basis,original_variables=n,residual_variables=len(U),residual_rows=len(Q))
