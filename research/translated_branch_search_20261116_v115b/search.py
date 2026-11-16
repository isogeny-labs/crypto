from pathlib import Path
import json,time,hashlib,random,sys,os
from .common import P,finish
from research.chart_compiler_20261116_v112.kernel import source_graph
from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
from research.parity_chart_20261116_v113d.kernel import optimized_import
from research.prefix_translation_20261116_v115.kernel import translate,bounded
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard,prefix_remainder
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
from research.residue_quotient_20261116_v99b.kernel import pmul,rem,apply
from research.xor_cut_20261116_v114b.backend import make_solver,readback
START=time.monotonic();out={'models':[],'actual_controls':0,'original_complete':False,'original_exact_root_count':None,'independent_unsat_certificate':False,'scope':'only contracted w=1necessary branch;w=0andotherfamilies unresolved'};error=None
try:
    d=json.loads((P/'DATA.json').read_text());cfg=json.loads(Path(d['config']).read_text());prior=json.loads(Path(d['rank_receipt']).read_text());assert prior['error']is None and prior['out']['advancement_rule_met'];chart=json.loads(Path(d['chart']).read_text())['translated_chart1'];assert chart['feasible']and chart['dimension']==57
    source,outputs=source_graph(cfg);C=Circuit(chart['dimension']);forms=[((chart['particular']>>i)&1)|sum(((v>>i)&1)<<(j+1)for j,v in enumerate(chart['columns']))for i in range(76)];C,oo,synthesis=optimized_import(source,outputs,C,forms,list(range(2,C.n+2)),256,8000000);rows=matrix(cfg['matrix']);rng=random.Random(d['seed'])
    for _ in range(16):
        sigma=rng.randrange(1<<C.n);theta=chart['particular']^apply(chart['columns'],sigma);a=source.evaluate(theta);b=C.evaluate(sigma);assert [[a[w]for field in r['wires']for w in field]for r in outputs]==[[b[w]for field in r['wires']for w in field]for r in oo];H=decode(rows,theta);assert valid_guard(H,theta>>73)and(H[1]>>68)&1;H[0]^=pmul(theta>>73,cfg['modulus']);assert bounded(translate(H,1<<17),18)
        for r in oo[:-1]:
            s,m=r['s'],r['m'];ok,_=predicate([rem(x,m)for x in H],Arithmetic(2,[(m>>j)&1 for j in range(s+1)]));assert (not any(b[w]for field in r['wires']for w in field))==ok
        out['actual_controls']+=1
    graph=json.dumps({'nodes':C.nodes,'outputs':oo},separators=(',',':')).encode();(P/'graph.json').open('xb').write(graph);out.update({'inputs':C.n,'variables':len(C.nodes)-2,'clauses':len(cnf(C,oo)),'graph_sha256':hashlib.sha256(graph).hexdigest(),'synthesis':synthesis,'cold_compile_and_controls_seconds':time.monotonic()-START})
    sys.path.insert(0,cfg['z3_path']);import z3
    assert z3.get_version_string()==cfg['z3_version'];S=make_solver(z3,d['seed']);out['backend_configuration_readback']=readback(z3);V=[z3.BoolVal(False),z3.BoolVal(True)]+[z3.Bool('v'+str(i-1))for i in range(2,len(C.nodes))]
    for i,node in enumerate(C.nodes[C.n+2:],C.n+2):
        kind,a,*rest=node;expr=z3.And(V[a],V[rest[0]])if kind=='and'else z3.Xor(V[a],V[rest[0]])if kind=='xor'else z3.Not(V[a]);S.add(V[i]==expr)
    for r in oo:
        for field in r['wires']:
            for w in field:S.add(z3.Not(V[w]))
    out.update({'formulation_seconds':time.monotonic()-START,'cpu_affinity':sorted(os.sched_getaffinity(0)),'version':z3.get_version_string()});bits={}
    def original_bit(i):
        if i not in bits:
            value=z3.BoolVal(bool(chart['particular']>>i&1))
            for j,v in enumerate(chart['columns']):
                if v>>i&1:value=z3.Xor(value,V[j+2])
            bits[i]=value
        return bits[i]
    for _ in range(d['model_cap']):
        remaining=d['total_seconds']-(time.monotonic()-START)
        if remaining<=0:out['termination']='total_budget';error='boundedsearchinconclusive';break
        S.set(timeout=max(1,int(remaining*1000)));answer=S.check();out['last_answer']=str(answer)
        if answer==z3.unknown:out.update({'termination':'unknown','reason_unknown':S.reason_unknown()});error='nativeunknown';break
        if answer==z3.unsat:out.update({'termination':'engine_unsat_claim','engine_unsat_claim':True});break
        model=S.model();sigma=sum(int(z3.is_true(model.eval(V[i+2],model_completion=True)))<<i for i in range(C.n));theta=chart['particular']^apply(chart['columns'],sigma);assert all(int(z3.is_true(model.eval(original_bit(i),model_completion=True)))==(theta>>i&1)for i in range(76));H=decode(rows,theta);assert valid_guard(H,theta>>73)and(H[1]>>68)&1;lifted=list(H);lifted[0]^=pmul(theta>>73,cfg['modulus']);assert bounded(translate(lifted,1<<17),18)
        for s,m in cfg['fields']:
            ok,_=predicate([rem(x,m)for x in lifted],Arithmetic(2,[(m>>j)&1 for j in range(s+1)]));assert ok
        remainder=prefix_remainder(H,83,cfg['field_modulus'],18);out['models'].append({'sigma':sigma,'theta':theta,'q':theta>>73,'H':H,'prefix_remainder8':remainder})
        if not remainder:out['termination']='supported_H_requires_original_recovery';error='candidatependingoriginalsource/Urecovery';break
        S.add(z3.Or(*[z3.Not(original_bit(i))if theta>>i&1 else original_bit(i)for i in range(73)]))
    else:out['termination']='model_cap';error='modelcapinconclusive'
    out['statistics']={name:str(S.statistics().get_key_value(name))for name in S.statistics().keys()};out['engine_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'Changed57-input common-prefix-translated necessary-branch diagnostic; allotherbranches remain unresolved.','If unknown, preserve the10rowcontraction and derive coprime leading-stratum/root-prefix lifting rather than repeat the same57bitsearch. NativeUNSATneedsindependentcertification.')
