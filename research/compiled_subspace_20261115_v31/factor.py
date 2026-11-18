"""Binary compiled-prefix extractor with generic-field fallback."""
from rootfinder import binaryfield as b
from research.subspace_factor_20261115_v29 import factor as generic
CACHE={}
def compile_basis(field,basis):
 key=(field.degree,field.modulus,tuple(basis))
 if key in CACHE:return CACHE[key]
 d=len(basis);E=[[0]*d for _ in range(d)];lam=[]
 for k,v in enumerate(basis):
  x=v;E[0][k]=x
  for i in range(k):x=field._raw_square(x)^field._raw_mul(lam[i],x);E[i+1][k]=x
  assert x;lam.append(x)
 result=(tuple(lam),tuple(tuple(row)for row in E))
 if len(CACHE)>=64:CACHE.pop(next(iter(CACHE)))
 CACHE[key]=result;return result
def pure_power(field,A):
 n=len(A)-1
 if n%2:x=A[-2]
 elif n in(2,4)and not any(A[1:-1]):
  x=A[0]
  for _ in range((-(n.bit_length()-1))%field.degree):x=field._raw_square(x)
 else:return None
 expected=(1,)
 for _ in range(n):expected=b._mul(expected,(x,1),field)
 return x if expected==A else None
def supported_roots(F,H,basis,offset=0,branch_cap=31):
 if F.p!=2 or F.p>branch_cap:return generic.supported_roots(F,H,basis,offset,branch_cap)
 field=F.field;H=b.trim(H,field);basis=tuple(field.validate(v)for v in basis);offset=field.validate(offset);assert H and H[-1]==1 and len(H)<=6 and len(basis)<=83
 if len(H)==1:return dict(status='complete_supported_factorization',supported=True,roots=[],nodes=0,gcd_calls=0)
 lam,E=compile_basis(field,basis);d=len(basis);remainders=[b._divmod((0,1),H,field)[1]];values=[offset]
 for c in lam:
  old=remainders[-1];scaled=tuple(field._raw_mul(c,v)for v in old);remainders.append(b._add(b._squaremod(old,H,field),scaled));values.append(field._raw_square(values[-1])^field._raw_mul(c,values[-1]))
 test=b._add(remainders[-1],(values[-1],));n=len(H)-1
 bound=1
 while bound<n:test=b._squaremod(test,H,field);bound*=2
 if test:return dict(status='not_fully_supported',supported=False,roots=None)
 roots=[];nodes=gcd_calls=0
 def visit(i,off,A,vs):
  nonlocal nodes,gcd_calls
  if len(A)==1:return
  nodes+=1;assert nodes<=1+n*d
  x=pure_power(field,A)
  if x is not None:roots.append((x,len(A)-1));return
  if i==0:
   expected=(1,)
   for _ in range(len(A)-1):expected=b._mul(expected,(off,1),field)
   assert expected==A;roots.append((off,len(A)-1));return
  R=b._divmod(remainders[i-1],A,field)[1];R=b._add(R,(vs[i-1],));power=1
  while power<len(A)-1:R=b._squaremod(R,A,field);power*=2
  G=b._gcd(A,R,field);Q,rem=b._divmod(A,G,field);assert not rem;gcd_calls+=1
  if len(G)>1:visit(i-1,off,G,vs[:i-1])
  if len(Q)>1:visit(i-1,off^basis[i-1],Q,tuple(vs[j]^E[j][i-1]for j in range(i-1)))
 visit(d,offset,H,tuple(values[:d]));assert sum(m for x,m in roots)==n and len({x for x,m in roots})==len(roots)
 check=(1,)
 for x,m in roots:
  for _ in range(m):check=b._mul(check,(x,1),field)
 assert check==H
 return dict(status='complete_supported_factorization',supported=True,roots=sorted(roots),nodes=nodes,gcd_calls=gcd_calls)
