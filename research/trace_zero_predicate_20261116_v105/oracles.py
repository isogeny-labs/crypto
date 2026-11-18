import json,time,random,itertools,hashlib
from .common import P,finish
from .kernel import normalization,inverse,field_isomorphism,translate_poly,depress_quintic
from research.residue_tables_20261116_v102b.kernel import multiplication,allowed
from research.table_relations_20261116_v103.kernel import derive,features
from research.relation_cover_20261116_v104b.kernel import separating_subset
from research.relation_zero_locus_20261116_v103b.kernel import table_mask
from research.singular_quadratic_guard_20261116_v104.kernel import bad_quadratic
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,roots_product
START=time.monotonic();out={'normalization_basis_controls':0,'isomorphism_pairs':0,'odd_controls':[],'templates':[],'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['oracle_seed']);templates=[]
 for s,m in d['canonical_fields']:
  q=1<<s;G=multiplication(s,m);f=Arithmetic(2,[(m>>i)&1 for i in range(s+1)])
  for t in range(q):
   for value in [0]+[1<<i for i in range(4*s)]:
    h=[value>>(s*i)&(q-1)for i in range(4)];g=normalization(h,t,G);assert inverse(g,t,G)==h;assert translate_poly(h+[t,1],t,f)==g+[0,1];out['normalization_basis_controls']+=1
  table=sorted(allowed(s,0,G));records=[]
  for degree in [2,3]:
   masks,basis,rank,_=derive(table,4*s,degree);records.append({'degree':degree,'relations':basis})
  selected,stats,truth=separating_subset(4*s,records,d['truth_storage_cap_bytes']);positive=table_mask(4*s,table)
  if s==5:
   bad={c<<s|a<<(3*s)for a in range(q)for c in range(q)if bad_quadratic(a,c,s,G)};assert truth&~table_mask(4*s,bad)==positive
  else:assert truth==positive
  templates.append({'s':s,'m':m,'relations':selected});out['templates'].append({'s':s,'m':m,**stats,'cubic_zero_count':truth.bit_count(),'allowed_count':len(table),'exact_guarded':True})
 for s,m in d['fields']:
  canonical=next(mod for n,mod in d['canonical_fields']if n==s);mapping=field_isomorphism(s,m,canonical);G=multiplication(s,m);K=multiplication(s,canonical)
  for a in range(1<<s):
   for b in range(1<<s):assert mapping[a^b]==mapping[a]^mapping[b]and mapping[G[a][b]]==K[mapping[a]][mapping[b]];out['isomorphism_pairs']+=1
 specs=[{'p':3,'mod':[0,1]},{'p':3,'mod':[1,0,1]}]+[x for x in json.loads(__import__('pathlib').Path(d['nist_data']).read_text())['large']if x['p']>2]
 for spec in specs:
  f=Arithmetic(spec['p'],spec['mod']);count=0
  for xs in [[0,1,1,2,2],[1]*5,[0,0,0,0,0]]:
   H=roots_product(xs,f);g,shift=depress_quintic(H,f);assert g[4]==0 and g==roots_product([f.sub(x,shift)for x in xs],f);count+=1
  out['odd_controls'].append({'p':spec['p'],'mod':spec['mod'],'cases':count,'scope':'exact algebraic normalization only; no solve/timing claim'})
 try:depress_quintic([0,0,0,0,1,1],Arithmetic(5,[0,1]));raise AssertionError('characteristic5 must branch')
 except ValueError:out['characteristic5_exception_checked']=True
 raw=json.dumps({'fields':templates},separators=(',',':')).encode();(P/'templates.json').open('xb').write(raw);out['templates_sha256']=hashlib.sha256(raw).hexdigest()
 finish(out,None,START,'Full canonical finite template certificates, affine-map basis proofs, every residue field isomorphism and odd/NIST normalization controls.','Freeze templates and measure same76parameter cold formulation; smaller template counts do not certify a speedup.')
except Exception as e:finish(out,repr(e),START,'Trace-zero template/oracle.','Preserve counterexample and register a new variant; no silent repair.')
