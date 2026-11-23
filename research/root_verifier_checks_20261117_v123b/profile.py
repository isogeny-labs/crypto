from pathlib import Path
import json,time,random,hashlib
from collections import Counter
from .common import P,finish
from .kernel import compile_all
from research.guarded_high_root_20261116_v118.kernel import raw_family
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.residue_quotient_20261116_v99b.kernel import apply
from research.prefix_translation_20261116_v115.kernel import product
from research.local_split_circuit_20261116_v100.kernel import cnf
START=time.monotonic();out={'actual_controls':0,'accepted_controls':0,'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());cfg=json.loads(Path(d['config']).read_text());chart=json.loads(Path(d['cover']).read_text())['records'][1]['chart'];candidate,control=compile_all(cfg,chart);C,oo,roots=candidate;D,co,oldroots=control;raw=raw_family(matrix(cfg['matrix']),cfg['modulus']);rng=random.Random(d['seed'])
    for _ in range(48):
        x=rng.randrange(1<<C.n);theta=chart['particular']^apply(chart['columns'],x);H=raw(theta);v=C.evaluate(x);w=D.evaluate(x);rr=sorted(sum(v[z]<<i for i,z in enumerate(row))for row in roots);oldrr=sorted(sum(w[z]<<i for i,z in enumerate(row))for row in oldroots);assert rr==oldrr;got=not any(v[z]for row in oo for field in row['wires']for z in field);old=not any(w[z]for row in co for field in row['wires']for z in field);assert got==old==(product(rr)==H);assert all(r<(1<<18)for r in rr)and sum(bool(r>>17)for r in rr)==3;out['actual_controls']+=1;out['accepted_controls']+=int(got)
    metrics={}
    for name,(G,outputs,rr)in [('candidate',candidate),('control',control)]:
        b=json.dumps({'nodes':G.nodes,'outputs':outputs,'roots':rr},separators=(',',':')).encode()
        with(P/(name+'_graph.json')).open('xb')as f:f.write(b)
        metrics[name]={'inputs':G.n,'variables':len(G.nodes)-2,'clauses':len(cnf(G,outputs)),'gate_kinds':dict(Counter(n[0]for n in G.nodes)),'graph_sha256':hashlib.sha256(b).hexdigest()}
    out.update({'formulations':metrics,'control_isolated':True,'same_exact_conditional_contract':True,'total_seconds':time.monotonic()-START,'scope':'exactfullprefixsupport/recoveryforoneconditionalnormfamily;notoriginalchain/outercases'})
except Exception as e:error=repr(e)
finish(out,error,START,'Isolatedrawcoefficient-product verification vs fullintermediatepredicate control,withsamefive-rootrecoverycontract.','Ifcostimproves,freezechangedhardsearchwithfullrawproductreadback;otherwisechangereconstructionnetworkbeforeanotherSATtimeout.')
