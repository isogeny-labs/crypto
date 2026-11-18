from pathlib import Path
from itertools import product,combinations_with_replacement
from collections import Counter
import hashlib,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.binary_norm_coverage_20261115_v57.oracle import points,hermite_norm
from research.positive_binary_charts_20261115_v53 import math as curve
from research.original_chain_lift_20261115_v55.solver import Lift
from research.original_chain_lift_20261115_v55.worker import eq
from research.torsion_shifted_cubic_20261115_v58.solver import shifted
P=Path(__file__).resolve().parent;START=time.monotonic()
def run():
 data=json.loads((P/'DATA.json').read_text());n,mod,A,B,d=data['profile'];f=BinaryField(n,mod);e,pool=points(f,A,B,d);targets={}
 for z in range(1<<d,f.order):
  if f.inv(z)<1<<d:continue
  Q=curve.lift(f,A,B,z)
  if Q is not None:targets[z]=Q
 mainbags={z:{}for z in targets};witnesses=[]
 for ps in combinations_with_replacement(pool,5):
  counts=Counter(ps)
  if not all(counts[(x,y)]==counts[(x,e.conjugate(y))]for x,y in counts):continue
  T=curve.total(e,A,B,ps)
  if T is None or T[0]not in targets or not e.real(T[1]):continue
  z=T[0];Q=targets[z];sources=list(ps)
  if(T[0],T[1]^T[0])!=Q:sources=[(x,y^x)for x,y in sources]
  bag=tuple(sorted(x for x,y in sources))
  if bag in mainbags[z]:continue
  V=hermite_norm(f,e,A,B,sources,Q);H=curve.polynomial.norm(f,A,B,z,V);expected=(1,)
  for x in bag:expected=curve.polynomial.b._mul(expected,(x,1),f)
  assert H==expected;mainbags[z][bag]=V;witnesses.append({'target':Q,'bag':bag,'function':V,'H':H})
  assert time.monotonic()-START<45
 cache={}
 def roots(a,b):
  key=tuple(sorted((a,b)))
  if key not in cache:cache[key]=tuple(c for c in range(f.order)if not eq(a,b,c,B,mod))
  return cache[key]
 truth={z:set()for z in targets}
 for xs in product(range(1<<d),repeat=5):
  for u in roots(xs[0],xs[1]):
   for v in roots(u,xs[2]):
    for w in roots(v,xs[3]):
     for z in roots(w,xs[4]):
      if z in truth:truth[z].add(xs+(u,v,w))
  assert time.monotonic()-START<60
 L=Lift(f,B);records=[];allroots={};missing_before=added=main_count=total=0
 for z,Q in targets.items():
  main=set()
  for bag in mainbags[z]:main.update(L.bag(sorted(Counter(bag).items()),z))
  twist=shifted(f,A,B,*Q,d);extra=set(twist['original_tuples']);union=main|extra;assert union==truth[z],z
  for a,b,c,dd,ee,u,v,w in union:assert all(not eq(x,y,k,B,mod)for x,y,k in[(a,b,u),(u,c,v),(v,dd,w),(w,ee,z)])
  missing_before+=len(truth[z]-main);added+=len(extra-main);main_count+=len(main);total+=len(union);records.append({'target':Q,'main_source_bags':len(mainbags[z]),'main_original_tuples':len(main),'shifted_original_tuples':len(extra),'union_original_tuples':len(union),'shifted_h_values':twist['h_values'],'shifted_r_candidates':twist['candidates']});allroots[z]=sorted(union);assert time.monotonic()-START<72
 assert missing_before==added and added>0
 return {'profile':data['profile'],'target_count':len(targets),'source_orders':(1<<d)**5,'literal_local_pairs':len(cache),'main_original_tuples':main_count,'missing_before_shift':missing_before,'restored_original_tuples':added,'complete_tiny_original_tuples':total,'records':records,'main_norm_witnesses':witnesses,'all_original_roots':allroots}
def main():
 out={};error=None
 try:out=run()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps({'out':out,'error':error},separators=(',',':'))
 if time.monotonic()-START>=78:error='late serialized output'
 if error:out['complete_tiny_original_tuples']=None;out['global_hard_completion']=False
 raw=json.dumps({'out':out,'error':error},sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if not error else'inconclusive','summary':'Main-plus-torsion-shifted norm chart union matches complete independent tiny original root sets.','evidence':{'error':error,'controls':{k:v for k,v in out.items()if k not in['all_original_roots','main_norm_witnesses','records']},'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'complete_global_root_search':False,'release_gate_claim':False},'next_actions':['Derive a bounded native h-conditioned cubic solver and full-width positive trace-one fixture; preserve global main h/k and genuineGB qualification as separate unresolved tasks.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
