from pathlib import Path
import json,time,itertools,hashlib,random
from collections import Counter
from .common import P,finish
from .circuit import support_circuit,compile_all
from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
from research.exact_cubic_prefix_20261117_v122.kernel import support,translate
from research.prefix_translation_20261116_v115.kernel import product
from research.guarded_high_root_20261116_v118.kernel import raw_family
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.coprime_prefix_factor_20261116_v119.kernel import lift_k3
from research.simple_prefix_lift_20261116_v117b.kernel import reverse
from research.residue_quotient_20261116_v99b.kernel import apply
START=time.monotonic();out={'small_boolean_assignments':0,'small_supported':0,'actual_controls':0,'actual_supported_controls':0,'original_complete':False};error=None
try:
    data=json.loads((P/'DATA.json').read_text());rng=random.Random(data['seed'])
    for m in [1,2,3]:
        C=Circuit(6*m-3);H=[];offset=2
        for j in range(3):
            width=(3-j)*(m-1)+1;H.append(list(range(offset,offset+width)));offset+=width
        H.append([1]);checks,roots=support_circuit(H,m,C);catalog={tuple(product(xs)):list(xs)for xs in itertools.combinations_with_replacement(range(1<<m),3)}
        for x in range(1<<C.n):
            values=C.evaluate(x);rawH=[sum(values[w]<<i for i,w in enumerate(v))for v in H];wanted=catalog.get(tuple(rawH));accepted=not any(values[w]for w in checks);assert accepted==(wanted is not None)
            if accepted:assert sorted(sum(values[w]<<j for j,w in enumerate(row))for row in roots)==wanted;out['small_supported']+=1
            out['small_boolean_assignments']+=1
    cfg=json.loads(Path(data['config']).read_text());chart=json.loads(Path(data['cover']).read_text())['records'][1]['chart'];candidate,control=compile_all(cfg,chart);C,oo,A,B,roots=candidate;D,co,Ac,Bc,rc=control;raw=raw_family(matrix(cfg['matrix']),cfg['modulus'])
    for _ in range(32):
        x=rng.randrange(1<<C.n);theta=chart['particular']^apply(chart['columns'],x);factors=lift_k3(raw(theta),18);values=C.evaluate(x);other=D.evaluate(x);gotA=[sum(values[w]<<i for i,w in enumerate(v))for v in A];gotB=[sum(values[w]<<i for i,w in enumerate(v))for v in B];assert gotA==factors['A']and gotB==factors['B'];rawB=[reverse(a,(3-j)*17)for j,a in enumerate(gotB)];wanted=support(translate(rawB,1<<17),17);cubic_ok=not any(values[w]for field in oo[2]['wires']for w in field);assert cubic_ok==wanted['supported'];accepted=not any(values[w]for row in oo for field in row['wires']for w in field);assert accepted==(not any(other[w]for row in co for field in row['wires']for w in field));assert [[values[w]for w in row]for row in roots]==[[other[w]for w in row]for row in rc]
        if accepted:
            rr=sorted(sum(values[w]<<j for j,w in enumerate(row))for row in roots);assert product(rr)==raw(theta);out['actual_supported_controls']+=1
        out['actual_controls']+=1
    metrics={}
    for name,(G,outputs,aa,bb,rr)in [('candidate',candidate),('control',control)]:
        rawgraph={'nodes':G.nodes,'outputs':outputs,'A':aa,'B':bb,'roots':rr};b=json.dumps(rawgraph,separators=(',',':')).encode()
        with(P/(name+'_graph.json')).open('xb')as f:f.write(b)
        metrics[name]={'inputs':G.n,'variables':len(G.nodes)-2,'clauses':len(cnf(G,outputs)),'gate_kinds':dict(Counter(n[0]for n in G.nodes)),'graph_sha256':hashlib.sha256(b).hexdigest()}
    out.update({'formulations':metrics,'total_seconds':time.monotonic()-START,'scope':'exactALLfiveprefixsupport/recoveryforconditionalk3rawnormfamily;notoriginalchain/otherstrata/outerfamilies'})
except Exception as e:error=repr(e)
finish(out,error,START,'Sharedstateexactcubicprefix circuit and completeconditionalfive-prefixsupport/recoverywithoutGF83inversion.','Ifcostgrowsorcaps,changebound/guardstatewithoutidenticalretry;exactconditionalnormsupportdoesnotcompleteoriginalallrootgoal.')
