import json,time,hashlib
from pathlib import Path
from .common import P,finish
from research.residue_quotient_20261116_v99b.kernel import basis,preimage,apply,code,pmul,rem,fmul
from research.residue_quotient_20261116_v99b.problem import build
START=time.monotonic();out={'profiles':[],'original_complete':False,'original_exact_root_count':None};error=None
try:
 d=json.loads((P/'DATA.json').read_text());oldP=Path('research/residue_quotient_20261116_v99b');E,eb,widths,tables,F,fb,last,details=build(oldP);piv,K=basis(E);R=[preimage(1<<i,piv)for i in range(72)];rows=[[int(x,16)for x in s.split()]for s in Path(d['matrix']).read_text().splitlines()[1:]];hc=rows[1:];M=d['modulus'];bits=16
 for index,item in enumerate([None]+d['extra_fields']):
  if item is not None:
   s,m=item;q=1<<s;assert all(any(fmul(a,b,m)==1 for b in range(1,q))for a in range(1,q));cols=[code(v,s,m)for v in hc]+[rem(pmul(1<<j,M),m)for j in range(3)];F=[v^(w<<bits)for v,w in zip(F,cols)];bits+=4*s
  FK=[apply(F,v)for v in K];FR=[apply(F,v)for v in R];kp,_=basis(FK)
  def reduce(v):
   for i in sorted(kp,reverse=True):
    if v>>i&1:v^=kp[i][0]
   return v
  active=len(basis([reduce(v)for v in FR])[0]);assert active==len(basis(F)[0])-len(kp);bytes_lower=(1<<active)*8
  out['profiles'].append({'added_field':item,'observation_bits':bits,'F_rank':len(basis(F)[0]),'kernel_image_rank':len(kp),'active_quotient_bits':active,'dense_uint64_bytes':bytes_lower,'fits_4GiB':bytes_lower<=d['dense_memory_cap_bytes'],'allocated_dense_states':False})
 finish(out,None,START,'Exact added-residueactivequotientrankandmemorypreflight;no infeasible arrays allocated.','Usecompactconstraintcircuits/sparsepartitionedstates ifdensequotients blowup;first8countisnotfullrootsearch.')
except Exception as e:finish(out,repr(e),START,'Residuerankgrowthpreflight.','Preservefailedprofile andcheckexactmap/rankderivationbeforeanydenseallocation.')
