from collections import Counter
from itertools import combinations
from research.chart_compiler_20261116_v112.kernel import import_forms as original_import
from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.systematic_residue_20261116_v109.kernel import prune

def affine_scan(source,inputs,outputs):
    forms=[None]*len(source.nodes);forms[:2]=[0,1];forms[2:source.n+2]=inputs;targets=set()
    for i,node in enumerate(source.nodes[source.n+2:],source.n+2):
        kind,a,*rest=node;fa=forms[a];fb=forms[rest[0]]if rest else None
        if kind=='xor'and fa is not None and fb is not None:forms[i]=fa^fb;continue
        if kind=='not'and fa is not None:forms[i]=fa^1;continue
        if kind=='and'and fa is not None and fb is not None:
            if fa==0 or fb==0 or fa^fb==1:forms[i]=0;continue
            if fa==1 or fa==fb:forms[i]=fb;continue
            if fb==1:forms[i]=fa;continue
        if fa is not None:targets.add(fa)
        if rest and fb is not None:targets.add(fb)
    for r in outputs:
        for field in r['wires']:
            for w in field:
                if forms[w]is not None:targets.add(forms[w])
    return targets

def synthesize(C,generators,targets,round_cap=64,event_cap=8000000):
    expressions={f:{j for j in range(len(generators))if f>>(j+1)&1}for f in sorted(targets)}
    wires=list(generators);events=0;rounds=0;reason='no_shared_pair'
    for _ in range(round_cap):
        needed=sum(len(v)*(len(v)-1)//2 for v in expressions.values())
        if events+needed>event_cap:reason='pair_event_cap';break
        events+=needed;counts=Counter(pair for v in expressions.values()for pair in combinations(sorted(v),2))
        if not counts:break
        pair=min(counts,key=lambda p:(-counts[p],p))
        if counts[pair]<=1:break
        index=len(wires);wires.append(C.xor(wires[pair[0]],wires[pair[1]]));rounds+=1
        for v in expressions.values():
            if pair[0]in v and pair[1]in v:v.difference_update(pair);v.add(index)
    else:reason='round_cap'
    return expressions,wires,{'synthesis_rounds':rounds,'pair_events':events,'stop_reason':reason,'target_forms':len(targets)}

def optimized_import(source,outputs,C,inputs,generators,round_cap=64,event_cap=8000000):
    targets=affine_scan(source,inputs,outputs);expr,gates,stats=synthesize(C,generators,targets,round_cap,event_cap)
    forms=[None]*len(source.nodes);forms[:2]=[0,1];forms[2:source.n+2]=inputs
    cache={0:0,1:1,**{1<<(j+1):w for j,w in enumerate(generators)}};known={v:f for f,v in cache.items()};wires={0:0,1:1};fallback=0
    def emit(form):
        nonlocal fallback
        if form not in cache:
            value=form&1
            if form in expr:
                for j in sorted(expr[form]):value=C.xor(value,gates[j])
            else:
                fallback+=1
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
    oo=[{**r,'wires':[[wire(w)for w in field]for field in r['wires']]}for r in outputs];C,oo=prune(C,oo);stats['fallback_forms']=fallback
    return C,oo,stats

def compile_map(source,outputs,mapping,round_cap=64,event_cap=8000000):
    k=len(mapping['common']);C=Circuit(k+2);product=C.op('and',k+2,k+3);generators=list(range(2,C.n+2))+[product];a,b=mapping['extra'];p,q=mapping['particular'];inputs=[]
    for i in range(source.n):
        form=((p>>i)&1)|sum(((v>>i)&1)<<(j+1)for j,v in enumerate(mapping['common']))
        form|=((a>>i)&1)<<(k+1);form|=(((p^q)>>i)&1)<<(k+2);form|=(((a^b)>>i)&1)<<(k+3);inputs.append(form)
    return optimized_import(source,outputs,C,inputs,generators,round_cap,event_cap)
