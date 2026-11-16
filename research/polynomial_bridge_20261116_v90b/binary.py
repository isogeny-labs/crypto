from .codec import field

def anf(instance,cap=100000):
 from research.twist_auxiliary_20261116_v86.algebra import Field
 z=instance['field'];assert z['characteristic']==2;f=Field(2,z['modulus']);n=f.n;v=len(instance['variables']);assert n*v<=64
 def plus(a,b):return [x^y for x,y in zip(a,b)]
 zero=lambda:[set()for _ in range(n)]
 const=lambda a:[{0}if a>>i&1 else set()for i in range(n)]
 def multiply(a,b):
  out=zero()
  for i,x in enumerate(a):
   for j,y in enumerate(b):
    coeff=f.mul(1<<i,1<<j)
    if not coeff:continue
    z=set()
    for u in x:
     for w in y:
      k=u|w
      if k in z:z.remove(k)
      else:z.add(k)
    for h in range(n):
     if coeff>>h&1:out[h]^=z
  assert sum(map(len,out))<=cap,'ANF term cap'
  return out
 def power(a,e):
  out=const(1)
  while e:
   if e&1:out=multiply(out,a)
   e>>=1
   if e:a=multiply(a,a)
  return out
 X=[[{1<<(j*n+i)}for i in range(n)]for j in range(v)];cache={};equations=[]
 for poly in instance['equations']:
  acc=zero()
  for t in poly:
   value=const(t['coefficient'])
   for j,e in enumerate(t['powers']):
    if e:
     cache.setdefault((j,e),None)
     if cache[j,e]is None:cache[j,e]=power(X[j],e)
     value=multiply(value,cache[j,e])
   acc=plus(acc,value)
  equations.extend(acc)
 # Field equations vanish identically under canonical coordinate representation.
 return {'schema':'neutral_boolean_polynomial_system_v1','instance_id':instance['instance_id'],'field':{'kind':'prime','p':2},'domain':[0,1],'n_vars':n*v,'equation_count':len(equations),'equations':[[[i for i in range(n*v)if mask>>i&1]for mask in sorted(poly)]for poly in equations]}

def evaluate(poly,mask):return sum((mask&sum(1<<i for i in term))==sum(1<<i for i in term)for term in poly)%2

def decode(mask,degree,variables):return tuple((mask>>(i*degree))&((1<<degree)-1)for i in range(variables))
