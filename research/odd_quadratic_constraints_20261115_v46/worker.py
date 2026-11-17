from pathlib import Path
import json,hashlib,resource,sys,time
from research.generic_block_20261115_v27.backend import Field
from research.odd_quadratic_constraints_20261115_v46 import solver as s
P=Path(__file__).resolve().parent;START=time.monotonic()

def run():
 profiles=[]
 for p in [3,5]:
  F=Field(p,2);seed=0x46ab;cases=[]
  for case in range(18):
   maps=[]
   for j in range(5):
    coeff=[]
    for k in range(3):seed=(seed*1664525+1013904223)&0xffffffff;coeff.append(seed%F.order)
    maps.append(coeff)
   if case==0:maps=[[0,0,0]]*5
   if case==1:maps=[[1,0,0]]*5
   if case==2:maps=[[0,1,0]]+[[1,0,0]]*4
   result=s.constraints(F,maps);G=result['polynomial'];got=set(range(F.order))if G is None else{t for t in range(F.order)if s.evaluate(F,G,t)==0};want={t for t in range(F.order)if all(s.value(F,M,t)<p for M in maps)};assert got==want
   assert G is None or len(G)-1<=2*p
   if G is not None:
    for t in got:assert s.p.power(F,t,F.order)==t
   cases.append(dict(maps=maps,result=result,roots=sorted(got)))
  profiles.append(dict(p=p,degree=2,cases=cases))
 return profiles

def main():
 out={};error=None
 try:out['profiles']=run()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),separators=(',',':'))
 if time.monotonic()-START>18:error='late serialized output'
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Odd-extension quadratic-map coefficient admission via bounded-degree Frobenius GCD.',evidence=dict(error=error,fields=['GF9','GF25'],cases=sum(len(r['cases'])for r in out.get('profiles',[])),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),qualified_speedup=None,complete_global_root_search=False,release_gate_claim=False),next_actions=['Integrate with odd curve interpolation and exact factor extraction; prime interval admission remains separate.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<24;print(text)
if __name__=='__main__':main()
