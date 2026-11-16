from pathlib import Path
import json,time,random,hashlib,ast
from collections import Counter
from .common import P,finish
from .kernel import synthesize,compile_map
from research.unified_chart_20261116_v112b.kernel import union_map,forward,literal
from research.frontier_certificate_20261116_v111b.kernel import guarded_charts
from research.chart_compiler_20261116_v112.kernel import source_graph
from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
from research.residue_quotient_20261116_v99b.kernel import rem,pmul
START=time.monotonic();out={'linear_cases':0,'linear_assignments':0,'toy_cases':0,'toy_original_assignments':0,'toy_compiled_assignments':0,'actual_controls':0,'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed'])
    for case in range(40):
        n=3+case%5;C=Circuit(n);targets=set(rng.randrange(1<<(n+1))for _ in range(12));expr,gates,stats=synthesize(C,list(range(2,n+2)),targets,16,100000)
        for x in range(1<<n):
            values=C.evaluate(x);out['linear_assignments']+=1
            for f in targets:
                value=f&1
                for j in expr[f]:value^=values[gates[j]]
                assert value==(f&1)^((x&(f>>1)).bit_count()%2)
        out['linear_cases']+=1
    # Replay the frozen complete tiny union oracle with the new compiler supplied.
    src=ast.parse(Path(d['tiny_oracle_source']).read_text());body=next(n for n in src.body if isinstance(n,ast.Try));loop=next(n for n in body.body if isinstance(n,ast.For))
    def test_compile(source,outputs,mapping):
        C,oo,stats=compile_map(source,outputs,mapping,d['round_cap'],d['event_cap']);return C,oo
    ns={'rng':random.Random(d['tiny_seed']),'out':out,'Circuit':Circuit,'union_map':union_map,'forward':forward,'compile_map':test_compile,'literal':literal,'guarded_charts':guarded_charts};exec(compile(ast.Module(body=[loop],type_ignores=[]),'frozen_tiny_union','exec'),ns)
    cfg=json.loads(Path(d['config']).read_text());mapping=json.loads(Path(d['mapping']).read_text());source,outputs=source_graph(cfg);C,oo,stats=compile_map(source,outputs,mapping,d['round_cap'],d['event_cap']);rows=matrix(cfg['matrix'])
    for _ in range(32):
        sigma=rng.randrange(1<<68);theta=forward(mapping,sigma);a=literal(source,theta);b=C.evaluate(sigma)
        assert [[a[w]for field in r['wires']for w in field]for r in outputs]==[[b[w]for field in r['wires']for w in field]for r in oo]
        H=decode(rows,theta);assert valid_guard(H,theta>>73);H[0]^=pmul(theta>>73,cfg['modulus'])
        for r in oo[:-1]:
            s,m=r['s'],r['m'];ok,_=predicate([rem(x,m)for x in H],Arithmetic(2,[(m>>j)&1 for j in range(s+1)]));assert (not any(b[w]for field in r['wires']for w in field))==ok
        out['actual_controls']+=1
    raw=json.dumps({'nodes':C.nodes,'outputs':oo},separators=(',',':')).encode()
    with(P/'graph.json').open('xb')as f:f.write(raw)
    variables=len(C.nodes)-2;clauses=len(cnf(C,oo));out.update({'inputs':C.n,'variables':variables,'clauses':clauses,'gate_kinds':dict(Counter(n[0]for n in C.nodes)),'synthesis':stats,'graph_sha256':hashlib.sha256(raw).hexdigest(),'native_search_promotion':variables<48887 and clauses<179895,'total_seconds':time.monotonic()-START})
except Exception as e:error=repr(e)
finish(out,error,START,'Bounded shared-parity synthesis with independent linear truth tables, full tiny root unions and actual14residue controls.','Only promote improved total formulation to changed native search; retain larger forms and caps, and change mechanism before another timeout.')
