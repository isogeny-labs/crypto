import json,time,collections
from .common import P,finish
from research.affine_dual_cells_20261116_v97.kernel import Cells,parity
from research.affine_dual_cells_20261116_v97.checker import verify
START=time.monotonic();out={'cells':[],'qualified_speedup':None};error=None
try:
 d=json.loads((P/'DATA.json').read_text());c=d['field'];C=Cells(c['n'],c['mod'],c['d'],1);f=C.S.f;trace_mask=sum(f.trace(1<<i)<<i for i in range(C.n));assert trace_mask&((1<<C.d)-1)==1
 out['trace_mask']=trace_mask;dualset=set()
 for qdeg in d['denominator_degrees']:
  for middle in range(1<<(qdeg-1)):
   Q=(1<<qdeg)|(middle<<1)|1
   for r in d['numerator_degrees']:
    assert qdeg>r;g=f.mul(1<<r,f.inv(Q))
    if not parity(trace_mask&g):continue
    v=f.sq(g);dual=0
    for i in range(C.n):dual|=parity(trace_mask&v)<<i;v=C.S.timesT(v)
    assert dual not in dualset;dualset.add(dual);cell=C.cell(dual);assert not cell['empty_cell']and verify(C,cell);assert cell['rank']==C.e+r
    covered=[i for i,w in enumerate(d['queries'])if all(parity(row&w)==rhs for row,rhs in cell['equations'])]
    out['cells'].append({'Q':Q,'numerator_degree':r,'g':g,'dual':dual,'rank':cell['rank'],'dimension':cell['dimension'],'covered_queries':covered})
 assert any(v['Q']==11 and v['numerator_degree']==1 for v in out['cells'])
 out['rank_histogram']=dict(collections.Counter(v['rank']for v in out['cells']));out['union_upper_bound']=sum(1<<v['dimension']for v in out['cells']);out['union_lower_bound']=max(1<<v['dimension']for v in out['cells']);out['query_coverage']=len(set(i for v in out['cells']for i in v['covered_queries']));out['whole_field_size']=1<<C.n
 finish(out,None,START,'Structured rational trace-duals yield verified lower-rank oblique cells;globalunionbounded,not assumed.','If fractionbound remains tiny,do notenumerate a hugecover;seek symbolic commoncell constraints or a differentglobal reduction.')
except Exception as e:finish(out,repr(e),START,'Structureddualprofile.','Preserve counterexample/inconclusivevariant;review no-wrap intersection andtrace assumptions beforeanyrepair.')
