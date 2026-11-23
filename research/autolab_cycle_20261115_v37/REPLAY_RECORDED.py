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
def load(folder,sha):
 blob=(R/'research'/folder/'raw.json').read_bytes();assert hashlib.sha256(blob).hexdigest()==sha;v=json.loads(blob);assert v['error']is None;return v['out']
def basis(vs):
 piv={}
 for v in vs:
  while v:
   p=v.bit_length()-1
   if p not in piv:piv[p]=v;break
   v^=piv[p]
 return sorted(piv.items(),reverse=True)
def rank(vs):
 piv={}
 for v in vs:
  while v:
   p=(v&-v).bit_length()-1
   if p not in piv:piv[p]=v;break
   v^=piv[p]
 return len(piv)
def remainder(v,B):
 for p,row in B:
  if v>>p&1:v^=row
 return v
def main():
 os.sched_setaffinity(0,{min(os.sched_getaffinity(0))});resource.setrlimit(resource.RLIMIT_AS,(4*1024**3,4*1024**3))
 data=load('norm_trace_dual_20261115_v37','e51e5e9941724c0995f05b89a3364f7f1be77a2e486e4abc7b8f6dba72096a62')['actual'];census=load('norm_vrank_census_20261115_v37','71be79802a7dcaec33b750e73f57443b5b6bcf569fdd01833dc0824e42d879f0');assert census['complete'];f=Field(83,data['modulus']);mu=data['mu'];half=data['mu_half'];deriv=data['derivative']
 for j,m in enumerate(mu):
  assert f.mul(m,deriv)==1<<j;r=m
  for _ in range(82):r=f.sq(r)
  assert r==half[j]and f.sq(r)==m
 lam=[f.mul(mu[0],1<<i)for i in range(31)]
 for row in data['records']:
  terms=[x^f.mul(row['k'],m)for x,m in zip(half,mu)]
  assert rank(terms+[f.mul(row['h'],v)for v in lam])==row['v_rank']==31+row['dual_rank']
 B=basis(half);assert len(B)==14
 def projected(h):return[remainder(f.mul(h,v),B)for v in lam]
 h=census['particular'];cols=projected(h);deltas=[projected(v)for v in census['basis']];hist=[0]*15;digest=hashlib.sha256()
 for i in range(census['total']):
  if i:
   j=(i&-i).bit_length()-1;h^=census['basis'][j];delta=deltas[j];cols=[a^b for a,b in zip(cols,delta)]
  r=rank(cols)+14-31;assert 0<=r<=14;hist[r]+=1;digest.update(h.to_bytes(11,'little')+bytes([r]))
  if i%4096==0:assert time.monotonic()-START<110
 assert hist==census['rank_histogram']and digest.hexdigest()==census['sequence_sha256']
 print(json.dumps(dict(independent_actual_cells=len(data['records']),independent_full_h_charts=census['total'],rank_histogram=hist,sequence_sha256=digest.hexdigest(),representation='Gray-updated projected31columns; no perh inversion',seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,qualified_speedup=None)))
if __name__=='__main__':main()
