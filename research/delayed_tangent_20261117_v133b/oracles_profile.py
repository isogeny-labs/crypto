from pathlib import Path
import json,time,random,hashlib
from .common import P,finish
from .kernel import jet,bitpolys,image_representatives,row_solutions
from research.precision_filtration_20261117_v132b.kernel import filtration,evaluate_family,actual_family
from research.residue_quotient_20261116_v99b.kernel import apply
from research.field_correction_basis_20261117_v128c.kernel import audited_lift
from research.odd_coprime_factor_20261116_v119c.kernel import initial
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
from research.correction_basis_checks_20261117_v128b.kernel import compile_all
START=time.monotonic();out={'binary_families':0,'binary_difference_checks':0,'early_truncation_checks':0,'field_contexts':0,'field_difference_checks':0,'boundary_checks':0,'actual':None,'original_complete':False};error=None

def add(F,D,f):return [[f.add(a,b)for a,b in zip(x,y)]for x,y in zip(F,D)]
def difference(F,D,d,n,f):
 a=audited_lift(F,d,f)['audit'][n-1]['correction'][1];b=audited_lift(add(F,D,f),d,f)['audit'][n-1]['correction'][1];return f.sub(b,a)
def randF(rng,d,f):return [[v]+[rng.randrange(f.q)for _ in range((5-j)*(d-1))]for j,v in enumerate([0,0]+initial(f))]
def randD(rng,d,ell,f):return [[0]+[(rng.randrange(f.q)if k>=ell else 0)for k in range(1,(5-j)*(d-1)+1)]for j in range(6)]
def apply_coeff(cols,v):
 return [apply([col[j]for col in cols],v)for j in range(6)]
def difference_padded(F,D,d,n,f):
 length=[(5-j)*(d-1)+1 for j in range(6)];F=[a+[0]*(k-len(a))for a,k in zip(F,length)];D=[a+[0]*(k-len(a))for a,k in zip(D,length)];return difference(F,D,d,n,f)
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed']);f=Arithmetic(2,[0,1])
 for case in range(data['binary_families']):
  d=2+case%3;ell=1+case%(3*(d-1));n=min(3*(d-1),2*ell-1);base=randF(rng,d,f);cols=[randD(rng,d,1,f)for _ in range(3)];D=randD(rng,d,ell,f)
  for x in range(8):
   F=base
   for i,col in enumerate(cols):
    if x>>i&1:F=add(F,col,f)
   z=jet(F,D,d,n,f)[-1]['tangent'][1];assert difference(F,D,d,n,f)==z;out['binary_difference_checks']+=1
   trunc=[a[:n-ell+1]for a in F];assert jet(trunc,D,d,n,f)[-1]['tangent'][1]==z;out['early_truncation_checks']+=1
  out['binary_families']+=1
 for spec in json.loads(Path(data['field_data']).read_bytes())['fields']:
  f=Arithmetic(spec['p'],spec['mod'])
  for _ in range(3):
   F=randF(rng,3,f);D=randD(rng,3,2,f);z=jet(F,D,3,3,f)[-1]['tangent'][1];assert difference(F,D,3,3,f)==z;assert jet([a[:2]for a in F],D,3,3,f)[-1]['tangent'][1]==z;out['field_difference_checks']+=1
  out['field_contexts']+=1
 f=Arithmetic(2,[1,1,1]);F=[[v]+[0]*(5-j)for j,v in enumerate([0,0]+initial(f))];D=[[0]*(6-j)for j in range(6)];D[1][1]=1;assert difference(F,D,2,2,f)==1 and jet(F,D,2,2,f)[-1]['tangent'][1]==0;out['boundary_checks']+=1
 cfg=json.loads(Path(data['config']).read_bytes());chart=json.loads(Path(data['cover']).read_bytes())['records'][1]['chart'];base,cols=actual_family(cfg,chart);ell,n=data['ell'],data['precision'];assert n<2*ell;K=filtration(cols,ell)['kernel'];early=filtration(cols,n-ell+1)['lower_columns'];reps=image_representatives(early,data['max_early_rank']);f=Arithmetic(2,[0,1]);rows=[]
 for x in reps:
  F=bitpolys(evaluate_family(base,cols,x));row=0
  for j,v in enumerate(K):
   D=bitpolys(apply_coeff(cols,v));assert all(not(a&((1<<ell)-1))for a in apply_coeff(cols,v));z=jet(F,D,18,n,f)[-1]['tangent'][1];row|=z<<j
  rows.append(row)
 answers=row_solutions(rows,len(K));units=[apply(K,v)for v in answers];out['actual']={'ell':ell,'precision':n,'kernel_dimension':len(K),'kernel_hex':[hex(v)for v in K],'early_rank':(len(reps)).bit_length()-1,'early_representatives_hex':[hex(v)for v in reps],'tangent_rows':rows,'unit_combinations':answers,'unit_directions_hex':[hex(v)for v in units],'finite_difference_checks':0,'circuit_checks':0,'selector_reconstruction_checks':0}
 for x in reps:
  F=bitpolys(evaluate_family(base,cols,x))
  for mask in range(1,1<<len(K)):
   v=apply(K,mask);D=bitpolys(apply_coeff(cols,v));z=jet(F,D,18,n,f)[-1]['tangent'][1];assert difference_padded(F,D,18,n,f)==z;assert z==((rows[reps.index(x)]&mask).bit_count()%2);out['actual']['finite_difference_checks']+=1
 G,oo,rr,meta=compile_all(cfg,chart,8);wire=oo[-1]['wires'][0][0];blob=json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr},separators=(',',':')).encode();out['actual']['graph_sha256']=hashlib.sha256(blob).hexdigest();prior=json.loads(Path(data['graph_receipt']).read_bytes());assert out['actual']['graph_sha256']==prior['out']['variants']['8']['graph_sha256']
 for v in units:
  pivot=(v&-v).bit_length()-1
  for _ in range(data['random_circuit_checks']):
   x=rng.randrange(1<<63);a=G.evaluate(x)[wire];b=G.evaluate(x^v)[wire];assert a^b==1;out['actual']['circuit_checks']+=1;x&=~(1<<pivot);a=G.evaluate(x)[wire];y=x^(v if a else 0);assert G.evaluate(y)[wire]==0;out['actual']['selector_reconstruction_checks']+=1
 out['total_seconds']=time.monotonic()-START
except Exception as exc:error=repr(exc)
finish(out,error,START,'Delayed tangent proof controls and bounded actual unit-direction certificate','Integrate a certified unit with exact nonlinear selector/q readback and full-set tiny oracles; otherwise derive broader early-state tangent rows rather than repeat a failed hard search.')
