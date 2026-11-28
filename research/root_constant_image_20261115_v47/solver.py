from research.root_conditioned_fibers_20261115_v45 import solver as old

def split(f,fiber):
 n=f.degree;mask=(1<<n)-1;U=[v&mask for v in fiber['basis']];V=[v>>n for v in fiber['basis']];piv={};outer=[];kernel=[]
 for j,u in enumerate(U):
  v=u;tag=1<<j
  while v:
   p=v.bit_length()-1
   if p not in piv:piv[p]=(v,tag);outer.append(1<<j);break
   a,b=piv[p];v^=a;tag^=b
  if not v:kernel.append(tag)
 VK=[old.b.apply(V,t)for t in kernel];vpiv={}
 for j,v in enumerate(VK):
  tag=1<<j
  while v:
   p=v.bit_length()-1
   if p not in vpiv:vpiv[p]=(v,tag);break
   a,b=vpiv[p];v^=a;tag^=b
  assert v,'noninjective uv fiber'
 return dict(U=[old.b.apply(U,t)for t in outer],V=[old.b.apply(V,t)for t in outer],outer=outer,kernel=kernel,vpiv=vpiv,u0=fiber['particular']&mask,v0=fiber['particular']>>n,dimension=len(fiber['basis']))

def reduce(S,value):
 tag=0
 for p,(v,t)in sorted(S['vpiv'].items(),reverse=True):
  if value>>p&1:value^=v;tag^=t
 return value,tag

def point(f,ctx,h,S,x,y):
 assert x and x!=ctx['z'] and h
 rhs=f.square(x)^f.mul(h,f.mul(f.mul(x,f.inv(x^ctx['z'])),f.mul(ctx['c'],x)^ctx['d']^y))^f.mul(x,S['u0'])^S['v0']
 cols=[reduce(S,f.mul(x,u)^v)[0]for u,v in zip(S['U'],S['V'])];sol=old.b.solve(old.b.rows_from(cols,reduce(S,rhs)[0],f.degree),len(cols))
 if sol is None:return None
 def lift(bits):
  rem,tag=reduce(S,rhs^f.mul(x,old.b.apply(S['U'],bits))^old.b.apply(S['V'],bits));assert not rem
  return old.b.apply(S['outer'],bits)^old.b.apply(S['kernel'],tag)
 p=lift(sol['particular']);return dict(particular=p,basis=[lift(sol['particular']^v)^p for v in sol['basis']],rank=S['dimension']-len(sol['basis']))
