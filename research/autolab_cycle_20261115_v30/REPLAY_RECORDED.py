"""Independent Hasse-coefficient multiplicity oracle and planted-factor replay."""
from pathlib import Path
import json,hashlib,itertools,math,collections
p=Path(__file__).resolve().parents[2]/'research/subspace_factor_oracles_20261115_v30/raw.json';data=p.read_bytes();assert hashlib.sha256(data).hexdigest()=='f5775d673e90e6462021ad859f5368739091ebdf67002e8014713d71d6966593';raw=json.loads(data);assert raw['error']is None;out=raw['out']
class Field:
 def __init__(self,p,d,mod=None,nu=None):self.p=p;self.d=d;self.mod=mod;self.nu=nu
 def add(self,a,b):return a^b if self.p==2 else sum((((a//self.p**i)%self.p+(b//self.p**i)%self.p)%self.p)*self.p**i for i in range(self.d))
 def neg(self,a):return a if self.p==2 else sum((-(a//self.p**i%self.p)%self.p)*self.p**i for i in range(self.d))
 def mul(self,a,b):
  if self.p==2:
   c=0
   for i in range(self.d):
    if b>>i&1:c^=a<<i
   while c.bit_length()>self.d:c^=self.mod<<(c.bit_length()-self.d-1)
   return c
  C=[0]*(2*self.d-1)
  for i in range(self.d):
   for j in range(self.d):C[i+j]+=(a//self.p**i%self.p)*(b//self.p**j%self.p)
  if self.d==2:C[0]+=self.nu*C[2]
  return sum((C[i]%self.p)*self.p**i for i in range(self.d))
 def power(self,a,n):
  v=1
  for _ in range(n):v=self.mul(v,a)
  return v
 def total(self,vs):
  v=0
  for x in vs:v=self.add(v,x)
  return v
 def roots_poly(self,roots):
  H=[1]
  for r in roots:
   N=[0]*(len(H)+1)
   for i,c in enumerate(H):N[i]=self.add(N[i],self.mul(c,self.neg(r)));N[i+1]=self.add(N[i+1],c)
   H=N
  return H
count=0
for f in out['controls']:
 F=Field(f['p'],f['degree'],f['modulus'],f['nu']);D=sorted({F.total([f['offset']]+[F.mul(c,v)for c,v in zip(cs,f['basis'])])for cs in itertools.product(range(F.p),repeat=len(f['basis']))});assert len(D)==F.p**len(f['basis']);q=F.p**F.d;digest=hashlib.sha256();cases=accepted=0
 polys=([*cs,1]for n in range(f['maxdegree']+1)for cs in itertools.product(range(q),repeat=n))
 for H in itertools.chain(polys,[F.roots_poly([D[0]]*4),F.roots_poly([D[-1]]*5)]):
  n=len(H)-1;roots=[]
  for r in D:
   m=next(j for j in range(n+1)if F.total([F.mul(F.mul(c,math.comb(i,j)%F.p),F.power(r,i-j))for i,c in enumerate(H)if i>=j])!=0)
   if m:roots.append((r,m))
  good=sum(m for r,m in roots)==n;row=dict(H=H,supported=good,roots=roots if good else None);digest.update(json.dumps(row,sort_keys=True,separators=(',',':')).encode()+b'\n');cases+=1;accepted+=good
 assert cases==f['cases']and accepted==f['accepted']and digest.hexdigest()==f['transcript_sha256'];count+=cases
actual=out['actual'];assert actual['modulus']==0x800000000200000000007 and actual['basis']==[1<<i for i in range(18)];F=Field(2,83,actual['modulus']);positive=negative=0
for row in actual['records']:
 roots=row['planted_roots'];assert F.roots_poly(roots)==row['H'];good=all(r<1<<18 for r in roots);result=row['result'];assert result['supported']==good
 if good:assert result['roots']==[list(x)for x in sorted(collections.Counter(roots).items())]and result['nodes']<=91;positive+=1
 else:assert result['roots']is None and result['status']=='not_fully_supported';negative+=1
assert count==3800 and positive==32 and negative==16 and out['cases']==3848
print('VERIFIED: all3800tiny support/multiplicity transcripts via independent Hasse coefficients;32completeGF83factorizations and16outside-support negatives.')
