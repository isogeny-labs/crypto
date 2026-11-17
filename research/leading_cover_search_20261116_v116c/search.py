from pathlib import Path
import json,time,hashlib,sys,os
from .common import P,finish
from research.cover_compiler_20261116_v116b.kernel import compile_map,forward
from research.chart_compiler_20261116_v112.kernel import source_graph
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard,prefix_remainder
from research.leading_cover_20261116_v116.kernel import limits,fits
from research.prefix_translation_20261116_v115.kernel import translate
from research.residue_quotient_20261116_v99b.kernel import pmul,rem
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
from research.xor_cut_20261116_v114b.backend import make_solver,readback
START=time.monotonic();out={'models':[],'original_complete':False,'original_exact_root_count':None,'independent_unsat_certificate':False,'scope':'allthreeleadingcases inonefixednecessarynormfamily;otherouterfamilies/source/Uunresolved'};error=None
try:
    d=json.loads((P/'DATA.json').read_text());pd=json.loads(Path(d['profile_data']).read_text());prior=json.loads(Path(d['profile_receipt']).read_text());assert prior['error']is None and prior['out']['all_three_cases_retained'];cfg=json.loads(Path(pd['config']).read_text());m=json.loads(Path(d['mapping']).read_text());source,outputs=source_graph(cfg);C,oo,stats=compile_map(source,outputs,m,pd['round_cap'],pd['event_cap']);raw=json.dumps({'nodes':C.nodes,'outputs':oo},separators=(',',':')).encode();assert hashlib.sha256(raw).hexdigest()==prior['out']['graph_sha256'];out.update({'inputs':C.n,'variables':len(C.nodes)-2,'synthesis':stats,'cold_compile_seconds':time.monotonic()-START})
    sys.path.insert(0,cfg['z3_path']);import z3
    assert z3.get_version_string()==cfg['z3_version'];S=make_solver(z3,d['seed']);out['backend_configuration_readback']=readback(z3);V=[z3.BoolVal(False),z3.BoolVal(True)]+[z3.Bool('v'+str(i-1))for i in range(2,len(C.nodes))]
    for i,node in enumerate(C.nodes[C.n+2:],C.n+2):
        kind,a,*rest=node;value=z3.And(V[a],V[rest[0]])if kind=='and'else z3.Xor(V[a],V[rest[0]])if kind=='xor'else z3.Not(V[a]);S.add(V[i]==value)
    for row in oo:
        for field in row['wires']:
            for wire in field:S.add(z3.Not(V[wire]))
    rows=matrix(cfg['matrix']);r=len(m['common']);w=m['width'];s0,s1=V[2+r+w],V[3+r+w];bits={}
    def original_bit(i):
        if i not in bits:
            value=z3.BoolVal(bool(m['particular'][0]>>i&1))
            for j,v in enumerate(m['common']):
                if v>>i&1:value=z3.Xor(value,V[j+2])
            for j in range(w):
                a=m['extras'][0][j]if j<len(m['extras'][0])else 0
                if a>>i&1:value=z3.Xor(value,V[2+r+j])
                for branch,s in[(1,s0),(2,s1)]:
                    b=m['extras'][branch][j]if j<len(m['extras'][branch])else 0
                    if (a^b)>>i&1:value=z3.Xor(value,z3.And(s,V[2+r+j]))
            for branch,s in[(1,s0),(2,s1)]:
                if(m['particular'][0]^m['particular'][branch])>>i&1:value=z3.Xor(value,s)
            bits[i]=value
        return bits[i]
    out.update({'formulation_seconds':time.monotonic()-START,'version':z3.get_version_string(),'cpu_affinity':sorted(os.sched_getaffinity(0))})
    for _ in range(d['model_cap']):
        remaining=d['total_seconds']-(time.monotonic()-START)
        if remaining<=0:out['termination']='total_budget';error='boundedsearchinconclusive';break
        S.set(timeout=max(1,int(remaining*1000)));answer=S.check();out['last_answer']=str(answer)
        if answer==z3.unknown:out.update({'termination':'unknown','reason_unknown':S.reason_unknown()});error='nativeunknown';break
        if answer==z3.unsat:out.update({'termination':'engine_unsat_claim','engine_unsat_claim':True});break
        model=S.model();x=sum(int(z3.is_true(model.eval(V[i+2],model_completion=True)))<<i for i in range(C.n));theta=forward(m,x);assert theta is not None and all(int(z3.is_true(model.eval(original_bit(i),model_completion=True)))==(theta>>i&1)for i in range(76));values=C.evaluate(x);assert not any(values[z]for row in oo for field in row['wires']for z in field);selector=(x>>(r+w))&3;k=[1,3,5][selector];H=decode(rows,theta);assert valid_guard(H,theta>>73);assert {(0,0,0):1,(1,1,0):3,(0,0,1):5}[((H[3]>>34)&1,(H[2]>>51)&1,(H[1]>>68)&1)]==k;lifted=list(H);lifted[0]^=pmul(theta>>73,cfg['modulus']);assert fits(lifted,limits(18,k))and fits(translate(lifted,1<<17),limits(18,5-k))
        for s,mod in cfg['fields']:
            ok,_=predicate([rem(a,mod)for a in lifted],Arithmetic(2,[(mod>>j)&1 for j in range(s+1)]));assert ok
        remainder=prefix_remainder(H,83,cfg['field_modulus'],18);out['models'].append({'encoding':x,'selector':selector,'k':k,'theta':theta,'q':theta>>73,'H':H,'prefix_remainder8':remainder})
        if not remainder:out['termination']='supported_H_requires_original_recovery';error='candidatependingoriginalsource/Urecovery';break
        S.add(z3.Or(*[z3.Not(original_bit(i))if theta>>i&1 else original_bit(i)for i in range(73)]))
    else:out['termination']='model_cap';error='modelcapinconclusive'
    out['statistics']={name:str(S.statistics().get_key_value(name))for name in S.statistics().keys()};out['engine_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'Changed65-input complete-leading-cover hard necessary-family diagnostic, one total45secondbudget.','Unknown yields unresolved counts and a changed nextmechanism;engineUNSAT needs independentcertification;allsources/U/outercoverage remainsrequired.')
