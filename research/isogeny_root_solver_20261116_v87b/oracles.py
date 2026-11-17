from itertools import product
from pathlib import Path
import json,time,hashlib
from .common import P,finish
from .solver import solve
from research.twist_three_isogeny_20261116_v86b.curve import add,scale,neg
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
START=time.monotonic()
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
out={'tiny':[],'actual':[],'qualified_speedup':None};error=None
try:
 d=json.load(open(P/'DATA.json'))
 for p,A,B in d['tiny_curves']:
  pts=[None]+[(x,y)for x in range(p)for y in range(p)if(y*y-x**3-A*x-B)%p==0];assert len(pts)%2==1;T=next(U for U in pts[1:]if scale(3,U,p,A)is None)
  for arity in[3,5]:
   case={'p':p,'A':A,'B':B,'R':pts[1:],'Q':T,'T':T,'arity':arity,'group_order':len(pts)};a=solve(case,'reference');b=solve(case,'auxiliary');expected=literal(case);assert a['complete']and b['complete']and set(a['roots'])==set(b['roots'])==expected;out['tiny'].append({'p':p,'arity':arity,'complete_original_roots':len(expected),'equal':True})
 for case in d['cases']:
  a=solve(case,'reference');b=solve(case,'auxiliary');assert a['complete']and b['complete']and a['roots']==b['roots'];out['actual'].append({'id':case['id'],'exact_root_count':a['exact_root_count'],'root_sha256':hashlib.sha256(json.dumps(a['roots']).encode()).hexdigest(),'reference_stats':a['stats'],'auxiliary_stats':b['stats']})
 out['complete']=True
except Exception as e:error=repr(e)
finish(out,error,START,'Complete originalS3root sets match independent tiny polynomial enumeration and actual-width reference/auxiliary solves.','Run frozen cold comparison with full setup/fiber/output costs;this changed-domain calibration is not a hard original-prefix benchmark.')
