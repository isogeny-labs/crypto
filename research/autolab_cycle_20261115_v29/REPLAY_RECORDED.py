"""Independent norm-product and coordinate-map replay; standard library only."""
from pathlib import Path
import json,hashlib,itertools
p=Path(__file__).resolve().parents[2]/'research/odd_norm_oracles_20261115_v29/raw.json';data=p.read_bytes();assert hashlib.sha256(data).hexdigest()=='731c9bec4fce47d83850e94993af6f615487aa67855a20b3f287d033e0db3802';raw=json.loads(data);assert raw['error']is None;out=raw['out']
class F:
 def __init__(self,p,d=1,nu=2):self.p=p;self.d=d;self.nu=nu
 def add(self,a,b):return sum((((a//self.p**i)%self.p+(b//self.p**i)%self.p)%self.p)*self.p**i for i in range(self.d))
 def neg(self,a):return sum((-(a//self.p**i%self.p)%self.p)*self.p**i for i in range(self.d))
 def mul(self,a,b):
  C=[0]*(2*self.d-1)
  for i in range(self.d):
   for j in range(self.d):C[i+j]+=(a//self.p**i%self.p)*(b//self.p**j%self.p)
  if self.d==2:C[0]+=self.nu*C[2]
  return sum((C[i]%self.p)*self.p**i for i in range(self.d))
def total(F,vs):
 x=0
 for v in vs:x=F.add(x,v)
 return x
def multiply(F,A,B):return [total(F,[F.mul(a,B[j-i])for i,a in enumerate(A)if 0<=j-i<len(B)])for j in range(len(A)+len(B)-1)]
def record(F,a,b,z,y,h,k,r,s):
 mul=F.mul;z2=mul(z,z);t=F.neg(total(F,[mul(z2,z),mul(r,z2),mul(s,z),mul(F.add(mul(h,z),k),y)]));C=[t,s,r,1];N=multiply(F,C,C);Q=multiply(F,[b,a,0,1],multiply(F,[k,h],[k,h]))
 for i,q in enumerate(Q):N[i]=F.add(N[i],F.neg(q))
 H=[0]*6;H[5]=1
 for i in range(4,-1,-1):H[i]=F.add(N[i+1],mul(z,H[i+1]))
 assert F.add(N[0],mul(z,H[0]))==0
 e1=F.neg(H[4]);e2=H[3];inv2=pow(2,-1,F.p);r2=mul(total(F,[mul(h,h),F.neg(z),F.neg(e1)]),inv2);s2=mul(total(F,[e2,F.neg(mul(r,r)),mul(2,mul(h,k)),mul(z,e1)]),inv2);assert r2==r and s2==s
 return dict(h=h,k=k,r=r,s=s,e1=e1,e2=e2,t=t,H=H)
def samples(q):
 yield 0,0,0,0
 state=0x29a55
 for i in range(255):
  v=[]
  for j in range(4):state=(state*6364136223846793005+1442695040888963407)&((1<<64)-1);v.append(state%q)
  if i%4==0:v[0]=0
  if i%4==1:v[1]=0
  yield tuple(v)
count=0
for f in out['small']:
 field=F(f['p'],f['degree'],f['nu']);q=f['p']**f['degree'];points=itertools.product(range(q),repeat=4)if q<=9 else samples(q);digest=hashlib.sha256();buckets={};cases=0
 for h,k,r,s in points:
  v=record(field,1,1,0,1,h,k,r,s);digest.update(json.dumps(v,sort_keys=True,separators=(',',':')).encode()+b'\n');cases+=1
  if q<=9:buckets.setdefault((h,k),set()).add((v['e1'],v['e2']))
 assert digest.hexdigest()==f['transcript_sha256']and cases==f['cases']
 if q<=9:assert len(buckets)==q*q and all(len(v)==q*q for v in buckets.values())
 for v in f['witnesses']:assert record(field,1,1,0,1,v['h'],v['k'],v['r'],v['s'])==v
 assert record(field,1,1,0,1,0,0,0,0)['H']==[0,0,0,0,0,1];count+=cases
for f in out['wide']:
 field=F(f['prime']);p=f['prime'];a,b,z,y=(f[k]for k in ['a','b','z','y']);assert (y*y-z**3-a*z-b)%p==0
 for v in f['records']:assert record(field,a,b,z,y,v['h'],v['k'],v['r'],v['s'])==v
 count+=len(f['records'])
assert count==out['cases']==7683
print('VERIFIED: all7683normtranscripts, fullsmall coordinatebijections, repeatedquintic andall160NISTcurve quotient records.')
