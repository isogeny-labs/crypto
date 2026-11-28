from pathlib import Path
import json,time,hashlib,sys,os
from .common import P,finish
from research.parity_chart_20261116_v113d.kernel import compile_map
from research.unified_chart_20261116_v112b.kernel import forward
from research.chart_compiler_20261116_v112.kernel import source_graph
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard,prefix_remainder
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
from research.residue_quotient_20261116_v99b.kernel import rem,pmul
START=time.monotonic();out={'models':[],'original_complete':False,'original_exact_root_count':None,'independent_unsat_certificate':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());pd=json.loads(Path(d['profile_data']).read_text());prior=json.loads(Path(d['profile_receipt']).read_text());assert prior['error']is None and prior['out']['native_search_promotion']
    cfg=json.loads(Path(pd['config']).read_text());mapping=json.loads(Path(pd['mapping']).read_text());source,outputs=source_graph(cfg);C,oo,stats=compile_map(source,outputs,mapping,pd['round_cap'],pd['event_cap']);raw=json.dumps({'nodes':C.nodes,'outputs':oo},separators=(',',':')).encode();assert hashlib.sha256(raw).hexdigest()==prior['out']['graph_sha256'];out.update({'inputs':C.n,'variables':len(C.nodes)-2,'synthesis':stats,'cold_compile_seconds':time.monotonic()-START})
    sys.path.insert(0,cfg['z3_path']);import z3
    assert z3.get_version_string()==cfg['z3_version'];z3.set_param('parallel.enable',False);z3.set_param('sat.threads',1)
    S=z3.Solver();S.set(threads=1,random_seed=d['seed']);V=[z3.BoolVal(False),z3.BoolVal(True)]+[z3.Bool('v'+str(i-1))for i in range(2,len(C.nodes))]
    for i,node in enumerate(C.nodes[C.n+2:],C.n+2):
        kind,a,*rest=node;expr=z3.And(V[a],V[rest[0]])if kind=='and'else z3.Xor(V[a],V[rest[0]])if kind=='xor'else z3.Not(V[a]);S.add(V[i]==expr)
    for r in oo:
        for field in r['wires']:
            for w in field:S.add(z3.Not(V[w]))
    out.update({'version':z3.get_version_string(),'formulation_seconds':time.monotonic()-START,'cpu_affinity':sorted(os.sched_getaffinity(0))});rows=matrix(cfg['matrix']);k=len(mapping['common']);t,s=V[k+2],V[k+3];st=z3.And(s,t);a,b=mapping['extra'];p,q=mapping['particular'];original={}
    def original_bit(i):
        if i not in original:
            terms=[V[j+2]for j,v in enumerate(mapping['common'])if v>>i&1]
            if a>>i&1:terms.append(t)
            if (p^q)>>i&1:terms.append(s)
            if (a^b)>>i&1:terms.append(st)
            value=z3.BoolVal(bool(p>>i&1))
            for term in terms:value=z3.Xor(value,term)
            original[i]=value
        return original[i]
    for iteration in range(d['model_cap']):
        remaining=d['total_search_seconds']-(time.monotonic()-START)
        if remaining<=0:out['termination']='total_worker_search_budget';error='boundedsearchinconclusive';break
        S.set(timeout=max(1,int(remaining*1000)));answer=S.check();out['last_answer']=str(answer)
        if answer==z3.unknown:out.update({'termination':'unknown','reason_unknown':S.reason_unknown()});error='nativeunknown';break
        if answer==z3.unsat:out.update({'termination':'engine_unsat_claim','engine_unsat_claim':True});break
        model=S.model();sigma=sum(int(z3.is_true(model.eval(V[i+2],model_completion=True)))<<i for i in range(C.n));theta=forward(mapping,sigma)
        assert all(int(z3.is_true(model.eval(original_bit(i),model_completion=True)))==(theta>>i&1)for i in range(76))
        values=C.evaluate(sigma);assert not any(values[w]for r in oo for field in r['wires']for w in field);H=decode(rows,theta);assert valid_guard(H,theta>>73);lifted=list(H);lifted[0]^=pmul(theta>>73,cfg['modulus'])
        for degree,mod in cfg['fields']:
            ok,_=predicate([rem(x,mod)for x in lifted],Arithmetic(2,[(mod>>i)&1 for i in range(degree+1)]));assert ok
        remainder=prefix_remainder(H,83,cfg['field_modulus'],18);out['models'].append({'sigma':sigma,'theta':theta,'H':H,'q':theta>>73,'prefix_remainder8':remainder})
        if not remainder:out['termination']='supported_H_requires_original_recovery';error='candidatependingoriginalsource/Urecovery';break
        S.add(z3.Or(*[z3.Not(original_bit(i))if theta>>i&1 else original_bit(i)for i in range(73)]))
    else:out['termination']='model_cap';error='modelcapinconclusive'
    out['statistics']={name:str(S.statistics().get_key_value(name))for name in S.statistics().keys()};out['engine_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'Changed68-input shared-parity hard necessary-family search with exact original theta/q model back-map.','Unknown remains unresolved; inspect source/atom parity dependencies and design a changed mechanism before another native retry. EngineUNSAT requires independent certificate.')
