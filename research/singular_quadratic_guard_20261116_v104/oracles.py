import json,time,hashlib
from pathlib import Path
from .common import P,finish
from .kernel import bad_quadratic,power
from research.table_relations_20261116_v103.kernel import features
from research.relation_zero_locus_20261116_v103b.kernel import zero_locus,table_mask
from research.residue_tables_20261116_v102b.kernel import multiplication,allowed
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.residue_quotient_20261116_v99b.kernel import rem
START=time.monotonic();out={'quadratic_controls':0,'fields':[],'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());prior=json.loads(Path(d['relations_raw']).read_text());assert prior['error']is None;rows=matrix(d['matrix'])
 for row in prior['out']['fields']:
  s,m=row['s'],row['m'];q=1<<s;G=multiplication(s,m);t=rem(rows[0][4],m);bad=set()
  for u in range(q):
   for v in range(q):
    split=any(G[x][x]^G[u][x]^v==0 for x in range(q));a,c=G[u][u],G[v][v];assert bad_quadratic(a,c,s,G)==(not split);out['quadratic_controls']+=1
    if not split:bad.add(G[t][c]|(c<<s)|(G[t][a]<<(2*s))|(a<<(3*s)))
  assert len(bad)==q*(q-1)//2
  table=allowed(s,t,G);relations=next(r['relations']for r in row['degrees']if r['degree']==3);truth,_=zero_locus(4*s,features(4*s,3),relations,d['truth_storage_cap_bytes']);positive=table_mask(4*s,table);extra=truth&~positive;badmask=table_mask(4*s,bad)
  if s==5:assert extra==badmask and (truth&~badmask)==positive
  else:assert truth==positive
  out['fields'].append({'s':s,'m':m,'allowed_count':len(table),'irreducible_quadratic_family':len(bad),'cubic_extras':extra.bit_count(),'family_exactly_all_extras':extra==badmask,'guarded_equivalence':(truth&~badmask)==positive if s==5 else truth==positive,'bad_family_sha256':hashlib.sha256(badmask.to_bytes(((1<<(4*s))+7)//8,'little')).hexdigest()})
 finish(out,None,START,'Complete cubic-extra masks equal precisely the squared irreducible-quadratic family at every GF32 residue; independent exhaustive trace tests.','Compile cubic plus narrow singular guard on same76parameters; measure all setup and search costs before any speed claim.')
except Exception as e:finish(out,repr(e),START,'Singular-family finite certificate.','Preserve failure and retain original full predicate; change hypothesis before a new variant.')
