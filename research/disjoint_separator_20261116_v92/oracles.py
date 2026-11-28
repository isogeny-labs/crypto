from itertools import product
import json,time,hashlib
from .common import P,finish
from .kernel import solve,Prefix,certificate,columns
from research.twist_auxiliary_20261116_v86.algebra import s3
START=time.monotonic();out={'tiny':[],'large':[],'complete':False,'qualified_speedup':None};error=None
try:
 data=json.load(open(P/'DATA.json'))
 for n,mod,d in data['tiny']:
  S=Prefix(n,mod,d);f=S.f
  for y in range(1<<n):
   for B in[1,2]:
    spec={'n':n,'mod':mod,'d':d,'y':y,'B':B,'node_cap':4096};want=sorted((x,u)for x in range(1<<d)for u in range(1<<n)if s3(x,y,u,0,B,f)==0)
    vals={method:solve(spec,method)for method in data['methods']};assert all(v[0]==want for v in vals.values());pruned=vals['cells'][1]['pruned'];covered=0
    for cert in pruned:
     covered+=1<<cert['k'];assert all(not s3(x,y,u,0,B,f)==0 for x in range(1<<d)for u in range(cert['u0'],cert['u0']+(1<<cert['k'])))
    assert covered+vals['cells'][1]['leaves']==1<<n
    out['tiny'].append({'n':n,'d':d,'y':y,'B':B,'roots':want,'pruned_separator_count':covered,'stats':vals['cells'][1]})
 for spec in data['large_panels']:
  S=Prefix(spec['n'],spec['mod'],spec['d']);rows=[]
  for k in data['cell_dimensions']:
   for q in spec['queries']:
    y=q['y'];u0=q['u']&~((1<<k)-1);v=certificate(S,y,u0,k,q['B'])
    if 'planted_x'in q:assert s3(q['planted_x'],y,q['u'],0,q['B'],S.f)==0 and not v['empty']
    rows.append({'id':q['id'],'kind':q['kind'],'y':y,'u0':u0,'k':k,**v})
  out['large'].append({'id':spec['id'],'cells':rows,'by_dimension':{k:{'empty':sum(r['empty']for r in rows if r['k']==k),'total':sum(r['k']==k for r in rows),'rank_min':min(r['rank']for r in rows if r['k']==k),'rank_max':max(r['rank']for r in rows if r['k']==k)}for k in data['cell_dimensions']}})
 out['complete']=True
except Exception as e:error=repr(e)
finish(out,error,START,'Disjointconditionalrootcoversmatch independentliteraloracles;largerGF83cellsverifydirectdualwitnesseswithoutenumeration.','Comparecompleteconditionalcoldcostagainst reversequadraticenumeration;profilelargecellranksbeforeglobalintegration;passedcellsremainunresolved.')
