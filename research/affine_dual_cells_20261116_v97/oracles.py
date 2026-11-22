import json,time
from .common import P,finish
from .kernel import Cells,parity
from .checker import verify
from research.twist_auxiliary_20261116_v86.algebra import s3
START=time.monotonic();out={'cells':[],'whole_field_checks':0,'original_pair_checks':0};error=None
try:
 d=json.loads((P/'DATA.json').read_text())
 for n,mod,dim in d['tiny']:
  for B in[1,2]:
   C=Cells(n,mod,dim,B)
   for dual in range(1,1<<n):
    if not parity(dual&B):continue
    r=C.cell(dual);assert verify(C,r);members=[w for w in range(1<<n)if all(parity(dual&v)==0 for v in C.columns(w))];out['whole_field_checks']+=1<<n
    assert len(members)==(0 if r['empty_cell']else 1<<r['dimension'])
    for w in members:
     for x in range(1<<dim):
      for y in range(1<<dim):assert s3(x,y,w,0,B,C.S.f)!=0;out['original_pair_checks']+=1
    out['cells'].append({'n':n,'d':dim,'B':B,'dual':dual,'rank':r['rank'],'size':len(members)})
 finish(out,None,START,'Independently checked affine annihilator cells; exhaustive separator membership and original pair emptiness.','Profile frozenGF83queries with bounded dual choices;largecellcardinalityalone isnotglobalpruning.')
except Exception as e:finish(out,repr(e),START,'Whole-cell oracle.','Preserve failedvariant and find counterexample;newversionrequired for repairs.')
