from pathlib import Path
import json,time,sys,random
from .common import P,finish
from .backend import make_solver,readback,CONFIG
from research.local_split_circuit_20261116_v100.kernel import Circuit
START=time.monotonic();out={'cases':0,'literal_assignments':0,'native_checks':0,'root_sets':[],'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());sys.path.insert(0,d['z3_path']);import z3
    assert z3.get_version_string()==d['z3_version'];rng=random.Random(d['seed'])
    for case in range(32):
        n=3+case%5;C=Circuit(n)
        for _ in range(18):
            kind=rng.choice(['xor','and','not']);C.op(kind,rng.randrange(len(C.nodes)),None if kind=='not'else rng.randrange(len(C.nodes)))
        outputs=[rng.randrange(len(C.nodes))for _ in range(2)];expected=set()
        for theta in range(1<<n):
            a=[0,1]+[(theta>>i)&1 for i in range(n)];out['literal_assignments']+=1
            for kind,u,*rest in C.nodes[n+2:]:a.append(a[u]&a[rest[0]]if kind=='and'else a[u]^a[rest[0]]if kind=='xor'else 1-a[u])
            if not any(a[w]for w in outputs):expected.add(theta)
        S=make_solver(z3,d['seed']+case);S.set(timeout=500);V=[z3.BoolVal(False),z3.BoolVal(True)]+[z3.Bool('c'+str(case)+'v'+str(i))for i in range(2,len(C.nodes))]
        for i,node in enumerate(C.nodes[n+2:],n+2):
            kind,u,*rest=node;expr=z3.And(V[u],V[rest[0]])if kind=='and'else z3.Xor(V[u],V[rest[0]])if kind=='xor'else z3.Not(V[u]);S.add(V[i]==expr)
        for w in outputs:S.add(z3.Not(V[w]))
        got=set()
        for _ in range((1<<n)+1):
            out['native_checks']+=1
            if out['native_checks']>4096:raise RuntimeError('native-check cap')
            answer=S.check();assert answer!=z3.unknown
            if answer==z3.unsat:break
            model=S.model();theta=sum(int(z3.is_true(model.eval(V[i+2],model_completion=True)))<<i for i in range(n));assert theta not in got;got.add(theta);S.add(z3.Or(*[z3.Not(V[i+2])if theta>>i&1 else V[i+2]for i in range(n)]))
        else:raise AssertionError('all-model cap')
        assert got==expected;out['root_sets'].append(sorted(got));out['cases']+=1
    out.update({'version':z3.get_version_string(),'explicit_configuration':CONFIG,'global_readback':readback(z3),'solver':'Tactic(sat).solver','anf_enabled':False})
except Exception as e:error=repr(e)
finish(out,error,START,'Independent complete small Boolean root-set oracle for explicit SAT XOR-cut configuration.','Only run a newly frozen same-graph45second hard diagnostic after full-set and parameter readback checks pass.')
