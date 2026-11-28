from pathlib import Path
import json,time,random,hashlib
from collections import Counter
from .common import P,finish
from .circuit import compile_all
from research.guarded_high_root_20261116_v118.kernel import raw_family
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.coprime_prefix_factor_20261116_v119.kernel import lift_k3
from research.triangular_prefix_quadratic_20261116_v121.kernel import solve
from research.simple_prefix_lift_20261116_v117b.kernel import reverse
from research.residue_quotient_20261116_v99b.kernel import apply
from research.local_split_circuit_20261116_v100.kernel import cnf
START=time.monotonic();out={'actual_controls':0,'prefix_valid_controls':0,'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());cfg=json.loads(Path(d['config']).read_text());chart=json.loads(Path(d['cover']).read_text())['records'][1]['chart'];candidate,control=compile_all(cfg,chart);C,oo,A,B,root=candidate;D,co,Ac,Bc,rc=control;raw=raw_family(matrix(cfg['matrix']),cfg['modulus']);rng=random.Random(d['seed'])
    for _ in range(48):
        x=rng.randrange(1<<C.n);theta=chart['particular']^apply(chart['columns'],x);factor=lift_k3(raw(theta),18);v=C.evaluate(x);w=D.evaluate(x);gotA=[sum(v[z]<<i for i,z in enumerate(row))for row in A];gotB=[sum(v[z]<<i for i,z in enumerate(row))for row in B];assert gotA==factor['A']and gotB==factor['B'];aa=reverse(gotA[1],17);bb=reverse(gotA[0],34);wanted=solve(aa,bb,17)['roots']if aa<(1<<17)and bb<(1<<33)else[];prefix_ok=not any(v[z]for field in oo[-1]['wires']for z in field);assert prefix_ok==bool(wanted)
        if wanted:assert sum(v[z]<<i for i,z in enumerate(root))==wanted[0][0]
        assert(not any(v[z]for row in oo for field in row['wires']for z in field))==(not any(w[z]for row in co for field in row['wires']for z in field));assert [v[z]for z in root]==[w[z]for z in rc];out['prefix_valid_controls']+=int(prefix_ok);out['actual_controls']+=1
    metrics={}
    for name,(G,outputs,aa,bb,rr)in [('candidate',candidate),('control',control)]:
        graph={'nodes':G.nodes,'outputs':outputs,'A':aa,'B':bb,'root':rr};b=json.dumps(graph,separators=(',',':')).encode()
        with(P/(name+'_graph.json')).open('xb')as f:f.write(b)
        metrics[name]={'inputs':G.n,'variables':len(G.nodes)-2,'clauses':len(cnf(G,outputs)),'gate_kinds':dict(Counter(n[0]for n in G.nodes)),'graph_sha256':hashlib.sha256(b).hexdigest()}
    out.update({'formulations':metrics,'total_seconds':time.monotonic()-START,'same_strong_parameter_contract':True,'scope':'k3only;exactquadraticprefix+boundedfactorization+cubiclocals;fullcubic/source/U/othercasesremain'})
except Exception as e:error=repr(e)
finish(out,error,START,'Fullconditionalfactorformula with EXactquadraticprefixsupport and onlycubicresiduetests,comparedtoequallystrongredundantlocalscontrol.','Onlyfreezechangedharddiagnosticafterformulationbounded;exactquadraticprefixstillnotfullcubic/chain/allfieldcoverage.')
