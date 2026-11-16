from fractions import Fraction
from pathlib import Path
import json
checks=0
for m in [3,5,7,9,11]:
 k=(m+1)//2;assert k*k>m
 for B in [1,2,4,16]:
  for M in [0,1,4,16,64]:
   for S in [0,1,4,16,64]:
    for I in [0,1,4,16,64]:
     if M==S==I==0:continue
     cache=Fraction((5*m*B-3)*M+m*B*S+I,B)
     base=Fraction((5*k*B-3)*M+k*B*S+I,k*B)
     assert cache>=k*base and cache*cache>m*base*base
     # Symbolic difference identity expressed with arbitrary exact weights.
     assert cache-k*base==(m-k)*(5*M+S)
     checks+=1
unbatched_cache=22+5+80;unbatched_base=2+1+80;equal_memory_base=Fraction(12+3+80,3)
assert unbatched_cache**2<5*unbatched_base**2
assert unbatched_cache**2>5*equal_memory_base**2
out={'all_checks_pass':True,'exact_weight_controls':checks,'general_identity':'C_cache-kappa*C_base=(m-kappa)*(5*cM+cS), kappa=(m+1)/2','raw_state_words':{'cached':'2m+2','baseline':4},'P192_state_bytes':{'cached':288,'baseline':96},'equal_walk_count_condition':'cI>3*cM+sqrt(m)*B*(5*cM+cS) for ideal gain under MSI model','equal_storage_ideal_work_ratio_lower_bound':'(m+1)/(2*sqrt(m))','P192_ratio_lower_bound_squared':'9/5','positive_unbatched_control':{'weights':{'M':1,'S':1,'I':80},'cache':107,'baseline':83,'candidate_over_baseline_squared_lt5':True},'equal_storage_control':{'baseline_per_transition':'95/3','candidate_over_baseline_squared_gt5':True},'limits':['Field-operation cost model, not total runtime','Ideal sqrt(m) collision gain assumed generously; actual walks may cycle','Raw state accounting excludes allocator overhead, histories, tables and compression','No lower bound on alternative orbit representations']}
Path('cost-theorem.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
