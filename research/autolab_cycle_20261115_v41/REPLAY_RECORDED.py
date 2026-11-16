"""Portable independent polynomial-convolution replay of recorded norm identities."""
from pathlib import Path
import hashlib,itertools,json,time,resource,os
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
class Field:
 def __init__(self,n,m):
  self.n=n;self.m=m;self.q=1<<n;self.table=None
  if n<=5:self.table=[[self.mul(a,b)for b in range(self.q)]for a in range(self.q)]
 def mul(self,a,b):
  if self.table is not None:return self.table[a][b]
  out=0
  while b:
   if b&1:out^=a
   b>>=1;a<<=1
   if a&self.q:a^=self.m
  return out
 def sq(self,a):return self.mul(a,a)
def mul(a,b):
 out=0
 while b:
  low=b&-b;out^=a<<(low.bit_length()-1);b^=low
 return out
def div(a,b):
 q=0
 while a.bit_length()>=b.bit_length():j=a.bit_length()-b.bit_length();q|=1<<j;a^=b<<j
 return q,a
def rank(cols):
 piv={}
 for v in cols:
  while v:
   p=(v&-v).bit_length()-1
   if p not in piv:piv[p]=v;break
   v^=piv[p]
 return len(piv)
def power(f,a,n):
 b=1
 while n:
  if n&1:b=f.mul(b,a)
  a=f.sq(a);n>>=1
 return b
def sha(v):return hashlib.sha256(json.dumps(v,separators=(',',':')).encode()).hexdigest()
def load(folder,digest):
 blob=(R/'research'/folder/'raw.json').read_bytes();assert hashlib.sha256(blob).hexdigest()==digest;d=json.loads(blob);assert d['error']is None;return d['out']
def gcd(a,b):
 while b:a,b=b,div(a,b)[1]
 return a
def irreducible(poly):
 n=poly.bit_length()-1;assert n>=1;x=div(2,poly)[1];q=x;primes=[p for p in range(2,n+1)if n%p==0 and all(p%d for d in range(2,int(p**.5)+1))];checks={n//p for p in primes}
 for i in range(1,n+1):
  q=div(mul(q,q),poly)[1]
  if i in checks and gcd(q^x,poly)!=1:return False
 return q==x
def divisors(factors,index=0,value=1):
 if index==len(factors):yield value;return
 g,e=factors[index];v=value
 for _ in range(e+1):yield from divisors(factors,index+1,v);v=mul(v,g)
def mobius(n):
 count=0;p=2
 while p*p<=n:
  if n%p==0:
   n//=p;count+=1
   if n%p==0:return 0
  p+=1
 if n>1:count+=1
 return-1 if count%2 else 1
def grouped_bound(N):
 I={w:sum(mobius(k)*(1<<(w//k))for k in range(1,w+1)if w%k==0)//w for w in range(1,N+1)};dp=[1]+[0]*N
 for w in range(1,N+1):
  new=[0]*(N+1)
  for used in range(N+1):
   for e in range(used//w+1):
    if not e:factor=1
    else:
     groups=min(I[w],e);q,r=divmod(e,groups);factor=(2*q+1)**(groups-r)*(2*q+3)**r
    new[used]=max(new[used],dp[used-e*w]*factor)
  dp=new
 return I,dp
def main():
 os.sched_setaffinity(0,{min(os.sched_getaffinity(0))});resource.setrlimit(resource.RLIMIT_AS,(4*1024**3,4*1024**3))
 bound=load('pair_divisor_bound_20261115_v40','2d0c69e4920a439afee810f17fe106d471a932675f7f2451106de626967a98a8');I,dp=grouped_bound(34);assert I=={int(k):v for k,v in bound['irreducible_counts'].items()}and dp==bound['max_divisors_by_degree'];assert max(dp)==229635<1<<20
 witness=bound['witness_degree34'];assert sum(w*e for w,c,e in witness)==34;prod=1
 for w,c,e in witness:assert c<I[w];prod*=2*e+1
 assert prod==229635 and len({(w,c)for w,c,e in witness})==len(witness)
 data=load('binary_wide_repair_20261115_v41','6414b9848cb0aa29f8ecd88557d81fe71117c274a10927b5623f1c7b045c3ba6');config=json.loads((R/'research/rational_pair_fields_20261115_v39/DATA.json').read_text());assert len(data['fields'])==6;cache={};verified=0
 for field in data['fields']:
  assert irreducible(field['modulus']);f=Field(field['n'],field['modulus']);assert len(field['queries'])==3
  for row in field['queries']:
   t=row['t'];frac=row['fraction']
   if frac is None:
    assert rank([f.mul(t,1<<j)>>18 for j in range(35)])==35;key=None;want=[]
   else:
    N,D=frac;assert f.mul(t,D)==N and N<1<<18 and D<1<<35;key=(N,D)
    if key in cache:count,digest=cache[key];assert count==row['count']and digest==row['pair_sha256'];verified+=1;continue
    if not N:want=[(a,a)for a in range(1,1<<18)]
    else:
     want=[]
     for a in range(1,1<<18):
      den=mul(N,a)^D
      if den:
       b,r=div(mul(D,a),den)
       if not r and 0<b<1<<18:want.append((a,b))
     for a,b in want:assert f.mul(a,b)!=0
   digest=sha(want);assert len(want)==row['count']and digest==row['pair_sha256'];cache[key]=(len(want),digest);verified+=1
  assert time.monotonic()-START<110
 print(json.dumps(dict(uniform_max_divisors=229635,independent_grouped_DP=True,binary_fields=[x['n']for x in data['fields']],complete_pair_queries=verified,independent_irreducibility=True,formal_pair_domains_reused_across_fields=len(cache)-1,seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,qualified_speedup=None)))
if __name__=='__main__':main()
