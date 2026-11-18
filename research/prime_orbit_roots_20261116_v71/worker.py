from pathlib import Path
from itertools import product
import json,hashlib,resource,sys,time
from .field import Prime
from .solver import solve
from research.translation_orbits_20261115_v69.algebra import literal_s3
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 out={'sqrt_checks':0,'systems':0,'literal_shift_aux_checks':0,'profiles':[],'qualified_speedup':None};error=None
 try:
  for p in [3,5,7,17,41,97,257]:
   f=Prime(p);squares={x*x%p for x in range(p)}
   for a in range(p):
    root=f.sqrt(a);assert(root is not None)==(a in squares)
    if root is not None:assert root*root%p==a
    out['sqrt_checks']+=1
  for p,A,B,z in [(3,1,1,1),(5,1,1,1),(7,1,1,1),(17,1,1,1),(41,1,1,1),(97,1,1,1),(5,0,0,0)]:
   f=Prime(p);count=positive=0
   for beta in product(range(3),repeat=3):
    inst={'p':p,'A':A,'B':B,'z':z,'beta':list(beta),'shift_low':0,'shift_high':p-1};expected=[]
    for t in range(p):
     a,b,c=[f.add(x,t)for x in beta]
     for u in range(p):
      if literal_s3(a,b,u,A,B,f)==0 and literal_s3(u,c,z,A,B,f)==0:expected.append([t,u])
      out['literal_shift_aux_checks']+=1
    factor=solve(inst,'factor');scan=solve(inst,'difference');assert factor['roots']==scan['roots']==expected;positive+=bool(expected);count+=1;out['systems']+=1;assert time.monotonic()-START<65
   out['profiles'].append({'p':p,'A':A,'B':B,'z':z,'systems':count,'nonempty':positive})
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<75
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Exact odd-prime orbit roots,interval scan,Tonelli andallfiniteintermediates against independent exhaustive small-prime oracles.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Validate source-identified NIST fields andcurvefixtures before any timed many-field comparison.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
