from pathlib import Path
import json,time,itertools,random,hashlib
from collections import Counter
from .kernel import compile_all
from research.local_split_circuit_20261116_v100.kernel import cnf
from research.root_verifier_checks_20261117_v123b.kernel import tiny as old_tiny,compile_all as old_compile_all
from .common import P,finish
from .kernel import tiny
from research.prefix_translation_20261116_v115.kernel import product
from research.simple_prefix_lift_20261116_v117b.kernel import reverse
from research.residue_quotient_20261116_v99b.kernel import rem
START=time.monotonic();out={'actual_controls':0,'formulation':None,'exhaustive_assignments':0,'planted_assignments':0,'random_assignments':0,'accepted':0,'q_alias_checks':0,'original_complete':False};error=None

def catalog(d):
    top=1<<(d-1);return {tuple(product(low+high)):list(low+high)for low in itertools.combinations_with_replacement(range(top),2)for high in itertools.combinations_with_replacement(range(top,1<<d),3)}
def decode(x,d):
    F=[];offset=0
    for j in range(5):
        width=(5-j)*(d-1);F.append((((x>>offset)&((1<<width)-1))<<1)|int(j>=2));offset+=width
    F.append(1);return tuple(reverse(v,(5-j)*(d-1))for j,v in enumerate(F))
def encode(H,d):
    x=0;offset=0
    for j,v in enumerate(H[:5]):x|=(reverse(v,(5-j)*(d-1))>>1)<<offset;offset+=(5-j)*(d-1)
    return x
try:
    data=json.loads((P/'DATA.json').read_text());rng=random.Random(data['seed'])
    for d in [2,3]:
        candidate,_=tiny(d);control,_=old_tiny(d);C,oo,roots=candidate;D,co,oldroots=control;expected=catalog(d)
        cases=list(range(1<<C.n))if d==2 else[encode(H,d)for H in expected]+[rng.randrange(1<<C.n)for _ in range(128)]
        for index,x in enumerate(cases):
            v=C.evaluate(x);w=D.evaluate(x);H=decode(x,d);want=expected.get(H);got=not any(v[z]for row in oo for field in row['wires']for z in field);old=not any(w[z]for row in co for field in row['wires']for z in field);assert got==old==(want is not None)
            if got:assert sorted(sum(v[z]<<i for i,z in enumerate(row))for row in roots)==sorted(sum(w[z]<<i for i,z in enumerate(row))for row in oldroots)==want;out['accepted']+=1
            if d==2:out['exhaustive_assignments']+=1
            elif index<len(expected):out['planted_assignments']+=1
            else:out['random_assignments']+=1
        if d==2:
            H=product([0,0,2,2,2]);alias=list(H);alias[0]^=11;assert [rem(a,11)for a in H]==[rem(a,11)for a in alias];v=C.evaluate(encode(alias,2));assert any(v[z]for row in oo for field in row['wires']for z in field);out['q_alias_checks']+=1
    cfg=json.loads(Path(data['config']).read_bytes());chart=json.loads(Path(data['cover']).read_bytes())['records'][1]['chart'];t=time.monotonic();candidate,_=compile_all(cfg,chart);out['cold_compile_seconds']=time.monotonic()-t;G,oo,rr=candidate;source,_=old_compile_all(cfg,chart);D,co,oldroots=source
    for _ in range(32):
        x=rng.randrange(1<<G.n);v=G.evaluate(x);w=D.evaluate(x);got=not any(v[z]for row in oo for field in row['wires']for z in field);old=not any(w[z]for row in co for field in row['wires']for z in field);assert got==old
        if got:assert sorted(sum(v[z]<<i for i,z in enumerate(row))for row in rr)==sorted(sum(w[z]<<i for i,z in enumerate(row))for row in oldroots)
        out['actual_controls']+=1
    raw=json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr},separators=(',',':')).encode()
    with(P/'graph.json').open('xb')as f:f.write(raw)
    out['formulation']={'inputs':G.n,'variables':len(G.nodes)-2,'clauses':len(cnf(G,oo)),'gate_kinds':dict(Counter(n[0]for n in G.nodes)),'graph_sha256':hashlib.sha256(raw).hexdigest(),'source_variables':len(D.nodes)-2,'source_clauses':len(cnf(D,co))};out['total_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'Degree-clipped Hensel generator checked against independent catalogs and old exact acceptance, then actual verifier profile.','If correct and smaller, freeze changed cold native diagnostic on degree-clipped generator; otherwise preserve counterexample and repair proof.')
