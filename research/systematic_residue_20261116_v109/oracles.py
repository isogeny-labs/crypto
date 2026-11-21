import json,time,random
from .common import P,finish
from .kernel import coordinate_map,old_value,new_value,rewire,actual_map
from research.residue_quotient_20261116_v99b.kernel import basis,apply
from research.local_split_circuit_20261116_v100.kernel import Circuit
START=time.monotonic();out={'toy_cases':0,'toy_assignments':0,'actual_inverse_controls':0,'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['oracle_seed'])
 for n in range(4,9):
  for _ in range(8):
   width=n-2
   while True:
    E=[rng.randrange(1<<width)for _ in range(n)]
    if len(basis(E)[0])==width:break
   mapping=coordinate_map(E,rng.randrange(1<<width),n);old=Circuit(n)
   for k in range(20):
    kind=rng.choice(['and','xor','not']);a=rng.randrange(len(old.nodes));b=rng.randrange(len(old.nodes));old.op(kind,a,None if kind=='not'else b)
   outputs=[{'wires':[[rng.randrange(len(old.nodes))for _ in range(4)]]}];new,rows=rewire(old,outputs,mapping);seen=set()
   for rho in range(1<<n):
    theta=old_value(mapping,rho);assert new_value(mapping,theta)==rho;seen.add(theta);a=old.evaluate(theta);b=new.evaluate(rho);assert [a[w]for field in outputs[0]['wires']for w in field]==[b[w]for field in rows[0]['wires']for w in field];out['toy_assignments']+=1
   assert len(seen)==1<<n;out['toy_cases']+=1
 mapping=actual_map(d)
 for _ in range(256):
  rho=rng.randrange(1<<76);theta=old_value(mapping,rho);assert new_value(mapping,theta)==rho;assert apply(mapping['E'],theta)^mapping['affine']==rho&((1<<72)-1);out['actual_inverse_controls']+=1
 out.update({'rank':72,'kernel_bits':4,'bijection_dimension':76});raw=json.dumps(mapping,separators=(',',':'));(P/'coordinates.json').open('x').write(raw)
 finish(out,None,START,'Independent exhaustive coordinate/circuit equivalence plus actual E/R/K inverse controls, preserving all original q bits.','Compile full fourteen-predicate hard family in systematic residue coordinates; verify seven disjoint scopes before search.')
except Exception as e:finish(out,repr(e),START,'Systematic-coordinate oracle.','Preserve failure and repair only a new immutable variant.')
