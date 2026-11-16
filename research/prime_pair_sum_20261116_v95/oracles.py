from .common import P,finish
from .kernel import solve
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
import json,time
START=time.monotonic();out={'tiny':[],'wrap_rejections':0,'identity_checks':0};error=None
try:
 d=json.loads((P/'DATA.json').read_text())
 for c in d['tiny']:
  p,A,B,u,M=[c[k]for k in['p','A','B','u','M']];f=Field(p,[0,1]);expected=[(x,y)for x in range(M+1)for y in range(M+1)if s3(x,y,u,A,B,f)==0];a=solve(c,'sum');b=solve(c,'source');assert a['roots']==b['roots']==expected
  for x in range(M+1):
   for y in range(M+1):
    S=x+y;T=x*y;assert s3(x,y,u,A,B,f)==((T-u*S-2*u*u-A)**2-4*(u**3+A*u+B)*(S+u))%p;out['identity_checks']+=1
  out['tiny'].append({'id':c['id'],'root_count':len(expected),'f_u_zero':(u**3+A*u+B)%p==0,'sum_stats':a['stats'],'source_stats':b['stats']})
 for c in d['bad_wrap']:
  for method in['source','sum']:
   try:solve(c,method)
   except AssertionError:out['wrap_rejections']+=1
   else:raise AssertionError('unproved wrap domain admitted')
 finish(out,None,START,'Exact conic sum scan and strong one-source quadratic scan match every literal interval pair.','Cold compare same fixed targets across all five NIST fields; no O(M^2) straw baseline.')
except Exception as e:finish(out,repr(e),START,'Prime interval sum/source oracle.','Preserve failed source and repair only under a new immutable version.')
