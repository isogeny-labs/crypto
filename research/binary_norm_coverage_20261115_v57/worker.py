from pathlib import Path
from itertools import product
from collections import Counter
import hashlib,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.original_chain_lift_20261115_v55.solver import Lift
from research.original_chain_lift_20261115_v55.worker import literal,eq
from research.general_torsion_kernel_20261115_v56.worker import kernel
from research.fused_affine_norm_20261115_v56.worker import independent_norm
from research.positive_binary_charts_20261115_v53 import math as curve
from .oracle import points,hermite_norm
P=Path(__file__).resolve().parent;START=time.monotonic()
def trace(f,x):
 out=0
 for _ in range(f.degree):out^=x;x=f.square(x)
 assert out in[0,1];return out

def guards(f,A,B,d,z=None):
 beta,w,cols,sol=kernel(f,d,B);bound=4*(d-1)<f.degree
 noO=bound and(B==1 or B.bit_length()-1>4*(d-1))
 noP0=len(sol['basis'])==0
 if B==1 and f.degree==83 and d==18:
  expected=[(1<<(2*i))|(((1<<(2*i))^(1<<i))<<w)for i in range(9)];assert len(sol['basis'])==9
  from research.norm_block_reuse_20261115_v36.backend import apply
  assert all(apply(cols,v)==0 for v in expected)
  for tag in range(512):
   v=apply(expected,tag);D=v&((1<<w)-1);S=v>>w;assert D==0 or(D==1 and S!=1)or(D>1 and D.bit_length()==S.bit_length()and D!=S)
  noP0=True
 pair=(2*(d-1)<f.degree and beta.bit_length()-1>2*(d-1))or(beta==1 and trace(f,A)==0)
 record={'beta':beta,'O_triple_excluded':noO,'P0_triple_excluded':noP0,'P0_pair_excluded':pair,'torsion_kernel_dimension':len(sol['basis'])}
 if z is not None:record.update(target_outside=z>=1<<d,translated_target_x=f.mul(beta,f.inv(z)),translated_target_outside=f.mul(beta,f.inv(z))>=1<<d)
 return record

def tiny(n,mod,A,B,d):
 f=BinaryField(n,mod);e,allpoints=points(f,A,B,d);q=f.order;beta=B
 for _ in range(n-1):beta=f.square(beta)
 source=list(range(1<<d));nonzero=source[1:]
 for a,b,c in product(nonzero,repeat=3):assert eq(a,b,c,B,mod)!=0 and eq(a,b,f.mul(beta,f.inv(c)),B,mod)!=0
 assert not any(f.mul(a,b)==beta for a,b in product(nonzero,repeat=2)if any(p[0]==a and not e.real(p[1])for p in allpoints)and any(p[0]==b and not e.real(p[1])for p in allpoints))
 targets=[]
 for z in range(1<<d,q):
  if f.mul(beta,f.inv(z))<1<<d:continue
  if any(literal(y,y,mod)^literal(z,y,mod)==literal(literal(z,z,mod),z,mod)^literal(A,literal(z,z,mod),mod)^B for y in range(q)):targets.append(z)
 truth={z:set()for z in targets};groups=0;witness={};ghost_hist=Counter();byx={x:[p for p in allpoints if p[0]==x]for x in source}
 for xs in product(source,repeat=5):
  for Ps in product(*(byx[x]for x in xs)):
   total=None;aux=[];valid=True
   for i,point in enumerate(Ps):
    total=curve.add(e,A,B,total,point)
    if i in[1,2,3]:
     if total is None or not e.real(total[0]):valid=False;break
     aux.append(total[0])
   if not valid or total is None or not e.real(total[0])or not e.real(total[1])or total[0]not in truth:continue
   z,y=total;values=xs+tuple(aux);truth[z].add(values);groups+=1;ghost_hist[sum(not e.real(p[1])for p in Ps)]+=1
   bag=tuple(sorted(xs));key=(bag,z)
   if key not in witness:
    Q=(z,y^z);C=Counter(Ps+(Q,));assert all(C[(x,yy)]==C[(x,e.conjugate(yy))]for x,yy in C)
    V=hermite_norm(f,e,A,B,list(Ps),Q);r,s,t,h,k=V;assert t==f.mul(f.square(z),z)^f.mul(r,f.square(z))^f.mul(s,z)^f.mul(f.mul(h,z)^k,Q[1]);H=independent_norm(f,A,B,z,Q[1],h,k,r,s);expected=[1]
    for x in xs:
     out=[0]*(len(expected)+1)
     for j,c in enumerate(expected):out[j]^=literal(c,x,mod);out[j+1]^=c
     expected=out
    assert H==tuple(expected);witness[key]={'bag':bag,'target':Q,'function':V,'H':H}
  assert time.monotonic()-START<62
 L=Lift(f,B);count=0
 for xs in product(source,repeat=5):
  for z in targets:
   got=set(L.ordered(xs,z));want={v for v in truth[z]if v[:5]==xs};assert got==want,(n,A,B,xs,z);count+=len(got)
  assert time.monotonic()-START<68
 return {'n':n,'modulus':mod,'A':A,'B':B,'d':d,'target_coordinates':len(targets),'ordered_sources':len(source)**5,'point_assignment_paths':groups,'complete_original_tuples':count,'original_set_sha256':hashlib.sha256(json.dumps({z:sorted(v)for z,v in truth.items()},sort_keys=True).encode()).hexdigest(),'ghost_histogram':dict(ghost_hist),'hermite_norm_witnesses':len(witness),'witnesses':list(witness.values())}

def main():
 out={'tiny':[],'actual':[]};error=None
 try:
  data=json.loads((P/'DATA.json').read_text())
  for args in data['tiny']:
   record=tiny(*args);raw=json.dumps(record,sort_keys=True,separators=(',',':')).encode();name='tiny_'+str(args[0])+'_'+str(args[2])+'_'+str(args[3])+'.json'
   with(P/name).open('xb')as file:file.write(raw)
   record.pop('witnesses');record['raw_sha256']=hashlib.sha256(raw).hexdigest();out['tiny'].append(record)
  for case in data['actual']:
   v=json.loads(Path(case['path']).read_text());f=BinaryField(v['field_degree'],v['modulus']);assert f.square(v['y'])^f.mul(v['z'],v['y'])==f.mul(f.square(v['z']),v['z'])^f.mul(v['A'],f.square(v['z']))^v['B'];record=guards(f,v['A'],v['B'],v['d'],v['z']);assert all(record[k]for k in['O_triple_excluded','P0_triple_excluded','P0_pair_excluded','target_outside','translated_target_outside']);record['name']=case['name'];out['actual'].append(record)
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps({'out':out,'error':error},separators=(',',':'))
 if time.monotonic()-START>=78:error='late serialized output'
 if error:
  for row in out['tiny']:row['verified_tuple_lower_bound']=row.pop('complete_original_tuples',0);row['complete_original_tuples']=None
 raw=json.dumps({'out':out,'error':error},sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if not error else'inconclusive','summary':'Guarded global binary norm-chart representation coverage, independently checked with full tiny original roots, quadratic-extension curve groups and multiplicity-aware Hermite norms.','evidence':{'error':error,'out':out,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'complete_global_root_search':False,'release_gate_claim':False},'next_actions':['If guards and independent tiny oracles pass, use the theorem only for explicit covered fields/targets; derive unbounded outer h/k search and handle failed guards separately.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
