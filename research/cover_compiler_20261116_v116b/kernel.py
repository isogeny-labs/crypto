from research.frontier_certificate_20261116_v111b.kernel import solve
from research.residue_quotient_20261116_v99b.kernel import basis,apply
from research.syndrome_frontier_20261116_v111.kernel import reduce_by
from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.parity_chart_20261116_v113d.kernel import optimized_import

def mapping(charts,n):
    assert len(charts)==3 and all(c['feasible']for c in charts)
    masks=list(dict.fromkeys(mask for c in charts for mask,rhs in c['rows']));common=solve([(mask,0)for mask in masks],n)['columns'];extras=[]
    for c in charts:
        B=list(common);piv,_=basis(B);extra=[]
        for v in c['columns']:
            if reduce_by(v,piv):extra.append(v);B.append(v);piv,_=basis(B)
        assert len(B)==c['dimension'];extras.append(extra)
    width=max(map(len,extras));return {'old_n':n,'new_n':len(common)+width+2,'common':common,'extras':extras,'width':width,'particular':[c['particular']for c in charts],'dimensions':[c['dimension']for c in charts]}

def forward(m,x):
    r=len(m['common']);w=m['width'];selector=(x>>(r+w))&3;t=(x>>r)&((1<<w)-1)
    if selector==3 or(t>>len(m['extras'][selector])):return None
    return m['particular'][selector]^apply(m['common'],x&((1<<r)-1))^apply(m['extras'][selector],t)

def compile_map(source,outputs,m,round_cap=256,event_cap=8000000):
    r=len(m['common']);w=m['width'];C=Circuit(m['new_n']);s0=2+r+w;s1=s0+1;generators=list(range(2,C.n+2));columns=list(m['common'])+m['extras'][0]+[0]*(w-len(m['extras'][0]));columns += [m['particular'][0]^m['particular'][1],m['particular'][0]^m['particular'][2]]
    for branch,s in[(1,s0),(2,s1)]:
        for j in range(w):
            a=m['extras'][0][j]if j<len(m['extras'][0])else 0;b=m['extras'][branch][j]if j<len(m['extras'][branch])else 0
            if a^b:generators.append(C.op('and',s,2+r+j));columns.append(a^b)
    forms=[((m['particular'][0]>>i)&1)|sum(((v>>i)&1)<<(j+1)for j,v in enumerate(columns))for i in range(source.n)]
    C,oo,stats=optimized_import(source,outputs,C,forms,generators,round_cap,event_cap)
    # Source and destination node namespaces differ; add destination guards only after import/pruning.
    s0=2+r+w;s1=s0+1;guards=[C.op('and',s0,s1)];selectors=[C.op('and',C.op('not',s0),C.op('not',s1)),s0,s1]
    for branch,selector in enumerate(selectors):
        for j in range(len(m['extras'][branch]),w):guards.append(C.op('and',selector,2+r+j))
    oo.append({'cover_guard':True,'wires':[guards]});stats.update({'shared_direction_dimension':r,'max_extra_directions':w,'primary_inputs':C.n,'generator_functions':len(generators)})
    return C,oo,stats
