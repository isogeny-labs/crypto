from pathlib import Path
import hashlib,itertools,json,resource,statistics,sys,time
from rootfinder.binaryfield import BinaryField
from research.general_binary_points_20261115_v52 import solver as old
from research.original_chain_lift_20261115_v55.worker import literal,eq
from research.original_chain_lift_20261115_v55.solver import Lift
from research.original_chain_join_20261115_v55.solver import Join
from . import compiler
from .solve import solve
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def independent_norm(f,A,B,z,y,h,k,r,s):
 mul=lambda a,b:literal(a,b,f.modulus)
 def pmul(a,b):
  c=[0]*(len(a)+len(b)-1)
  for i,x in enumerate(a):
   for j,v in enumerate(b):c[i+j]^=mul(x,v)
  return c
 t=mul(mul(z,z),z)^mul(r,mul(z,z))^mul(s,z)^mul(mul(h,z)^k,y);F=[t,s,r,1];G=[k,h];N=[0]*7
 for values in [pmul(F,F),[0]+pmul(F,G),pmul([B,0,A,1],pmul(G,G))]:
  for i,v in enumerate(values):N[i]^=v
 H=[0]*6
 for i in range(6,0,-1):H[i-1]=N[i];N[i-1]^=mul(z,N[i])
 assert not N[0];return tuple(H)
def main():
 out={'tiny_systems':0,'literal_states':0,'actual':[],'component_times':[]};error=None
 try:
  profiles=[]
  for n,mod,A in[(3,11,0),(3,11,1),(4,19,0),(4,19,8)]:
   f=BinaryField(n,mod);z=4;y=0;B=f.mul(f.square(z),z)^f.mul(A,f.square(z));profiles.append((f,A,B,z,y,[(h,k)for h in range(f.order)for k in sorted({0,1,h,f.mul(h,2),f.mul(h,3)})]))
  for n,mod in[(1,3),(2,7),(4,19)]:
   f=BinaryField(n,mod);profiles.append((f,0,1,0,1,[(h,k)for h in range(min(4,f.order))for k in range(min(4,f.order))]))
  for f,A,B,z,y,params in profiles:
   for h,k in params:
    d=min(2,f.degree);base,cols,sol=compiler.model(f,A,B,z,y,h,k,d);ref=old.model(f,A,B,z,y,h,k,d);assert(base,cols,sol)==ref
    for r,s in itertools.product(range(f.order),repeat=2):
     H=tuple(a^old.norm.linear.xor(c[j]for i,c in enumerate(cols)if(r|(s<<f.degree))>>i&1)for j,a in enumerate(base));assert H==independent_norm(f,A,B,z,y,h,k,r,s);out['literal_states']+=1
    out['tiny_systems']+=1;assert time.monotonic()-START<55
  data=json.loads(Path('research/original_chain_cold_20261115_v55/DATA.json').read_text())
  for entry in data['instances']:
   inst=json.loads((R/entry['path']).read_text());f=BinaryField(inst['field_degree'],inst['modulus']);args=(f,*[inst[j]for j in['A','B','z','y','h','k','d']]);times={'generic':[],'fused':[]}
   for rep in range(3):
    for name,fn in [('generic',old.model),('fused',compiler.model)]:
     start=time.monotonic();v=fn(*args);times[name].append(time.monotonic()-start)
     if name=='generic':ref=v
     else:assert v==ref
   out['component_times'].append({'name':entry['name'],'times':times,'median_ratio':statistics.median(times['generic'])/statistics.median(times['fused'])})
   result=solve(inst,R/'research/general_native_20261115_v54/compressed',R/'research/general_native_20261115_v54/validation.so');previous=json.loads((R/'research/general_native_20261115_v54'/(entry['name']+'_reference.json')).read_text());assert result['input_sha256']==previous['input_sha256'];assert result['candidate_pairs']==[tuple(x)for x in previous['candidate_pairs']];assert json.loads(json.dumps(result['solutions']))==previous['solutions'];tuples=set()
   for norm in result['solutions']:
    obj=(Join if len(norm['roots'])==5 else Lift)(f,inst['B']);tuples.update(obj.bag(norm['roots'],inst['z']))
   tuples=sorted(tuples);expected=json.loads((R/'research/original_chain_lift_20261115_v55'/(entry['name']+'.json')).read_text())['tuples'];assert tuples==[tuple(v)for v in expected]
   for a,b,c,d,e,u,v,w in tuples:assert not any(eq(x,y,z,inst['B'],inst['modulus'])for x,y,z in[(a,b,u),(u,c,v),(v,d,w),(w,e,inst['z'])])
   result['original_tuples']=tuples;raw=json.dumps(result,sort_keys=True,separators=(',',':')).encode()
   with(P/(entry['name']+'.json')).open('xb')as file:file.write(raw)
   out['actual'].append({'name':entry['name'],'native_input_sha256':result['input_sha256'],'norm_candidates':len(result['candidate_pairs']),'original_tuples':len(tuples),'raw_sha256':hashlib.sha256(raw).hexdigest()});assert time.monotonic()-START<72
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps({'out':out,'error':error},separators=(',',':'))
 if time.monotonic()-START>=78:error='late serialized output'
 if error:
  for row in out['actual']:row['verified_tuple_lower_bound']=row.pop('original_tuples',0);row['original_tuples']=None
 raw=json.dumps({'out':out,'error':error},sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if not error else'inconclusive','summary':'Direct sparse affine norm compiler preserves exact matrices, fibers, native inputs and conditional original tuples.','evidence':{'error':error,'out':out,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['If exact equality passes, freeze a fully paid original-output cold comparison using fused compilation in both arms; no unchanged timing retry.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
