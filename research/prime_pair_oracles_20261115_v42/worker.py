"""Independent complete smallprime pair maps and exact-proved NIST controls."""
from pathlib import Path
import hashlib,json,math,resource,sys,time
from fractions import Fraction
from research.prime_reciprocal_prepared_20261115_v41 import solver as s
from research.prime_pair_oracles_20261115_v42 import field_proof
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic();CURRENT={}
def controls():
 profiles=[];queries=assignments=0
 for p,M in[(101,2),(1009,3),(10007,7),(65537,15)]:
  assert all(p%d for d in range(2,math.isqrt(p)+1));inv={a:pow(a,-1,p)for a in range(1,M+1)};truth={}
  for a,ia in inv.items():
   for b,ib in inv.items():truth.setdefault((ia+ib)%p,[]).append((a,b));assignments+=1
  digest=hashlib.sha256();positive=[]
  for t in range(p):
   global CURRENT
   CURRENT=dict(p=p,M=M,t=t);result=s.query(p,t,M);want=sorted(truth.get(t,[]));assert result['complete']and result['pairs']==want
   digest.update(len(want).to_bytes(4,'little'));queries+=1
   if want:positive.append(dict(t=t,pairs=want))
  target=(1+pow(M,-1,p))%p
  assert s.query(p,target,M,max_work=0)['status']=='inconclusive';assert s.query(p,target,M,max_output=0)['status']=='inconclusive'
  profiles.append(dict(p=p,M=M,queries=p,count_sha256=digest.hexdigest(),positive=positive));assert time.monotonic()-START<65
 try:s.query(101,1,3)
 except ValueError:height_refused=True
 else:raise AssertionError('unsupported height admitted')
 return dict(complete_queries=queries,literal_pairs=assignments,profiles=profiles,cap_demotion_checked=True,unsupported_height_refused=height_refused)
def actual():
 data=json.loads((P/'DATA.json').read_text());lib=R/'vendor/flint-proof-20261115-v42/libflint.so';results=[]
 for row in data['fields']:
  p=row['p'];d=row['d'];M=(1<<d)-1;proof=field_proof.prove_prime(p,lib);proof.pop('library_path',None);a=1<<(d-1);b=a+1;target=(pow(a,-1,p)+pow(b,-1,p))%p;cases=[]
  for name,t,want in[('near_half',target,[(a,b),(b,a)]),('diagonal',(2*pow(M,-1,p))%p,[(M,M)]),('zero',0,[]),('query',row['query'],None),('unit_fraction',1,[(2,2)])]:
   global CURRENT
   CURRENT=dict(field=row['name'],mode=name);result=s.query(p,t,M)
   if name=='unit_fraction':assert not result['complete']and result['status']=='inconclusive'
   else:
    assert result['complete']
    if want is not None:assert result['pairs']==sorted(want)
    for x,y in result['pairs']:assert 1<=x<=M and 1<=y<=M and(pow(x,-1,p)+pow(y,-1,p))%p==t
   approx=Fraction(t,p).limit_denominator(M*M);rn=t*approx.denominator-p*approx.numerator;reference=[rn,approx.denominator]if 0<rn<=2*M else None
   assert result.get('fraction')==reference
   cases.append(dict(mode=name,t=t,result=result,independent_fraction=reference))
  results.append(dict(name=row['name'],p=p,d=d,M=M,prime_proof=proof,cases=cases));assert time.monotonic()-START<78
 return dict(fields=results,scope='Primeinterval reciprocal-pair controls; unitfraction unresolved, no oddcurve integration/allroot or qualifiedperformance claim')
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc);out['failed_case']=CURRENT
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<131072
 with(P/'raw.json').open('xb')as f:f.write(raw)
 aa=out.get('actual',{});ev=dict(error=error,tiny_complete_queries=out.get('controls',{}).get('complete_queries'),fields=[v['name']for v in aa.get('fields',[])],complete_actual=sum(c['result']['complete']for f in aa.get('fields',[])for c in f['cases']),unresolved_actual=sum(not c['result']['complete']for f in aa.get('fields',[])for c in f['cases']),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),qualified_speedup=None,release_gate_claim=False,complete_global_root_search=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Unchanged primeinterval prototype: independent complete smalloracles, exactproved NISTcontrols and retained unitfraction capcases.',evidence=ev,next_actions=['Resolve observed unitfraction workcap using a newadaptive a-range variant, preserving existingraw/source; separate curveintegrationproof before indexcalculus claims.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
