"""Prepared derivative-free roots of a supported small-degree subspace polynomial."""
def trim(A):
 A=list(A)
 while A and A[-1]==0:A.pop()
 return A
def add(F,A,B):
 C=[0]*max(len(A),len(B))
 for i,a in enumerate(A):C[i]=F.add(C[i],a)
 for i,b in enumerate(B):C[i]=F.add(C[i],b)
 return trim(C)
def scale(F,A,c):return trim([F.mul(a,c)for a in A])
def product(F,A,B):
 if not A or not B:return []
 C=[0]*(len(A)+len(B)-1)
 for i,a in enumerate(A):
  for j,b in enumerate(B):C[i+j]=F.add(C[i+j],F.mul(a,b))
 return trim(C)
def divrem(F,A,B):
 A=trim(A);B=trim(B);assert B
 Q=[0]*max(0,len(A)-len(B)+1);inv=F.inv(B[-1])
 while len(A)>=len(B):
  j=len(A)-len(B);c=F.mul(A[-1],inv);Q[j]=c
  for i,b in enumerate(B):A[i+j]=F.sub(A[i+j],F.mul(c,b))
  A=trim(A)
 return trim(Q),A
def monic(F,A):return scale(F,A,F.inv(A[-1]))if A else[]
def gcd(F,A,B):
 while B:A,B=B,divrem(F,A,B)[1]
 return monic(F,A)
def power(F,a,n):
 v=1
 while n:
  if n&1:v=F.mul(v,a)
  a=F.mul(a,a);n//=2
 return v
def powmod(F,A,n,H):
 out=[1];A=divrem(F,A,H)[1]
 while n:
  if n&1:out=divrem(F,product(F,out,A),H)[1]
  A=divrem(F,product(F,A,A),H)[1];n//=2
 return out
def evaluate_linearized(F,L,x,p):
 out=0
 for a in L:out=F.add(out,F.mul(a,x));x=power(F,x,p)
 return out
def linearized_mod(F,L,H,p):
 out=[];x=divrem(F,[0,1],H)[1]
 for a in L:out=add(F,out,scale(F,x,a));x=powmod(F,x,p,H)
 return out
def supported_roots(F,H,basis,offset=0,branch_cap=31):
 """All roots+mult iff every H root is in offset+span_Fp(basis)."""
 H=trim(H);assert H and H[-1]==1 and len(H)<=6 and len(basis)<=83
 p=F.p
 if p>branch_cap:return dict(status='unsupported_characteristic',supported=None,roots=None)
 if len(H)==1:return dict(status='complete_supported_factorization',supported=True,roots=[],nodes=0,gcd_calls=0)
 Ls=[[1]];lambdas=[]
 for v in basis:
  previous=Ls[-1];lam=evaluate_linearized(F,previous,v,p);assert lam
  factor=power(F,lam,p-1);new=[0]*(len(previous)+1)
  for j,a in enumerate(previous):new[j]=F.sub(new[j],F.mul(factor,a));new[j+1]=F.add(new[j+1],power(F,a,p))
  Ls.append(new);lambdas.append(lam)
 degree=len(H)-1;res=add(F,linearized_mod(F,Ls[-1],H,p),[F.neg(evaluate_linearized(F,Ls[-1],offset,p))])
 if powmod(F,res,degree,H):return dict(status='not_fully_supported',supported=False,roots=None)
 roots=[];nodes=gcd_calls=0
 def visit(i,off,A):
  nonlocal nodes,gcd_calls
  if len(A)==1:return
  nodes+=1;assert nodes<=1+degree*len(basis)
  n=len(A)-1
  if i==0:
   expected=[1]
   for _ in range(n):expected=product(F,expected,[F.neg(off),1])
   assert expected==A;roots.append((off,n));return
  L=Ls[i-1];R=linearized_mod(F,L,A,p);base=evaluate_linearized(F,L,off,p);factors=[1]
  for a in range(p):
   target=F.add(base,F.mul(a,lambdas[i-1]));r=add(F,R,[F.neg(target)]);G=gcd(F,A,powmod(F,r,n,A));gcd_calls+=1
   if len(G)>1:visit(i-1,F.add(off,F.mul(a,basis[i-1])),G);factors=product(F,factors,G)
  assert factors==A
 visit(len(basis),offset,H);assert sum(m for x,m in roots)==degree and len({x for x,m in roots})==len(roots)
 reconstructed=[1]
 for x,m in roots:
  for _ in range(m):reconstructed=product(F,reconstructed,[F.neg(x),1])
 assert reconstructed==H
 return dict(status='complete_supported_factorization',supported=True,roots=sorted(roots),nodes=nodes,gcd_calls=gcd_calls)
