from pathlib import Path
from itertools import product
import hashlib,json,resource,statistics,sys,time
from rootfinder.binaryfield import BinaryField
from research.original_chain_lift_20261115_v55.solver import Lift
from research.original_chain_lift_20261115_v55.worker import eq
from .solver import Join
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 out={'tiny':[],'actual':[]};error=None
 try:
  data=json.loads(Path('research/original_chain_lift_20261115_v55/DATA.json').read_text())
  for n,mod,B,domain in data['tiny']:
   f=BinaryField(n,mod);j=Join(f,B);table={(a,b):set(v for v in range(f.order)if not eq(a,b,v,B,mod))for a in range(f.order)for b in range(f.order)};pairs=list(table);count=0
   for a,b in pairs:
    for c,d in pairs:
     assert set(j.common(a,b,c,d))==table[a,b]&table[c,d];count+=1
   out['tiny'].append({'n':n,'B':B,'quadratic_pair_comparisons':count,'branches':j.join_branches});assert time.monotonic()-START<65
  for case in data['actual']:
   f=BinaryField(case['n'],case['modulus']);reference=json.loads(Path('research/original_chain_lift_20261115_v55/'+case['name']+'.json').read_text())['tuples'];expected=[tuple(v)for v in reference];times={x:[]for x in ['forward','join']};checks=[]
   for rep in range(3):
    for method,cls in ([('forward',Lift),('join',Join)]if rep%2==0 else[('join',Join),('forward',Lift)]):
     started=time.monotonic();obj=cls(f,case['B']);values=obj.bag(case['roots'],case['target']);dt=time.monotonic()-started;assert values==expected;times[method].append(dt)
     if method=='join':checks.append({'cached_pairs':len(obj.cache),'cached_common':len(obj.common_cache),'branches':obj.join_branches})
   out['actual'].append({'name':case['name'],'tuples':len(expected),'times':times,'median_ratio':statistics.median(times['forward'])/statistics.median(times['join']),'join_profiles':checks});assert time.monotonic()-START<72
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 if time.monotonic()-START>=78:error='late serialized result'
 raw=json.dumps({'out':out,'error':error},sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if not error else'inconclusive','summary':'Guarded two-boundary quadratic intersection preserves original finite chain roots; cost prediction measured separately.','evidence':{'error':error,'out':out,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Select a measured reconstruction implementation on training evidence; integrate its complete original tuples into fully paid cold conditional runs.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
