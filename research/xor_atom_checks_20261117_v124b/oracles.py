import json,time,random
from .common import P,finish
from .kernel import rewrite
from research.local_split_circuit_20261116_v100.kernel import Circuit
START=time.monotonic();out={'circuits':0,'configurations':0,'assignments':0,'value_checks':0,'forced_cuts':0,'work_cap_checks':0,'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed'])
    for case in range(80):
        n=3+case%4;C=Circuit(n)
        for _ in range(64):
            kind=rng.choice(['and','xor','xor','not']);a=rng.randrange(len(C.nodes));b=rng.randrange(len(C.nodes));C.op(kind,a,None if kind=='not'else b)
        a=C.op('and',2,3);b=C.op('and',3,4);cancel=C.xor(a,C.xor(a,b));complement=C.xor(cancel,C.op('not',b));outputs=[{'wires':[[cancel,complement]+[rng.randrange(len(C.nodes))for _ in range(10)]]}];roots=[[rng.randrange(len(C.nodes))for _ in range(6)]for _ in range(2)];before=(tuple(C.nodes),json.dumps(outputs),json.dumps(roots));out['circuits']+=1
        for terms in [2,4,8,16]:
            G,oo,rr,stats=rewrite(C,outputs,roots,terms,d['support_work_cap']);assert before==(tuple(C.nodes),json.dumps(outputs),json.dumps(roots));out['configurations']+=1;out['forced_cuts']+=stats['cuts']
            for x in range(1<<n):
                old=C.evaluate(x);new=G.evaluate(x);a=[old[w]for row in outputs for field in row['wires']for w in field]+[old[w]for row in roots for w in row];b=[new[w]for row in oo for field in row['wires']for w in field]+[new[w]for row in rr for w in row];assert a==b;assert old[cancel]==old[C.op('and',3,4)]and old[complement]==1;out['assignments']+=1;out['value_checks']+=len(a)
    C=Circuit(4);w=C.op('and',C.xor(C.xor(2,3),4),5)
    try:rewrite(C,[{'wires':[[w]]}],[[w]],16,0)
    except OverflowError as e:assert str(e)=='XORsupportworkcap';out['work_cap_checks']+=1
    else:raise AssertionError('workcapnottriggered')
except Exception as e:error=repr(e)
finish(out,error,START,'Exhaustive fulloutput/recovery equivalence forXORnormalizationovernonlinearvalueswithforcedcuts/controlimmutability/capchecks.','Onlyprofilefrozenactualverifierafteroraclespass;cutsareexactbutdenseflatteningmayincreasecost.')
