"""Independent complete tiny triple oracle, then actual full-domain lift."""
from pathlib import Path
import hashlib,itertools,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.reciprocal_triple_lift_20261115_v40 import solver as s
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def controls():
 queries=states=positive=0;profiles=[]
 for n,m in[(3,11),(4,19),(5,37),(6,67),(7,131),(8,283)]:
  f=BinaryField(n,m)
  for d in range(1,4):
   if 3*d-3>=n:continue
   inv={a:f.inv(a)for a in range(1,1<<d)};truth={t:[]for t in range(f.order)}
   for a,ia in inv.items():
    for b,ib in inv.items():
     for c,ic in inv.items():truth[ia^ib^ic].append((a,b,c));states+=1
   counts=[];maxdim=0
   for t,want in truth.items():
    got=s.solve(f,t,d);assert got['complete']and got['triples']==sorted(want);queries+=1;positive+=len(want);maxdim=max(maxdim,got['evidence']['kernel_dimension']);counts.append(len(want))
   profiles.append(dict(n=n,modulus=m,d=d,queries=f.order,triple_counts=counts,max_lift_dimension=maxdim))
 return dict(complete_queries=queries,literal_triple_assignments=states,positive_ordered_triples=positive,profiles=profiles)
def actual():
 data=json.loads((P/'DATA.json').read_text());f=BinaryField(83,data['modulus']);z=data['z'];t=f.inv(z);assert f.mul(t,z)==1;result=s.solve(f,t,18);return dict(z=z,query=result,global_nonzero_reciprocal_triples_complete=result['complete'],scope='Full low18 reciprocal triple domain; at k0 excludes nozero repeated-source normbranch iff count0. Originalcurve/chain/ghost/otherk/zero>=2 branches remain open')
def main():
 out={};error=None
 try:out['controls']=controls();assert time.monotonic()-START<65;out['actual']=actual();assert out['actual']['query']['complete'],'actual lift incomplete'
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<262144
 with(P/'raw.json').open('xb')as f:f.write(raw)
 aa=out.get('actual',{});qq=aa.get('query',{});ev=dict(error=error,controls=out.get('controls'),actual_complete=qq.get('complete'),actual_triple_count=qq.get('exact_count'),actual_lift_dimension=qq.get('evidence',{}).get('kernel_dimension'),product_lifts=len(qq.get('evidence',{}).get('products',[])),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),qualified_speedup=None,release_gate_claim=False,complete_global_root_search=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Exact global nonzero reciprocal triple lift on actualGF83, independent complete tiny triple oracles; originalroot search remainsopen.',evidence=ev,next_actions=['If completezero retain product/factor certificates and exclude all nozero repeated-source k0norms; pursue >=2zero/v0 cubic branch and distinctfive-source support. If cap/failure retain partialevidence and change lift/factor representation.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<16384 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
