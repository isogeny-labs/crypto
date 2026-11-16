"""Exact uniform polynomial divisor-count bound and independent small enumeration."""
from pathlib import Path
import collections,hashlib,json,resource,sys,time
from research.reciprocal_rational_20261115_v38 import prepared as p
P=Path(__file__).resolve().parent;START=time.monotonic()
def counts(N):
 out={}
 for n in range(1,N+1):
  num=(1<<n)-sum(d*out[d]for d in range(1,n)if n%d==0);assert num%n==0;out[n]=num//n
 return out
def bound(N):
 I=counts(N);dp=[1]+[0]*N;paths=[[]]+[None]*N
 for degree in range(1,N+1):
  for copy in range(min(I[degree],N//degree)):
   new=list(dp);witness=list(paths)
   for used in range(N+1):
    for exponent in range(1,used//degree+1):
     old=used-exponent*degree
     if dp[old]and dp[old]*(2*exponent+1)>new[used]:new[used]=dp[old]*(2*exponent+1);witness[used]=paths[old]+[(degree,copy,exponent)]
   dp,paths=new,witness
 return I,dp,paths

def exhaustive(N,I):
 primes=[]
 for n in range(1,N+1):
  for value in range(1<<n,1<<(n+1)):
   if all(p.divide(value,g)[1]for degree in range(1,n//2+1)for g in range(1<<degree,1<<(degree+1))):primes.append(value)
  assert sum(p.degree(v)==n for v in primes)==I[n]
 maxima=[1]+[0]*N;seen=0
 for n in range(1,N+1):
  for poly in range(1<<n,1<<(n+1)):
   work=poly;divisors=1
   for g in primes:
    if work==1:break
    e=0
    while work!=1:
     q,r=p.divide(work,g)
     if r:break
     work=q;e+=1
    if e:divisors*=2*e+1
   assert work==1;maxima[n]=max(maxima[n],divisors);seen+=1
 return maxima,seen

def main():
 out={};error=None
 try:
  I,dp,paths=bound(34);small,seen=exhaustive(10,I);assert small==dp[:11]
  assert max(dp)==dp[34];witness=paths[34];assert sum(d*e for d,c,e in witness)==34;product=1
  for d,c,e in witness:assert c<I[d];product*=2*e+1
  assert len({(d,c)for d,c,e in witness})==len(witness)and product==dp[34];out=dict(irreducible_counts=I,max_divisors_by_degree=dp,witness_degree34=witness,witness_count=product,small_exact_maxima=small,small_polynomials_checked=seen,within_default_cap=dp[34]<=1<<20,scope='Uniform Ddegree<=34 divisorbound; not originalroot complexity or timing')
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,maximum=out.get('witness_count'),within_default_cap=out.get('within_default_cap'),small_polynomials=out.get('small_polynomials_checked'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),qualified_speedup=None,release_gate_claim=False,complete_global_root_search=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Exact uniform divisor-count certificate for reciprocal pair denominators, independent smallpolynomial enumeration.',evidence=ev,next_actions=['If max<=2^20 currentpairquery workcap nevertruncates low18domain; outputcap also covers all2^18-1pairs. Keep walltimeouts unresolved and all originalroot/outersearch gates.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
