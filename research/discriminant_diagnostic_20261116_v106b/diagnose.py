import json,time,random
from .common import P,finish
from research.discriminant_roots_20261116_v106.kernel import Field,Arithmetic,solve,linear_part,radical,extract,divrem
START=time.monotonic();out={'cases':[],'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed'])
 for q in [8,16]:
  for _ in range(320):rng.randrange(q)
 cases=[[rng.randrange(3)for _ in range(5)]+[1]for _ in range(64)];f=Field(3,[0,1])
 for index,H in enumerate(cases[:4]):
  expected=[]
  for x in range(3):
   a=list(H);count=0
   while len(a)>1:
    q,r=divrem(a,[f.neg(x),1],f)
    if r:break
    count+=1;a=q
   if count:expected.append([x,count])
  row={'index':index,'H':H,'expected':expected,'linear_part':linear_part(H,f),'radical':radical(H,f),'methods':{}}
  for method in ['direct','radical']:
   try:row['methods'][method]=solve(H,f,method,d['seed'])
   except Exception as e:row['methods'][method]={'error':repr(e)}
  out['cases'].append(row)
 finish(out,None,START,'Changed diagnostic isolates exact odd-field counterexamples rather than retrying failed broad oracle.','Repair the identified arithmetic/representation mismatch in a fresh variant, then rerun full independent oracle before timing.')
except Exception as e:finish(out,repr(e),START,'Odd-field failure diagnosis.','Preserve diagnostic and derive a new changed assumption.')
