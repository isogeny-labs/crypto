from itertools import product
import json,time
from .common import P,finish
from .kernel import polynomial,certificate,evaluate
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
START=time.monotonic();out={'tiny':[],'actual':[],'complete':False,'qualified_speedup':None};error=None
try:
 data=json.load(open(P/'DATA.json'))
 for n,mod,d in data['tiny']:
  f=Field(2,mod)
  for k in[1,2]:
   for u0 in range(0,1<<n,1<<k):
    for B in[1,2]:
     expected={(x,y,u)for x in range(1<<d)for y in range(1<<d)for u in range(u0,u0+(1<<k))if s3(x,y,u,0,B,f)==0}
     for r in range(d+1):
      covered=set();recovered=set();empty=0
      for gx,gy in product(range(1<<r),repeat=2):
       poly=polynomial(n,mod,d,k,u0,r,gx,gy,B);v=certificate(poly,n)
       if r==0 and B==1:assert not v['empty']and 1 in [c for m,c in poly.items()if m]
       roots=set()
       for x in range(gx,1<<d,1<<r):
        for y in range(gy,1<<d,1<<r):
         for u in range(u0,u0+(1<<k)):
          point=x,y,u;assert point not in covered;covered.add(point);assignment=(x>>r)|((y>>r)<<(d-r))|((u^u0)<<(2*(d-r)));val=evaluate(poly,assignment);assert val==s3(x,y,u,0,B,f)
          if not val:roots.add(point)
       assert not v['empty']or not roots
       if not v['empty']:recovered|=roots
       empty+=v['empty']
      assert recovered==expected and len(covered)==(1<<(2*d+k));out['tiny'].append({'n':n,'d':d,'k':k,'u0':u0,'B':B,'guard_bits':r,'guards':1<<(2*r),'empty_guards':empty,'root_count':len(expected)})
 spec=data['actual'];n,mod,d=spec['n'],spec['mod'],spec['d'];f=Field(2,mod)
 for q in spec['queries']:
  for k in data['cell_dimensions']:
   u0=q['u']&~((1<<k)-1)
   for r in data['guard_depths']:
    rows=[]
    for gx,gy in product(range(1<<r),repeat=2):
     poly=polynomial(n,mod,d,k,u0,r,gx,gy);v=certificate(poly,n)
     if r==0:assert not v['empty']
     if 'planted_x'in q and gx==(q['planted_x']&((1<<r)-1))and gy==(q['y']&((1<<r)-1)):
      assignment=(q['planted_x']>>r)|((q['y']>>r)<<(d-r))|((q['u']^u0)<<(2*(d-r)));assert evaluate(poly,assignment)==0 and not v['empty']
     rows.append({'gx':gx,'gy':gy,'terms':len(poly),'constant':poly.get(0,0),**v})
    out['actual'].append({'id':q['id'],'u0':u0,'k':k,'guard_bits':r,'guards':rows,'empty_guards':sum(v['empty']for v in rows),'entire_cell_empty':all(v['empty']for v in rows)})
 out['complete']=True
except Exception as e:error=repr(e)
finish(out,error,START,'Guarded exact Boolean coefficient spans checked against complete independent tiny S3 covers and frozenGF83 cells.','If shallow guards do not prune,change to symmetric sum/product coordinates retaining product support;do not extend identical rank-only guard searches blindly.')
