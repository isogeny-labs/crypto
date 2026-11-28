from pathlib import Path
import json,time,random,itertools,hashlib,gc
from .common import P,finish
from .kernel import tiny,compile_all,trees
from research.local_split_circuit_20261116_v100.kernel import cnf
from research.prefix_translation_20261116_v115.kernel import product
from research.simple_prefix_lift_20261116_v117b.kernel import reverse
from research.degree_clipped_checks_20261117_v125c.kernel import tiny as old_tiny,compile_all as old_compile
START=time.monotonic();out={'variants':{},'exhaustive_assignments':0,'positive_assignments':0,'random_assignments':0,'actual_value_controls':0,'original_complete':False};error=None

def catalog(d):
    top=1<<(d-1);return {tuple(product(low+high)):list(low+high)for low in itertools.combinations_with_replacement(range(top),2)for high in itertools.combinations_with_replacement(range(top,1<<d),3)}
def encode(H,d):
    x=0;offset=0
    for j,v in enumerate(H[:5]):x|=(reverse(v,(5-j)*(d-1))>>1)<<offset;offset+=(5-j)*(d-1)
    return x

def values(C,oo,rr,x):
    v=C.evaluate(x);return [v[w]for row in oo for field in row['wires']for w in field]+[v[w]for row in rr for w in row]
try:
    d=json.loads((P/'DATA.json').read_bytes());rng=random.Random(d['seed']);T=trees(list(range(5)));assert len(T)==14
    for width in [2,3]:
        (old,oo,rr),_=old_tiny(width);expected=catalog(width);cases=list(range(1<<old.n))if width==2 else [encode(H,width)for H in expected]+[rng.randrange(1<<old.n)for _ in range(128)];controls=[values(old,oo,rr,x)for x in cases];supported={encode(H,width):roots for H,roots in expected.items()}
        for index,tree in enumerate(T):
            C,co,cr=tiny(width,tree)
            for i,x in enumerate(cases):
                got=values(C,co,cr,x);assert got==controls[i]
                v=C.evaluate(x);accepted=not any(v[w]for row in co for field in row['wires']for w in field);assert accepted==(x in supported)
                if accepted:assert sorted(sum(v[w]<<j for j,w in enumerate(row))for row in cr)==supported[x]
            if width==2:out['exhaustive_assignments']+=len(cases)
            else:out['positive_assignments']+=len(expected);out['random_assignments']+=128
    cfg=json.loads(Path(d['config']).read_bytes());chart=json.loads(Path(d['cover']).read_bytes())['records'][1]['chart'];(old,oo,rr),_=old_compile(cfg,chart);prior=json.loads(Path(d['clipped_receipt']).read_bytes());raw=json.dumps({'nodes':old.nodes,'outputs':oo,'roots':rr},separators=(',',':')).encode();assert hashlib.sha256(raw).hexdigest()==prior['out']['formulation']['graph_sha256'];out['source']=prior['out']['formulation'];samples=[rng.randrange(1<<old.n)for _ in range(d['actual_controls'])];controls=[values(old,oo,rr,x)for x in samples]
    for index,tree in enumerate(T):
        t=time.monotonic();G,outputs,roots=compile_all(cfg,chart,tree)
        for x,wanted in zip(samples,controls):assert values(G,outputs,roots,x)==wanted;out['actual_value_controls']+=1
        raw=json.dumps({'nodes':G.nodes,'outputs':outputs,'roots':roots},separators=(',',':')).encode()
        with(P/('graph_'+str(index)+'.json')).open('xb')as f:f.write(raw)
        out['variants'][str(index)]={'tree':tree,'variables':len(G.nodes)-2,'clauses':len(cnf(G,outputs)),'graph_sha256':hashlib.sha256(raw).hexdigest(),'compile_and_check_seconds':time.monotonic()-t};gc.collect()
    out['selected_index']=min((v['clauses'],v['variables'],int(k))for k,v in out['variants'].items())[2];out['total_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'All14fixed-order product trees with exact observable controls and actualchart formula profiles.','Only a measured representation change justifies another hard diagnostic; otherwise profile precision-block dependencies and factor constraints.')
