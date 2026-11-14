#!/usr/bin/env python3
"""CT109 modular proposal: minimal relations in Fp[X]/(psi11)."""
import json

def ck(ok,msg):
    if not ok: raise ArithmeticError(msg)
def trim(a):
    a=list(a)
    while len(a)>1 and a[-1]==0:a.pop()
    return a
def add(a,b):
    return trim([(a[i] if i<len(a) else 0)+(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))])
def sub(a,b):return add(a,[-x for x in b])
def mul(a,b):
    z=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):z[i+j]+=x*y
    return trim(z)
def power(a,n):
    z=[1]
    while n:
        if n&1:z=mul(z,a)
        a=mul(a,a);n>>=1
    return z
D=[-79,-40,-4,4]
f3=[-21,-237,-60,-4,3]
f4=[-5821,-1496,-210,-790,-100,-4,2]
f5=sub(mul(power(D,2),f4),power(f3,3))
f6=mul(f3,sub(f5,power(f4,2)))
f7=sub(mul(f5,power(f3,3)),mul(power(D,2),power(f4,3)))
f11=sub(mul(f7,power(f5,3)),mul(mul(power(D,2),f4),power(f6,3)))
ck(len(f11)==61 and f11[-1]==11,"eleven division polynomial")
X=[0,1]
nums=[X,sub(mul(X,D),f3),
      sub(mul(X,power(f3,2)),mul(D,f4)),
      sub(mul(X,mul(D,power(f4,2))),mul(f5,f3)),
      sub(mul(X,power(f5,2)),mul(D,mul(f6,f4)))]
dens=[[1],D,power(f3,2),mul(D,power(f4,2)),power(f5,2)]
class PolyMod:
    def __init__(self,p):
        self.p=p;self.mod=self.trim(f11);self.degree=len(self.mod)-1
        ck(self.degree==60 and self.mod[-1],"bad quotient")
    def trim(self,a):
        a=[z%self.p for z in a]
        while len(a)>1 and a[-1]==0:a.pop()
        return a
    def plus(self,a,b):return self.trim(add(a,b))
    def minus(self,a,b):return self.trim(sub(a,b))
    def times(self,a,b):
        o=[0]*(len(a)+len(b)-1)
        for i,x in enumerate(a):
            if x:
                for j,y in enumerate(b):o[i+j]=(o[i+j]+x*y)%self.p
        return self.trim(o)
    def qr(self,a,b):
        a=self.trim(a);b=self.trim(b)
        ck(b!=[0],"zero divisor")
        q=[0]*max(1,len(a)-len(b)+1);u=pow(b[-1],-1,self.p)
        while len(a)>=len(b) and a!=[0]:
            k=len(a)-len(b);z=a[-1]*u%self.p;q[k]=z
            for j,y in enumerate(b):a[j+k]=(a[j+k]-z*y)%self.p
            a=self.trim(a)
        return self.trim(q),a
    def rem(self,a):return self.qr(a,self.mod)[1]
    def prod(self,a,b):return self.rem(self.times(a,b))
    def inv(self,a):
        a=self.rem(a);ck(a!=[0],"zero element")
        r0,r1=self.mod[:],a;t0,t1=[0],[1]
        while r1!=[0]:
            q,r=self.qr(r0,r1);r0,r1=r1,r
            t0,t1=t1,self.rem(self.minus(t0,self.times(q,t1)))
        ck(len(r0)==1,"not invertible")
        return self.rem([z*pow(r0[0],-1,self.p) for z in t0])
    def invariant(self):
        s=[0]
        for a,b in zip(nums,dens):
            s=self.rem(self.plus(s,self.prod(self.rem(a),self.inv(b))))
        return s
def relation(F,s):
    pivots={};v=[1];p=F.p
    for k in range(13):
        row=v+[0]*(60-len(v));coeff=[0]*k+[1]
        for j in sorted(pivots):
            if row[j]:
                saved,poly=pivots[j];z=row[j]
                row=[(a-z*b)%p for a,b in zip(row,saved)]
                for i,b in enumerate(poly):coeff[i]=(coeff[i]-z*b)%p
        pivot=next((i for i,z in enumerate(row) if z),None)
        if pivot is None:return F.trim(coeff)
        z=pow(row[pivot],-1,p)
        pivots[pivot]=([x*z%p for x in row],[x*z%p for x in coeff])
        v=F.prod(v,s)
    raise ArithmeticError("no degree <=12 relation")
def panel(primes):
    out={}
    for p in primes:
        F=PolyMod(p);s=F.invariant();q=relation(F,s)
        value=[0];v=[1]
        for c in q:
            value=F.rem(F.plus(value,[c*z%p for z in v]))
            v=F.prod(v,s)
        ck(value==[0],"relation invalid")
        out[str(p)]={'degree':len(q)-1,'coefficients_low_to_high':q}
    return out
if __name__=="__main__":
    import sys
    ps=[int(x) for x in sys.argv[1:]] or [13,17,19,23,29,31,37,41,43,47,59]
    print(json.dumps(panel(ps),sort_keys=True))
