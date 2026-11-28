from pathlib import Path
import json,time,random,hashlib
from collections import Counter
from .common import P,finish
from .circuit import lift,normalized_affine
from research.coprime_prefix_factor_20261116_v119.kernel import lift_k3
from research.guarded_high_root_20261116_v118.kernel import raw_family
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.residue_quotient_20261116_v99b.kernel import apply
from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
START=time.monotonic();out={'small_boolean_assignments':0,'actual_controls':0,'original_complete':False};error=None
try:
    data=json.loads((P/'DATA.json').read_text());rng=random.Random(data['seed'])
    for d in [2,3]:
        n=5;C=Circuit(n);F=[]
        for j in range(5):F.append([int(j>=2)]+[C.xor(rng.randrange(2),2+rng.randrange(n))for _ in range((5-j)*(d-1))])
        F.append([1]);A,B,checks=lift(F,d,C)
        for x in range(1<<n):
            values=C.evaluate(x);Fn=[sum(values[w]<<i for i,w in enumerate(v))for v in F];from research.simple_prefix_lift_20261116_v117b.kernel import reverse
            H=[reverse(a,(5-j)*(d-1))for j,a in enumerate(Fn)];want=lift_k3(H,d);gotA=[sum(values[w]<<i for i,w in enumerate(v))for v in A];gotB=[sum(values[w]<<i for i,w in enumerate(v))for v in B];assert gotA==want['A']and gotB==want['B'];assert(not any(values[w]for w in checks))==want['bounded_factorization'];out['small_boolean_assignments']+=1
    cfg=json.loads(Path(data['config']).read_text());record=json.loads(Path(data['cover']).read_text())['records'][1];assert record['k']==3;chart=record['chart'];C=Circuit(chart['dimension']);raw=raw_family(matrix(cfg['matrix']),cfg['modulus']);F=normalized_affine(raw,chart['particular'],chart['columns'],18,C);A,B,checks=lift(F,18,C)
    for _ in range(48):
        x=rng.randrange(1<<C.n);theta=chart['particular']^apply(chart['columns'],x);want=lift_k3(raw(theta),18);values=C.evaluate(x);gotA=[sum(values[w]<<i for i,w in enumerate(v))for v in A];gotB=[sum(values[w]<<i for i,w in enumerate(v))for v in B];assert gotA==want['A']and gotB==want['B'];assert(not any(values[w]for w in checks))==want['bounded_factorization'];out['actual_controls']+=1
    outputs=[{'bounded_coprime_factorization':True,'wires':[checks]}];graph={'nodes':C.nodes,'outputs':outputs,'A':A,'B':B};b=json.dumps(graph,separators=(',',':')).encode()
    with(P/'graph.json').open('xb')as f:f.write(b)
    out.update({'inputs':C.n,'variables':len(C.nodes)-2,'clauses':len(cnf(C,outputs)),'gate_kinds':dict(Counter(n[0]for n in C.nodes)),'constraint_wires':len(checks),'graph_sha256':hashlib.sha256(b).hexdigest(),'total_seconds':time.monotonic()-START,'scope':'conditional k3factorizationonly;nofactorsupport/search/originalrootcompletion'})
except Exception as e:error=repr(e)
finish(out,error,START,'Actual63-input deterministic coprime2+3factorization circuit, with full bounded-product contract.','Measureseparatefactorlocal-supportcircuitsbeforeanyreplacementorhardsearch;factorizationcounterexamplemuststayrejectedbyfullsupport.')
