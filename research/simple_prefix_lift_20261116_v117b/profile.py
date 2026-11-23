from pathlib import Path
import json,time,random,hashlib
from collections import Counter
from .common import P,finish
from .kernel import high_root,normalize,lift_circuit,evaluate
from research.guarded_local_search_20261116_v101.kernel import matrix,decode
from research.residue_quotient_20261116_v99b.kernel import pmul,apply
from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
START=time.monotonic();out={'actual_controls':0,'sampled_tail_zero':0,'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());cfg=json.loads(Path(d['config']).read_text());record=json.loads(Path(d['cover']).read_text())['records'][0];assert record['k']==1;chart=record['chart'];rows=matrix(cfg['matrix']);C=Circuit(chart['dimension'])
    def raw(theta):
        H=decode(rows,theta);H[0]^=pmul(theta>>73,cfg['modulus']);return H
    base=normalize(raw(chart['particular']),18);origin=raw(0);directions=[]
    for v in chart['columns']:directions.append(normalize([a^b for a,b in zip(raw(v),origin)],18))
    G=[]
    for j,a in enumerate(base):
        width=(5-j)*17+1;bits=[]
        for bit in range(width):
            value=(a>>bit)&1
            for i,v in enumerate(directions):
                if v[j]>>bit&1:value=C.xor(value,2+i)
            bits.append(value)
        G.append(bits)
    root_wires,tail,low=lift_circuit(G,18,C);rng=random.Random(d['seed'])
    for _ in range(64):
        sigma=rng.randrange(1<<chart['dimension']);theta=chart['particular']^apply(chart['columns'],sigma);H=raw(theta);root,residual,series=high_root(H,18);values=C.evaluate(sigma);assert sum(values[w]<<i for i,w in enumerate(root_wires))==root and sum(values[w]<<i for i,w in enumerate(tail))==residual;assert not any(values[w]for w in low);assert (residual==0)==(evaluate(H,root)==0);out['sampled_tail_zero']+=int(residual==0);out['actual_controls']+=1
    outputs=[{'unique_high_root_tail':True,'wires':[tail]}];data={'nodes':C.nodes,'outputs':outputs,'root_wires':root_wires,'low_residual_wires':low};b=json.dumps(data,separators=(',',':')).encode()
    with(P/'graph.json').open('xb')as f:f.write(b)
    out.update({'inputs':C.n,'variables':len(C.nodes)-2,'clauses':len(cnf(C,outputs)),'gate_kinds':dict(Counter(n[0]for n in C.nodes)),'root_bit_count':len(root_wires),'tail_bit_count':len(tail),'graph_sha256':hashlib.sha256(b).hexdigest(),'total_seconds':time.monotonic()-START,'scope':'exact existence/recovery oftheuniquehighpolynomialrootink1;remainingfourroots/sourceUunresolved'})
except Exception as e:error=repr(e)
finish(out,error,START,'Actual61-input deterministic simple-high-root lifting circuit with exact finite-polynomial tail criterion.','Measure guarded integration into the allthree-case cover only if circuit size is bounded;retain all othercases/multiplicities and require full-root-set equivalence.')
