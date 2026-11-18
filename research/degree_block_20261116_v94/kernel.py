"""Exact Boolean degree-block preprocessing; no hidden S3 coefficients supplied."""
def xor_polys(polys, witness):
 out=set()
 for i,p in enumerate(polys):
  if witness>>i&1:out.symmetric_difference_update(p)
 return out

def extract(polys):
 piv={};linear=[]
 for i,p in enumerate(polys):
  q=set(p);w=1<<i
  while True:
   high=[m for m in q if m.bit_count()>1]
   if not high:linear.append((q,w));break
   m=max(high)
   if m not in piv:piv[m]=(q,w);break
   v,h=piv[m];q^=v;w^=h
 return linear,len(piv)

def affine_map(linear,n):
 piv={}
 for q,w in linear:
  row=sum(1<<(m.bit_length()) for m in q if m);row|=int(0 in q)
  while row>>1:
   p=row.bit_length()-1
   if p not in piv:piv[p]=row;break
   row^=piv[p]
  if row==1:return None
 free=[j for j in range(n)if j+1 not in piv];maps={j:{1<<i}for i,j in enumerate(free)}
 for p,row in sorted(piv.items()):
  q={0}if row&1 else set()
  for j in range(p-1):
   if row>>(j+1)&1:q^=maps[j]
  maps[p-1]=q
 return [maps[j]for j in range(n)],len(free)

def substitute(poly,maps):
 out=set()
 for mon in poly:
  q={0}
  for j in range(len(maps)):
   if mon>>j&1:
    v=set()
    for a in q:
     for b in maps[j]:
      c=a|b
      if c in v:v.remove(c)
      else:v.add(c)
    q=v
  out^=q
 return out

def preprocess(polys,n,max_stages=65):
 current=[set(p)for p in polys];maps=[{1<<j}for j in range(n)];stages=[]
 for _ in range(max_stages):
  linear,rank=extract(current);a=affine_map(linear,n)
  record={'n':n,'nonlinear_rank':rank,'linear_witnesses':[w for q,w in linear]}
  if a is None:
   record['inconsistent']=True;stages.append(record)
   return {'unsat':True,'n':0,'polys':[],'maps':[],'stages':stages}
  local,k=a
  if k==n:break
  record.update({'inconsistent':False,'next_n':k,'maps':[sorted(q)for q in local]});stages.append(record)
  current=[substitute(q,local)for q in current];maps=[substitute(q,local)for q in maps];n=k
 return {'unsat':False,'n':n,'polys':[sorted(p)for p in current if p],'maps':[sorted(q)for q in maps],'stages':stages}

def value(poly,z):return sum((z&m)==m for m in poly)%2

def lift(result,z):return sum(value(q,z)<<j for j,q in enumerate(result['maps']))

def residual_roots(result,cap=20):
 if result['unsat']:return []
 if result['n']>cap:raise OverflowError('exhaustive residual cap')
 return sorted(lift(result,z)for z in range(1<<result['n'])if not any(value(p,z)for p in result['polys']))
