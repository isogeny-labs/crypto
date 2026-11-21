from pathlib import Path
from itertools import product
import json,hashlib,resource,sys,time
from .solver import solve,gray_values,evaluate,Context
from research.translation_orbits_20261115_v69.algebra import literal_s3
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 out={'small_systems':0,'literal_shift_aux_checks':0,'full_width':[],'qualified_speedup':None};error=None
 try:
  for n,mod,B,z in [(2,7,1,3),(3,11,3,4),(4,19,7,6),(3,11,0,0)]:
   f=Context(n,mod).f
   for beta in product(range(4),repeat=3):
    inst={'n':n,'modulus':mod,'B':B,'z':z,'A':0,'beta':list(beta),'shift_dimension':2};want=[]
    for t in range(4):
     a,b,c=[x^t for x in beta]
     for u in range(f.q):
      if not literal_s3(a,b,u,0,B,f)and not literal_s3(u,c,z,0,B,f):want.append([t,u])
      out['literal_shift_aux_checks']+=1
    for method in ['gcd','horner','gray']:assert solve(inst,method)['roots']==want,(inst,method,want)
    out['small_systems']+=1
  for row in json.loads((P/'DATA.json').read_text())['instances']:
   inst=json.loads(Path(row['path']).read_text());reference=solve(inst,'horner');assert any(t==0 for t,u in reference['roots']);results={}
   for method in ['gcd','gray']:
    begin=time.monotonic();v=solve(inst,method);assert v['roots']==reference['roots'];assert v['candidate_shifts']==reference['candidate_shifts'];results[method]=time.monotonic()-begin
   ctx=Context(inst['n'],inst['modulus']);values=gray_values(reference['orbit_polynomial'],inst['shift_dimension'],ctx.f)
   for t,value in enumerate(values):assert value==evaluate(reference['orbit_polynomial'],t,ctx.f)
   out['full_width'].append({'name':row['name'],'input_sha256':hashlib.sha256(Path(row['path']).read_bytes()).hexdigest(),'root_count':len(reference['roots']),'roots':reference['roots'],'R':reference['orbit_polynomial'],'candidate_shifts':reference['candidate_shifts'],'component_development_seconds':results});assert time.monotonic()-START<65
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<75
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Exact degree12common-shift solver with domainGCD,coset roots,all finite intermediates,and independent literal/scan oracles.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Freeze complete conditional orbit comparisons against strongANF/zeta enumeration,charging setup andallroots;do not conflate with independent-source solving.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
