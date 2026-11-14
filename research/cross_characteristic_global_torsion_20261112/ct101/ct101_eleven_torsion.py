#!/usr/bin/env python3
"""CT101: exact F59/F2 splitting and explicit characteristic-two E[11] rings.
Only Python stdlib, fixed public 11a1 fixture; no source-to-target transfer.
"""
import json

def trim(a):
    a=list(a)
    while len(a)>1 and a[-1]==0:a.pop()
    return a

def add(a,b):
    return trim([(a[i] if i<len(a) else 0)+(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))])

def neg(a):return [-v for v in a]
def sub(a,b):return add(a,neg(b))
def mul(a,b):
    c=[0]*(len(a)+len(b)-1)
    for i,u in enumerate(a):
        for j,v in enumerate(b):c[i+j]+=u*v
    return trim(c)
def power(a,n):
    r=[1]
    while n:
        if n&1:r=mul(r,a)
        a=mul(a,a);n>>=1
    return r

D=[-79,-40,-4,4]
f3=[-21,-237,-60,-4,3]
f4=[-5821,-1496,-210,-790,-100,-4,2]
f5=sub(mul(power(D,2),f4),power(f3,3))
f6=mul(f3,sub(f5,power(f4,2)))
f7=sub(mul(f5,power(f3,3)),mul(power(D,2),power(f4,3)))
f11=sub(mul(f7,power(f5,3)),mul(mul(power(D,2),f4),power(f6,3)))
assert len(f11)==61 and f11[-1]==11

def rem(a,b,p):
    a=trim([z%p for z in a]);b=trim([z%p for z in b]);iv=pow(b[-1],-1,p)
    while len(a)>=len(b) and a!=[0]:
        k=len(a)-len(b);v=a[-1]*iv%p
        for i,z in enumerate(b):a[k+i]=(a[k+i]-v*z)%p
        a=trim(a)
    return a
def gcdpoly(a,b,p):
    a=trim([z%p for z in a]);b=trim([z%p for z in b])
    while b!=[0]:a,b=b,rem(a,b,p)
    return trim([(z*pow(a[-1],-1,p))%p for z in a])
def modmul(a,b,m,p):return rem(mul(a,b),m,p)
def modpow(a,e,m,p):
    r=[1]
    while e:
        if e&1:r=modmul(r,a,m,p)
        a=modmul(a,a,m,p);e>>=1
    return r

q59=[z%59 for z in f11]
source_roots=[i for i in range(59) if rem(q59,[-i,1],59)==[0]]
assert source_roots==[19,20,22,43,46]
x=[0,1]
g1=gcdpoly(q59,sub(modpow(x,59,q59,59),x),59)
g5=gcdpoly(q59,sub(modpow(x,59**5,q59,59),x),59)
assert len(g1)-1==5 and len(g5)-1==60
assert gcdpoly(q59,[(i*q59[i])%59 for i in range(1,len(q59))],59)==[1]

def bits(a):return sum((z&1)<<i for i,z in enumerate(a))
def p2mul(a,b):
    r=0
    while b:
        if b&1:r^=a
        a<<=1;b>>=1
    return r
def p2rem(a,m):
    d=m.bit_length()-1
    while a.bit_length()-1>=d:a^=m<<(a.bit_length()-1-d)
    return a
def p2gcd(a,b):
    while b:a,b=b,p2rem(a,b)
    return a
def p2square(a,m):return p2rem(p2mul(a,a),m)

f2=bits(f11)
factors=[0x130027,0x131203,0x131211]
prod=1
for f in factors:prod=p2mul(prod,f)
assert f2==prod
for f in factors:
    assert f.bit_length()==21 and f&1
    z=2
    for k in range(1,21):
        z=p2square(z,f)
        if k in (4,10):assert p2gcd(z^2,f)==1
    assert z==2
assert p2gcd(f2, p2rem(sum(((i&1)&(f2>>i)) << (i-1) for i in range(1,61)),f2))==1

mask=(1<<20)-1
def kmul(a,b,f):
    r=0
    for _ in range(20):
        if b&1:r^=a
        b>>=1;a<<=1
        if a&(1<<20):a^=f
    assert b==0
    return r&mask
def kpow(a,n,f):
    r=1
    while n:
        if n&1:r=kmul(r,a,f)
        a=kmul(a,a,f);n>>=1
    return r
def ksq(a,f):return kmul(a,a,f)
def kinv(a,f):
    assert a
    r=kpow(a,(1<<20)-2,f)
    assert kmul(a,r,f)==1
    return r
def curve_rhs(x,f):return kmul(ksq(x,f),x,f)^ksq(x,f)
def on_curve(P,f,rad):
    if P is None:return True
    xx,yy=P
    return ksq(yy,f)^yy^rad==curve_rhs(xx,f)
def point_add(P,Q,f,rad):
    if P is None:return Q
    if Q is None:return P
    x1,y1=P;x2,y2=Q
    if x1==x2:
        if y1^y2==1:return None
        assert y1==y2
        slope=ksq(x1,f)
    else:slope=kmul(y1^y2,kinv(x1^x2,f),f)
    x3=ksq(slope,f)^1^x1^x2
    y3=y1^kmul(slope,x1^x3,f)^1
    R=(x3,y3)
    assert on_curve(R,f,rad)
    return R

summaries=[]
for f in factors:
    t=2
    rad=curve_rhs(t,f)
    tr=0;cur=rad
    for _ in range(20):tr^=cur;cur=ksq(cur,f)
    assert tr==1 and cur==rad
    P=(t,0);assert on_curve(P,f,rad)
    multiples=[None]
    for j in range(1,12):multiples.append(point_add(multiples[-1],P,f,rad))
    assert multiples[11] is None and len(set(multiples[:11]))==11
    for i in range(11):
        for j in range(11):
            assert point_add(multiples[i],multiples[j],f,rad)==multiples[(i+j)%11]
    conjugates=[];u=t;y0=0
    for k in range(40):
        conjugates.append((u,y0))
        u=ksq(u,f);y0=ksq(y0,f)^rad
    assert len(set(conjugates))==40 and u==t and y0==0
    assert conjugates[20]==(t,1)
    for xx,yy in conjugates:assert on_curve((xx,yy),f,rad)
    summaries.append({'factor':hex(f),'trace_rhs':tr,'x_orbit':20,'oriented_orbit':40,'order11_verified':True,'group_pairs':121})

p=59
rhs=lambda x:(x**3-x*x-10*x-20)%p
source_count=1+sum(1 for a in range(p) for b in range(p) if (b*b+b-rhs(a))%p==0)
assert source_count==55
def add59(P,Q):
    if P is None:return Q
    if Q is None:return P
    x1,y1=P;x2,y2=Q
    if x1==x2:
        if (y1+y2+1)%p==0:return None
        slope=((3*x1*x1-2*x1-10)*pow(2*y1+1,-1,p))%p
    else:slope=((y2-y1)*pow((x2-x1)%p,-1,p))%p
    xx=(slope*slope+1-x1-x2)%p
    yy=(-y1-slope*(xx-x1)-1)%p
    return (xx,yy)

Q=(19,22);A=None;points=[]
for j in range(11):
    points.append(A)
    A=add59(A,Q)
assert A is None and len(set(points))==11
assert sorted({P[0] for P in points[1:]})==source_roots
assert all((P is None or (P[1]**2+P[1]-rhs(P[0]))%p==0) for P in points)

print(json.dumps({'case':'ct101-eleven-field-gate','ell':11,'source_p':59,
    'source_order':55,'source_11_generator':[19,22],
    'source_x_roots':source_roots,'source_x_factor_degrees':[1]*5+[5]*11,
    'target_p':2,'target_x_factor_degrees':[20]*3,'binary_factors':summaries,
    'source_group_pairs':121,'target_group_pairs':363,
    'required_oriented_target_degree':40,'cross_characteristic_lift_evaluated':False},sort_keys=True,separators=(',',':')))
