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
def pmul(f,a,b):
 v=[0]*(len(a)+len(b)-1)
 for i,x in enumerate(a):
  for j,y in enumerate(b):v[i+j]^=f.mul(x,y)
 return v
def literal(f,A,B,z,y,cc,h,k,u,v):
 r=z^f.mul(cc,h)^u;w=f.sq(z)^f.mul(y,h)^f.mul(cc,k)^v;s=w^f.mul(z,r)
 t=f.mul(f.sq(z),z)^f.mul(r,f.sq(z))^f.mul(s,z)^f.mul(f.mul(h,z)^k,y)
 F=[t,s,r,1];G=[k,h];a=pmul(f,F,F);b=pmul(f,pmul(f,[0,1],F),G);e=pmul(f,[B,0,A,1],pmul(f,G,G));N=[0]*7
 for p in[a,b,e]:
  for i,x in enumerate(p):N[i]^=x
 H=[0]*6;H[5]=N[6]
 for i in range(5,0,-1):H[i-1]=N[i]^f.mul(z,H[i])
 assert N[0]==f.mul(z,H[0])and H[5]==1;return r,s,H

def load(folder,sha):
 p=R/'research'/folder/'raw.json';blob=p.read_bytes();assert hashlib.sha256(blob).hexdigest()==sha;d=json.loads(blob);assert d['error']is None;return d['out']
def replay_tiny(records,full):
 count=0
 for row in records:
  n=row['n'];f=Field(n,row['modulus']);q=f.q;dig=hashlib.sha256();cc=row['c'];z=row['z'];y=row['y'];A=row['A'];B=row['B']
  assert f.sq(cc)^cc==A^z
  if not full:pts=((h,0,u,v)for h,u,v in itertools.product(range(q),repeat=3))
  elif row['complete']:pts=itertools.product(range(q),repeat=4)
  else:pts=((i%q,(i//q)%q,((i//(q*q))+3)%q,(i*11+5)%q)for i in range(1024))
  seen=0
  for h,k,u,v in pts:
   r,s,H=literal(f,A,B,z,y,cc,h,k,u,v);dig.update(bytes(([h,k,u,v]if full else[h,u,v])+[r,s]+H));seen+=1
  assert seen==row['count']and dig.hexdigest()==row['sha256'];count+=seen
 return count

def replay_actual(row,full):
 f=Field(83,row['modulus']);ctx=row['context'];cc=ctx['c'];z=row['z'];y=row['y'];assert f.sq(cc)^cc==row['A']^z and ctx['d']==y^f.mul(z,cc)
 n=row['boolean_variables'];points=itertools.chain([0],(1<<i for i in range(n)),((1<<i)|(1<<j)for j in range(n)for i in range(j)));mask=(1<<83)-1;dig=hashlib.sha256();count=0
 for bits in points:
  h=bits&mask
  if full:k=(bits>>83)&mask;u=(bits>>166)&mask;v=bits>>249
  else:k=0;u=(bits>>83)&mask;v=bits>>166
  _,_,H=literal(f,row['A'],row['B'],z,y,cc,h,k,u,v)
  for x in H:dig.update(x.to_bytes(11,'little'))
  count+=1
  if count%4096==0:assert time.monotonic()-START<110
 assert count==row['complete_anf_evaluations']and dig.hexdigest()==row['sha256'];return count

def main():
 os.sched_setaffinity(0,{min(os.sched_getaffinity(0))});resource.setrlimit(resource.RLIMIT_AS,(4*1024**3,4*1024**3))
 a=load('inversion_free_oracles_20261115_v35','86208cadba8b473551b31c7caae6171cddd1e85773aa125c0b573f211aa9c12f');b=load('inversion_free_full_20261115_v35','e5d314dec3b2baf029004d7e6c9ec66f05a5108e9d52b49a04905cf51966ae82');t=load('inversion_free_cost_20261115_v35','0632c4864d0157cbff47211ded83c9ff6ffd58ef262c59e82c8b342f7974d99a')
 counts=[replay_tiny(a['tiny']['records'],False),replay_tiny(b['controls']['records'],True),replay_actual(a['actual'],False),replay_actual(b['actual'],True)]
 import statistics
 assert len(t['records'])==6 and len({x['output_sha256']for x in t['records']})==1
 for method in['strong','normalized']:
  vals=[x['cold_wall_seconds']for x in t['records']if x['method']==method];assert statistics.median(vals)==t['median_cold_wall_seconds'][method];assert max(vals)/min(vals)-1==t['AA_spread'][method]
 print(json.dumps(dict(independent_literal_counts=counts,paid_component_ratio=t['paid_component_ratio'],qualified_speedup=None,seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss)))
if __name__=='__main__':main()
