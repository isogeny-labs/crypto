from research.frontier_certificate_20261116_v111b.kernel import solve
from research.residue_quotient_20261116_v99b.kernel import basis,apply
from research.syndrome_frontier_20261116_v111.kernel import reduce_by
from research.chart_compiler_20261116_v112.kernel import import_forms
from research.local_split_circuit_20261116_v100.kernel import Circuit

def union_map(charts,n):
    assert len(charts)==2 and all(c['feasible']for c in charts)
    dimension=charts[0]['dimension'];assert charts[1]['dimension']==dimension
    masks=list(dict.fromkeys(mask for c in charts for mask,rhs in c['rows']))
    common=solve([(mask,0)for mask in masks],n)['columns']
    assert len(common)==dimension-1
    piv,_=basis(common);extra=[]
    for c in charts:
        extra.append(next(v for v in c['columns']if reduce_by(v,piv)))
        assert len(basis(common+[extra[-1]])[0])==dimension
    return {'old_n':n,'new_n':len(common)+2,'common':common,'extra':extra,
            'particular':[c['particular']for c in charts]}

def forward(mapping,sigma):
    k=len(mapping['common']);t=(sigma>>k)&1;s=(sigma>>(k+1))&1
    return mapping['particular'][s]^apply(mapping['common'],sigma&((1<<k)-1))^(mapping['extra'][s]if t else 0)

def compile_map(source,outputs,mapping):
    k=len(mapping['common']);C=Circuit(k+2);t=2+k;s=3+k;product=C.op('and',t,s)
    generators=list(range(2,C.n+2))+[product]
    a,b=mapping['extra'];p,q=mapping['particular'];forms=[]
    for i in range(source.n):
        form=((p>>i)&1)|sum(((v>>i)&1)<<(j+1)for j,v in enumerate(mapping['common']))
        form|=((a>>i)&1)<<(k+1)
        form|=(((p^q)>>i)&1)<<(k+2)
        form|=(((a^b)>>i)&1)<<(k+3)
        forms.append(form)
    return import_forms(source,outputs,C,forms,generators)

def literal(C,x):
    a=[0,1]+[(x>>i)&1 for i in range(C.n)]
    for kind,u,*tail in C.nodes[C.n+2:]:a.append(a[u]&a[tail[0]]if kind=='and'else a[u]^a[tail[0]]if kind=='xor'else 1-a[u])
    return a
