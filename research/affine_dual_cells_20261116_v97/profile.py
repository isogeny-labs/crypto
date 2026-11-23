import json,time,collections
from .common import P,finish
from .kernel import Cells,parity
from .checker import verify
START=time.monotonic();out={'queries':[],'qualified_speedup':None};error=None
try:
 d=json.loads((P/'DATA.json').read_text());c=d['actual'];C=Cells(c['n'],c['mod'],c['d'],c['B'])
 for w in c['queries']:
  cells=[]
  for dual in C.duals(w,c['dual_cap']):
   r=C.cell(dual);assert not r['empty_cell'];assert all(parity(a&w)==b for a,b in r['equations']);assert verify(C,r)
   cells.append({'dual':dual,'rank':r['rank'],'dimension':r['dimension'],'unit_direction':all(parity(a)==0 for a,b in r['equations'])})
  out['queries'].append({'w':w,'cells':cells})
 out['rank_histogram']=dict(collections.Counter(r['rank']for q in out['queries']for r in q['cells']));out['union_upper_bound']=sum(1<<r['dimension']for q in out['queries']for r in q['cells']);out['covered_query_count']=sum(bool(q['cells'])for q in out['queries']);assert not any(r['unit_direction']for q in out['queries']for r in q['cells'])
 finish(out,None,START,'Frozen query-derived obliqueGF83cells,independent rank/cell certificate checks;unionboundonly.','If cellfractionsremain tiny,seekstructured lowrankduals ratherthanenumeratingallqueries ordualchoices.')
except Exception as e:finish(out,repr(e),START,'Query-cell profile.','Preserveinconclusivework;reduce verificationcost or change dualfamily under a newimmutableplan.')
