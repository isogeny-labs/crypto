import time
START=time.monotonic()
import json,random,itertools,math
from pathlib import Path
from .common import P,finish
from research.weighted_frontier_20261117_v143b.kernel import frontier_count,signed_normalizations,solve
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
from research.field_tangent_20261117_v139c.kernel import dot,family,tangent_row,evaluate,correction
from research.joint_translation_20261117_v142c.kernel import solve_subspace
out={'contexts':[],'numeric_calls':0,'nist_unit_predictions':0,'all_registered_contexts_complete':False,'original_complete':False};error=None

def rank(rows,p,n):
 A=[list(v)for v in rows];r=0
 for j in range(n):
  k=next((i for i in range(r,len(A))if A[i][j]%p),None)
  if k is None:continue
  A[r],A[k]=A[k],A[r];iv=pow(A[r][j],-1,p);A[r]=[a*iv%p for a in A[r]]
  for i in range(r+1,len(A)):
   a=A[i][j];A[i]=[(x-a*y)%p for x,y in zip(A[i],A[r])]
  r+=1
 return r

def stabilizer_dimension(f,v,d):
 rows=[]
 for a in v:
  images=[f.mul(a,f.p**j)for j in range(f.n)]
  rows.extend([[x//f.p**k%f.p for x in images]for k in range(d,f.n)])
 return f.n-rank(rows,f.p,f.n)
try:
 data=json.loads(Path('research/weighted_frontier_20261117_v143b/DATA.json').read_bytes());specs=json.loads(Path(data['profile_field_data']).read_bytes())['fields'];old=json.loads(Path(data['certificate_raw']).read_bytes());assert old['error']is None;records={r['name']:r for r in old['out']['contexts']};rng=random.Random(json.loads(Path('research/field_tangent_20261117_v139c/DATA.json').read_bytes())['seed'])
 def value(F,f):
  assert out['numeric_calls']<data['numeric_call_cap']and time.monotonic()-START<70,'profilecallbudget';out['numeric_calls']+=1;return correction(F,f)
 for spec in specs:
  f=Arithmetic(spec['p'],spec['mod']);name=spec.get('name',str(f.p)+'^'+str(f.n));saved=records[name];v=saved['direction'];assert dot(saved['R0'],v,f)==saved['slope']and saved['slope']and all(dot(row,v,f)==0 for row in saved['variations']);rec={'name':name,'scope':'certified coefficient-family example;not original hard all-roots'}
  if f.n==1:
   bits=data['nist_dimensions'].get(name);M=(1<<bits)-1 if bits else min(3,f.p-1);widths=[M+1]*3;options=signed_normalizations(f.p,v)
   for q in options:q['reflected_count']=frontier_count(widths,[abs(a)for a in q['signed']]);q['unsigned_count']=frontier_count(widths,q['unsigned'])
   best=min(options,key=lambda q:(q['reflected_count'],sum(a<0 for a in q['signed']),q['pivot']));w=best['signed'];rec.update(best_direction=w,scale=best['scale'],input_count=(M+1)**3,frontier_count=best['reflected_count'],unsigned_count=best['unsigned_count'],source_bits=bits,log2_candidate_reduction=math.log2((M+1)**3)-math.log2(best['reflected_count']))
   if bits:assert [abs(a)for a in w]==[1,1,1];out['nist_unit_predictions']+=1;large=solve(f.p,[M]*3,w,1,lambda z:0);assert not large['complete']and large['roots']is None
   direction=[a%f.p for a in w];small_upper=[1]*3;small_D=[0,1]
  else:
   pivot=next(i for i,a in enumerate(v)if a);iv=f.inv(v[pivot]);direction=[f.mul(a,iv)for a in v];d=18 if name=='GF2^83'else 1;tdim=stabilizer_dimension(f,direction,d);assert tdim==d;rec.update(normalized_direction=direction,prefix_dimension=d,stabilizer_dimension=tdim,exact_translation_capacity=f.p**tdim,input_count=f.p**(3*d),quotient_count=f.p**(3*d-tdim));assert direction==[1,1,1]or all(a<f.p for a in direction)
   small_D=list(range(f.p));small_upper=None
  base,E,D=family(f,rng);rng.randrange(1,f.q)
  for _ in range(6):
   for _ in range(9):rng.randrange(f.q)
  R0=tangent_row(base,D,f);variations=[]
  for j in range(2):
   y=[0,0];y[j]=1;row=tangent_row(evaluate(base,E,y,f),D,f);variations.append([f.sub(a,b)for a,b in zip(row,R0)])
  assert R0==saved['R0']and variations==saved['variations'],'historical_family_replay_mismatch'
  k=dot(R0,direction,f);assert k and all(dot(row,direction,f)==0 for row in variations);c0=value(base,f);shift=[f.neg(f.mul(a,f.mul(c0,f.inv(k))))for a in direction];F=evaluate(base,D,shift,f);assert value(F,f)==0
  c=lambda z,F=F,D=D,f=f:value(evaluate(F,D,z,f),f);expected=sorted(z for z in itertools.product(small_D,repeat=3)if c(z)==0)
  if f.n==1:got=solve(f.p,small_upper,w,k,c)
  else:
   # Actual normalized examples may have unequal base-field components. Use
   # an exact original-domain fallback for those examples;do not silently
   # apply the all-one quotient to another direction.
   if direction==[1,1,1]:got=solve_subspace(f,3,small_D,k,c)
   else:got={'complete':True,'roots':expected,'route':'independent_direct_fallback_only'}
  assert got['complete']and got['roots']==expected;rec['complete_small_roots']=len(expected);rec['small_route']=got.get('route','proved_frontend');out['contexts'].append(rec)
 out['actual_hard_selector_scope']={'field':'GF2^83','boolean_selector_width':63,'unit_direction_hex':'0x25bc29bd1e36b336','domains':'F2bits embedded inGF2^83','exact_common_translation_capacity':2,'warning':'coefficient prefix18capacity2^18does not transfer to the original63selector chart;existing unit cofactor only removes one bit'};out['all_registered_contexts_complete']=len(out['contexts'])==17;out['total_seconds']=time.monotonic()-START
except Exception as exc:error=repr(exc)
if error:out['all_registered_contexts_complete']=False
finish(out,error,START,'Signed normalization/reflection and exact additive-domain stabilizers on17certified coefficient-family examples','If NIST unit predictions pass,integrate the actual reflected coefficient-family frontend with all original constraints;retain hard-selector capacity2limit. Do not interpret candidate counts as speedups. Complete independent fallback quotient directions before any universal extension claim.')
