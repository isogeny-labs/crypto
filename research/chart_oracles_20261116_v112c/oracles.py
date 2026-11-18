from pathlib import Path
import json,time,random,itertools
from .common import P,finish
from research.chart_compiler_20261116_v112.kernel import affine_import,forward
from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.frontier_certificate_20261116_v111b.kernel import guarded_charts
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard
START=time.monotonic();out={'affine_cases':0,'affine_assignments':0,'union_cases':0,'union_original_assignments':0,'actual_guard_controls':0,'original_complete':False};error=None
def literal(C,x):
    a=[0,1]+[(x>>i)&1 for i in range(C.n)]
    for kind,u,*tail in C.nodes[C.n+2:]:a.append(a[u]&a[tail[0]]if kind=='and'else a[u]^a[tail[0]]if kind=='xor'else 1-a[u])
    return a
def zeros(values,outputs):return not any(values[w]for r in outputs for field in r['wires']for w in field)
try:
    d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed'])
    for case in range(60):
        n=3+case%5;k=case%(n+1);old=Circuit(n)
        for _ in range(30):
            op=rng.choice(['and','xor','not']);old.op(op,rng.randrange(len(old.nodes)),None if op=='not'else rng.randrange(len(old.nodes)))
        outputs=[{'wires':[[rng.randrange(len(old.nodes))for _ in range(4)]]}];chart={'particular':rng.randrange(1<<n),'columns':[rng.randrange(1<<n)for _ in range(k)]};new,oo=affine_import(old,outputs,chart)
        for sigma in range(1<<k):
            a=literal(old,forward(chart,sigma));b=new.evaluate(sigma)
            assert [[a[w]for w in field]for field in outputs[0]['wires']]==[[b[w]for w in field]for field in oo[0]['wires']];out['affine_assignments']+=1
        out['affine_cases']+=1
    for case in range(40):
        n=5+case%3;old=Circuit(n)
        for _ in range(20):
            op=rng.choice(['and','xor','not']);old.op(op,rng.randrange(len(old.nodes)),None if op=='not'else rng.randrange(len(old.nodes)))
        outputs=[{'wires':[[rng.randrange(len(old.nodes))for _ in range(2)]]}];rows=[(rng.randrange(1<<n),rng.randrange(2))for _ in range(case%3)];flags=[(rng.randrange(1<<n),rng.randrange(2))for _ in range(3)]+[(1<<(n-2),0),(1<<(n-1),0)];charts=guarded_charts(rows,n,*flags);actual=set()
        for chart in charts:
            if not chart['feasible']:continue
            C,oo=affine_import(old,outputs,chart)
            actual.update(forward(chart,sigma)for sigma in range(1<<chart['dimension'])if zeros(C.evaluate(sigma),oo))
        expected=set()
        for theta in range(1<<n):
            out['union_original_assignments']+=1
            if any((m&theta).bit_count()%2!=rhs for m,rhs in rows):continue
            u,v,w,q1,q2=[b^((m&theta).bit_count()%2)for m,b in flags]
            if (u,v,w)in[(0,0,0),(1,1,0),(0,0,1)]and q2==w and(w or not q1)and zeros(literal(old,theta),outputs):expected.add(theta)
        assert actual==expected;out['union_cases']+=1
    charts=json.loads(Path(d['charts']).read_text())['charts'];rows=matrix(d['matrix'])
    for chart in charts:
        for _ in range(64):
            theta=forward(chart,rng.randrange(1<<67));assert valid_guard(decode(rows,theta),theta>>73);out['actual_guard_controls']+=1
except Exception as e:error=repr(e)
finish(out,error,START,'Independent different-dimension affine compiler and complete two-chart union-root-set oracles.','Compile both actual67-input charts, charge duplicated formulation and retain losses before any native search.')
