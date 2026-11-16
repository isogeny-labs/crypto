from pathlib import Path
import json,time,hashlib,random
from .common import P,finish
from .kernel import equations,limits,fits
from research.prefix_translation_20261116_v115.kernel import translate
from research.frontier_certificate_20261116_v111b.kernel import solve
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard
from research.residue_quotient_20261116_v99b.kernel import apply,pmul
START=time.monotonic();out={'strata':[],'recovered_controls':0,'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());cfg=json.loads(Path(d['config']).read_text());raw=Path(cfg['matrix']).read_bytes();assert hashlib.sha256(raw).hexdigest()==cfg['matrix_sha256'];rows=matrix(cfg['matrix']);old=json.loads(Path(cfg['charts']).read_text())['charts'];directions=rows[1:]+[[cfg['modulus']<<j,0,0,0,0,0]for j in range(3)];mu=sum(((v[3]>>34)&1)<<i for i,v in enumerate(directions));bu=(rows[0][3]>>34)&1;rng=random.Random(d['seed']);records=[]
    for k in[1,3,5]:
        parent=old[1 if k==5 else 0];constraints=list(parent['rows'])
        if k!=5:constraints.append((mu,bu^(k==3)))
        e0,l0=equations(rows[0],directions,18,k,False);e1,l1=equations(rows[0],directions,18,k,True);chart=solve(constraints+e0+e1,76);record={'k':k,'chart':chart,'degree_equations':e0+e1,'labels':l0+l1,'unshifted_bounds':limits(18,k),'shifted_bounds':limits(18,5-k)};records.append(record)
        result={'k':k,'feasible':chart['feasible'],'dimension':chart.get('dimension'),'additional_rank_vs_old67chart':chart['rank']-parent['rank'],'degree_rows':len(e0)+len(e1)};out['strata'].append(result)
        if chart['feasible']:
            for _ in range(64):
                theta=chart['particular']^apply(chart['columns'],rng.randrange(1<<chart['dimension']));H=decode(rows,theta);assert valid_guard(H,theta>>73);u,v,w=(H[3]>>34)&1,(H[2]>>51)&1,(H[1]>>68)&1;assert {(0,0,0):1,(1,1,0):3,(0,0,1):5}[(u,v,w)]==k;H[0]^=pmul(theta>>73,cfg['modulus']);assert fits(H,limits(18,k))and fits(translate(H,1<<17),limits(18,5-k));out['recovered_controls']+=1
    total=sum(1<<r['chart']['dimension']for r in records if r['chart']['feasible']);payload={'records':records,'old_charts':old,'raw_parameter_bits':76,'original_complete':False};b=json.dumps(payload,separators=(',',':')).encode()
    with(P/'cover.json').open('xb')as f:f.write(b)
    out.update({'affine_parameter_count':total,'old_guarded_hull_count':1<<68,'nominal_count_ratio_old_over_new_numerator':1<<68,'nominal_count_ratio_old_over_new_denominator':total,'advancement_rule_met':all(r['feasible']and r['dimension']<=63 for r in out['strata']if r['k']in[1,3]),'cover_sha256':hashlib.sha256(b).hexdigest(),'total_seconds':time.monotonic()-START})
except Exception as e:error=repr(e)
finish(out,error,START,'Actual exact affine weighted-degree cover retains allthree leading cases and original quotient semantics.','If allstrata contract,profile a common-kernel65inputthree-way encoding with valid-selector/inactive-coordinate guards before newhardsearch.')
