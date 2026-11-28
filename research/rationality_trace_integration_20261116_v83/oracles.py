import json,time,hashlib
from itertools import combinations_with_replacement
from .common import P,finish
from .kernel import label,reference,support,reject,bundle,Frobenius
from research.image_cells_20261115_v67.algebra import BField,roots_poly,trim,binary_tower,supported
START=time.monotonic();out={'small_cases':0,'small_support_cases':0,'actual_cases':0,'new_main_rejections':0,'fixture_anti_counts':{},'qualified_speedup':None};error=None
try:
 d=json.loads((P/'DATA.json').read_text())
 for r in d['records']:assert hashlib.sha256(json.dumps(r,sort_keys=True,separators=(',',':')).encode()).hexdigest()==d['record_sha256'][r['name']]
 for n,M in d['small_fields']:
  f=BField(n,M);F=Frobenius(f);dim=min(2,n);scalars=binary_tower(f,dim)
  for A in [0,1]:
   rat={x:any(f.sq(y)^f.mul(x,y)==f.mul(f.sq(x),x)^f.mul(A,f.sq(x))^1 for y in range(f.q))for x in range(f.q)}
   for degree in d['degrees']:
    for xs in combinations_with_replacement(range(f.q),degree):
     H=roots_poly(xs,f);C,L,z=label(H,A,1,f,F);c,l,rz=reference(H,A,1,f);anti=sum(not rat[x]for x in xs if x);expected=roots_poly([0]*(len(xs)-xs.count(0)-anti)+[1]*anti,f);assert C==c==expected and L==trim(l)and z==rz;out['small_cases']+=1
     assert support(H,scalars,f)==supported(H,range(1<<dim),f)['supported'];out['small_support_cases']+=1
 f=BField(d['n'],d['modulus']);a=bundle(d['records'],f,'reference',d['dimension']);b=bundle(d['records'],f,'shared',d['dimension']);assert a==b
 for r,row in zip(d['records'],b):
  ref=d['expected_labels'][r['name']];assert row['support']==r['support_truth'];assert row['image_polynomial']==ref['image_polynomial']and row['lambda_remainder']==trim(ref['lambda_remainder'])and row['rationality_reject']==ref['reject'];out['actual_cases']+=1
  if r['name'].startswith(('relaxed_','fresh_')):out['new_main_rejections']+=row['support']and row['rationality_reject']
  if 'independent_anti_count'in ref:out['fixture_anti_counts'][r['name']]=ref['independent_anti_count']
 out['actual']=b;out['complete']=True
except Exception as e:error=repr(e)
finish(out,error,START,'Complete rationality/support integration checked against exhaustive small point/factor oracles and all36 historical cases.','Measure cold setup-inclusive bundle and standalone support costs; zero new pruning should redirect to whole-cell or odd-characteristic hypotheses.')
