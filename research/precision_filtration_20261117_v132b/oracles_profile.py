from pathlib import Path
import json,time,random,itertools,hashlib
from .common import P,finish
from .kernel import filtration,evaluate_family,numeric_correction,actual_family
from research.residue_quotient_20261116_v99b.kernel import apply
from research.field_correction_basis_20261117_v128c.kernel import audited_lift
from research.odd_coprime_factor_20261116_v119c.kernel import initial,inverse_block
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
from research.correction_basis_checks_20261117_v128b.kernel import compile_all
START=time.monotonic();out={'families':0,'rank_oracles':0,'unit_identity_checks':0,'selector_reconstruction_checks':0,'positive_certificates':0,'counterexample_checks':0,'field_contexts':0,'field_direction_checks':0,'actual_witness_checks':0,'records':[],'rank_ladder':[],'original_complete':False};error=None

def rand_family(rng,d,w):
 base=[int(j>=2)|sum(rng.randrange(2)<<k for k in range(1,(5-j)*(d-1)+1))for j in range(6)];columns=[[sum(rng.randrange(2)<<k for k in range(1,(5-j)*(d-1)+1))for j in range(6)]for _ in range(w)];return base,columns
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed'])
 for case in range(24):
  d=2+case%2;w=3+case%3;base,columns=rand_family(rng,d,w)
  if case%2==0:columns[0]=[1<<d,0,0,0,0,0]
  for n in range(d,3*(d-1)+1):
   rec=filtration(columns,n);images={apply(rec['lower_columns'],x)for x in range(1<<w)};aug=[a|(((rec['row']>>i)&1)<<(5*n))for i,a in enumerate(rec['lower_columns'])];assert len(images)==1<<rec['rank']and len({apply(aug,x)for x in range(1<<w)})==1<<rec['augmented_rank'];assert {apply(rec['kernel'],k)for k in range(1<<len(rec['kernel']))}=={x for x in range(1<<w)if apply(rec['lower_columns'],x)==0};out['rank_oracles']+=1
   v=rec['witness']
   if v is not None:
    values={x:numeric_correction(base,columns,x,d,n)for x in range(1<<w)};pivot=(v&-v).bit_length()-1;solved=set();out['positive_certificates']+=1
    for x in range(1<<w):
     assert values[x]^values[x^v]==1;out['unit_identity_checks']+=1
     if not(x>>pivot&1):y=x^(v if values[x]else 0);assert values[y]==0;solved.add(y);out['selector_reconstruction_checks']+=1
    assert solved=={x for x,value in values.items()if value==0}
  out['families']+=1
 base=[0,0,1,1,1,1];columns=[[0,2,0,0,0,0]];rec=filtration(columns,2);assert rec['rank']==rec['augmented_rank']==1 and rec['witness']is None;assert [numeric_correction(base,columns,x,2,2)for x in [0,1]]==[0,1];out['counterexample_checks']+=1
 contexts=json.loads(Path(data['field_data']).read_bytes())['fields']
 for spec in contexts:
  f=Arithmetic(spec['p'],spec['mod']);S,I=inverse_block(f);j=next(j for j,c in enumerate(I[1])if c);slope=I[1][j];base=[[v]+[rng.randrange(f.q)for _ in range(5-k)]for k,v in enumerate([0,0]+initial(f))]
  def value(t):
   F=[list(a)for a in base];F[j][2]=f.add(F[j][2],t);return audited_lift(F,2,f)['audit'][1]['correction'][1]
  for _ in range(4):
   t=rng.randrange(f.q);delta=rng.randrange(f.q);assert f.sub(value(f.add(t,delta)),value(t))==f.mul(slope,delta);out['field_direction_checks']+=1
  out['field_contexts']+=1
 cfg=json.loads(Path(data['config']).read_bytes());chart=json.loads(Path(data['cover']).read_bytes())['records'][1]['chart'];base,columns=actual_family(cfg,chart);out['actual_dimension']=len(columns);out['affine_family_sha256']=hashlib.sha256(json.dumps({'base':base,'columns':columns},separators=(',',':')).encode()).hexdigest()
 for n in data['rank_ladder']:
  rec=filtration(columns,n);out['rank_ladder'].append({'precision':n,'rank':rec['rank'],'kernel_dimension':rec['kernel_dimension']})
 G,oo,rr,meta=compile_all(cfg,chart,8);b=json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr},separators=(',',':')).encode();prior=json.loads(Path(data['graph_receipt']).read_bytes());assert prior['error']is None and hashlib.sha256(b).hexdigest()==prior['out']['variants']['8']['graph_sha256'];out['source_graph_sha256']=hashlib.sha256(b).hexdigest()
 for n in data['precisions']:
  rec=filtration(columns,n);v=rec['witness']
  if v is not None:
   for _ in range(16):
    x=rng.randrange(1<<G.n);a=G.evaluate(x);b=G.evaluate(x^v);wire=oo[-1]['wires'][0][n-18];assert a[wire]^b[wire]==1;out['actual_witness_checks']+=1
  out['records'].append({'precision':n,'rank':rec['rank'],'augmented_rank':rec['augmented_rank'],'kernel_dimension':rec['kernel_dimension'],'witness_hex':None if v is None else hex(v),'current_row_hex':hex(rec['row']),'lower_columns_hex':[hex(c)for c in rec['lower_columns']],'kernel_hex':[hex(k)for k in rec['kernel']]})
 out['total_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'Exact filtration rank/witness oracle,selector reconstruction,counterexample,17field direction controls and frozen actual chart profile.','If actual ranks block this sufficient route,study unit derivatives outside the lower-coefficient kernel with exact cofactor proofs;do not infer absence of all unit directions.')
