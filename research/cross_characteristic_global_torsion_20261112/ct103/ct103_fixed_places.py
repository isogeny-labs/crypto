#!/usr/bin/env python3
"""CT103: two explicit residue maps of one degree-120 global eleven-torsion algebra.
No arbitrary-public-Q evaluation or target DLP is performed.
"""
import json
from collections import Counter

def check(v,msg):
    if not v: raise ArithmeticError(msg)
def add(a,b):
    c=[(a[i] if i<len(a) else 0)+(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))]
    while len(c)>1 and c[-1]==0:c.pop()
    return c
def sub(a,b):return add(a,[-v for v in b])
def mul(a,b):
    c=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):c[i+j]+=x*y
    return c
def ppow(a,n):
    r=[1]
    while n:
        if n&1:r=mul(r,a)
        a=mul(a,a);n//=2
    return r
D=[-79,-40,-4,4]
a=[-21,-237,-60,-4,3]
b=[-5821,-1496,-210,-790,-100,-4,2]
f5=sub(mul(ppow(D,2),b),ppow(a,3))
f6=mul(a,sub(f5,ppow(b,2)))
f7=sub(mul(f5,ppow(a,3)),mul(ppow(D,2),ppow(b,3)))
f11=sub(mul(f7,ppow(f5,3)),mul(mul(ppow(D,2),b),ppow(f6,3)))
check(len(f11)==61 and f11[60]==11,"degree-60 division polynomial")
def ev(f,x,p):
    z=0
    for c in reversed(f):z=(z*x+c)%p
    return z
check(ev(f11,19,59)==0,"source f11 root")
check((22**2+22-(19**3-19**2-10*19-20))%59==0,"source point")
check((2*22+1)%59==45,"source y derivative")
check(sum((i*c*pow(19,i-1,59)) for i,c in enumerate(f11) if i)%59==17,
      "source f derivative")
# Target F2^20 via g and oriented F2^40 via u^2+u=t^3+t^2.
g=0x130027; mask=(1<<20)-1
def kmul(a,b):
    r=0
    for _ in range(20):
        if b&1:r^=a
        b>>=1;a<<=1
        if a>>20:a^=g
    check(b==0,"20-bit field multiply")
    return r&mask
def kpow(a,n):
    z=1
    while n:
        if n&1:z=kmul(z,a)
        a=kmul(a,a);n//=2
    return z
def bmul(a,b):
    z=0
    while b:
        if b&1:z^=a
        a<<=1;b>>=1
    return z
def brem(a,b):
    while a and a.bit_length()>=b.bit_length():a^=b<<(a.bit_length()-b.bit_length())
    return a
def bgcd(a,b):
    while b:a,b=b,brem(a,b)
    return a
f2=sum((c&1)<<i for i,c in enumerate(f11))
check(f2==bmul(bmul(g,0x131203),0x131211),"target f11 factorization")
check(kpow(2,1<<20)==2,"target x degree divides 20")
for n in (4,10):check(bgcd(kpow(2,1<<n)^2,g)==1,"target Rabin irreducibility")
sq=lambda x:kmul(x,x)
r=kmul(sq(2),2)^sq(2)
tr=0;z=r
for _ in range(20):tr^=z;z=sq(z)
check(tr==1 and z==r,"quadratic u^2+u+r irreducible")
def orbits(A):
    seen=set();counts=Counter()
    def act(v):return ((A[0][0]*v[0]+A[0][1]*v[1])%11,(A[1][0]*v[0]+A[1][1]*v[1])%11)
    for x in range(11):
        for y in range(11):
            v=(x,y)
            if v==(0,0) or v in seen:continue
            w=v;n=0
            while w not in seen:
                seen.add(w);n+=1;w=act(w)
            check(w==v,"Frobenius cycle")
            counts[n]+=1
    check(len(seen)==120,"full E[11] minus identity")
    return sorted([list(z) for z in counts.items()])
src=orbits(((0,7),(1,5)));dst=orbits(((0,9),(1,9)))
check(src==[[1,10],[5,22]],"source 59 splitting")
check(dst==[[40,3]],"target 2 splitting")
print(json.dumps({
 "case":"ct103-fixed-oriented-primes","ell":11,"fixed_specialization_ideals":True,
 "global_oriented_degree":120,"global_x_degree":60,
 "public_point_transfer_evaluated":False,"source_generator":[19,22],"source_modulus":59,
 "source_oriented_frobenius_cycles":src,"source_point_scalar_recovered":False,
 "source_y_derivative":45,"target_dlp_evaluated":False,"target_modulus":2,
 "target_oriented_frobenius_cycles":dst,"target_residue_degree":40,
 "target_rhs_trace":tr,"target_x_factor":"0x130027"
},sort_keys=True,separators=(',',':')))
