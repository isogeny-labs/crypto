from pathlib import Path
from itertools import product,combinations_with_replacement
import json,hashlib,resource,sys,time
from .algebra import BField,roots_poly,evaluate,rem,image,supported,compose_mod
from research.odd_quadratic_lift_20261115_v63.worker import F as Odd
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 out={'profiles':[],'cases':0,'collision_counterexample':None};error=None
 try:
  profiles=[('F8-cubic',BField(3,11),3,list(range(4)),[0,1]),('F16-cubic',BField(4,19),3,list(range(8)),[0,1]),('F4-quintic',BField(2,7),5,list(range(4)),[0,1]),('F3-cubic',Odd(3),3,list(range(3)),[0]),('F9-cubic',Odd(3,2),3,list(range(9)),[0,1,2]),('F5-interval',Odd(5),3,list(range(4)),[0])]
  for name,f,m,V,U in profiles:
   L=roots_poly(U,f);W=sorted({evaluate(L,x,f)for x in V});positive=0;digest=hashlib.sha256()
   for cs in product(range(f.q),repeat=m):
    H=list(cs)+[1];q=rem(L,H,f);B=image(H,q,f);assert not compose_mod(B,q,H,f);a=supported(H,V,f);b=supported(B,W,f);assert a['supported']==b['supported'];positive+=a['supported'];out['cases']+=1;digest.update(json.dumps([H,B,a['supported']],separators=(',',':')).encode())
    if a['supported']:
     mapped=[]
     for x,count in a['roots']:mapped +=[evaluate(L,x,f)]*count
     assert B==roots_poly(mapped,f)
   out['profiles'].append({'name':name,'characteristic':f.p,'field_size':f.q,'degree':m,'domain':V,'kernel':U,'image_domain':W,'cases':f.q**m,'supported':positive,'receipt_sha256':digest.hexdigest()});assert time.monotonic()-START<65
  f=BField(3,11);L=roots_poly([0,1],f);H0=roots_poly([0,0,0],f);H1=roots_poly([1,1,1],f);B0=image(H0,rem(L,H0,f),f);B1=image(H1,rem(L,H1,f),f);assert H0!=H1 and B0==B1;out['collision_counterexample']={'H0':H0,'H1':H1,'common_image':B0,'different_original_root_bags':[[0,0,0],[1,1,1]]}
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<75
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Division-free image-cell support equivalence with exhaustive binary andodd-field finite-algebra oracles andcollision control.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Test actualfive-source quintic norms andboundedwhole-cell certificates;do not identify image collisions with original-root equality.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
