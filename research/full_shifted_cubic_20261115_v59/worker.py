from pathlib import Path
from types import SimpleNamespace
from collections import Counter
import hashlib,json,resource,subprocess,sys,time
from rootfinder.binaryfield import BinaryField
from research.native_shifted_cubic_20261115_v59.oracles import family
from research.torsion_shifted_cubic_20261115_v58.solver import coefficients
from research.compiled_subspace_20261115_v31 import factor
from research.norm_block_reuse_20261115_v36 import backend as linear
from research.original_chain_lift_20261115_v55.solver import Lift
from research.original_chain_lift_20261115_v55.worker import eq
from research.positive_binary_charts_20261115_v53 import math as curve
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def main():
 out={'complete_cubic_chart':False,'verified_norms':[],'original_tuple_count_exact':None};error=None
 try:
  source=R/'research/native_shifted_cubic_20261115_v59';instance=json.loads((source/'full_width_A1_instance.json').read_text());f=BinaryField(instance['field_degree'],instance['modulus']);z=instance['z'];y=instance['source_ordinate'];d=instance['d'];A=instance['A'];command=[str(source/'scan'),'--A',hex(A),'--z',hex(z),'--y',hex(instance['cli_ordinate']),'--dimension',str(d),'--max-outer','262144','--max-candidates','16777216','--max-nullity','12','--max-accepted','1024','--record-families','16','--max-seconds','55'];start=time.monotonic();run=subprocess.run(command,capture_output=True,text=True,timeout=60);out.update(native_wall_seconds=time.monotonic()-start,native_exit_code=run.returncode,native_stdout=run.stdout,native_stderr=run.stderr);native=json.loads(run.stdout);out['native']=native
  outercols=[(f.square(1<<i)^(1<<i))>>d for i in range(83)];outer=linear.solve(linear.rows_from(outercols,z>>d,83-d),83);assert outer is not None and len(outer['basis'])==native['stats']['outer_nullity'];assert native['stats']['outer_cardinality_exact']==1<<len(outer['basis'])
  for rec in native['conditional_family_records']:
   h=int(rec['b'],16);ref=family(f,instance,h);assert rec['consistent']==(ref is not None)
   if ref is not None:assert linear.same(ref,{'particular':int(rec['particular'],16),'basis':[int(v,16)for v in rec['kernel']]})
  original=set();L=Lift(f,1)
  for row in native['accepted_norms']:
   h=int(row['b'],16);r=int(row['c'],16);C=tuple(int(x,16)for x in row['cubic']);assert C==coefficients(f,A,1,z,y,h,r);roots=factor.supported_roots(SimpleNamespace(p=2,field=f),C,[1<<i for i in range(d)]);assert roots['supported']and sum(m for x,m in roots['roots'])==3;assert h
   s=f.square(z)^f.mul(r,z)^f.mul(h,y);points=[]
   for x,m in roots['roots']:
    yp=f.mul(f.square(x)^f.mul(r,x)^s,f.inv(h));point=(x,yp);assert curve.on(f,A,1,point);points +=[point]*m
   assert curve.total(f,A,1,points+[(z,y)])is None
   counts=Counter(x for x,m in roots['roots']for _ in range(m));counts[1]+=2;values=L.bag(sorted(counts.items()),instance['original_target'][0])
   for a,b,c,dd,ee,u,v,w in values:assert all(not eq(x,yy,t,1,f.modulus)for x,yy,t in[(a,b,u),(u,c,v),(v,dd,w),(w,ee,instance['original_target'][0])])
   original.update(values);out['verified_norms'].append({'h':h,'r':r,'C':C,'roots':roots['roots'],'original_tuples':values})
  # Witness is validation-only and is loaded after the native solve and replay.
  witness=json.loads((source/'full_width_A1_witness.json').read_text());assert any(v['h']==witness['h']and v['r']==witness['r']and v['C']==tuple(witness['C'])for v in out['verified_norms']);assert set(map(tuple,witness['original_tuples']))<=original
  complete=run.returncode==0 and native['complete']is True and native['stats']['outer_visited']==native['stats']['outer_cardinality_exact']and native['stats']['candidates_visited']==native['stats']['candidates_planned'];out.update(complete_cubic_chart=complete,original_tuples=sorted(original),original_tuple_count_exact=len(original)if complete else None,supported_norm_count_exact=len(out['verified_norms'])if complete else None,parent_peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,native_child_peak_rss_kib=resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss)
  if not complete:error='bounded native chart unresolved'
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps({'out':out,'error':error},separators=(',',':'))
 if time.monotonic()-START>=78:error='late serialized output'
 if error:
  out['complete_cubic_chart']=False;out['original_tuple_count_exact']=None;out['supported_norm_count_exact']=None
  if 'native'in out:
   out['native']['complete']=False
   if 'supported_norm_count_exact'in out['native']:out['native']['supported_norm_count_lower_bound']=out['native'].pop('supported_norm_count_exact')
 raw=json.dumps({'out':out,'error':error},sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if not error else'inconclusive','summary':'Complete full-widthGF83 torsion-shifted cubic chart with independent row, coefficient, factor, group and original-root replay.','evidence':{'error':error,'complete_cubic_chart':out['complete_cubic_chart'],'supported_norm_count_exact':out.get('supported_norm_count_exact'),'original_tuple_count_exact':out.get('original_tuple_count_exact'),'native_wall_seconds':out.get('native_wall_seconds'),'native_stats':out.get('native',{}).get('stats'),'parent_peak_rss_kib':out.get('parent_peak_rss_kib'),'native_child_peak_rss_kib':out.get('native_child_peak_rss_kib'),'worker_seconds':time.monotonic()-START,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'complete_global_root_search':False,'release_gate_claim':False},'next_actions':['If complete, expand the preregistered cubic target ladder and isolate phase costs; separately reduce the full main quintic h/k search and build genuine matchedGB interfaces. If capped, diagnose the actual rank/candidate bottleneck before a changed experiment.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
