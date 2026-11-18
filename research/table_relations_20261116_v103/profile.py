import json,time,hashlib
from .common import P,finish
from .kernel import derive
from research.residue_tables_20261116_v102b.kernel import multiplication,allowed
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.residue_quotient_20261116_v99b.kernel import rem
START=time.monotonic();out={'fields':[],'original_complete':False,'original_exact_root_count':None};error=None
try:
 d=json.loads((P/'DATA.json').read_text());rows=matrix(d['matrix'])
 for s,m in d['fields']:
  table=sorted(allowed(s,rem(rows[0][4],m),multiplication(s,m)));record={'s':s,'m':m,'allowed':len(table),'degrees':[]}
  for degree in d['degrees']:
   begin=time.monotonic();masks,basis,rank,ev=derive(table,4*s,degree,d['feature_cap']);record['degrees'].append({'degree':degree,'features':len(masks),'rank':rank,'nullity':len(basis),'relations':basis,'relation_terms':sum(v.bit_count()for v in basis),'seconds':time.monotonic()-begin,'all_positive_rows_checked':True})
  out['fields'].append(record)
 finish(out,None,START,'Exact degree2/3 vanishing-space ranks and verified relation witnesses across all fourteen fixed-trace residue tables.','If degree-five residues have no useful relations, change mechanism; if positive, measure exact cut strength and shared-parameter substitution before native search.')
except Exception as e:finish(out,repr(e),START,'Local vanishing-space profile.','Preserve partial profile; use changed algebraic hypothesis rather than identical failed execution.')
