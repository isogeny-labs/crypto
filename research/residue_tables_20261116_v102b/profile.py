import json,time,random,hashlib,math
from pathlib import Path
from .common import P,finish
from .kernel import multiplication,allowed,diagram,evaluate,literal_split
from research.guarded_local_search_20261116_v101.kernel import matrix,decode
from research.residue_quotient_20261116_v99b.kernel import rem,pmul
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
START=time.monotonic();out={'fields':[],'literal_controls':0,'actual_binding_controls':0,'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());rows=matrix(d['matrix']);rng=random.Random(d['table_seed'])
 for s,m in d['fields']:
  begin=time.monotonic();q=1<<s;G=multiplication(s,m);trace=rem(rows[0][4],m);table=allowed(s,trace,G);assert len(table)==math.comb(q+4,5)//q
  nodes,root=diagram(4*s,table)
  for code in range(1<<(4*s))if s<=2 else [rng.randrange(1<<(4*s))for _ in range(d['samples_per_field'])]+list(sorted(table))[:12]:
   H=[(code>>(s*i))&(q-1)for i in range(4)]+[trace,1]
   assert bool(evaluate(nodes,root,code))==(code in table)==literal_split(H,q,G);out['literal_controls']+=1
  f=Arithmetic(2,[(m>>i)&1 for i in range(s+1)])
  for _ in range(16):
   theta=rng.randrange(1<<76);H=decode(rows,theta);H[0]^=pmul(theta>>73,d['modulus']);H=[rem(x,m)for x in H];code=sum(a<<(s*i)for i,a in enumerate(H[:4]));ok,_=predicate(H,f);assert H[4]==trace and bool(evaluate(nodes,root,code))==ok;out['actual_binding_controls']+=1
  wire=bytearray()
  for code in sorted(table):wire.extend(code.to_bytes((4*s+7)//8,'little'))
  out['fields'].append({'s':s,'m':m,'trace':trace,'allowed_count':len(table),'table_sha256':hashlib.sha256(wire).hexdigest(),'bdd_nodes':len(nodes),'root':root,'seconds':time.monotonic()-begin})
 out['total_bdd_nodes']=sum(r['bdd_nodes']for r in out['fields']);out['previous_and_gates']=17323;out['previous_all_gates']=51089
 finish(out,None,START,'Exact fixed-trace local split tables and shared-parameter bindings; ROBDD sizes include every registered residue.','Freeze a table/BDD solver only if the full formulation is smaller; otherwise use algebraic low-degree truth-table relations and a fill-aware dispatcher. No original completion from table correctness.')
except Exception as e:finish(out,repr(e),START,'Fixed-trace residue table diagnostic.','Preserve failure; change table or representation assumption before retry.')
