"""Independent original-norm affine fiber replay, different Gauss-Jordan solver."""
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
 blob=(R/'research'/folder/'raw.json').read_bytes();assert hashlib.sha256(blob).hexdigest()==sha;v=json.loads(blob);assert v['error']is None;return v['out']
def reduce(rows,width):
 rows=[list(r)for r in rows];pivot=[];i=0
 for bit in range(width):
  j=next((j for j in range(i,len(rows))if rows[j][0]>>bit&1),None)
  if j is None:continue
  rows[i],rows[j]=rows[j],rows[i]
  for j in range(len(rows)):
   if j!=i and rows[j][0]>>bit&1:rows[j][0]^=rows[i][0];rows[j][1]^=rows[i][1]
  pivot.append(bit);i+=1
 if any(not m and r for m,r in rows):return None
 p=sum(rows[j][1]<<bit for j,bit in enumerate(pivot));B=[]
 for free in range(width):
  if free not in pivot:B.append((1<<free)^sum(((rows[j][0]>>free)&1)<<bit for j,bit in enumerate(pivot)))
 return p,B

def canonical(B):
 B=list(B);i=0
 for bit in range(165,-1,-1):
  j=next((j for j in range(i,len(B))if B[j]>>bit&1),None)
  if j is None:continue
  B[i],B[j]=B[j],B[i]
  for j in range(len(B)):
   if j!=i and B[j]>>bit&1:B[j]^=B[i]
  i+=1
 assert i==len(B);return list(reversed(B))
def main():
 os.sched_setaffinity(0,{min(os.sched_getaffinity(0))});resource.setrlimit(resource.RLIMIT_AS,(4*1024**3,4*1024**3))
 data=load('norm_block_reuse_20261115_v36','3190d2fc3cad1bc58e56e7f1d4b8649f8077ef357312c5a93903d3520fc03a20');aa=data['actual'];prior=load('inversion_free_full_20261115_v35','e5d314dec3b2baf029004d7e6c9ec66f05a5108e9d52b49a04905cf51966ae82')['actual'];ctx=prior['context'];f=Field(83,aa['modulus']);z=prior['z'];y=prior['y'];cc=ctx['c'];outputs=[];total=0
 for row in aa['fibers']:
  h=row['h'];k=row['k']
  def packed(r,s):
   u=r^z^f.mul(cc,h);v=s^f.mul(z,r)^f.sq(z)^f.mul(y,h)^f.mul(cc,k)
   rr,ss,H=literal(f,0,1,z,y,cc,h,k,u,v);assert(rr,ss)==(r,s)
   return(H[3]>>35)|((H[2]>>52)<<48)|((H[1]>>69)<<79)
  base=packed(0,0);cols=[packed((1<<i)if i<83 else 0,(1<<(i-83))if i>=83 else 0)^base for i in range(166)]
  rows=[(sum(((v>>bit)&1)<<j for j,v in enumerate(cols)),(base>>bit)&1)for bit in range(93)];sol=reduce(rows,166);assert(sol is not None)==row['consistent']
  if sol is None:p=None;B=[]
  else:
   p,B=sol;B=canonical(B)
   for b in reversed(B):
    if p>>(b.bit_length()-1)&1:p^=b
   assert len(B)==row['nullity']
  digest=hashlib.sha256(json.dumps([p,B],separators=(',',':')).encode()).hexdigest();assert digest==row['canonical_sha256'];outputs.append(dict(h=h,k=k,particular=p,basis=B));total+=167
  assert time.monotonic()-START<90
 output=json.dumps(outputs,separators=(',',':')).encode();assert hashlib.sha256(output).hexdigest()=='25ab19b648eca5e259679e9707b74727e86d7d005b75a6ad9f7c667dc2b1f4e8'
 print(json.dumps(dict(full_fibers=len(outputs),literal_norm_evaluations=total,full_output_bytes=len(output),seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,qualified_speedup=None)))
if __name__=='__main__':main()
