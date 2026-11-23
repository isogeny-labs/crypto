import time
START=time.monotonic()
import json,random,hashlib
from pathlib import Path
from .common import P,finish
from .kernel import context,make_planes,plane_rows,parallel_rank,column_rank
from research.delayed_block_20261117_v137b.kernel import solve
from research.precision_filtration_20261117_v132b.kernel import evaluate_family
from research.residue_quotient_20261116_v99b.kernel import apply
from research.shifted_tangent_20261117_v144b.kernel import packed
out={'rank_oracles':0,'actual_selector_readbacks':0,'complete_early_census':False,'conditional_parameter_upper_bound':None,'global_original_root_count':None,'original_complete':False};error=None
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed']);full=(1<<64)-1;A=[[sum(((code>>(i*3+j))&1)<<code for code in range(64))for j in range(3)]for i in range(2)];a=parallel_rank(A,full);b=column_rank(A,full);assert a==b
 for code in range(64):assert next(r for r,m in enumerate(a)if m>>code&1)==solve([code&7,code>>3],0,3)['rank'];out['rank_oracles']+=1
 full=(1<<256)-1;A=[[rng.getrandbits(256)for _ in range(7)]for _ in range(6)];a=parallel_rank(A,full);b=column_rank(A,full);assert a==b
 for code in range(256):assert next(r for r,m in enumerate(a)if m>>code&1)==solve([sum(((A[i][j]>>code)&1)<<j for j in range(7))for i in range(6)],0,7)['rank'];out['rank_oracles']+=1
 cfg=json.loads(Path(data['cfg']).read_bytes());chart=json.loads(Path(data['cover']).read_bytes())['records'][1]['chart'];base,cols,images,selectors,Ds=context(cfg,chart,int(data['known_unit'],16));r=len(images);assert r==data['full_rank'];N=1<<r;assert (N//8)*(40+7*28+42+35+24+36+14)<data['static_byte_budget'],'memoryadmission';F,full=make_planes(base,images);rows=plane_rows(F,Ds,full)
 for code in [rng.randrange(N)for _ in range(data['scalar_full_checks'])]:
  actual=evaluate_family(base,cols,apply(selectors,code));q=packed(actual,Ds,18,16,23)['correction_masks'];assert [q[k-16][1]for k in range(18,24)]==[sum(((rows[i][j]>>code)&1)<<j for j in range(7))for i in range(6)];out['actual_selector_readbacks']+=1
 assert rows[0][0]==full,'global_unit_plane';nbytes=(N+7)//8;fingerprints=[[hashlib.sha256(v.to_bytes(nbytes,'little')).hexdigest()for v in row]for row in rows];del F
 a=parallel_rank(rows,full);b=column_rank(rows,full);assert a==b,'independent_full_rank_masks';counts={str(i):mask.bit_count()for i,mask in enumerate(a)};assert sum(counts.values())==N and counts['0']==0;upper=sum(v*(1<<(63-r-int(k)))for k,v in counts.items());assert time.monotonic()-START<70,'censuscompletionguard';out.update(complete_early_census=True,all_registered_contexts_complete=True,early_rank=r,early_states=N,rank_distribution=counts,conditional_parameter_upper_bound=upper,development_target_met=upper<=data['upper_target'],independent_rank_masks_identical=True,plane_sha256=fingerprints,intermediate_planes_retained=False,total_seconds=time.monotonic()-START)
except Exception as exc:error=repr(exc)
if error:out['complete_early_census']=False;out['conditional_parameter_upper_bound']=None;out['global_original_root_count']=None
finish(out,error,START,'Repaired complete2^24early-cellcensus with typed original-selector checks and two independent rank algorithms','If complete,derive a singularity-safeK16compiler and conditional compatibility reductions before changed hard solving;the bound counts necessary chart parameters,not original roots or speed. If refuted,freeze a new exact counterexample repair.')
