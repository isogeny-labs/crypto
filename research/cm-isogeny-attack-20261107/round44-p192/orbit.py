from pathlib import Path
import importlib.util,hashlib
s=importlib.util.spec_from_file_location('cache18',Path(__file__).resolve().parent.parent/'round18-p192/cached_orbit.py')
old=importlib.util.module_from_spec(s);s.loader.exec_module(old)
p,r,G,mu=old.p,old.r,old.G,old.mu
class Field:
 def __init__(self):self.cost={k:0 for k in ['M','S','I','A','K']}
 def mul(self,a,b):self.cost['M']+=1;return a*b%p
 def sq(self,a):self.cost['S']+=1;return a*a%p
 def inv(self,a):self.cost['I']+=1;return pow(a,-1,p)
 def add(self,a,b):self.cost['A']+=1;return (a+b)%p
 def sub(self,a,b):self.cost['A']+=1;return (a-b)%p
 def small(self,k,a):self.cost['K']+=1;return k*a%p
def invert(ds,F):
 if not ds:return []
 pre=[ds[0]]
 for d in ds[1:]:pre.append(F.mul(pre[-1],d))
 z=F.inv(pre[-1]);out=[0]*len(ds)
 for i in range(len(ds)-1,0,-1):out[i]=F.mul(z,pre[i-1]);z=F.mul(z,ds[i])
 out[0]=z;return out
def add_many(pairs,F):
 out=[None]*len(pairs);work=[];branches=[]
 for i,(P,Q) in enumerate(pairs):
  if P is None:out[i]=Q;branches.append('identity');continue
  if Q is None:out[i]=P;branches.append('identity');continue
  x,y=P;u,v=Q
  if x==u and (y+v)%p==0:branches.append('cancel');continue
  if P==Q:
   den=F.small(2,y);num=F.add(F.small(3,F.sq(x)),p-3);branches.append('double')
  else:den=F.sub(u,x);num=F.sub(v,y);branches.append('generic')
  work.append((i,P,Q,num,den))
 invs=invert([w[4] for w in work],F)
 for (i,P,Q,num,_),di in zip(work,invs):
  x,y=P;u,v=Q;s=F.mul(num,di);X=F.sub(F.sub(F.sq(s),x),u)
  out[i]=(X,F.sub(F.mul(s,F.sub(x,X)),y))
 return out,branches
def canonical(T,F):
 if T[0] is None:return list(T),0,1
 j=min(range(len(T)),key=lambda i:T[i][0]);minus=F.sub(0,T[j][1]);sign=1 if T[j][1]<minus else -1
 out=list(T[j:])+list(T[:j])
 if sign==-1:out=[(P[0],minus if i==0 else F.sub(0,P[1])) for i,P in enumerate(out)]
 return out,j,sign
def label(a,b,j,sign):
 z=sign*pow(mu,j,r)%r;return [z*a%r,z*b%r]
def key(P):return b'identity' if P is None else P[0].to_bytes(24,'big')+P[1].to_bytes(24,'big')
def partition(P):return int.from_bytes(hashlib.sha256(key(P)).digest()[:8],'big')%16
def encode(P,m):return list(old.encode(P)) if m==5 else [P]
def recover(left,right):
 a,b=left;c,d=right;den=(d-b)%r
 return None if not den else (a-c)*pow(den,-1,r)%r
