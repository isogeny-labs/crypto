import json,time,random,itertools
from .common import P,finish
from .kernel import compile_all
from research.cover_compiler_20261116_v116b.kernel import mapping,forward
from research.frontier_certificate_20261116_v111b.kernel import solve
from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.simple_prefix_lift_20261116_v117b.kernel import high_root
from research.prefix_translation_20261116_v115.kernel import product
START=time.monotonic();out={'cases':0,'original_assignments':0,'encoded_assignments':0,'complete_root_multisets':0,'offbranch_assignments':0,'original_complete':False};error=None
try:
    data=json.loads((P/'DATA.json').read_text());rng=random.Random(data['seed'])
    for case in range(32):
        d=2+case%2;n=6+case%2;top=1<<(d-1);catalog={tuple(product(xs)):xs for xs in itertools.combinations_with_replacement(range(1<<d),5)};planted=[]
        for k in [1,3,5]:planted.append(product([rng.randrange(top)for _ in range(5-k)]+[top|rng.randrange(top)for _ in range(k)]))
        base=planted[0];dirs=[[a^b for a,b in zip(base,planted[i])]for i in [1,2]]
        for _ in range(n-2):dirs.append([rng.randrange(1<<(max(1,(5-j)*(d-2)+min(5-j,1))))for j in range(5)]+[0])
        def raw(theta):
            H=list(base)
            for i,v in enumerate(dirs):
                if theta>>i&1:H=[a^b for a,b in zip(H,v)]
            return H
        charts=[solve([(1,a),(2,b)],n)for a,b in [(0,0),(1,0),(0,1)]];m=mapping(charts,n);source=Circuit(n);expected={theta:catalog[tuple(raw(theta))]for theta in range(1<<n)if theta&3!=3 and tuple(raw(theta))in catalog};satisfied=0
        for theta in expected:
            term=1
            for i in range(n):term=source.op('and',term,2+i if theta>>i&1 else source.op('not',2+i))
            satisfied=source.xor(source.xor(satisfied,term),source.op('and',satisfied,term))
        outputs=[{'full_prefix_support':True,'wires':[[source.op('not',satisfied)]]}];C,oo,stats,extra=compile_all(source,outputs,m,raw,d);got={};image=set();r=len(m['common']);w=m['width']
        for x in range(1<<C.n):
            out['encoded_assignments']+=1;theta=forward(m,x);values=C.evaluate(x);cover_ok=not any(values[z]for field in oo[-2]['wires']for z in field);assert cover_ok==(theta is not None)
            if theta is None:continue
            assert theta not in image;image.add(theta);selector=x>>(r+w);H=raw(theta);branch_ok=not any(values[z]for field in oo[-1]['wires']for z in field)
            if selector==0:
                root,tail,_=high_root(H,d);assert branch_ok==(tail==0);assert root==sum(values[z]<<i for i,z in enumerate(extra['root_wires']))
            else:assert branch_ok;out['offbranch_assignments']+=1
            old=source.evaluate(theta);assert [[old[z]for f in row['wires']for z in f]for row in outputs]==[[values[z]for f in row['wires']for z in f]for row in oo[:-2]]
            if not any(values[z]for row in oo for field in row['wires']for z in field):got[theta]=catalog[tuple(H)]
        assert got==expected and all(t in got for t in [0,1,2]);out['complete_root_multisets']+=len(got);out['original_assignments']+=1<<n;out['cases']+=1
except Exception as e:error=repr(e)
finish(out,error,START,'Guarded high-root integration preserves complete small prefix root multisets across allthree leading cases.','Profile actual complete65inputcover;no globaloriginal339roots or speed ratio follows from small complete checks.')
