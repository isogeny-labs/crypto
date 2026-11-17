import time
START=time.monotonic()
from pathlib import Path
import json,random,itertools,hashlib
from collections import Counter
from .common import P,finish
from .kernel import solve,solutions,complement,coefficients,rhsbits,row_action
from research.residue_quotient_20261116_v99b.kernel import apply,basis,preimage
from research.delayed_tangent_20261117_v133b.kernel import jet,bitpolys,image_representatives
from research.precision_filtration_20261117_v132b.kernel import filtration,evaluate_family,actual_family
from research.field_correction_basis_20261117_v128c.kernel import audited_lift
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
from research.prefix_translation_20261116_v115.kernel import product
from research.simple_prefix_lift_20261116_v117b.kernel import reverse
out={'matrix_rhs_oracles':0,'complement_inverse_checks':0,'tiny_families':[],'root_bag_oracle_size':0,'actual':None,'original_complete':False};error=None

def numeric(F,d,f,ns):
 z=audited_lift(bitpolys(F),d,f)['audit'];return [z[n-1]['correction'][1]for n in ns]
def tangent_rows(F,cols,K,d,ns,f):
 rows=[0]*len(ns)
 for j,v in enumerate(K):
  z=jet(bitpolys(F),bitpolys(coefficients(cols,v)),d,max(ns),f)
  for i,n in enumerate(ns):rows[i]|=z[n-1]['tangent'][1]<<j
 return rows
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed']);f=Arithmetic(2,[0,1])
 for code in range(512):
  rows=[code>>(3*i)&7 for i in range(3)]
  for rhs in range(8):
   rec=solve(rows,rhs,3);expected={x for x in range(8)if row_action(rows,x)==rhs};assert solutions(rec)==expected;assert len(rec['kernel'])==3-rec['rank'];assert all(apply(rows,v)==0 for v in rec['left_null']);assert (rec['particular']is not None)==all((v&rhs).bit_count()%2==0 for v in rec['left_null']);out['matrix_rhs_oracles']+=1
 for _ in range(12):
  K=[]
  while len(K)<3:
   v=rng.randrange(1,64)
   if len(basis(K+[v])[0])>len(K):K.append(v)
  U=complement(K,6);T=U+K;piv,_=basis(T)
  for x in range(64):assert apply(T,preimage(x,piv))==x;out['complement_inverse_checks']+=1
 bags={}
 for aa in itertools.combinations_with_replacement(range(4),2):
  for bb in itertools.combinations_with_replacement(range(4,8),3):
   xs=aa+bb;H=tuple(product(xs));assert H not in bags;bags[H]=xs
 assert len(bags)==200;out['root_bag_oracle_size']=len(bags)
 for case in range(data['tiny_families']):
  base=[0,0,1,1,1,1];cols=[[0]*6 for _ in range(5)];cols[0][0]=2;cols[0][4]=2;cols[1][1]=2;cols[1][3]=2;cols[2][0]=8;cols[3][0]=16;cols[4][1]=8
  if case:
   for i in range(2):
    for j in range(3):cols[i][j]^=sum(rng.randrange(2)<<k for k in range(2,(5-j)*2+1))
  K=[4,8,16];ns=[3,4,5];direct=set();direct_supported=set()
  for theta in range(32):
   F=evaluate_family(base,cols,theta);c=numeric(F,3,f,ns)
   if not any(c):
    direct.add(theta);raw=tuple(reverse(a,(5-j)*2)for j,a in enumerate(F))
    if raw in bags:direct_supported.add((theta,bags[raw]))
  rebuilt=set();supported=set()
  for y in range(4):
   F=evaluate_family(base,cols,y);b=rhsbits(numeric(F,3,f,ns));rows=tangent_rows(F,cols,K,3,ns,f);rec=solve(rows,b,3)
   for z in range(8):assert rhsbits(numeric(evaluate_family(base,cols,y^apply(K,z)),3,f,ns))==b^row_action(rows,z)
   for z in solutions(rec):
    theta=y^apply(K,z);rebuilt.add(theta);F=evaluate_family(base,cols,theta);raw=tuple(reverse(a,(5-j)*2)for j,a in enumerate(F))
    if raw in bags:supported.add((theta,bags[raw]))
  assert rebuilt==direct and supported==direct_supported and supported;out['tiny_families'].append({'all_assignments':32,'exact_zero_correction_assignments':len(direct),'exact_supported_component_tuples':len(supported),'supported_tuple_sha256':hashlib.sha256(json.dumps(sorted(supported),separators=(',',':')).encode()).hexdigest()})
 cfg=json.loads(Path(data['config']).read_bytes());chart=json.loads(Path(data['cover']).read_bytes())['records'][1]['chart'];base,cols=actual_family(cfg,chart);ell=data['ell'];ns=data['precisions'];assert max(ns)<2*ell;K=filtration(cols,ell)['kernel'];cert=json.loads(Path(data['certificate']).read_bytes());assert cert['error']is None and [hex(v)for v in K]==cert['out']['actual']['kernel_hex'];assert len(K)==data['max_kernel_dimension'];early_rec=filtration(cols,max(ns)-ell+1);assert early_rec['rank']<=data['max_early_rank'];early=early_rec['lower_columns'];reps=image_representatives(early,data['max_early_rank']);assert len(reps)<=data['max_states'];assert len({apply(early,x)for x in reps})==len(reps);U=complement(K,len(cols));assert len(U)==60;assert len(basis([apply(early,u)for u in U])[0])==early_rec['rank'];out['actual']={'complete':False,'ell':ell,'precisions':ns,'kernel_hex':[hex(v)for v in K],'quotient_basis_hex':[hex(u)for u in U],'early_rank':early_rec['rank'],'states':len(reps),'records':[],'conditional_parameter_upper_bound':None,'finite_difference_checks':0};hist=Counter();pivhist=Counter()
 for x in reps:
  if time.monotonic()-START>=72:raise RuntimeError('boundedranktableincomplete')
  F=evaluate_family(base,cols,x);low=[a&15 for a in F];rows=tangent_rows(low,cols,K,18,ns,f);rec=solve(rows,0,3);assert rec['rank']>=1;hist[rec['rank']]+=1;pivhist[str(rec['pivots'])]+=1;out['actual']['records'].append({'early_representative_hex':hex(x),'signature_hex':hex(apply(early,x)),'rows':rows,'rank':rec['rank'],'pivots':rec['pivots'],'free_delayed':3-rec['rank'],'left_null':rec['left_null']})
 for idx in rng.sample(range(len(reps)),data['actual_finite_difference_states']):
  x=reps[idx];F=evaluate_family(base,cols,x);b=rhsbits(numeric(F,18,f,ns));rows=out['actual']['records'][idx]['rows']
  for z in range(8):assert rhsbits(numeric(evaluate_family(base,cols,x^apply(K,z)),18,f,ns))==b^row_action(rows,z);out['actual']['finite_difference_checks']+=1
 bound=sum(1<<(55-r['rank'])for r in out['actual']['records']);assert bound<=1<<62;out['actual'].update({'complete':True,'rank_distribution':dict(hist),'pivot_distribution':dict(pivhist),'conditional_parameter_upper_bound':bound,'development_parameter_bound':data['development_parameter_bound'],'development_target_met':bound<=data['development_parameter_bound'],'matrix_table_sha256':hashlib.sha256(json.dumps(out['actual']['records'],separators=(',',':')).encode()).hexdigest()});out['total_seconds']=time.monotonic()-START
except Exception as exc:error=repr(exc)
finish(out,error,START,'Exact delayed3direction/3correction rank table with singular compatibility and complete small oracles','If target met,design a singularity-safe block compiler with full original theta/q readback;otherwise derive earlier-precision directions or a higher-correction compatibility certificate before any unchanged hard retry.')
