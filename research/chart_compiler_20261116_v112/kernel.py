import json,hashlib
from pathlib import Path
from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.systematic_residue_20261116_v109.kernel import prune
from research.residue_quotient_20261116_v99b.kernel import apply

def import_forms(source,outputs,C,input_forms,generators):
    assert len(input_forms)==source.n
    forms=[None]*len(source.nodes);forms[0:2]=[0,1]
    forms[2:source.n+2]=input_forms
    cache={0:0,1:1,**{1<<(j+1):w for j,w in enumerate(generators)}}
    known={v:f for f,v in cache.items()};wires={0:0,1:1}
    def emit(form):
        if form not in cache:
            value=form&1
            for j,w in enumerate(generators):
                if form>>(j+1)&1:value=C.xor(value,w)
            cache[form]=value;known[value]=form
        return cache[form]
    def wire(i):return emit(forms[i])if forms[i]is not None else wires[i]
    for i,node in enumerate(source.nodes[source.n+2:],source.n+2):
        kind,a,*rest=node;fa=forms[a];fb=forms[rest[0]]if rest else None
        if kind=='xor'and fa is not None and fb is not None:forms[i]=fa^fb;continue
        if kind=='not'and fa is not None:forms[i]=fa^1;continue
        if kind=='and'and fa is not None and fb is not None:
            if fa==0 or fb==0 or fa^fb==1:forms[i]=0;continue
            if fa==1 or fa==fb:forms[i]=fb;continue
            if fb==1:forms[i]=fa;continue
        w=C.op(kind,wire(a),wire(rest[0])if rest else None);wires[i]=w
        if w in known:forms[i]=known[w]
    result=[{**r,'wires':[[wire(w)for w in field]for field in r['wires']]}for r in outputs]
    return prune(C,result)

def affine_import(source,outputs,chart):
    C=Circuit(len(chart['columns']))
    forms=[((chart['particular']>>i)&1)|sum(((v>>i)&1)<<(j+1)for j,v in enumerate(chart['columns']))for i in range(source.n)]
    return import_forms(source,outputs,C,forms,list(range(2,C.n+2)))

def source_graph(d):
    b=Path(d['source_graph']).read_bytes();assert hashlib.sha256(b).hexdigest()==d['source_graph_sha256'];g=json.loads(b);C=Circuit(76);C.nodes=[tuple(n)for n in g['nodes']];return C,g['outputs']

def forward(chart,sigma):return chart['particular']^apply(chart['columns'],sigma)

def compile_all(d):
    old,outputs=source_graph(d);charts=json.loads(Path(d['charts']).read_text())['charts'];assert len(charts)==2 and all(c['dimension']==67 for c in charts)
    return [(affine_import(old,outputs,c),c)for c in charts]
