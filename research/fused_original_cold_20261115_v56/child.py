from pathlib import Path
import hashlib,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.fused_affine_norm_20261115_v56.solve import solve
from research.original_chain_lift_20261115_v55.solver import Lift
from research.original_chain_lift_20261115_v55.worker import eq
from research.original_chain_join_20261115_v55.solver import Join
P=Path(__file__).resolve().parent;R=P.parents[1]
if __name__=='__main__':
 begin=time.monotonic();method=sys.argv[1];data=json.loads((P/'DATA.json').read_text());rows=[]
 for entry in data['instances']:
  instance=json.loads((R/entry['path']).read_text());result=solve(instance,R/'research/general_native_20261115_v54'/method,R/'research/general_native_20261115_v54/validation.so');assert result['complete'];f=BinaryField(instance['field_degree'],instance['modulus']);start=time.monotonic();tuples=set();methods=[]
  for norm in result['solutions']:
   cls=Join if len(norm['roots'])==5 else Lift;methods.append(cls.__name__);obj=cls(f,instance['B']);tuples.update(obj.bag(norm['roots'],instance['z']))
   if len(tuples)>100000:raise RuntimeError('original output cap')
  tuples=sorted(tuples);lift_seconds=time.monotonic()-start;start=time.monotonic()
  for values in tuples:
   a,b,c,d,e,u,v,w=values
   assert not any(eq(x,y,z,instance['B'],instance['modulus'])for x,y,z in[(a,b,u),(u,c,v),(v,d,w),(w,e,instance['z'])])
  verify_seconds=time.monotonic()-start;rows.append(dict(name=entry['name'],solutions=result['solutions'],complete=True,original_contract='all finite original chain tuples whose source multiset is a supported norm in this fixed h/k chart',original_tuples=tuples,original_reconstruction_methods=methods,original_lift_seconds=lift_seconds,original_literal_verify_seconds=verify_seconds,candidates=len(result['candidate_pairs']),candidate_sha256=hashlib.sha256(json.dumps(result['candidate_pairs'],separators=(',',':')).encode()).hexdigest(),native_wall_seconds=result['native_wall_seconds'],norm_total_seconds=result['total_seconds'],native_peak_rss_kib=result['native']['peak_rss_kib']))
 print(json.dumps(dict(method=method,cases=rows,internal_seconds=time.monotonic()-begin,parent_peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss),separators=(',',':')))
