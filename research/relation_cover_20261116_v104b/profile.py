import json,time,hashlib,random
from pathlib import Path
from collections import Counter
from .common import P,finish
from .kernel import separating_subset
from research.table_relations_20261116_v103.kernel import derive,features,accepted
from research.singular_quadratic_guard_20261116_v104.kernel import compile_all
from research.residue_tables_20261116_v102b.kernel import multiplication,allowed
from research.relation_zero_locus_20261116_v103b.kernel import table_mask
from research.local_split_circuit_20261116_v100.kernel import cnf
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard
from research.residue_quotient_20261116_v99b.kernel import rem,pmul
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
START=time.monotonic();out={'toy_cases':0,'fields':[],'actual_controls':0,'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());rng=random.Random(1041612)
 for width in range(1,9):
  table=[x for x in range(1<<width)if rng.randrange(4)==0];records=[]
  for degree in [2,3]:
   masks,basis,_,_=derive(table,width,degree);records.append({'degree':degree,'relations':basis})
  selected,stats,truth=separating_subset(width,records,d['truth_storage_cap_bytes']);masks=features(width,3)
  for x in range(1<<width):assert accepted(x,masks,selected)==bool(truth>>x&1)==accepted(x,masks,records[1]['relations'])
  out['toy_cases']+=1
 prior=json.loads(Path(d['relations_raw']).read_text());selected_fields=[];rows=matrix(d['matrix'])
 for row in prior['out']['fields']:
  begin=time.monotonic();s,m=row['s'],row['m'];selected,stats,truth=separating_subset(4*s,row['degrees'],d['truth_storage_cap_bytes']);table=allowed(s,rem(rows[0][4],m),multiplication(s,m));positive=table_mask(4*s,table);assert not(positive&~truth)
  assert (truth&~positive).bit_count()==(496 if s==5 else 0);out['fields'].append({'s':s,'m':m,**stats,'seconds':time.monotonic()-begin,'exact_same_zero_set':True});selected_fields.append({'s':s,'m':m,'degrees':[{'degree':3,'relations':selected}]})
 selected_raw=json.dumps({'out':{'fields':selected_fields},'error':None},separators=(',',':')).encode();(P/'selected.json').open('xb').write(selected_raw);work=dict(d);work['relations_raw']=str(P/'selected.json');C,outputs,st=compile_all(work)
 for _ in range(32):
  theta=rng.randrange(1<<76);values=C.evaluate(theta);H=decode(rows,theta);H[0]^=pmul(theta>>73,d['modulus'])
  for r in outputs[:-1]:
   s,m=r['s'],r['m'];ok,_=predicate([rem(x,m)for x in H],Arithmetic(2,[(m>>i)&1 for i in range(s+1)]));assert (not any(values[w]for field in r['wires']for w in field))==ok
  assert (not any(values[w]for field in outputs[-1]['wires']for w in field))==valid_guard(decode(rows,theta),theta>>73);out['actual_controls']+=1
 graph=json.dumps({'nodes':C.nodes,'outputs':outputs},separators=(',',':')).encode();(P/'graph.json').open('xb').write(graph);out.update({'variables':len(C.nodes)-2,'clauses':len(cnf(C,outputs)),'gate_kinds':dict(Counter(n[0]for n in C.nodes)),'graph_sha256':hashlib.sha256(graph).hexdigest(),'selected_sha256':hashlib.sha256(selected_raw).hexdigest(),'search_executed':False})
 finish(out,None,START,'Exact finite rejection-set cover compresses local cubic constraints, retaining every old accepted tuple and singular guard; actual76parameter bindings checked.','If compiled size improves, freeze changed cold hard search including relation loading, compilation and verification; otherwise change basis/field representation before another timeout.')
except Exception as e:finish(out,repr(e),START,'Separating-subset exact compression and compile profile.','Preserve failure; change representation in a new immutable variant; no identical retry.')
