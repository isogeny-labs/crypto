from pathlib import Path
import json,time,random,hashlib
from collections import Counter
from .common import P,finish
from .kernel import compile_all
from research.guarded_high_root_20261116_v118.kernel import raw_family
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.coprime_prefix_factor_20261116_v119.kernel import lift_k3
from research.simple_prefix_lift_20261116_v117b.kernel import reverse
from research.residue_quotient_20261116_v99b.kernel import apply,rem
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,literal_split,predicate
from research.local_split_circuit_20261116_v100.kernel import cnf
START=time.monotonic();out={'actual_controls':0,'local_factor_checks':0,'factor_valid_controls':0,'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());cfg=json.loads(Path(d['config']).read_text());record=json.loads(Path(d['cover']).read_text())['records'][1];assert record['k']==3;chart=record['chart'];candidate,control,stats=compile_all(cfg,chart);C,oo,A,B=candidate;D,co,Ac,Bc=control;raw=raw_family(matrix(cfg['matrix']),cfg['modulus']);rng=random.Random(d['seed'])
    for _ in range(48):
        x=rng.randrange(1<<C.n);theta=chart['particular']^apply(chart['columns'],x);H=raw(theta);wanted=lift_k3(H,18);v=C.evaluate(x);w=D.evaluate(x);factors=[]
        for wires,degree,label in [(A,2,'A'),(B,3,'B')]:
            values=[sum(v[z]<<i for i,z in enumerate(row))for row in wires];assert values==wanted[label];factors.append([reverse(a,(degree-j)*17)for j,a in enumerate(values)])
        assert [[v[z]for z in row]for row in A+B]==[[w[z]for z in row]for row in Ac+Bc];factor_ok=not any(v[z]for field in oo[0]['wires']for z in field);assert factor_ok==wanted['bounded_factorization'];out['factor_valid_controls']+=int(factor_ok)
        for row in oo[1:]:
            s,mod=row['s'],row['m'];f=Arithmetic(2,[(mod>>j)&1 for j in range(s+1)]);factor=factors[0 if row['factor']=='A'else 1];expected=literal_split([rem(a,mod)for a in factor],f);assert(not any(v[z]for field in row['wires']for z in field))==expected;out['local_factor_checks']+=1
        assert(not any(v[z]for row in oo for field in row['wires']for z in field))==(not any(w[z]for row in co for field in row['wires']for z in field));out['actual_controls']+=1
    metrics={}
    for name,(G,outputs,aa,bb)in [('candidate',candidate),('control',control)]:
        graph={'nodes':G.nodes,'outputs':outputs,'A':aa,'B':bb};b=json.dumps(graph,separators=(',',':')).encode()
        with(P/(name+'_graph.json')).open('xb')as f:f.write(b)
        metrics[name]={'inputs':G.n,'variables':len(G.nodes)-2,'clauses':len(cnf(G,outputs)),'gate_kinds':dict(Counter(n[0]for n in G.nodes)),'graph_sha256':hashlib.sha256(b).hexdigest()}
    out.update({'formulations':metrics,'synthesis':stats,'total_seconds':time.monotonic()-START,'same_conditional_parameter_contract':True,'scope':'k3only;factorization+14residuefieldlocalconditions;notfullprefixsupport/originalsource/U/alloutercases'})
except Exception as e:error=repr(e)
finish(out,error,START,'Combinedfactorizationandbothlocalsupport circuits vs originalquinticpredicatesunderthesameboundedfactorizationcontract.','Onlyfreezechangedconditionalhardsearchifcostbounded;retainotherstrataandfulloriginalrecoveryworkstream;noendtoendgainfromgraphs.')
