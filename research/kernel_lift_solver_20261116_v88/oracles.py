from itertools import product
from pathlib import Path
import json,time,hashlib
from .common import P,finish
from .solver import solve
from research.isogeny_root_solver_20261116_v87b.oracles import literal
from research.twist_three_isogeny_20261116_v86b.curve import scale,phi,target
START=time.monotonic();out={'tiny':[],'actual':[],'qualified_speedup':None};error=None
try:
 d=json.load(open(P/'DATA.json'))
 for p,A,B in d['tiny_curves']:
  pts=[None]+[(x,y)for x in range(p)for y in range(p)if(y*y-x**3-A*x-B)%p==0];T=next(U for U in pts[1:]if scale(3,U,p,A)is None);full=pts[1:];xs=sorted({U[0]for U in full})[:2];restricted=[U for U in full if U[0]in xs]
  for label,R in [('full',full),('restricted',restricted)]:
   for arity in[3,5]:
    c={'p':p,'A':A,'B':B,'R':R,'Q':T,'T':T,'arity':arity,'group_order':len(pts)};expected=literal(c)
    for method in d['methods']:
     v=solve(c,method);assert v['complete']and set(v['roots'])==expected
    out['tiny'].append({'p':p,'domain':label,'arity':arity,'complete_original_roots':len(expected),'equal':True})
 for c in d['cases']:
  p,A=c['p'],c['A'];U=tuple(c['R'][0]);Q=tuple(c['Q']);T=tuple(c['T']);V=phi(U,T,p,A);AA=target(T,p,A,c['B'])[0];assert Q==scale(5,U,p,A)and all(scale(k,V,p,AA)is not None for k in range(1,11));values={method:solve(c,method)for method in d['methods']};v=values['reference'];assert all(x['complete']and x['roots']==v['roots']for x in values.values())
  if len(c['R'])==6:assert v['exact_root_count']==81
  out['actual'].append({'id':c['id'],'exact_root_count':v['exact_root_count'],'root_sha256':hashlib.sha256(json.dumps(v['roots']).encode()).hexdigest(),'stats':{m:r['stats']for m,r in values.items()}})
 out['complete']=True
except Exception as e:error=repr(e)
finish(out,error,START,'Positive-root kernel-constrained lifts match exhaustive polynomial oracles,partial fibers and actual-width reference outputs.','Measure full cold setup/lift/output cost;extend the finite-kernel label algebra to binary kernels and finite-dimensional kernels under a separate complete-root proof.')
