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
def main():
 os.sched_setaffinity(0,{min(os.sched_getaffinity(0))});resource.setrlimit(resource.RLIMIT_AS,(4*1024**3,4*1024**3))
 frac=load('rational_fraction_oracles_20261115_v39','f2abd7ce12f16e65dff7ddceacd3c4cc86a8dc6f02c16f3e12f8357d30b14bb5');pairs=load('rational_pair_query_20261115_v39','92b803fe6bf51e974342559cc9bc0af9b2c3e6fa4df28434b40d7bedb9fa3fc7');fields={};maps={}
 for row in frac['controls']['records']:
  f=fields.setdefault(row['n'],Field(row['n'],row['modulus']));d=row['d'];t=row['t'];B=2*d-2;dim=B+1-rank([f.mul(t,1<<j)>>d for j in range(B+1)]);assert dim==row['kernel_dim'];r=row['result']
  if r['status']=='fraction':assert dim and f.mul(t,r['denominator'])==r['numerator']and r['numerator']<1<<d
  else:assert dim==0
 for row in pairs['controls']['records']:
  f=fields[row['n']];key=(row['n'],row['d'])
  if key not in maps:
   iv={a:power(f,a,f.q-2)for a in range(1,1<<row['d'])};m={t:[]for t in range(f.q)}
   for a,x in iv.items():
    for b,y in iv.items():m[x^y].append((a,b))
   maps[key]=m
  want=maps[key][row['t']];assert len(want)==row['count']and sha(want)==row['sha256']
 f=Field(83,pairs['actual']['modulus']);states=0;cache={}
 for row in pairs['actual']['queries']:
  t=row['t'];fraction=row['fraction'];dim=35-rank([f.mul(t,1<<j)>>18 for j in range(35)])
  if fraction is None:assert dim==0;want=[]
  else:
   N,D=fraction;assert dim and f.mul(t,D)==N and N<1<<18 and D<1<<35
   if t in cache:assert cache[t]==(row['exact_count'],row['pair_sha256']);continue
   want=[]
   if not N:want=[(a,a)for a in range(1,1<<18)];states+=(1<<18)-1
   else:
    for a in range(1,1<<18):
     den=mul(N,a)^D
     if den:
      b,r=div(mul(D,a),den)
      if not r and 0<b<1<<18:want.append((a,b))
     states+=1
    for a,b in want:assert power(f,a,f.q-2)^power(f,b,f.q-2)==t
  assert len(want)==row['exact_count']and sha(want)==row['pair_sha256'];cache[t]=(len(want),row['pair_sha256']);assert time.monotonic()-START<110
 print(json.dumps(dict(tiny_fraction_queries=len(frac['controls']['records']),tiny_pair_queries=len(pairs['controls']['records']),actual_queries=len(pairs['actual']['queries']),independent_unique_a_states=states,seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,qualified_speedup=None)))
if __name__=='__main__':main()
