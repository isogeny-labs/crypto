import time
START=time.monotonic()
import json,random,math,itertools
from pathlib import Path
from .common import P,finish
from .kernel import packed
from research.precision_filtration_20261117_v132b.kernel import actual_family,filtration,evaluate_family
from research.delayed_block_20261117_v137b.kernel import coefficients,complement,solve,solutions,row_action
from research.residue_quotient_20261116_v99b.kernel import basis,apply
from research.delayed_tangent_20261117_v133b.kernel import jet,bitpolys
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
out={'actual':None,'full_jet_checks':0,'finite_difference_checks':0,'all_registered_contexts_complete':False,'original_complete':False};error=None
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed']);cfg=json.loads(Path(data['cfg']).read_bytes());chart=json.loads(Path(data['cover']).read_bytes())['records'][1]['chart'];base,cols=actual_family(cfg,chart);ell=data['ell'];n=data['last_precision'];Ns=data['rows'];f=Arithmetic(2,[0,1]);K0=filtration(cols,ell)['kernel'];known=int(data['known_unit_direction_hex'],16);assert apply(filtration(cols,ell)['lower_columns'],known)==0;K=[known]
 for v in K0:
  if len(basis(K+[v])[0])>len(K):K.append(v)
 assert len(K)==data['expected_kernel_dimension'];U=complement(K,len(cols));Ds=[coefficients(cols,v)for v in K];early_rank=filtration(cols,n-ell+1)['rank'];assert early_rank==data['expected_early_rank'];records=[];seen=set();samples=[]
 def newrows(F):
  a=packed(F,Ds,18,ell,n);return [a['correction_masks'][k-ell][1]for k in Ns]
 def oldrows(F):
  rows=[0]*len(Ns)
  for j,D in enumerate(Ds):
   q=jet(bitpolys(F),bitpolys(D),18,n,f)
   for i,k in enumerate(Ns):rows[i]|=q[k-1]['tangent'][1]<<j
  return rows
 while len(samples)<data['actual_states']:
  assert time.monotonic()-START<65,'actualprofiletimecap';x=0 if not samples else rng.getrandbits(len(cols));F=evaluate_family(base,cols,x);state=tuple(a&((1<<(n-ell+1))-1)for a in F)
  if state in seen:continue
  seen.add(state);samples.append((x,F,state))
 for i,(x,F,state)in enumerate(samples):
  rows=newrows(F);assert rows[0]&1,'knownunitregression';r=solve(rows,0,len(K));records.append({'sample':i,'selector_hex':hex(x),'early_state_hex':[hex(a)for a in state],'rows':rows,'rank':r['rank'],'free_dimension':len(K)-r['rank'],'left_null':r['left_null']})
  if i<data['general_jet_checks']:assert rows==oldrows(F);out['full_jet_checks']+=1
  if i<data['actual_difference_states']:
   zero=[0]*6;b=jet(bitpolys(F),bitpolys(zero),18,n,f);c=[b[k-1]['correction'][1]for k in Ns]
   for _ in range(data['actual_difference_assignments']):
    bits=rng.randrange(1<<len(K));v=apply(K,bits);G=evaluate_family(base,cols,x^v);a=jet(bitpolys(G),bitpolys(zero),18,n,f);diff=sum((a[k-1]['correction'][1]^c[j])<<j for j,k in enumerate(Ns));assert diff==row_action(rows,bits);out['finite_difference_checks']+=1
 timings=[]
 for i,(x,F,state)in enumerate(samples[:data['timed_states']]):
  t={'new':[],'full':[]}
  for rep in range(data['timing_repetitions']):
   for label,call in ([('new',newrows),('full',oldrows)]if rep%2==0 else[('full',oldrows),('new',newrows)]):
    start=time.perf_counter();rows=call(F);assert rows==records[i]['rows'];json.dumps(rows,separators=(',',':'));t[label].append(time.perf_counter()-start)
  ratio=sorted(t['full'])[1]/sorted(t['new'])[1];timings.append({'sample':i,'component_seconds':t,'full_over_new':ratio})
 distribution={str(r):sum(a['rank']==r for a in records)for r in range(8)};ratios=[a['full_over_new']for a in timings];gain=math.exp(sum(math.log(a)for a in ratios)/len(ratios));fraction=distribution.get('6',0)/len(records);minimum=min(a['rank']for a in records);out['actual']={'scope':'conditional k3necessarynormchartonly;sampled early states,not original complete roots','ell':ell,'last_precision':n,'early_rank':early_rank,'unexhausted_early_cells':(1<<early_rank)-len(records),'kernel_dimension':len(K),'quotient_dimension':len(U),'kernel_hex':[hex(a)for a in K],'Ds_hex':[[hex(a)for a in D]for D in Ds],'records':records,'rank_distribution':distribution,'sample_minrank':minimum,'sample_rank6_fraction':fraction,'kernel_component_geomean':gain,'timings':timings,'development_target_met':gain>=data['kernel_geomean_target']and fraction>=data['rank6_fraction_target']and minimum>=data['sample_minrank_target'],'global_parameter_bound':None,'global_original_root_count':None};out['all_registered_contexts_complete']=True;out['total_seconds']=time.monotonic()-START
except Exception as exc:error=repr(exc)
if error:out['all_registered_contexts_complete']=False
finish(out,error,START,'Actual63selectorK16packed tangent rank/cost profile with full-jet and finite-difference controls','Use observed matrices to design a singularity-safe dynamic block circuit with measured construction cost,retaining every unobserved/rank-deficient cell;no global bound or release ratio from sampled ranks. Refuted cost/rank targets require a changed graph/pruning hypothesis,not an identical hard retry.')
