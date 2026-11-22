import json,time,hashlib
from pathlib import Path
from .common import P,finish
from .kernel import build_code,Widths,recover,accepted
from research.residue_quotient_20261116_v99b.problem import build
from research.residue_quotient_20261116_v99b.kernel import code,rem,apply
from research.residue_tables_20261116_v102b.kernel import multiplication,allowed
from research.guarded_local_search_20261116_v101.kernel import matrix
START=time.monotonic();out={'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());config=json.loads(Path(d['config']).read_text());rows=matrix(config['matrix'])
    E,eb,widths,first,F,fb,last,details=build(Path(d['old_problem']));tables=first+[last]
    for s,m in config['fields'][8:]:tables.append(allowed(s,rem(rows[0][4],m),multiplication(s,m)))
    hashes=[hashlib.sha256(json.dumps(sorted(t)).encode()).hexdigest()for t in tables]
    prior=json.loads(Path(d['table_receipt']).read_text())['out'];assert hashes==prior['field_table_hashes']
    directions=rows[1:]+[[config['modulus']<<j,0,0,0,0,0]for j in range(3)];observations=[0]*76;affine=0;offset=0;widths=[]
    for s,m in config['fields']:
        for j,v in enumerate(directions):observations[j]^=code(v,s,m)<<offset
        affine^=code(rows[0],s,m)<<offset;widths.append(4*s);offset+=4*s
    result=build_code(observations,affine,widths,tables,76);W=Widths(result['blocks'],d['rank_cap']);profiles=W.profiles()
    peak=profiles['greedy']['peak_width'];natural=profiles['natural']['peak_width']
    out.update({'observation_rank':result['observation_rank'],'observation_kernel_bits':len(result['observation_kernel']),'ambient_bits':result['ambient_bits'],'quotient_bits':result['quotient_bits'],'syndrome_rank':result['syndrome_rank'],'hull_dimensions':[len(h['columns'])for h in result['hulls']],'table_counts':list(map(len,tables)),'profiles':profiles,'affine_feasible':result['affine_feasible'],'advancement_rule_met':peak<=20 and natural-peak>=6,'dense_states_at_greedy_peak':1<<peak,'no_frontier_allocated':True,'worker_seconds':time.monotonic()-START})
    for name,v in [('code',result),('observations',{'columns':observations,'affine':affine})]:
        with(P/(name+'.json')).open('x')as f:json.dump(v,f,separators=(',',':'))
except Exception as e:error=repr(e)
finish(out,error,START,'Actual affine-hull syndrome-code frontier rank preflight; finite-domain state counts are not inferred from affine widths.','If all trial orderings exceed20bits, certify a balanced-cut lower bound over every partition before retiring this dense frontier representation.')
