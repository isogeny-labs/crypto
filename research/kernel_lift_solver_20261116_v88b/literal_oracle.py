from itertools import product
from research.twist_auxiliary_20261116_v86.algebra import Field,s3

def literal(case):
 p=case['p'];f=Field(p,[0,1]);A=case['A'];B=case['B'];D=sorted({U[0]for U in case['R']});m=case['arity'];z=case['Q'][0];N={(x,y):[u for u in range(p)if s3(x,y,u,A,B,f)==0]for x,y in product(range(p),repeat=2)};out=set()
 for xs in product(D,repeat=m):
  def walk(i,u,aux):
   if i==m-1:
    if z in N[u,xs[i]]:out.add(xs+tuple(aux))
    return
   for v in N[u,xs[i]]:walk(i+1,v,aux+[v])
  for u in N[xs[0],xs[1]]:walk(2,u,[u])
 return out
