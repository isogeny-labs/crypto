from itertools import product
from functools import lru_cache
import json,time,random,hashlib
from .common import P,finish
from .trellis import paths,fresh_stats
from .fiber_solver import solve
from research.twist_auxiliary_20261116_v86.algebra import Field,short_points,s3
from research.twist_auxiliary_20261116_v86.groups import add as gadd
START=time.monotonic();out={'groups':[],'curves':[],'complete':False,'qualified_speedup':None};error=None
try:
 d=json.load(open(P/'DATA.json'));rng=random.Random(891611)
 for dims in [(2,),(3,),(3,3)]:
  elems=list(product(*(range(n)for n in dims)));tab=[[elems.index(tuple((a+b)%n for a,b,n in zip(u,v,dims)))for v in elems]for u in elems]
  for m in [3,5]:
   for rep in range(24):
    allowed=[tuple(k for k in range(len(elems))if rng.randrange(3))for _ in range(m)];forbidden={j:{k for k in range(len(elems))if rng.randrange(5)==0}for j in range(2,m)};target=rng.randrange(len(elems));want=[]
    for ks in product(*allowed):
     s=0;ok=True
     for j,k in enumerate(ks,1):
      s=tab[s][k]
      if s in forbidden.get(j,()):ok=False;break
     if ok and s==target:want.append(ks)
    stats=fresh_stats();got=list(paths(allowed,tab,target,forbidden,stats));assert got==want and stats['dp_feasible_paths']==len(want)
    out['groups'].append({'dims':dims,'arity':m,'case':rep,'paths':len(want),'stats':stats})
 for p,mod,A,B,n in d['small_configs']:
  f=Field(p,mod);binary=p==2;byx=short_points(f,A,B);pts=[None]+[(x,y)for x,ys in byx.items()for y in ys];add=lambda U,V:gadd(U,V,f,A if binary else 0,B if binary else A,binary);neg=lambda U:None if U is None else(U[0],f.add(U[0],U[1])if binary else f.neg(U[1]))
  def image(U):
   V=None
   for _ in range(n):V=add(V,U)
   return V
  K=[U for U in pts if image(U)is None];xs=sorted(x for x in byx if byx[x])[:2];R=[U for U in pts[1:]if U[0]in xs]
  assert all(neg(U)in R for U in R)
  @lru_cache(None)
  def N(x,y):return tuple(z for z in range(f.q)if s3(x,y,z,A,B,f)==0)
  for Q in list(dict.fromkeys([pts[1],pts[-1]])):
   for m in ([3,5]if f.q<=9 else[3]):
    want=set()
    for sources in product(xs,repeat=m):
     def walk(j,u,aux):
      if j==m-1:
       if Q[0]in N(u,sources[j]):want.add(sources+tuple(aux))
      else:
       for v in N(u,sources[j]):walk(j+1,v,aux+[v])
     for u in N(sources[0],sources[1]):walk(2,u,[u])
    got,stats=solve(R,Q,m,add,neg,image,K);assert got==want,('roots',p,mod,A,B,n,Q,m,len(got),len(want),sorted(want-got)[:1]);out['curves'].append({'p':p,'mod':mod,'A':A,'B':B,'multiplier':n,'Q':Q,'arity':m,'source_points':len(R),'complete_original_roots':len(want),'root_sha256':hashlib.sha256(json.dumps(sorted(want)).encode()).hexdigest(),'stats':stats})
 out['complete']=True
except Exception as e:error=repr(e)
finish(out,error,START,'Generic finite-kernel trellis compared to independent exhaustive original S3 roots over prime,binary and odd extension fields.','Measure same P224 cases;derive genuine matched finite-domain GB inputs and sparse block interfaces;retain partial fibers and all original variables.')
