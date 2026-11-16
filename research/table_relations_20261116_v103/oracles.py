import json,time,itertools,random
from .common import P,finish
from .kernel import features,evaluation,derive,accepted
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,roots_product,literal_split
START=time.monotonic();out={'boolean_cases':0,'polynomials_exhausted':0,'odd_fields':[],'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed'])
 for width in range(1,5):
  for degree in range(3):
   for kind in range(4):
    table=[]if kind==0 else list(range(1<<width))if kind==1 else [x for x in range(1<<width)if rng.randrange(2)]
    masks,basis,rank,rows=derive(table,width,degree,d['feature_cap']);literal_rows=[sum(int(all(x>>j&1 for j in range(width)if m>>j&1))<<i for i,m in enumerate(masks))for x in table];assert literal_rows==rows
    expected={r for r in range(1<<len(masks))if all(not((r&row).bit_count()&1)for row in literal_rows)};observed=set()
    for selector in range(1<<len(basis)):
     value=0
     for j,b in enumerate(basis):
      if selector>>j&1:value^=b
     observed.add(value)
    assert observed==expected;assert all(accepted(v,masks,basis)for v in table);out['boolean_cases']+=1;out['polynomials_exhausted']+=1<<len(masks)
 for p,mod in [(3,[0,1]),(3,[1,0,1])]:
  f=Arithmetic(p,mod);s=(f.q-1).bit_length();table=set();positive=0
  for xs in itertools.combinations_with_replacement(range(f.q),3):
   H=roots_product(xs,f);assert literal_split(H,f);table.add(sum(c<<(s*i)for i,c in enumerate(H[:3])));positive+=1
  masks,basis,rank,rows=derive(sorted(table),3*s,2,d['feature_cap']);assert all(accepted(v,masks,basis)for v in table)
  out['odd_fields'].append({'p':p,'mod':mod,'q':f.q,'positive_multisets':positive,'features':len(masks),'nullity':len(basis),'scope':'literal finite-field bit-encoding control only; no NIST or unrelated-prime reduction'})
 finish(out,None,START,'Exhaustive nullspace versus brute polynomial-coefficient oracle, including empty/full sets; odd prime/extension positive controls.','Profile all fourteen fixed-trace tables and verify every relation on every allowed row; no hard search before useful measured cuts.')
except Exception as e:finish(out,repr(e),START,'Low-degree relation oracle.','Preserve counterexample; repair a new immutable variant.')
