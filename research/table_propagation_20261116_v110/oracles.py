import itertools,random,json,time
from .kernel import propagate,Budget,WorkCap
from .common import finish,P
START=time.monotonic();out={'cases':0,'literal_assignments':0,'cap_checked':False,'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed'])
    for case in range(120):
        bits=1+case%4;size=1<<bits;n=2+case%4
        domains=[[(t,rng.randrange(size)) for t in range(rng.randrange(5))]for i in range(n)]
        target=rng.randrange(size)
        valid=[]
        supports=[set()for x in domains]
        for assignment in itertools.product(*domains):
            out['literal_assignments']+=1;value=0
            for t,v in assignment:value^=v
            if value==target:
                valid.append(assignment)
                for i,(t,v)in enumerate(assignment):supports[i].add(t)
        result=propagate(domains,bits,target,Budget())
        assert result['exact_single_relation_tuple_count']==len(valid)
        assert all({t for t,v in kept}==s for kept,s in zip(result['kept'],supports))
        again=propagate(result['kept'],bits,target,Budget())
        assert again['kept']==result['kept'] and again['exact_single_relation_tuple_count']==len(valid)
        out['cases']+=1
    try:propagate([[(0,0)],[(0,1)]],4,1,Budget(1))
    except WorkCap:out['cap_checked']=True
    assert out['cap_checked']
except Exception as e:error=repr(e)
finish(out,error,START,'Independent exhaustive finite-domain XOR support and fixed-point oracle.','Profile actual eighth-residue quotient; retain no-cut results and never convert a work cap to UNSAT.')
