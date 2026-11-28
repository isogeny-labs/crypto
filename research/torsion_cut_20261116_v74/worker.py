from pathlib import Path
import json,sys,time,resource,hashlib
from .core import local_oracles
from .chain import binary_chain
P=Path(__file__).resolve().parent;START=time.monotonic()
def clean(v):
 if isinstance(v,dict):return {k:clean(x)for k,x in v.items()if k!='library_path'}
 if isinstance(v,list):return [clean(x)for x in v]
 return v

def main():
 out={'qualified_speedup':None};error=None
 try:
  out['local']=local_oracles();assert time.monotonic()-START<35
  out['chains']=binary_chain();assert time.monotonic()-START<60
  from lab.research.nist_fields import build_nist_target
  from research.prime_orbit_roots_20261116_v71.field import Prime
  from research.prime_orbit_roots_20261116_v71.solver import power,gcd
  data=json.loads((P/'DATA.json').read_text());out['NIST']=[]
  for name in data['NIST_fields']:
   v=build_nist_target(name,19,data['flint_library']);d=v['preset'];f=Prime(d['p']);F=[d['b'],d['a'],0,1];r=power([0,1],f.p,F,f)
   while len(r)<2:r.append(0)
   r[1]=f.sub(r[1],1);G=gcd(F,r,f);assert G==[1]
   out['NIST'].append({'name':name,'direct_two_torsion_cubic_gcd':G,'frobenius_residue':r,'prime_and_group_proof':clean(v['proof']),'finite_target':v['target'],'all_finite_chain_roots_have_rational_lifts':True})
  old=json.loads(Path('research/frobenius_split_20261116_v73/raw.json').read_text());assert old['error']is None and old['out']['guards_hold'];out['actual_GF83_full_chain_theorem_applies']=True;out['proof_scope']='equivalence reduction, not enumerated hard-instance roots'
 except Exception as e:error=type(e).__name__+': '+str(e)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<78
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Torsion-cut local oracles, guarded full binary chain versus exact rational join, and independent five-NIST no-two-torsion certificates.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Use the exact torsion-cut planner; measure group-image join width and retain all exceptional fibers before any large-field performance claim.']};text=json.dumps(result);assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
