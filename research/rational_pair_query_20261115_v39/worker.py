"""Complete small pair maps and full low18 a-scan actual oracle."""
from pathlib import Path
import hashlib,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.rational_fraction_oracles_20261115_v39 import worker as ref
from research.rational_pair_query_20261115_v39 import solver as s
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def checksum(pairs):return hashlib.sha256(json.dumps(pairs,separators=(',',':')).encode()).hexdigest()
def controls():
 records=[];states=0
 for n,m in[(4,19),(5,37),(6,67),(7,131)]:
  f=BinaryField(n,m)
  for d in range(1,4):
   if 3*d-3>=n:continue
   inv={a:f.inv(a)for a in range(1,1<<d)};truth={t:[]for t in range(f.order)}
   for a,ia in inv.items():
    for b,ib in inv.items():truth[ia^ib].append((a,b));states+=1
   for t,want in truth.items():
    got=s.query(f,t,d);assert got['complete']and got['pairs']==sorted(want);records.append(dict(n=n,modulus=m,d=d,t=t,count=len(want),sha256=checksum(want)))
 return dict(complete_queries=len(records),literal_pair_assignments=states,records=records)
def actual():
 data=json.loads((R/'research/rational_fraction_oracles_20261115_v39/DATA.json').read_text());f=BinaryField(83,data['modulus']);rows=[];states=0
 for case in data['cases']:
  t=f.inv(case['a'])^f.inv(case['b'])if case['kind']=='planted'else case['t'];got=s.query(f,t,18);assert got['complete'];fraction,_=ref.reference(f,t,18)
  want=[]
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
  else:assert not got['pairs']
  assert all((a==b)if not t else(f.inv(a)^f.inv(b)==t)for a,b in got['pairs'])
  row={k:v for k,v in got.items()if k!='pairs'};row.update(case_id=case['id'],t=t,pair_sha256=checksum(got['pairs']));rows.append(row)
  assert time.monotonic()-START<78
 return dict(modulus=f.modulus,d=18,queries=rows,exhaustive_a_states=states,scope='Complete auxiliary reciprocal pair domains only; global triple/source/curve/root search unresolved')
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<262144
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,tiny_complete_queries=out.get('controls',{}).get('complete_queries'),actual_queries=len(out.get('actual',{}).get('queries',[])),actual_a_states=out.get('actual',{}).get('exhaustive_a_states'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),qualified_speedup=None,release_gate_claim=False,complete_global_root_search=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Exact bounded pair query via verifiedfactor/divisor reconstruction, independent complete tiny and fullactuala-scan.',evidence=ev,next_actions=['If completes compare paid pairoracle vs directscan on fresh frozen query panels with output/memory and repetitions; primarypriorart review; no claim global2^54triple enumeration tractable.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
