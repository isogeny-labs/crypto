from pathlib import Path
import json,time,random,hashlib
from collections import Counter
from .common import P,finish
from research.unified_chart_20261116_v112b.kernel import union_map,forward,compile_map,literal
from research.frontier_certificate_20261116_v111b.kernel import guarded_charts
from research.chart_compiler_20261116_v112.kernel import source_graph,compile_all as separate_compile
from research.residue_quotient_20261116_v99b.kernel import basis,preimage
from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
from research.residue_quotient_20261116_v99b.kernel import rem,pmul
START=time.monotonic();out={'toy_cases':0,'toy_original_assignments':0,'toy_compiled_assignments':0,'actual_controls':0,'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed'])
    for case in range(64):
        n=5+case%4;rows=[]
        if n>5 and case%3:rows=[(1<<5,rng.randrange(2))]
        charts=guarded_charts(rows,n,(1,0),(2,0),(4,0),(8,0),(16,0));mapping=union_map(charts,n);source=Circuit(n)
        for _ in range(24):
            op=rng.choice(['and','xor','not']);source.op(op,rng.randrange(len(source.nodes)),None if op=='not'else rng.randrange(len(source.nodes)))
        outputs=[{'wires':[[rng.randrange(len(source.nodes))for _ in range(2)]]}];C,oo=compile_map(source,outputs,mapping);got=set();image=set()
        for sigma in range(1<<mapping['new_n']):
            out['toy_compiled_assignments']+=1;theta=forward(mapping,sigma);assert theta not in image;image.add(theta);a=literal(source,theta);b=C.evaluate(sigma)
            assert [a[w]for w in outputs[0]['wires'][0]]==[b[w]for w in oo[0]['wires'][0]]
            if not any(b[w]for w in oo[0]['wires'][0]):got.add(theta)
        want=set();guarded=set()
        for theta in range(1<<n):
            out['toy_original_assignments']+=1
            if any((mask&theta).bit_count()%2!=rhs for mask,rhs in rows):continue
            u,v,w,q1,q2=[(theta>>i)&1 for i in range(5)]
            if (u,v,w)not in[(0,0,0),(1,1,0),(0,0,1)]or q2!=w or(not w and q1):continue
            guarded.add(theta);a=literal(source,theta)
            if not any(a[w]for w in outputs[0]['wires'][0]):want.add(theta)
        assert image==guarded and got==want;out['toy_cases']+=1
    cfg=json.loads(Path(d['config']).read_text());charts=json.loads(Path(cfg['charts']).read_text())['charts'];mapping=union_map(charts,76);assert len(mapping['common'])==66 and mapping['new_n']==68
    source,outputs=source_graph(cfg);separate=separate_compile(cfg);separate_metrics=[]
    for i,((SC,SO),chart)in enumerate(separate):
        separate_metrics.append({'chart':i,'inputs':SC.n,'variables':len(SC.nodes)-2,'clauses':len(cnf(SC,SO))})
        raw=json.dumps({'nodes':SC.nodes,'outputs':SO},separators=(',',':')).encode()
        with(P/('separate-'+str(i)+'.json')).open('xb')as f:f.write(raw)
        separate_metrics[-1]['graph_sha256']=hashlib.sha256(raw).hexdigest()
    C,oo=compile_map(source,outputs,mapping);rows=matrix(cfg['matrix'])
    for control in range(64):
        sigma=rng.randrange(1<<67)|((control%2)<<67);theta=forward(mapping,sigma);a=literal(source,theta);b=C.evaluate(sigma)
        (SC,SO),chart=separate[control%2];cp,_=basis(chart['columns']);local=preimage(theta^chart['particular'],cp);sv=SC.evaluate(local)
        assert [[a[w]for field in r['wires']for w in field]for r in outputs]==[[sv[w]for field in r['wires']for w in field]for r in SO]
        assert [[a[w]for field in r['wires']for w in field]for r in outputs]==[[b[w]for field in r['wires']for w in field]for r in oo]
        H=decode(rows,theta);assert valid_guard(H,theta>>73);H[0]^=pmul(theta>>73,cfg['modulus'])
        for r in oo[:-1]:
            s,m=r['s'],r['m'];ok,_=predicate([rem(x,m)for x in H],Arithmetic(2,[(m>>j)&1 for j in range(s+1)]));assert (not any(b[w]for field in r['wires']for w in field))==ok
        assert not any(b[w]for field in oo[-1]['wires']for w in field);out['actual_controls']+=1
    raw=json.dumps({'nodes':C.nodes,'outputs':oo},separators=(',',':')).encode()
    with(P/'graph.json').open('xb')as f:f.write(raw)
    with(P/'mapping.json').open('x')as f:json.dump(mapping,f,separators=(',',':'))
    variables=len(C.nodes)-2;clauses=len(cnf(C,oo));out.update({'separate_charts':separate_metrics,'separate_total_variables':sum(r['variables']for r in separate_metrics),'separate_total_clauses':sum(r['clauses']for r in separate_metrics),'inputs':C.n,'common_dimension':66,'selector_products_before_import':1,'variables':variables,'clauses':clauses,'gate_kinds':dict(Counter(n[0]for n in C.nodes)),'graph_sha256':hashlib.sha256(raw).hexdigest(),'native_search_promotion':variables<48887 and clauses<179895,'total_seconds':time.monotonic()-START})
except Exception as e:error=repr(e)
finish(out,error,START,'Exact68-input unified two-chart circuit and exhaustive independent union-root oracles.','If source formulation improves, freeze shared45second changed hard search with original theta/q back-map; otherwise factor emitted parity networks before any native retry.')
