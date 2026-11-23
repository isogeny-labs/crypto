from itertools import product
import json,time,hashlib
from .common import P,finish
from .subspace import encode,value,rank
from research.polynomial_bridge_20261116_v90b.codec import field,encode as explicit,evaluate
from research.polynomial_bridge_20261116_v90b.oracle import roots
START=time.monotonic();out={'small':[],'actual':None,'complete':False,'qualified_speedup':None};error=None
try:
 data=json.load(open(P/'DATA.json'))
 for c in data['small']:
  f=field(c);D=[]
  for ks in product(range(f.p),repeat=len(c['basis'])):
   x=c['offset']
   for k,b in zip(ks,c['basis']):x=f.add(x,f.mul(k,b))
   D.append(x)
  assert len(set(D))==f.p**len(c['basis']);g,cs=encode(c);want=explicit({**c,'D':sorted(D)})
  def canonical(poly):return {tuple(t['powers']):t['coefficient']for t in poly}
  assert all(canonical(x)==canonical(y)for x,y in zip(g['equations'],want['equations']))
  for x in range(f.q):assert (value(cs,f.sub(x,c['offset']),f)==0)==(x in D)
  r=roots({**c,'D':D});assert all(all(evaluate(e,row,f)==0 for e in g['equations'])for row in r)
  out['small'].append({'id':c['id'],'p':f.p,'degree':f.n,'dimension':len(c['basis']),'domain_cardinality':len(D),'root_count':len(r),'equal_explicit_polynomials':True})
 c=data['actual'];f=field(c);g,cs=encode(c);n=f.n;cols=[value(cs,f.p**j,f)for j in range(n)];matrix=[[x//f.p**i%f.p for x in cols]for i in range(n)];r=rank(matrix,f.p);assert r==n-len(c['basis'])and all(x==0 for x in cols[:len(c['basis'])]);assert len(cs)==19
 name=P/'GF83_affine_D18_original_encoded.json'
 with name.open('x')as w:json.dump(g,w,separators=(',',':'))
 out['actual']={'field_degree':n,'source_dimension':len(c['basis']),'annihilator_terms':sum(x!=0 for x in cs),'domain_cardinality':f.p**len(c['basis']),'field_variable_count':len(g['variables']),'boolean_coordinates_before_source_elimination':len(g['variables'])*n,'source_linear_constraints':c['arity']*r,'coordinates_after_eliminating_source_constraints':len(g['variables'])*n-c['arity']*r,'original_S3_coordinate_equations':(c['arity']-1)*n,'linear_map_rank':r,'linear_kernel_dimension':n-r,'encoded_bytes':name.stat().st_size,'encoded_sha256':hashlib.sha256(name.read_bytes()).hexdigest(),'solve_performed':False,'exact_root_count':None};out['complete']=True
except Exception as e:error=repr(e)
finish(out,error,START,'Compressed exact affine finite-field domains match explicit product polynomials;actualGF83kernel rank and339/332coordinate layout certified.','Build sparse symbolic binaryFrobenius descent with sourcebit substitution before multiplication;prove equivalence to all332original equations before bounded hardsearch;primefields use distinct interval support.')
