"""Exact linearized support and affine Gold moment filters in characteristic2."""
from rootfinder.binaryfield import powmod_poly
from research.k_linear_20261115_v19 import worker as b
CACHE={}
def xor(values):
 out=0
 for v in values:out^=v
 return out
def project(v,piv,free):
 for p in sorted(piv,reverse=True):
  if v>>p&1:v^=piv[p]
 return sum(((v>>j)&1)<<i for i,j in enumerate(free))
def image(f,d,count):
 key=(f.degree,f.modulus,d,count)
 if key in CACHE:return CACHE[key]
 piv={}
 for i in range(d):
  for j in range(i+1,d):
   v=sum((f.mul(f.pow(1<<i,q),1<<j)^f.mul(1<<i,f.pow(1<<j,q)))<<(f.degree*t)for t,q in enumerate([2,4,8][:count]))
   while v:
    p=v.bit_length()-1
    if p in piv:v^=piv[p]
    else:piv[p]=v;break
 out=(piv,[i for i in range(count*f.degree)if i not in piv]);CACHE[key]=out;return out
def spanpack(H,n,d):
 value=shift=0
 for j in range(1,6):
  width=min(j*(d-1)+1,n);value|=(H[5-j]>>width)<<shift;shift+=n-width
 return value,shift
def moments(f,H):
 e=[0]+list(reversed(H[:5]));s=[1]
 for k in range(1,10):
  v=xor(f.mul(e[j],s[k-j])for j in range(1,min(5,k-1)+1))
  if k<=5 and k%2:v^=e[k]
  s.append(v)
 return [s[k]^f.pow(e[1],k)for k in[3,5,9]]
def lin_constants(f,d):
 key=('L',f.degree,f.modulus,d)
 if key in CACHE:return CACHE[key]
 coeff=[1];out=[]
 for i in range(d):
  x=1<<i;lam=0
  for c in coeff:lam^=f.mul(c,x);x=f.square(x)
  b.check(lam!=0,'independent subspace basis')
  out.append(lam);new=[0]*(len(coeff)+1)
  for j,c in enumerate(coeff):new[j]^=f.mul(lam,c);new[j+1]^=f.square(c)
  coeff=new
 CACHE[key]=out;return out
def trim(a):
 a=list(a)
 while a and not a[-1]:a.pop()
 return tuple(a)
def supported(f,H,d):
 H=tuple(H);rem=(0,1)
 for lam in lin_constants(f,d):
  square=powmod_poly(rem,2,H,f);scaled=[f.mul(lam,c)for c in rem]
  rem=trim([(square[i]if i<len(square)else 0)^(scaled[i]if i<len(scaled)else 0)for i in range(max(len(square),len(scaled)))])
 return powmod_poly(rem,8,H,f)==()
def brute_support(f,H,d):
 rem=list(H)
 for x in range(1<<d):
  while len(rem)>1:
   val=0
   for c in reversed(rem):val=f.mul(val,x)^c
   if val:break
   q=[0]*(len(rem)-1);q[-1]=rem[-1]
   for i in range(len(rem)-2,0,-1):q[i-1]=rem[i]^f.mul(x,q[i])
   rem=q
 return rem==[1]
def polynomial(base,cols,t):return [v^xor(c[j]for i,c in enumerate(cols)if t>>i&1)for j,v in enumerate(base)]
def solve_fiber(f,base,cols,d,limit):
 b.check(all(c[4]==0 and c[3]==0 for c in cols),'centroid/E2fixed')
 n=f.degree;piv,free=image(f,d,2);a=base[4];e2=base[3];a2=f.square(a);g3=f.mul(a,e2)^base[2]
 g5=f.mul(e2,f.pow(a,3)^g3)^f.mul(a2,base[2])^f.mul(a,base[1])^base[0]
 span,width=spanpack(base,n,d);rhs=span|(project(g3|(g5<<n),piv,free)<<width);columns=[]
 for c in cols:
  v3=c[2];v5=f.mul(e2^a2,c[2])^f.mul(a,c[1])^c[0];columns.append(spanpack(c,n,d)[0]|(project(v3|(v5<<n),piv,free)<<width))
 rows=[(sum(((v>>j)&1)<<i for i,v in enumerate(columns)),(rhs>>j)&1)for j in range(width+len(free))];sol=b.norm.solve(rows,len(cols));out=dict(linear=sol,rows=rows,complete=True,enumerated=0,Gold9passed=0,support_calls=0,supported=[])
 if not sol['consistent']:return out
 count=1<<sol['nullity']
 if sol['nullity']>12 or count>limit:out.update(complete=False,reason='residualcandidate cap');return out
 p9,free9=image(f,d,3)
 for v in range(count):
  t=sol['particular']^xor(x for i,x in enumerate(sol['basis'])if v>>i&1);H=polynomial(base,cols,t);out['enumerated']+=1
  ms=moments(f,H)
  b.check(project(ms[0]|(ms[1]<<n),piv,free)==0 and spanpack(H,n,d)[0]==0,'literal Newton/linear moment replay')
  if project(sum(x<<(n*i)for i,x in enumerate(ms)),p9,free9):continue
  out['Gold9passed']+=1;out['support_calls']+=1
  if supported(f,H,d):out['supported'].append(dict(parameter=t,H=H))
 return out
def decompose(columns):
 piv={};outer=[];kernel=[]
 for i,value in enumerate(columns):
  v=value;t=1<<i
  while v:
   p=v.bit_length()-1
   if p in piv:v^=piv[p][0];t^=piv[p][1]
   else:piv[p]=(v,t);outer.append(1<<i);break
  if not v:kernel.append(t)
 return outer,kernel
