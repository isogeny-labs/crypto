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
def power(f,x,n):
 y=1
 while n:
  if n&1:y=f.mul(y,x)
  x=f.sq(x);n>>=1
 return y
def inverses(f,values):
 values=sorted(set(values));prefix=[1]
 for x in values:assert x;prefix.append(f.mul(prefix[-1],x))
 running=power(f,prefix[-1],f.q-2);out={}
 for i in range(len(values)-1,-1,-1):out[values[i]]=f.mul(prefix[i],running);running=f.mul(running,values[i])
 assert running==1 and all(f.mul(x,y)==1 for x,y in out.items());return out
def main():
 os.sched_setaffinity(0,{min(os.sched_getaffinity(0))});resource.setrlimit(resource.RLIMIT_AS,(4*1024**3,4*1024**3))
 raw=(R/'research/k0_reciprocal_cells_20261115_v38/raw.json').read_bytes();assert hashlib.sha256(raw).hexdigest()=='d4d2b610aad90de87d43401e16d98e45bf9704cd19bc87d9d9870af0b9d7747f';data=json.loads(raw);assert data['error']is None;out=data['out'];mods={2:7,3:11,4:19,5:37};small=0
 import itertools,math
 for row in out['controls']['profiles']:
  f=Field(row['n'],mods[row['n']]);D=range(1,row['domain_size']);iv=inverses(f,list(D)+[row['z']]);reverse={iv[x]:x for x in D};count=0
  for vals in itertools.product(D,repeat=4):
   t=iv[row['z']]
   for x in vals:t^=iv[x]
   count+=t in reverse
  assert count==row['nonzero_ordered_count'];assert sum(math.comb(5,j)*(row['domain_size']-1)**(5-j)for j in range(2,6))==row['zero_ordered_count'];small+=1
 actual=out['actual'];f=Field(83,actual['modulus']);z=actual['z'];counts=[];first=[]
 for row in actual['cells']:
  domains=[[a^j for j in range(128)]for a in row['offsets']];assert all(x and x<1<<18 for d in domains for x in d);first.append(row['offsets'][0]);iv=inverses(f,[z]+[x for d in domains for x in d]);pairs={}
  for x in domains[0]:
   for y in domains[3]:pairs[iv[x]^iv[y]]=pairs.get(iv[x]^iv[y],0)+1
  count=0
  for x in domains[1]:
   for y in domains[2]:
    t=iv[z]^iv[x]^iv[y]
    for zz in domains[4]:count+=pairs.get(t^iv[zz],0)
  assert count==row['exact_reciprocal_count'];counts.append(count);assert time.monotonic()-START<90
 assert len(first)==len(set(first));assert actual['source_assignments_covered']==len(counts)*128**5
 print(json.dumps(dict(independent_small_profiles=small,actual_cell_counts=counts,ordered_source_assignments_covered=actual['source_assignments_covered'],algorithm='different pair partition and independent batch/Fermat inverse',seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,qualified_speedup=None)))
if __name__=='__main__':main()
