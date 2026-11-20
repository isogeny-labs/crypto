from pathlib import Path
import json,time,random,hashlib
from collections import Counter
from .common import P,finish
from .kernel import compile_all,raw_family
from research.cover_compiler_20261116_v116b.kernel import mapping,forward
from research.chart_compiler_20261116_v112.kernel import source_graph
from research.local_split_circuit_20261116_v100.kernel import cnf
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.simple_prefix_lift_20261116_v117b.kernel import high_root
START=time.monotonic();out={'actual_controls':0,'branch_controls':[0,0,0],'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());cfg=json.loads(Path(d['config']).read_text());records=json.loads(Path(d['cover']).read_text())['records'];m=mapping([r['chart']for r in records],76);source,outputs=source_graph(cfg);raw=raw_family(matrix(cfg['matrix']),cfg['modulus']);C,oo,stats,extra=compile_all(source,outputs,m,raw,18);rng=random.Random(d['seed']);r=len(m['common']);w=m['width']
    for branch in range(3):
        for _ in range(24):
            x=rng.randrange(1<<r)|(rng.randrange(1<<len(m['extras'][branch]))<<r)|(branch<<(r+w));theta=forward(m,x);assert theta is not None;old=source.evaluate(theta);values=C.evaluate(x);assert [[old[z]for f in row['wires']for z in f]for row in outputs]==[[values[z]for f in row['wires']for z in f]for row in oo[:-2]];assert not any(values[z]for f in oo[-2]['wires']for z in f);new_ok=not any(values[z]for f in oo[-1]['wires']for z in f)
            if branch==0:
                root,tail,_=high_root(raw(theta),18);assert new_ok==(tail==0);assert sum(values[z]<<i for i,z in enumerate(extra['root_wires']))==root;assert not any(values[z]for z in extra['low_residual_wires'])
            else:assert new_ok
            out['actual_controls']+=1;out['branch_controls'][branch]+=1
    graph={'nodes':C.nodes,'outputs':oo,'recovery':extra};b=json.dumps(graph,separators=(',',':')).encode()
    with(P/'graph.json').open('xb')as f:f.write(b)
    with(P/'mapping.json').open('x')as f:json.dump(m,f,separators=(',',':'))
    out.update({'inputs':C.n,'variables':len(C.nodes)-2,'clauses':len(cnf(C,oo)),'gate_kinds':dict(Counter(n[0]for n in C.nodes)),'synthesis':stats,'graph_sha256':hashlib.sha256(b).hexdigest(),'cold_total_seconds':time.monotonic()-START,'all_three_cases_retained':True})
except Exception as e:error=repr(e)
finish(out,error,START,'Actual65input completeleadingcover with guarded deterministic high-root lifting.','Freeze changed coldsearch with exactmodelbackmap and numerical high-rootchecks only if source formulation remainsbounded;UNKNOWN staysunresolved.')
