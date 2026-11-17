import json,time
from .common import P,finish
from .kernel import solve
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
START=time.monotonic();out={'tiny':[],'actual':[],'complete':False,'qualified_speedup':None};error=None
try:
 data=json.load(open(P/'DATA.json'))
 for n,mod,d in data['tiny']:
  f=Field(2,mod)
  for u in range(1<<n):
   for B in[1,2]:
    c={'n':n,'mod':mod,'d':d,'u':u,'B':B,'fiber_cap_bits':16};want=sorted((x,y)for x in range(1<<d)for y in range(1<<d)if s3(x,y,u,0,B,f)==0);v=solve(c);assert v['complete']and v['roots']==want
    out['tiny'].append({'n':n,'d':d,'u':u,'B':B,'root_count':len(want),'stats':v['stats']})
 for c in data['cases']:
  v=solve(c);f=Field(2,c['mod'])
  for x,y in v['roots']:assert x<1<<c['d']and y<1<<c['d']and s3(x,y,c['u'],0,c['B'],f)==0
  out['actual'].append({'id':c['id'],**v})
 out['complete']=True
except Exception as e:error=repr(e)
finish(out,error,START,'Symmetric coefficient fibers plus exact quadratic domain support match independent tiny all-pair roots;actual exceptions and caps remain explicit.','Run matching genuine F4/F5 original-source-bit diagnostics;for capped coefficient fibers derive support factorization instead of enumerating them blindly.')
