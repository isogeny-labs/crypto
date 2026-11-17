from research.twist_auxiliary_20261116_v86.algebra import Field

class Arithmetic(Field):
 def power(self,a,k):return pow(a,k,self.p)if self.n==1 else super().power(a,k)

def nonsquare(f):
 for a in range(2,min(f.q,258)):
  if f.power(a,(f.q-1)//2)==f.neg(1):return a
 raise OverflowError('bounded nonsquare search; unresolved')

def square_root(a,f,delta):
 if not a:return 0
 if f.power(a,(f.q-1)//2)!=1:return None
 q=f.q-1;s=0
 while q%2==0:q//=2;s+=1
 x=f.power(a,(q+1)//2);t=f.power(a,q);c=f.power(delta,q);m=s
 while t!=1:
  v=t;i=0
  while v!=1 and i<m:v=f.sq(v);i+=1
  assert i<m
  b=f.power(c,1<<(m-i-1));x=f.mul(x,b);c=f.sq(b);t=f.mul(t,c);m=i
 assert f.sq(x)==a;return x

def coefficients(x,y,A,B,f):
 xy=f.mul(x,y);s=f.add(x,y)
 return(f.sq(f.sub(x,y)),f.neg(f.mul(2%f.p,f.add(f.mul(s,f.add(xy,A)),f.mul(2%f.p,B)))),f.sub(f.sq(f.sub(xy,A)),f.mul(4%f.p,f.mul(B,s))))

def batch_inverse(values,f):
 prefix=[1]
 for a in values:assert a;prefix.append(f.mul(prefix[-1],a))
 inv=f.inv(prefix[-1]);out=[0]*len(values)
 for i in range(len(values)-1,-1,-1):out[i]=f.mul(inv,prefix[i]);inv=f.mul(inv,values[i])
 return out

def solve(spec,method):
 f=Arithmetic(spec['p'],spec['mod']);assert f.p%2==1;A=spec['A'];B=spec['B'];D=spec['domain'];assert len(D)==len(set(D))and len(D)<=128
 assert f.add(f.mul(4%f.p,f.mul(f.sq(A),A)),f.mul(27%f.p,f.sq(B)))!=0
 delta=nonsquare(f);di=f.inv(delta);cache={};stats={'sqrt_calls':0,'inverse_calls':0,'pair_count':len(D)**2};out=[];pending=[]
 if method=='cached':
  for x in D:
   fx=f.add(f.mul(f.sq(x),x),f.add(f.mul(A,x),B))
   cls=0 if fx==0 else(1 if f.power(fx,(f.q-1)//2)==1 else -1)
   r=0 if not fx else square_root(fx if cls==1 else f.mul(fx,di),f,delta);stats['sqrt_calls']+=int(fx!=0);cache[x]=(cls,r)
 for x in D:
  for y in D:
   a,b,c=coefficients(x,y,A,B,f)
   if not a:
    if b:pending.append((x,y,f.neg(c),None,b))
    else:assert c!=0 # smoothness excludes an identically-zero diagonal fiber
    continue
   if method=='cached':
    cx,rx=cache[x];cy,ry=cache[y]
    if cx*cy==-1:continue
    root=f.mul(4%f.p,f.mul(rx,ry))
    if cx==cy==-1:root=f.mul(delta,root)
   else:
    disc=f.sub(f.sq(b),f.mul(4%f.p,f.mul(a,c)));root=square_root(disc,f,delta);stats['sqrt_calls']+=1
    if root is None:continue
   pending.append((x,y,f.neg(b),root,f.mul(2%f.p,a)))
 if method=='cached':inverses=batch_inverse([v[4]for v in pending],f)if pending else[];stats['inverse_calls']+=int(bool(pending))
 else:inverses=[f.inv(v[4])for v in pending];stats['inverse_calls']+=len(pending)
 for(x,y,num,r,den),iv in zip(pending,inverses):
  roots=[f.mul(num,iv)]if r is None else sorted({f.mul(f.add(num,r),iv),f.mul(f.sub(num,r),iv)})
  out.extend((x,y,z)for z in roots)
 return{'complete':True,'roots':sorted(out),'stats':stats,'exact_root_count':len(out)}
