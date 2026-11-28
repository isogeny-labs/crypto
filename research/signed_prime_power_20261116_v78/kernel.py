from research.prime_orbit_roots_20261116_v71.solver import power as generic_power,gcd
from research.image_cells_20261115_v67.algebra import rem,pmul,trim,evaluate
COUNTS={k:0 for k in ['squarings','multiplications','chain_calls','signed_calls','inverse_calls','nonunit_fallbacks','fallback_calls']}
def reset():
 for k in COUNTS:COUNTS[k]=0
def mersenne(a,k,H,f):
 assert k>=0 and len(H)>=2
 if not k:return [1]
 a=rem(a,H,f);r=a;length=1
 for bit in bin(k)[3:]:
  t=r
  for _ in range(length):t=rem(pmul(t,t,f),H,f);COUNTS['squarings']+=1
  r=rem(pmul(r,t,f),H,f);COUNTS['multiplications']+=1;length*=2
  if bit=='1':
   r=rem(pmul(r,r,f),H,f);COUNTS['squarings']+=1;r=rem(pmul(r,a,f),H,f);COUNTS['multiplications']+=1;length+=1
 assert length==k
 return r

def naf(n):
 out=[];i=0
 while n:
  if n&1:d=2-(n&3);out.append((i,d));n-=d
  n>>=1;i+=1
 return out

def plus(a,b,f):
 r=list(a)+[0]*max(0,len(b)-len(a))
 for i,x in enumerate(b):r[i]=f.sub(r[i],x)
 return trim(r)
def inverse(a,H,f):
 COUNTS['inverse_calls']+=1;r,s=list(H),rem(a,H,f);u,v=[],[1]
 while s:
  x=list(r);q=[0]*max(0,len(x)-len(s)+1);iv=f.inv(s[-1])
  while len(x)>=len(s):
   j=len(x)-len(s);z=f.mul(x[-1],iv);q[j]=z
   for i,y in enumerate(s):x[i+j]=f.sub(x[i+j],f.mul(z,y))
   x=trim(x)
  r,s=s,x;u,v=v,plus(u,pmul(q,v,f),f)
 if len(r)!=1:return None
 iv=f.inv(r[0]);u=rem([f.mul(x,iv)for x in u],H,f);assert rem(pmul(a,u,f),H,f)==[1];return u

def eligible(n):return n.bit_length()>=64 and len(naf(n))<=7

def power(a,n,H,f):
 assert n>=0 and len(H)>=2
 if not(n&(n+1)):COUNTS['chain_calls']+=1;return mersenne(a,(n+1).bit_length()-1,H,f)
 if not eligible(n):COUNTS['fallback_calls']+=1;return generic_power(a,n,H,f)
 a=rem(a,H,f)
 if len(a)==2 and a[1]==1:unit=bool(evaluate(H,f.neg(a[0]),f))
 else:unit=gcd(H,a,f)==[1]
 if not unit:COUNTS['nonunit_fallbacks']+=1;COUNTS['fallback_calls']+=1;return generic_power(a,n,H,f)
 ds=naf(n);assert sum(v*(1<<i)for i,v in ds)==n;digits=dict(ds);top=ds[-1][0];num=[1];den=[1];v=a
 for i in range(top+1):
  sign=digits.get(i,0)
  if sign:
   dest=num if sign==1 else den
   if dest==[1]:dest=list(v)
   else:dest=rem(pmul(dest,v,f),H,f);COUNTS['multiplications']+=1
   if sign==1:num=dest
   else:den=dest
  if i<top:v=rem(pmul(v,v,f),H,f);COUNTS['squarings']+=1
 if den!=[1]:
  iv=inverse(den,H,f)
  if iv is None:COUNTS['nonunit_fallbacks']+=1;COUNTS['fallback_calls']+=1;return generic_power(a,n,H,f)
  num=rem(pmul(num,iv,f),H,f);COUNTS['multiplications']+=1
 COUNTS['signed_calls']+=1
 return num
