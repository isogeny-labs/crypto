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
def main():
 os.sched_setaffinity(0,{min(os.sched_getaffinity(0))});resource.setrlimit(resource.RLIMIT_AS,(4*1024**3,4*1024**3))
 out=load('reciprocal_triple_lift_20261115_v40','deae1b65dde0c5653b82194ecd2ccc69438da5050d014f72de82e23baf87642f');profiles=0
 import itertools
 for row in out['controls']['profiles']:
  f=Field(row['n'],row['modulus']);inv={a:power(f,a,f.q-2)for a in range(1,1<<row['d'])};counts=[0]*f.q
  for a,b,c in itertools.product(inv,repeat=3):counts[inv[a]^inv[b]^inv[c]]+=1
  assert counts==row['triple_counts'];profiles+=1
 actual=out['actual'];result=actual['query'];assert result['complete'];E=result['evidence'];f=Field(83,E['modulus']);assert irreducible(f.m)and f.mul(actual['z'],E['t'])==1;cols=[f.mul(E['t'],1<<j)>>35 for j in range(52)];K=E['kernel_basis'];assert rank(cols)==52-len(K)and rank(K)==len(K)==4
 for bits in K:
  v=0
  for j,c in enumerate(cols):
   if bits>>j&1:v^=c
  assert not v
 Ds={0}
 for k in K:Ds|={v^k for v in list(Ds)}
 assert Ds-{0}=={r['D']for r in E['products']};triples=[];smooth=0
 for row in E['products']:
  D=row['D'];N=f.mul(E['t'],D);assert N==row['N']and N<1<<35;product=1
  for poly,e in row['factors']:
   assert irreducible(poly)
   for _ in range(e):product=mul(product,poly)
  assert product==D
  if row['large_factor']is not None:
   g=row['large_factor'];assert g.bit_length()-1>=18 and div(D,g)[1]==0 and irreducible(g);continue
  smooth+=1;wanted=[]
  for a in divisors(row['factors']):
   if not 0<a<1<<18:continue
   rem,r=div(D,a);assert not r
   for b in divisors(row['factors']):
    if not 0<b<1<<18:continue
    c,r=div(rem,b)
    if r or not 0<c<1<<18:continue
    if mul(a,b)^mul(a,c)^mul(b,c)==N:wanted.append((a,b,c))
  assert sorted(wanted)==sorted(map(tuple,row['triples']));triples+=wanted
 assert not triples and result['exact_count']==0
 print(json.dumps(dict(tiny_profiles=profiles,complete_tiny_queries=out['controls']['complete_queries'],actual_product_lifts=len(Ds)-1,kernel_dimension=len(K),largefactor_rejections=len(E['products'])-smooth,smooth_products=smooth,exact_reciprocal_triple_count=0,excluded_branch='all nozero repeatedsource k0norms only',seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,original_root_search_complete=False)))
if __name__=='__main__':main()
