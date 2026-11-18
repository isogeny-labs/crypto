from research.polynomial_bridge_20261116_v90b.codec import encode as explicit,field

def value(cs,x,f):
 out=0
 for i,c in enumerate(cs):out=f.add(out,f.mul(c,f.power(x,f.p**i)))
 return out

def annihilator(basis,f):
 cs=[1]
 for b in basis:
  v=value(cs,b,f);assert v,'dependent basis';factor=f.power(v,f.p-1);n=[0]*(len(cs)+1)
  for i,c in enumerate(cs):n[i]=f.sub(n[i],f.mul(factor,c));n[i+1]=f.add(n[i+1],f.power(c,f.p))
  cs=n
 return cs

def encode(c):
 f=field(c);basis=c['basis'];a=c['offset'];cs=annihilator(basis,f);g=explicit({**c,'D':[]});constant=f.neg(value(cs,a,f));nv=len(g['variables'])
 for j in range(c['arity']):
  terms=[]
  if constant:terms.append({'coefficient':constant,'powers':[0]*nv})
  for i,co in enumerate(cs):
   if co:
    powers=[0]*nv;powers[j]=f.p**i;terms.append({'coefficient':co,'powers':powers})
  g['equations'][c['arity']-1+j]=terms
 g['source_domain']={'kind':'affine_prime_subspace','basis':basis,'offset':a,'cardinality':f.p**len(basis)}
 return g,cs

def rank(matrix,p):
 rows=[r[:]for r in matrix];r=0
 for j in range(len(rows[0])):
  k=next((i for i in range(r,len(rows))if rows[i][j]%p),None)
  if k is None:continue
  rows[r],rows[k]=rows[k],rows[r];iv=pow(rows[r][j],-1,p);rows[r]=[x*iv%p for x in rows[r]]
  for i in range(len(rows)):
   if i!=r:
    a=rows[i][j];rows[i]=[(x-a*y)%p for x,y in zip(rows[i],rows[r])]
  r+=1
 return r
