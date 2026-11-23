import time
START=time.monotonic()
import json,random,hashlib
from pathlib import Path
from .common import P,finish
from research.typed_cell_planes_20261117_v145b.kernel import context,make_planes,plane_rows,parallel_rank
from research.precision_filtration_20261117_v132b.kernel import filtration
from research.residue_quotient_20261116_v99b.kernel import basis,apply,preimage

class NodeCap(Exception):pass

def make_bdd(mask,n,cap):
 nodes=[None,None];unique={};memo={}
 def rec(m,k):
  if not m:return 0
  if m.bit_count()==1<<k:return 1
  key=(m,k)
  if key in memo:return memo[key]
  half=1<<(k-1);low=rec(m&((1<<half)-1),k-1);high=rec(m>>half,k-1)
  if low==high:out=low
  else:
   triple=(k-1,low,high)
   if triple not in unique:
    if len(nodes)>=cap:raise NodeCap('bdd_nodecap')
    unique[triple]=len(nodes);nodes.append(triple)
   out=unique[triple]
  memo[key]=out;return out
 root=rec(mask,n);return nodes,root

def bdd_mask(nodes,root,n):
 memo={}
 def rec(v,k):
  if v==0:return 0
  if v==1:return (1<<(1<<k))-1
  key=(v,k)
  if key in memo:return memo[key]
  j,lo,hi=nodes[v];a=rec(lo,j)|(rec(hi,j)<<(1<<j));width=1<<(j+1)
  while width<1<<k:a|=a<<width;width*=2
  memo[key]=a;return a
 return rec(root,n)

def eval_bdd(nodes,root,state):
 while root>1:
  j,lo,hi=nodes[root];root=hi if state>>j&1 else lo
 return root

out={'strata':[],'bdd_oracles':0,'selector_checks':0,'complete_sparse_guards':False,'all_registered_contexts_complete':False,'original_complete':False};error=None
try:
 data=json.loads((P/'DATA.json').read_bytes());prior=json.loads(Path(data['prior_census']).read_bytes());assert prior['error']is None and prior['out']['complete_early_census'];rng=random.Random(data['seed'])
 for n in range(1,7):
  for _ in range(16):
   mask=rng.getrandbits(1<<n);nodes,root=make_bdd(mask,n,data['node_cap']);assert bdd_mask(nodes,root,n)==mask;assert all(eval_bdd(nodes,root,s)==(mask>>s&1)for s in range(1<<n));out['bdd_oracles']+=1
 cfg=json.loads(Path('research/chart_compiler_20261116_v112/DATA.json').read_bytes());chart=json.loads(Path('research/leading_cover_20261116_v116/cover.json').read_bytes())['records'][1]['chart'];base,cols,images,selectors,Ds=context(cfg,chart,int('25bc29bd1e36b336',16));F,full=make_planes(base,images);rows=plane_rows(F,Ds,full);del F;masks=parallel_rank(rows,full);assert {str(i):m.bit_count()for i,m in enumerate(masks)}==prior['out']['rank_distribution'];del rows
 piv,_=basis(images);E=filtration(cols,8)['lower_columns'];coordinate_columns=[preimage(v,piv)for v in E];coordinate_masks=[sum(((v>>k)&1)<<j for j,v in enumerate(coordinate_columns))for k in range(24)]
 for rank in data['rank_strata']:
  mask=masks[rank];nodes,root=make_bdd(mask,24,data['node_cap']);assert bdd_mask(nodes,root,24)==mask,'full_guard_equivalence';out['strata'].append({'rank':rank,'states':mask.bit_count(),'nodes':nodes,'root':root,'nonterminal_nodes':len(nodes)-2,'truth_sha256':hashlib.sha256(mask.to_bytes((1<<24)//8,'little')).hexdigest(),'parameter_upper_bound':mask.bit_count()*(1<<(39-rank))})
 for _ in range(data['scalar_checks']):
  x=rng.getrandbits(63);s=preimage(apply(E,x),piv);bits=sum(((x&v).bit_count()%2)<<j for j,v in enumerate(coordinate_masks));assert s==bits
  for record in out['strata']:assert eval_bdd(record['nodes'],record['root'],s)==(masks[record['rank']]>>s&1)
  out['selector_checks']+=1
 out.update(complete_sparse_guards=True,all_registered_contexts_complete=True,early_coordinate_selector_masks_hex=[hex(v)for v in coordinate_masks],total_seconds=time.monotonic()-START)
except Exception as exc:error=repr(exc)
if error:out['complete_sparse_guards']=False;out['all_registered_contexts_complete']=False
finish(out,error,START,'Exact bounded BDDguards for rare rank2/rank3earlycells with full truth-plane equivalence and original-selector readback','If compact,compile rank-stratified normal/exceptionalK16branches with every singular free/compatibility condition and full original recovery. If nodecaprefutescompactness,derive a different variable order or algebraic cell predicate in a new variant;no blind retry.')
