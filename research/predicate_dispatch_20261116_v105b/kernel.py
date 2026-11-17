from research.local_split_circuit_20261116_v100.kernel import Circuit,compile_actual
from research.trace_zero_predicate_20261116_v105.kernel import compile_all as normalized
from research.guarded_local_search_20261116_v101.kernel import matrix

def merge(C,source,outputs):
    needed=set();stack=[w for r in outputs for field in r['wires']for w in field]
    while stack:
        i=stack.pop()
        if i in needed:continue
        needed.add(i);node=source.nodes[i]
        if node[0]in ['and','xor','not']:stack.extend(node[1:])
    mapped={i:i for i in range(78)}
    for i in sorted(needed):
        if i<78:continue
        kind,a,*rest=source.nodes[i];mapped[i]=C.op(kind,mapped[a],mapped[rest[0]]if rest else None)
    return [{'s':r['s'],'m':r['m'],'wires':[[mapped[w]for w in field]for field in r['wires']]}for r in outputs]

def clone(C):
    out=Circuit(76);out.nodes=list(C.nodes);out.cache=dict(C.cache);return out

def compile_all(d):
    rows=matrix(d['matrix']);base,hc=rows[0],rows[1:];C=Circuit(76);outputs=[];choices=[]
    for s,m in d['fields']:
        A,ao=compile_actual(base,hc,d['modulus'],[(s,m)]);single=dict(d);single['fields']=[(s,m)];B,bo,_=normalized(single);candidates=[];before=len(C.nodes)
        for mode,local,raw in [('arithmetic',A,ao),('normalized_cubic',B,bo[:1])]:
            trial=clone(C);out=merge(trial,local,raw);cost=sum(3 if n[0]=='and'else 4 if n[0]=='xor'else 2 if n[0]=='not'else 0 for n in trial.nodes[before:])+sum(w!=0 for r in out for field in r['wires']for w in field);candidates.append((cost,mode,trial,out))
        best=min(range(2),key=lambda i:(candidates[i][0],i));cost,mode,C,out=candidates[best];outputs.extend(out);choices.append({'s':s,'m':m,'mode':mode,'added_variables':len(C.nodes)-before,'incremental_clause_cost':cost,'alternative_clause_cost':candidates[1-best][0]})
    def flag(k,b):
        w=base[k]>>b&1
        for j,h in enumerate(hc):
            if h[k]>>b&1:w=C.xor(w,2+j)
        return w
    u,v,w=flag(3,34),flag(2,51),flag(1,68);outputs.append({'guard':True,'wires':[[C.xor(u,v)],[C.op('and',u,w)],[C.xor(77,w)],[C.op('and',76,C.op('not',w))]]})
    return C,outputs,choices
