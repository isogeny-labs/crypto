"""Prepared odd-characteristic quintic norm compiler; not executed."""
def addpoly(F,A,B):
 out=[0]*max(len(A),len(B))
 for i,v in enumerate(A):out[i]=F.add(out[i],v)
 for i,v in enumerate(B):out[i]=F.add(out[i],v)
 return out
def product(F,A,B):
 out=[0]*(len(A)+len(B)-1)
 for i,a in enumerate(A):
  for j,b in enumerate(B):out[i+j]=F.add(out[i+j],F.mul(a,b))
 return out
def divide_target(F,N,z):
 assert len(N)==7 and N[6]==1
 H=[0]*6;H[5]=1
 for i in range(4,-1,-1):H[i]=F.add(N[i+1],F.mul(z,H[i+1]))
 assert F.add(N[0],F.mul(z,H[0]))==0
 return H
def literal(F,a,b,z,y,h,k,r,s):
 two=F.add(1,1);assert two
 z2=F.mul(z,z);t=F.neg(F.add(F.add(F.mul(z2,z),F.mul(r,z2)),F.add(F.mul(s,z),F.mul(F.add(F.mul(h,z),k),y))))
 C=[t,s,r,1];G=[k,h]
 norm=addpoly(F,product(F,C,C),[F.neg(x)for x in product(F,[b,a,0,1],product(F,G,G))])
 return dict(r=r,s=s,t=t,H=divide_target(F,norm,z))
def compile_moments(F,a,b,z,y,h,k,e1,e2):
 two=F.add(1,1);assert two
 inv2=F.inv(two);sq=lambda x:F.mul(x,x);mul=F.mul
 assert sq(y)==F.add(F.add(mul(sq(z),z),mul(a,z)),b)
 r=mul(F.sub(F.sub(sq(h),z),e1),inv2)
 s=mul(F.add(F.add(F.sub(e2,sq(r)),mul(two,mul(h,k))),mul(z,e1)),inv2)
 result=literal(F,a,b,z,y,h,k,r,s)
 assert result['H'][4]==F.neg(e1)and result['H'][3]==e2
 return result
