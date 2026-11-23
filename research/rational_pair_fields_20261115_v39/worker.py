"""Cross-binary-field complete pair-query oracle, not curve coverage."""
from pathlib import Path
import hashlib,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.rational_fraction_oracles_20261115_v39 import worker as ref
from research.rational_pair_query_20261115_v39 import solver as s
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 out=dict(fields=[]);error=None;states=0
 try:
  data=json.loads((P/'DATA.json').read_text())
  for spec in data['fields']:
   begin=time.monotonic();f=BinaryField(spec['n'],spec['modulus']);t=f.inv(data['a'])^f.inv(data['b']);records=[]
   for target in[t,0,spec['query']]:
    got=s.query(f,target,18);assert got['complete'];fraction,_=ref.reference(f,target,18);want=[]
    if fraction is not None:
     N,D=fraction
     if not N:want=[(a,a)for a in range(1,1<<18)];states+=(1<<18)-1
     else:
      for a in range(1,1<<18):
       den=ref.p.multiply(N,a)^D
       if den:
        b,r=ref.division(ref.p.multiply(D,a),den)
        if not r and 0<b<1<<18:want.append((a,b))
       states+=1
    assert got['pairs']==want
    for a,b in got['pairs']:
     assert(a==b)if not target else(f.inv(a)^f.inv(b)==target)
    records.append(dict(t=target,count=len(want),pair_sha256=hashlib.sha256(json.dumps(want,separators=(',',':')).encode()).hexdigest(),fraction=got.get('fraction'),divisors=got.get('divisors')))
   assert records[0]['count']>=2 and records[1]['count']==(1<<18)-1
   out['fields'].append(dict(n=f.degree,modulus=f.modulus,queries=records,complete=True,field_seconds=time.monotonic()-begin,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss));assert time.monotonic()-START<78
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 out['exhaustive_a_states']=states;raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,fields=[v['n']for v in out['fields']],complete_pair_queries=sum(len(v['queries'])for v in out['fields']),actual_a_states=states,worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),qualified_speedup=None,release_gate_claim=False,complete_global_root_search=False,curve_coverage_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Exact pair primitive across6binaryfields, fullprefix independent scan; no curve or endtoendperformance claim.',evidence=ev,next_actions=['Freeze paid query panel then compare with strongest completed pairscanner; keep genericprimitive correctness separate from curve/source/rootperformance. Pursue odd-extension/prime-interval rational heights with separate proofs.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
