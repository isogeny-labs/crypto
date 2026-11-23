from pathlib import Path
import json,time,random,hashlib
from collections import Counter
from .common import P,finish
from .kernel import mapping,forward,compile_map
from research.frontier_certificate_20261116_v111b.kernel import solve
from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
from research.chart_compiler_20261116_v112.kernel import source_graph
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard
from research.leading_cover_20261116_v116.kernel import limits,fits
from research.prefix_translation_20261116_v115.kernel import translate
from research.residue_quotient_20261116_v99b.kernel import apply,pmul,rem
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
START=time.monotonic();out={'toy_cases':0,'toy_original_assignments':0,'toy_encoded_assignments':0,'actual_controls':0,'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed'])
    for case in range(48):
        n=5+case%4;rows=[[(1,0),(2,0),(4,0)],[(1,1),(2,0)],[(1,0),(2,1),(4,rng.randrange(2)),(8,rng.randrange(2))]];charts=[solve(rs,n)for rs in rows];m=mapping(charts,n);source=Circuit(n)
        for _ in range(24):
            kind=rng.choice(['and','xor','not']);source.op(kind,rng.randrange(len(source.nodes)),None if kind=='not'else rng.randrange(len(source.nodes)))
        outputs=[{'wires':[[rng.randrange(len(source.nodes))for _ in range(2)]]}];C,oo,stats=compile_map(source,outputs,m);image=set();got=set()
        for x in range(1<<C.n):
            out['toy_encoded_assignments']+=1;theta=forward(m,x);a=C.evaluate(x);valid=not any(a[w]for field in oo[-1]['wires']for w in field);assert valid==(theta is not None)
            if not valid:continue
            assert theta not in image;image.add(theta);old=source.evaluate(theta);assert [old[w]for w in outputs[0]['wires'][0]]==[a[w]for w in oo[0]['wires'][0]]
            if not any(a[w]for r in oo for field in r['wires']for w in field):got.add(theta)
        expected_image=set();want=set()
        for theta in range(1<<n):
            out['toy_original_assignments']+=1
            if any(all((mask&theta).bit_count()%2==rhs for mask,rhs in rs)for rs in rows):
                expected_image.add(theta);a=source.evaluate(theta)
                if not any(a[w]for w in outputs[0]['wires'][0]):want.add(theta)
        assert image==expected_image and got==want;out['toy_cases']+=1
    cfg=json.loads(Path(d['config']).read_text());records=json.loads(Path(d['cover']).read_text())['records'];charts=[r['chart']for r in records];m=mapping(charts,76);assert m['new_n']<=65;source,outputs=source_graph(cfg);C,oo,stats=compile_map(source,outputs,m,d['round_cap'],d['event_cap']);rows=matrix(cfg['matrix']);r=len(m['common']);w=m['width']
    for branch,record in enumerate(records):
        for _ in range(16):
            gamma=rng.randrange(1<<r);t=rng.randrange(1<<len(m['extras'][branch]));x=gamma|(t<<r)|(branch<<(r+w));theta=forward(m,x);assert theta is not None;old=source.evaluate(theta);a=C.evaluate(x)
            assert [[old[z]for field in row['wires']for z in field]for row in outputs]==[[a[z]for field in row['wires']for z in field]for row in oo[:-1]]and not any(a[z]for field in oo[-1]['wires']for z in field)
            H=decode(rows,theta);assert valid_guard(H,theta>>73);H[0]^=pmul(theta>>73,cfg['modulus']);k=record['k'];assert fits(H,limits(18,k))and fits(translate(H,1<<17),limits(18,5-k))
            for row in oo[:-2]:
                s,mod=row['s'],row['m'];ok,_=predicate([rem(v,mod)for v in H],Arithmetic(2,[(mod>>j)&1 for j in range(s+1)]));assert (not any(a[z]for field in row['wires']for z in field))==ok
            out['actual_controls']+=1
    raw=json.dumps({'nodes':C.nodes,'outputs':oo},separators=(',',':')).encode()
    with(P/'graph.json').open('xb')as f:f.write(raw)
    with(P/'mapping.json').open('x')as f:json.dump(m,f,separators=(',',':'))
    out.update({'inputs':C.n,'dimensions':m['dimensions'],'common_dimension':r,'variables':len(C.nodes)-2,'clauses':len(cnf(C,oo)),'gate_kinds':dict(Counter(row[0]for row in C.nodes)),'synthesis':stats,'graph_sha256':hashlib.sha256(raw).hexdigest(),'all_three_cases_retained':True,'total_seconds':time.monotonic()-START})
except Exception as e:error=repr(e)
finish(out,error,START,'Single canonical three-stratum circuit with exact selector/padding guards and complete tiny union-root oracles.','Freeze a whole-cover changed hard diagnostic only after this representation is assessed;do notinfer speed from counts or discarda stratum.')
