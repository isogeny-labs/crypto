from pathlib import Path
import json,time,random,hashlib
from .common import P,finish
from .kernel import degree_equations,translate,bounded
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard
from research.frontier_certificate_20261116_v111b.kernel import solve
from research.residue_quotient_20261116_v99b.kernel import pmul,apply
START=time.monotonic();out={'original_complete':False,'recovered_point_controls':0};error=None
try:
    d=json.loads((P/'DATA.json').read_text());cfg=json.loads(Path(d['config']).read_text());raw=Path(cfg['matrix']).read_bytes();assert hashlib.sha256(raw).hexdigest()==cfg['matrix_sha256'];rows=matrix(cfg['matrix']);charts=json.loads(Path(cfg['charts']).read_text())['charts'];old=charts[1]
    assert old['dimension']==67 and(rows[0][4]>>17)&1 and all(v[4]==0 for v in rows[1:])
    directions=rows[1:]+[[cfg['modulus']<<j,0,0,0,0,0]for j in range(3)];eq,labels=degree_equations(rows[0],directions,18);new=solve(old['rows']+eq,76)
    out.update({'old_dimension':67,'degree_equations':len(eq),'equation_labels':labels,'new_feasible':new['feasible'],'new_dimension':new.get('dimension'),'additional_rank':new['rank']-old['rank'],'advancement_rule_met':new['feasible']and new['dimension']<=63,'scope':'necessary raw translated-degree conditions only on w=1;not roots or alloutercharts'})
    if new['feasible']:
        rng=random.Random(d['seed'])
        for _ in range(128):
            theta=new['particular']^apply(new['columns'],rng.randrange(1<<new['dimension']));H=decode(rows,theta);assert valid_guard(H,theta>>73)and(H[1]>>68)&1;H[0]^=pmul(theta>>73,cfg['modulus']);assert bounded(translate(H,1<<17),18);out['recovered_point_controls']+=1
    payload={'chart0_unchanged':charts[0],'old_chart1':old,'translated_chart1':new,'degree_equations':eq,'labels':labels,'field_modulus':cfg['modulus'],'translation':1<<17,'original_complete':False};b=json.dumps(payload,separators=(',',':')).encode()
    with(P/'contracted_chart.json').open('xb')as f:f.write(b)
    out.update({'contracted_chart_sha256':hashlib.sha256(b).hexdigest(),'total_seconds':time.monotonic()-START})
except Exception as e:error=repr(e)
finish(out,error,START,'Actual exact affine rank contraction from conditional common-prefix translation; original quotient lifts retained.','If useful independent rows appear, freeze a changed conditional chart diagnostic;do notclaim wholefamilygain or remove w=0.')
