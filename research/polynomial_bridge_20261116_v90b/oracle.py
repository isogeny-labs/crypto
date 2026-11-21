from itertools import product
from functools import lru_cache
from research.twist_auxiliary_20261116_v86.algebra import s3
from .codec import field

def roots(c):
 f=field(c);m=c['arity'];out=set()
 @lru_cache(None)
 def nexts(x,y):return [u for u in range(f.q)if s3(x,y,u,c['A'],c['B'],f)==0]
 for xs in product(c['D'],repeat=m):
  def walk(j,u,aux):
   if j==m-1:
    if c['z']in nexts(u,xs[j]):out.add(xs+tuple(aux))
   else:
    for v in nexts(u,xs[j]):walk(j+1,v,aux+[v])
  for u in nexts(xs[0],xs[1]):walk(2,u,[u])
 return out
