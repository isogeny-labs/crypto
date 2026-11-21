import time
START=time.monotonic()
from pathlib import Path
import json,random,itertools
from .common import P,finish
from .kernel import dot,add_scaled,evaluate,kernel,certificate,family,tangent_row,correction,reconstruct
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
from research.delayed_tangent_20261117_v133b.kernel import jet
from research.odd_coprime_factor_20261116_v119c.kernel import initial
out={'contexts':[],'numeric_calls':0,'boundary_checks':0,'all_registered_contexts_complete':False,'original_complete':False};error=None
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed']);specs=json.loads(Path(data['field_data']).read_bytes())['fields']
 def value(F,f):
  assert out['numeric_calls']<data['numeric_call_cap'],'numericcallcap';assert time.monotonic()-START<72,'workerinternalbudget';out['numeric_calls']+=1;return correction(F,f)
 for spec in specs:
  f=Arithmetic(spec['p'],spec['mod']);base,E,D=family(f,rng);R0=tangent_row(base,D,f);variations=[]
  for i in range(2):
   y=[0,0];y[i]=1;r=tangent_row(evaluate(base,E,y,f),D,f);variations.append([f.sub(a,b)for a,b in zip(r,R0)])
  cert=certificate(R0,variations,f);assert cert is not None;v,slope=cert;scale=rng.randrange(1,f.q);v=[f.mul(a,scale)for a in v];slope=dot(R0,v,f);assert slope and all(dot(row,v,f)==0 for row in variations);assert certificate([0,0,0],variations,f)is None;pivot=next(i for i,a in enumerate(v)if a);rec={'name':spec.get('name',str(spec['p'])+'^'+str(len(spec['mod'])-1)),'q_bits':f.q.bit_length(),'R0':R0,'variations':variations,'direction':v,'slope':slope,'pivot':pivot,'row_checks':0,'finite_difference_checks':0,'line_checks':0,'reconstruction_checks':0,'complete_small_fibers':0,'small_zero_counts':[],'zero_slope_rejection':True}
  for _ in range(data['samples_per_field']):
   y=[rng.randrange(f.q)for _ in range(2)];F=evaluate(base,E,y,f);r=[f.add(R0[j],dot([row[j]for row in variations],y,f))for j in range(3)];assert tangent_row(F,D,f)==r;rec['row_checks']+=1;c=value(F,f)
   for j in range(3):assert f.sub(value(add_scaled(F,D[j],1,f),f),c)==r[j];rec['finite_difference_checks']+=1
   z=[rng.randrange(f.q)for _ in range(3)];t=rng.randrange(f.q);z2=[f.add(a,f.mul(t,b))for a,b in zip(z,v)];assert f.sub(value(evaluate(F,D,z2,f),f),value(evaluate(F,D,z,f),f))==f.mul(t,slope);rec['line_checks']+=1;x=[rng.randrange(f.q)for _ in range(3)];x[pivot]=0;cx=value(evaluate(F,D,x,f),f);root=reconstruct(x,v,cx,slope,f);assert value(evaluate(F,D,root,f),f)==0;back=[f.sub(a,f.mul(b,f.mul(root[pivot],f.inv(v[pivot]))))for a,b in zip(root,v)];assert back==x;rec['reconstruction_checks']+=1
  early=list(itertools.product(range(f.q),repeat=2))if f.q<=data['full_early_fiber_q_max']else[(0,0),(1,1)]if f.q<=data['fixed_early_fiber_q_max']else[]
  for y in early:
   F=evaluate(base,E,y,f);values={z:value(evaluate(F,D,z,f),f)for z in itertools.product(range(f.q),repeat=3)};direct={z for z,c in values.items()if c==0};rebuilt=set()
   for x,c in values.items():
    if x[pivot]==0:rebuilt.add(tuple(reconstruct(x,v,c,slope,f)))
   assert rebuilt==direct and len(direct)==f.q**2;rec['complete_small_fibers']+=1;rec['small_zero_counts'].append(len(direct))
  out['contexts'].append(rec)
 f=Arithmetic(2,[1,1,1]);F=[[a]+[0]*(5-j)for j,a in enumerate([0,0]+initial(f))];D=[[0]*len(a)for a in F];D[1][1]=1;# precision2separate general oracle
 from research.field_correction_basis_20261117_v128c.kernel import audited_lift
 diff=f.sub(audited_lift(add_scaled(F,D,1,f),2,f)['audit'][1]['correction'][1],audited_lift(F,2,f)['audit'][1]['correction'][1]);assert diff==1 and jet(F,D,2,2,f)[-1]['tangent'][1]==0;out['boundary_checks']+=1;out['all_registered_contexts_complete']=len(out['contexts'])==len(specs);out['total_seconds']=time.monotonic()-START
except Exception as exc:error=repr(exc)
if error:out['all_registered_contexts_complete']=False
finish(out,error,START,'Symbolic r+1tangent interpolation and exact field-line reconstruction across17coefficient fields','If passed,integrate the symbolic tangent certificate into an odd-characteristic coefficient-family presolver with explicit prime-domain pullbacks;otherwise preserve the counterexample and repair the exact failed assumption in a new variant.')
