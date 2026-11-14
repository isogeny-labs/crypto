#!/usr/bin/env python3
"""CT111: independently evaluate CT110's U=h(S)/2 at fixed binary primes.

The 2-adic arithmetic is in Z/4[t]/g(t) with g irreducible mod2.
Every assertion uses require() to survive python -O. No source scalar is read.
"""
import json

def require(ok,msg):
    if not ok: raise ArithmeticError(msg)
def trim(a):
    a=list(a)
    while len(a)>1 and a[-1]==0:a.pop()
    return a
def add(a,b):
    return trim([(a[i] if i<len(a) else 0)+(b[i] if i<len(b) else 0)
                 for i in range(max(len(a),len(b)))])
def sub(a,b):return add(a,[-z for z in b])
def mul(a,b):
    v=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):v[i+j]+=x*y
    return trim(v)
def power(a,n):
    v=[1]
    while n:
        if n&1:v=mul(v,a)
        a=mul(a,a);n>>=1
    return v

# Independent integer division-polynomial recurrences from CT109.
D=[-79,-40,-4,4];f3=[-21,-237,-60,-4,3]
f4=[-5821,-1496,-210,-790,-100,-4,2]
f5=sub(mul(power(D,2),f4),power(f3,3))
f6=mul(f3,sub(f5,power(f4,2)))
f7=sub(mul(f5,power(f3,3)),mul(power(D,2),power(f4,3)))
F=sub(mul(f7,power(f5,3)),mul(mul(power(D,2),f4),power(f6,3)))
require(len(F)==61 and F[-1]==11,'bad eleven division polynomial')
X=[0,1]
nums=[X,sub(mul(X,D),f3),
      sub(mul(X,power(f3,2)),mul(D,f4)),
      sub(mul(X,mul(D,power(f4,2))),mul(f5,f3)),
      sub(mul(X,power(f5,2)),mul(D,mul(f6,f4)))]
dens=[[1],D,power(f3,2),mul(D,power(f4,2)),power(f5,2)]
Qmod4=[1,3,3,1,3,2,3,0,3,3,0,0,1]

class Ring4:
    def __init__(self,gbits):
        self.gbits=gbits;self.n=gbits.bit_length()-1
        require(self.n==20,'degree must be twenty')
        self.gs=[(gbits>>i)&1 for i in range(self.n)]
    def red(self,a):
        a=[z%4 for z in a]
        if len(a)<self.n:a+=[0]*(self.n-len(a))
        for k in reversed(range(self.n,len(a))):
            if a[k]:
                for j,b in enumerate(self.gs):
                    if b:a[k-self.n+j]=(a[k-self.n+j]-a[k])%4
        return tuple(a[:self.n])
    def zero(self):return (0,)*self.n
    def const(self,x):return self.red([x])
    def sub(self,a,b):return tuple((x-y)%4 for x,y in zip(a,b))
    def add(self,a,b):return tuple((x+y)%4 for x,y in zip(a,b))
    def mul(self,a,b):
        c=[0]*(2*self.n-1)
        for i,x in enumerate(a):
            if x:
                for j,y in enumerate(b):c[i+j]=(c[i+j]+x*y)%4
        return self.red(c)
    def pow(self,a,k):
        z=self.const(1)
        while k:
            if k&1:z=self.mul(z,a)
            a=self.mul(a,a);k//=2
        return z
    def eval(self,pol,a):
        z=self.zero()
        for x in reversed(pol):z=self.add(self.mul(z,a),self.const(x))
        return z
    def bits(self,a):return sum((x&1)<<i for i,x in enumerate(a))
    def embed(self,z):return self.red([(z>>i)&1 for i in range(self.n)])
    def f2mul(self,a,b):
        r=0
        while b:
            if b&1:r^=a
            b>>=1;a<<=1
            if a&(1<<self.n):a^=self.gbits
        return r
    def f2inv(self,a):
        require(a!=0,'zero binary denominator')
        u,v=a,self.gbits;t0,t1=1,0
        while u!=1:
            k=u.bit_length()-v.bit_length()
            if k<0:u,v=v,u;t0,t1=t1,t0;k=-k
            u^=v<<k;t0^=t1<<k
        while t0.bit_length()>self.n:
            t0^=self.gbits<<(t0.bit_length()-self.n-1)
        require(self.f2mul(a,t0)==1,'bad binary inverse')
        return t0
    def inv(self,a):
        seed=self.embed(self.f2inv(self.bits(a)))
        z=self.mul(seed,self.sub(self.const(2),self.mul(a,seed)))
        require(self.mul(a,z)==self.const(1),'bad modulus-four inverse')
        return z

results=[]
for g in (0x130027,0x131203,0x131211):
    R=Ring4(g);t=R.red([0,1])
    ft=R.eval(F,t)
    require(all(v%2==0 for v in ft),'not a binary division root')
    fprime=[i*F[i] for i in range(1,len(F))]
    derivative=R.bits(R.eval(fprime,t))
    delta=R.f2mul(R.bits(tuple(v//2 for v in ft)),R.f2inv(derivative))
    alpha=R.add(t,tuple(2*((delta>>i)&1) for i in range(R.n)))
    require(R.eval(F,alpha)==R.zero(),'root lift mod4 failed')
    s=R.zero()
    for a,b in zip(nums,dens):
        s=R.add(s,R.mul(R.eval(a,alpha),R.inv(R.eval(b,alpha))))
    require(R.eval(Qmod4,s)==R.zero(),'CT109 q(S) mod4 failed')
    hs=R.add(R.add(R.pow(s,4),s),R.const(1))
    require(all(v%2==0 for v in hs),'h(S) must be even')
    u=R.bits(tuple(v//2 for v in hs))
    sb=R.bits(s);w=R.f2mul(sb,sb)^sb;w2=R.f2mul(w,w)
    label={1:'1',w:'w',w2:'w2'}.get(u,'INVALID')
    require(label!='INVALID','unrecognized prime')
    results.append(dict(alpha_correction=hex(delta),g=hex(g),label=label,
                        s=hex(sb),u=hex(u),w=hex(w),w2=hex(w2)))
require([r['label'] for r in results]==['1','w','w2'],'prime labels swapped')
print(json.dumps({'binary_prime_labels':results,'case':'ct111-fixed-binary-prime-label',
    'division_polynomial_degree':60,'fixed_ct103_prime_u_label':'1',
    'modulus':4,'source_scalar_used':False},
    sort_keys=True,separators=(',',':')))
