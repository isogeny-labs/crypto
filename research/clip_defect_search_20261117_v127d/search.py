from pathlib import Path
import json,time,hashlib,sys,os
from .common import P,finish
from research.clip_defect_checks_20261117_v127c.kernel import compile_all
from research.exact_cubic_prefix_20261117_v122.kernel import support as cubic_support,translate as cubic_translate
from research.prefix_translation_20261116_v115.kernel import product
from research.triangular_prefix_quadratic_20261116_v121.kernel import solve
from research.simple_prefix_lift_20261116_v117b.kernel import reverse
from research.residue_quotient_20261116_v99b.kernel import apply,pmul,rem
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard,prefix_remainder
from research.coprime_prefix_factor_20261116_v119.kernel import lift_k3
from research.leading_cover_20261116_v116.kernel import limits,fits
from research.prefix_translation_20261116_v115.kernel import translate
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
from research.xor_cut_20261116_v114b.backend import make_solver,readback
START=time.monotonic();out={'models':[],'original_complete':False,'original_exact_root_count':None,'independent_unsat_certificate':False,'scope':'conditional k3only,fixednecessarynormfamily;otherstrata/outerfamilies/source/Uunresolved'};error=None
try:
    d=json.loads((P/'DATA.json').read_bytes());cfg=json.loads(Path(d['config']).read_bytes());record=json.loads(Path(d['cover']).read_bytes())['records'][1];assert record['k']==3;chart=record['chart'];C,oo,source_roots,defect_rows=compile_all(cfg,chart,d['selected_count']);prior=json.loads(Path(d['profile_receipt']).read_bytes());assert prior['error']is None and prior['out']['selected_count']==d['selected_count'];wanted=prior['out']['variants'][str(d['selected_count'])];raw=json.dumps({'nodes':C.nodes,'outputs':oo,'roots':source_roots},separators=(',',':')).encode();assert hashlib.sha256(raw).hexdigest()==wanted['graph_sha256'];assert len(C.nodes)-2==wanted['variables'];out.update({'inputs':C.n,'variables':len(C.nodes)-2,'cold_compile_seconds':time.monotonic()-START,'soft_total_target_seconds':d['soft_total_seconds'],'graph_sha256':wanted['graph_sha256'],'selected_defect_count':d['selected_count'],'defect_rows':defect_rows})
    sys.path.insert(0,cfg['z3_path']);import z3
    assert z3.get_version_string()==cfg['z3_version'];S=make_solver(z3,d['seed']);out['backend_configuration_readback']=readback(z3);V=[z3.BoolVal(False),z3.BoolVal(True)]+[z3.Bool('v'+str(i-1))for i in range(2,len(C.nodes))]
    for i,node in enumerate(C.nodes[C.n+2:],C.n+2):
        kind,a,*rest=node;value=z3.And(V[a],V[rest[0]])if kind=='and'else z3.Xor(V[a],V[rest[0]])if kind=='xor'else z3.Not(V[a]);S.add(V[i]==value)
    for row in oo:
        for field in row['wires']:
            for wire in field:S.add(z3.Not(V[wire]))
    rows=matrix(cfg['matrix']);oldbits={}
    def original_bit(i):
        if i not in oldbits:
            value=z3.BoolVal(bool(chart['particular']>>i&1))
            for j,column in enumerate(chart['columns']):
                if column>>i&1:value=z3.Xor(value,V[j+2])
            oldbits[i]=value
        return oldbits[i]
    out.update({'formulation_seconds':time.monotonic()-START,'version':z3.get_version_string(),'cpu_affinity':sorted(os.sched_getaffinity(0))})
    for _ in range(d['model_cap']):
        remaining=d['soft_total_seconds']-(time.monotonic()-START)
        if remaining<=0:out['termination']='soft_total_target';error='boundedsearchinconclusive';break
        S.set(timeout=max(1,int(remaining*1000)));answer=S.check();out['last_answer']=str(answer)
        if answer==z3.unknown:out.update({'termination':'unknown','reason_unknown':S.reason_unknown()});error='nativeunknown';break
        if answer==z3.unsat:out.update({'termination':'engine_unsat_claim','engine_unsat_claim':True});error='independentcertificatepending';break
        model=S.model();x=sum(int(z3.is_true(model.eval(V[i+2],model_completion=True)))<<i for i in range(C.n));theta=chart['particular']^apply(chart['columns'],x);assert all(int(z3.is_true(model.eval(original_bit(i),model_completion=True)))==(theta>>i&1)for i in range(76));values=C.evaluate(x);assert not any(values[z]for row in oo for field in row['wires']for z in field);H=decode(rows,theta);assert valid_guard(H,theta>>73);rawH=list(H);rawH[0]^=pmul(theta>>73,cfg['modulus']);assert fits(rawH,limits(18,3))and fits(translate(rawH,1<<17),limits(18,2));factors=lift_k3(rawH,18);assert factors['bounded_factorization']
        aa=reverse(factors['A'][1],17);bb=reverse(factors['A'][0],34);quad=solve(aa,bb,17);assert quad['roots'];rawB=[reverse(a,(3-j)*17)for j,a in enumerate(factors['B'])];cubic=cubic_support(cubic_translate(rawB,1<<17),17);assert cubic['supported'];expected=sorted([r for r,mult in quad['roots']for _ in range(mult)]+[r^(1<<17)for r,mult in cubic['roots']for _ in range(mult)]);recovered=sorted(sum(values[z]<<i for i,z in enumerate(row))for row in source_roots);assert recovered==expected and product(recovered)==rawH
        for s,mod in cfg['fields']:
            ok,_=predicate([rem(a,mod)for a in rawH],Arithmetic(2,[(mod>>j)&1 for j in range(s+1)]));assert ok
        remainder=prefix_remainder(H,83,cfg['field_modulus'],18);assert not remainder;out['models'].append({'encoding':x,'theta':theta,'q':theta>>73,'H':H,'A':factors['A'],'B':factors['B'],'quadratic_roots':quad['roots'],'cubic_roots':cubic['roots'],'allfive_prefix_roots':recovered,'prefix_remainder8':remainder})
        if not remainder:out['termination']='supported_H_requires_original_recovery';error='candidatependingoriginalsource/Urecovery';break
        S.add(z3.Or(*[z3.Not(original_bit(i))if theta>>i&1 else original_bit(i)for i in range(73)]))
    else:out['termination']='model_cap';error='modelcapinconclusive'
    out['statistics']={name:str(S.statistics().get_key_value(name))for name in S.statistics().keys()};out['engine_seconds']=time.monotonic()-START;out['soft_target_overshoot_seconds']=max(0,out['engine_seconds']-d['soft_total_seconds'])
except Exception as e:error=repr(e)
finish(out,error,START,'Changed conditional k3 diagnostic with two necessary early clipping corrections and exact raw product;40s soft total90sstrictouter.','If unknown, exploit proven duplicate defects and test a minimal correction basis with a genuinely different stronger prefix; retain full original recovery obligations.')
