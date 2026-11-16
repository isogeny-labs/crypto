from itertools import product
import json,time,random,hashlib
from .common import P,finish
from .kernel import Solver
from research.twist_auxiliary_20261116_v86.algebra import s3
START=time.monotonic();out={'small':[],'large':[],'complete':False,'qualified_speedup':None};error=None
try:
 d=json.load(open(P/'DATA.json'))
 for n,mod,dim in d['tiny']:
  S=Solver(n,mod,dim);f=S.f;count=0;nonempty=0;two=0
  for a,b,c in product(range(f.q),repeat=3):
   want=[x for x in range(1<<dim)if f.mul(a,f.sq(x))^f.mul(b,x)==c]
   for method in d['methods']:assert S.solve(a,b,c,method)==want,(n,dim,a,b,c,method)
   count+=1;nonempty+=bool(want);two+=len(want)==2
  for y,u in product(range(f.q),repeat=2):
   want=[x for x in range(1<<dim)if s3(x,y,u,0,1,f)==0];a,b,c=S.coefficients(y,u)
   for method in d['methods']:assert S.solve(a,b,c,method)==want
  out['small'].append({'degree':n,'dimension':dim,'linear_queries':count,'S3_queries':f.q**2,'nonempty':nonempty,'two_root_fibers':two})
 for spec in d['panels']:
  S=Solver(spec['n'],spec['mod'],spec['d']);rows=[]
  for q in spec['queries']:
   a,b,c=S.coefficients(q['y'],q['u'],q['B'])if q['kind']=='S3'else(q['a'],q['b'],q['c']);vals={method:S.solve(a,b,c,method)for method in d['methods']};want=vals['matrix'];assert all(v==want for v in vals.values())
   for x in want:assert S.f.mul(a,S.f.sq(x))^S.f.mul(b,x)==c
   if q['kind']=='S3':assert all(s3(x,q['y'],q['u'],0,q['B'],S.f)==0 for x in want)
   rows.append({'id':q['id'],'kind':q['kind'],'root_count':len(want),'roots':want})
  out['large'].append({'id':spec['id'],'queries':rows,'root_sha256':hashlib.sha256(json.dumps(rows).encode()).hexdigest()})
 out['complete']=True
except Exception as e:error=repr(e)
finish(out,error,START,'Exact imageannihilator,streamedcolumns,matrix and normalizedAS fibers match exhaustive tiny roots and frozenactualfieldqueries.','Measure full conditionalsetup/membership/recovery across all methods;imagepredicatealone not allroots;consider simultaneousannihilator cells only afterrepresentationbound.')
