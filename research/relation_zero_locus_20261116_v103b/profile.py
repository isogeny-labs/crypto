import json,time,random,hashlib
from pathlib import Path
from .common import P,finish
from .kernel import variable_truth,zero_locus,table_mask
from research.table_relations_20261116_v103.kernel import features,accepted,derive
from research.residue_tables_20261116_v102b.kernel import allowed,multiplication,literal_split
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.residue_quotient_20261116_v99b.kernel import rem
START=time.monotonic();out={'toy_cases':0,'fields':[],'original_complete':False,'original_exact_root_count':None};error=None
try:
 d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed']);prior=json.loads(Path(d['relations_raw']).read_text());assert prior['error']is None;rows=matrix(d['matrix'])
 for width in range(1,9):
  table=[v for v in range(1<<width)if rng.randrange(4)==0];masks,rels,rank,_=derive(table,width,2);truth,_=zero_locus(width,masks,rels,d['truth_storage_cap_bytes'])
  assert all(((truth>>v)&1)==accepted(v,masks,rels)for v in range(1<<width));assert table_mask(width,table)==sum(1<<v for v in table)
  for j in range(width):assert all((variable_truth(width,j)>>v&1)==(v>>j&1)for v in range(1<<width))
  out['toy_cases']+=1
 for row in prior['out']['fields']:
  begin=time.monotonic();s,m=row['s'],row['m'];G=multiplication(s,m);trace=rem(rows[0][4],m);table=allowed(s,trace,G);degree=next(r for r in row['degrees']if r['degree']==d['degree']);masks=features(4*s,d['degree']);assert len(masks)==degree['features'];truth,storage=zero_locus(4*s,masks,degree['relations'],d['truth_storage_cap_bytes']);positive=table_mask(4*s,table);assert positive&~truth==0
  extra=truth&~positive;examples=[];left=extra
  for _ in range(8):
   if not left:break
   bit=left&-left;code=bit.bit_length()-1;H=[(code>>(s*i))&((1<<s)-1)for i in range(4)]+[trace,1];assert not literal_split(H,1<<s,G);examples.append({'code':code,'H':H});left^=bit
  for code in [rng.randrange(1<<(4*s))for _ in range(32)]+list(sorted(table))[:8]:assert bool(truth>>code&1)==accepted(code,masks,degree['relations'])
  out['fields'].append({'s':s,'m':m,'allowed_count':len(table),'cut_zero_count':truth.bit_count(),'extraneous_count':extra.bit_count(),'exact_local_equivalence':truth==positive,'states_exhausted':1<<(4*s),'estimated_truth_bytes':storage,'truth_sha256':hashlib.sha256(truth.to_bytes(((1<<(4*s))+7)//8,'little')).hexdigest(),'extraneous_examples':examples,'seconds':time.monotonic()-begin})
 finish(out,None,START,'Exhaustive local cubic-zero-set versus exact split-table equivalence over every fourteen residue coefficient cube.','If exact, compile cubic shared-parameter predicates and measure total search cost; if extra tuples survive, retain exact predicates and derive multiplicity/factorization strata for those counterexamples.')
except Exception as e:finish(out,repr(e),START,'Bit-parallel exact relation zero-locus diagnostic.','Preserve failure and register changed assumptions; never infer equivalence from samples.')
