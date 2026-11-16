import json,time,hashlib,random,collections
from pathlib import Path
from .common import P,finish
from .kernel import Circuit,BF,split_output,compile_actual,cnf
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate,power,trim
from research.residue_quotient_20261116_v99b.kernel import rem,pmul
START=time.monotonic();out={'tiny_checks':0,'actual_assignments':0,'original_complete':False,'original_exact_root_count':None};error=None
try:
 d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed'])
 for s,m in[[1,2],[2,7]]:
  C=Circuit(5*s);f=BF(s,m,C);H=[[2+j*s+i for i in range(s)]for j in range(5)];output=split_output(H,f);F=Arithmetic(2,[(m>>i)&1 for i in range(s+1)])
  for value in range(1<<(5*s)):
   coeff=[(value>>(j*s))&((1<<s)-1)for j in range(5)]+[1];states=C.evaluate(value);got=[sum(states[w]<<i for i,w in enumerate(v))for v in output];ok,L=predicate(coeff,F);want=power(L,8,coeff,F);assert trim(got)==want;out['tiny_checks']+=1
 raw=Path(d['matrix']).read_bytes();assert hashlib.sha256(raw).hexdigest()==d['matrix_sha256'];rows=[[int(x,16)for x in line.split()]for line in raw.decode().splitlines()[1:]];base,hc=rows[0],rows[1:];C,outputs=compile_actual(base,hc,d['modulus'],d['fields'])
 for _ in range(d['actual_samples']):
  value=rng.randrange(1<<76);H=list(base)
  for j,v in enumerate(hc):
   if value>>j&1:H=[a^b for a,b in zip(H,v)]
  H[0]^=pmul(value>>73,d['modulus']);states=C.evaluate(value)
  for row in outputs:
   s,m=row['s'],row['m'];coeff=[rem(v,m)for v in H];F=Arithmetic(2,[(m>>i)&1 for i in range(s+1)]);ok,L=predicate(coeff,F);want=power(L,8,coeff,F);got=[sum(states[w]<<i for i,w in enumerate(v))for v in row['wires']];assert trim(got)==want
  out['actual_assignments']+=1
 clauses=cnf(C,outputs);text='p cnf '+str(len(C.nodes)-2)+' '+str(len(clauses))+'\n'+''.join(' '.join(map(str,c))+' 0\n'for c in clauses)
 with(P/'formula.cnf').open('x')as f:f.write(text)
 graph=json.dumps({'nodes':C.nodes,'outputs':outputs,'primaries':76},separators=(',',':'))
 with(P/'graph.json').open('x')as f:f.write(graph)
 out.update({'primaries':76,'gates':dict(collections.Counter(n[0]for n in C.nodes)),'cnf_variables':len(C.nodes)-2,'cnf_clauses':len(clauses),'cnf_bytes':len(text.encode()),'cnf_sha256':hashlib.sha256(text.encode()).hexdigest(),'graph_sha256':hashlib.sha256(graph.encode()).hexdigest(),'solver_called':False,'scope':'All14local split predicates on SAME76globalparameterbits;necessaryconstraints only'})
 finish(out,None,START,'CompactmonicquinticlocalnilpotencyBooleancompiler,equivalencecheckedbeforeanySATsearch.','FreezeexactCNF/library/runtimeandboundedall-modelsearch withfullsupportchecks;nooriginalcompletionfromcompileonly.')
except Exception as e:finish(out,repr(e),START,'Localsplittingcircuitprofile.','Preservefailedformulaandfindexactcounterexample;newimmutablevariantrequired for repairs.')
