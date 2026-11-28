import json,time,sys,hashlib,os
from .common import P,finish
from .kernel import compile_all,old_value,new_value
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard,prefix_remainder
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
from research.residue_quotient_20261116_v99b.kernel import rem,pmul

START=time.monotonic()
out={'models':[],'original_complete':False,'original_exact_root_count':None,'independent_unsat_certificate':False}
error=None
try:
    d=json.loads((P/'DATA.json').read_text())
    prior=json.loads((P/'compile_raw.json').read_text())
    assert prior['error'] is None
    C,outputs,mapping,blocks=compile_all(d)
    raw=json.dumps({'nodes':C.nodes,'outputs':outputs},separators=(',',':')).encode()
    assert hashlib.sha256(raw).hexdigest()==prior['out']['graph_sha256']
    out.update({'cold_compile_seconds':time.monotonic()-START,'variables':len(C.nodes)-2,'blocks':blocks})
    sys.path.insert(0,d['z3_path'])
    import z3
    assert z3.get_version_string()==d['z3_version']
    z3.set_param('parallel.enable',False)
    z3.set_param('sat.threads',1)
    S=z3.Solver()
    S.set(threads=1,random_seed=d['seed'])
    V=[z3.BoolVal(False),z3.BoolVal(True)]+[z3.Bool('v'+str(i-1))for i in range(2,len(C.nodes))]
    for i,node in enumerate(C.nodes[78:],78):
        kind,a,*rest=node
        expr=z3.And(V[a],V[rest[0]])if kind=='and'else z3.Xor(V[a],V[rest[0]])if kind=='xor'else z3.Not(V[a])
        S.add(V[i]==expr)
    for r in outputs:
        for field in r['wires']:
            for w in field:
                S.add(z3.Not(V[w]))
    out.update({'formulation_seconds':time.monotonic()-START,'version':z3.get_version_string(),'cpu_affinity':sorted(os.sched_getaffinity(0))})
    rows=matrix(d['matrix'])
    old_expr={}
    def original_bit(i):
        if i not in old_expr:
            values=[V[2+j]for j,column in enumerate(mapping['T'])if column>>i&1]
            while len(values)>1:
                values=[z3.Xor(values[j],values[j+1])if j+1<len(values)else values[j]for j in range(0,len(values),2)]
            expr=values[0]if values else z3.BoolVal(False)
            if mapping['offset']>>i&1:
                expr=z3.Not(expr)
            old_expr[i]=expr
        return old_expr[i]
    for iteration in range(d['model_cap']):
        remaining=d['search_deadline_seconds']-(time.monotonic()-START)
        if remaining<=0:
            out['termination']='worker_search_budget'
            error='boundedsearchinconclusive'
            break
        S.set(timeout=max(1,int(1000*remaining)))
        answer=S.check()
        out['last_answer']=str(answer)
        if answer==z3.unknown:
            out.update({'termination':'unknown','reason_unknown':S.reason_unknown()})
            error='nativeunknown'
            break
        if answer==z3.unsat:
            out.update({'termination':'engine_unsat_claim','engine_unsat_claim':True})
            break
        model=S.model()
        rho=sum(int(z3.is_true(model.eval(V[2+i],model_completion=True)))<<i for i in range(76))
        theta=old_value(mapping,rho)
        assert new_value(mapping,theta)==rho
        values=C.evaluate(rho)
        assert not any(values[w]for r in outputs for field in r['wires']for w in field)
        H=decode(rows,theta)
        assert valid_guard(H,theta>>73)
        lifted=list(H)
        lifted[0]^=pmul(theta>>73,d['modulus'])
        for s,m in d['fields']:
            ok,_=predicate([rem(x,m)for x in lifted],Arithmetic(2,[(m>>i)&1 for i in range(s+1)]))
            assert ok
        remainder=prefix_remainder(H,83,d['field_modulus'],18)
        out['models'].append({'rho':rho,'theta':theta,'H':H,'q':theta>>73,'prefix_remainder8':remainder})
        if not remainder:
            out['termination']='supported_H_requires_original_recovery'
            error='candidatependingoriginalsource/Urecovery'
            break
        S.add(z3.Or(*[z3.Not(original_bit(i))if theta>>i&1 else original_bit(i)for i in range(73)]))
    else:
        out['termination']='model_cap'
        error='modelcapinconclusive'
    out['statistics']={k:str(S.statistics().get_key_value(k))for k in S.statistics().keys()}
    out['engine_seconds']=time.monotonic()-START
except Exception as e:
    error=repr(e)
finish(out,error,START,'Same hard necessary family under exact systematic coordinates; original back-map and support checked for every model.','If unknown, measure cross-block coupling and choose a changed exact separator/propagation mechanism; no identical timeout retry or original completeness claim.')
