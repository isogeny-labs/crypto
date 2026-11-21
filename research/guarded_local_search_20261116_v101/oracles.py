import json,time,hashlib,random,sys
from .common import P,finish
from .kernel import matrix,decode,valid_guard,guarded,source_product,divrem,prefix_remainder
from research.local_split_circuit_20261116_v100.kernel import cnf
START=time.monotonic();out={'source_controls':0,'binding_controls':0,'prefix_controls':0,'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());rows=matrix(d['matrix']);assert rows[0][4]>>17==1 and all(v[4]==0 for v in rows[1:]);rng=random.Random(d['seed']);C,outputs,delta=guarded(d['graph'],rows)
 for i in range(96):
  k=[1,3,5][i%3];xs=[(1<<17)|rng.randrange(1<<17)for _ in range(k)]+[rng.randrange(1<<17)for _ in range(5-k)]
  if i%8==0 and k==5:xs=[xs[0]]*5
  if i%8==1 and k<5:xs[-1]=0
  H=source_product(xs);q,h0=divrem(H[0],d['modulus']);H[0]=h0;assert q<8 and valid_guard(H,q);out['source_controls']+=1
 for _ in range(256):
  t=rng.randrange(1<<76);v=C.evaluate(t);assert all(v[w]==0 for field in outputs[-1]['wires']for w in field)==valid_guard(decode(rows,t),t>>73);out['binding_controls']+=1
 for xs in[[0,1,1,2,3],[1]*5]:
  H=source_product(xs);assert not prefix_remainder(H,83,d['field_modulus'],18);out['prefix_controls']+=1
 # A factor X^2+X+1 is irreducible over GF(2^83), sinceTr(1)=1.
 assert prefix_remainder([0,0,0,1,1,1],83,d['field_modulus'],18);out['prefix_controls']+=1
 text='p cnf '+str(len(C.nodes)-2)+' '+str(len(cnf(C,outputs)))+'\n'+''.join(' '.join(map(str,c))+' 0\n'for c in cnf(C,outputs))
 with(P/'guarded.cnf').open('x')as f:f.write(text)
 with(P/'guard_delta.json').open('x')as f:json.dump(delta,f,separators=(',',':'))
 out.update({'variables':len(C.nodes)-2,'clauses':len(cnf(C,outputs)),'cnf_sha256':hashlib.sha256(text.encode()).hexdigest(),'base_graph_sha256':hashlib.sha256(__import__('pathlib').Path(d['graph']).read_bytes()).hexdigest()})
 finish(out,None,START,'Independent polynomial-leading/infinity necessity and exact76-bitguardbinding;prefixsupport controls.','Runonlyfrozenboundednecessarysearch;unknown/candidatewithoutoriginalrecovery remainsunresolved.')
except Exception as e:finish(out,repr(e),START,'Guard/support oracle.','Preservecounterexample;repairnewimmutablevariant only.')
