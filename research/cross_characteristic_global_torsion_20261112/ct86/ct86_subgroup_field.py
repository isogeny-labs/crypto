#!/usr/bin/env python3
"""CT86, self-contained exact subgroup-kernel field witness for 11a1[7].

All checks are arithmetic. This does not certify cryptanalytic complexity.
"""

def trim(a):
    a=list(a)
    while len(a)>1 and a[-1]==0:a.pop()
    return a

def plus(a,b):return trim([(a[i] if i<len(a) else 0)+(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))])
def minus(a,b):return trim([(a[i] if i<len(a) else 0)-(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))])
def times(a,b):
    z=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):z[i+j]+=x*y
    return trim(z)
def ppow(a,k):
    z=[1]
    while k:
        if k&1:z=times(z,a)
        a=times(a,a);k>>=1
    return z

def pseudorem(a,b):
    z=trim(a)
    while len(z)>=len(b) and z!=[0]:
        degree=len(z)-len(b);lead=z[-1]
        z=minus([b[-1]*u for u in z],[0]*degree+[lead*v for v in b])
    return z

def verified(cond):
    if not cond:raise AssertionError('CT86 exact certificate failed')

X=[0,1]; D=[-79,-40,-4,4]; psi3=[-21,-237,-60,-4,3]; f4=[-5821,-1496,-210,-790,-100,-4,2]
psi5=minus(times(ppow(D,2),f4),ppow(psi3,3))
psi7=minus(times(psi5,ppow(psi3,3)),times(ppow(D,2),ppow(f4,3)))
verified(len(psi7)==25 and psi7[-1]==7)
A2=plus(ppow([-10,-2,3],2),times([1,-2],D))
B2=D
A3=minus(times(X,ppow(psi3,2)),times(D,f4))
B3=ppow(psi3,2)
# trace from the three x-coordinates of one order-seven subgroup
Den=times(B2,B3)
Num=plus(plus(times(X,Den),times(A2,B3)),times(A3,B2))
# Resultant_res_X(psi7(X), Z*Den-Num) = constant*g(Z)^3.
# Verify the small candidate g by exact integer polynomial divisibility.
G8=[-2102915,3828478,-6423347,-1696464,-39690,-29862,-840,-8,1]
poly=[0]
for i,c in enumerate(G8):
    poly=plus(poly,[c*v for v in times(ppow(Num,i),ppow(Den,8-i))])
verified(pseudorem(poly,psi7)==[0])

def modpoly(a,p):return trim([x%p for x in a])
def divrem(a,b,p):
    z=modpoly(a,p);b=modpoly(b,p)
    verified(b!=[0]);inv=pow(b[-1],-1,p)
    while len(z)>=len(b) and z!=[0]:
        k=len(z)-len(b);lc=z[-1]*inv%p
        z=modpoly(minus(z,[0]*k+[lc*u for u in b]),p)
    return z

def gcd(a,b,p):
    while b!=[0]:a,b=b,divrem(a,b,p)
    return modpoly([x*pow(a[-1],-1,p) for x in a],p)

def mpow(a,e,m,p):
    out=[1]
    while e:
        if e&1:out=divrem(times(out,a),m,p)
        a=divrem(times(a,a),m,p);e>>=1
    return out

# Rabin's exact criterion, degree eight (prime divisor 2)
g3=modpoly(G8,3)
verified(mpow(X,3**8,g3,3)==X)
verified(gcd(g3,minus(mpow(X,3**4,g3,3),X),3)==[1])
# Source subgroup = rational 1-eigenspace of Frobenius on E[7].
p=37
def src_add(P,Q):
    if P is None:return Q
    if Q is None:return P
    x,y=P;u,v=Q
    if x==u and (y+v+1)%p==0:return None
    if P==Q:
        k=pow((2*y+1)%p,-1,p);lam=(3*x*x-2*x-10)*k%p
    else:lam=(v-y)*pow((u-x)%p,-1,p)%p
    nu=(y-lam*x)%p
    X3=(lam*lam+1-x-u)%p;Y3=(-lam*X3-nu-1)%p
    verified((Y3*Y3+Y3-X3**3+X3**2+10*X3+20)%p==0)
    return X3,Y3
src=[None];t=None
for _ in range(7):
    t=src_add(t,(9,4));src.append(t)
verified(src[7] is None and len(set(src[:7]))==7)
verified(src[1:7]==[(9,4),(27,9),(28,0),(28,36),(27,27),(9,32)])
sxs=[src[j][0] for j in (1,2,3)]
s1=sum(sxs)%37;s2=(sxs[0]*sxs[1]+sxs[0]*sxs[2]+sxs[1]*sxs[2])%37;s3=sxs[0]*sxs[1]*sxs[2]%37
verified((s1,s2,s3)==(27,30,33))
def evaluate(a,x,p):
    v=0
    for c in a[::-1]:v=(v*x+c)%p
    return v
verified(evaluate(G8,s1,37)==0)
verified(evaluate(Den,9,37)!=0 and evaluate(Num,9,37)*pow(evaluate(Den,9,37),-1,37)%37==s1)
# Binary F_2^12 modulus Z^12+Z^9+Z^5+Z^2+1; tower ordinate y^2+y=alpha^3+alpha^2.
MOD=0x1225;mask=4095
def gf_mul(u,v):
    a=0
    while v:
        if v&1:a^=u
        u<<=1;v>>=1
        if u&4096:u^=MOD
    return a&mask
def gf_pow(u,n):
    z=1
    while n:
        if n&1:z=gf_mul(z,u)
        u=gf_mul(u,u);n>>=1
    return z
h=gf_mul(gf_mul(2,2),2)^gf_mul(2,2)
O=(0,0);one=(1,0);alpha=(2,0);Y=(0,1)
def bf_add(a,b):return a[0]^b[0],a[1]^b[1]
def bf_mul(a,b):
    t=gf_mul(a[1],b[1]);return gf_mul(a[0],b[0])^gf_mul(t,h),gf_mul(a[0],b[1])^gf_mul(a[1],b[0])^t
def bf_pow(u,n):
    z=one
    while n:
        if n&1:z=bf_mul(z,u)
        u=bf_mul(u,u);n>>=1
    return z
def bf_div(a,b):return bf_mul(a,bf_pow(b,2**24-2))
def dst_add(P,Q):
    if P is None:return Q
    if Q is None:return P
    x,y=P;u,v=Q
    if x==u and bf_add(bf_add(y,v),one)==O:return None
    if P==Q:lam=bf_mul(x,x)
    else:lam=bf_div(bf_add(y,v),bf_add(x,u))
    nu=bf_add(y,bf_mul(lam,x))
    z=bf_add(bf_add(bf_add(bf_mul(lam,lam),one),x),u)
    w=bf_add(bf_add(bf_mul(lam,z),nu),one)
    verified(bf_add(bf_mul(w,w),w)==bf_add(bf_mul(bf_mul(z,z),z),bf_mul(z,z)))
    return z,w
dst=[None];t=None
for _ in range(7):t=dst_add(t,(alpha,Y));dst.append(t)
verified(dst[7] is None and len(set(dst[:7]))==7)
xx=[dst[j][0] for j in (1,2,3)]
cs=[bf_add(bf_add(xx[0],xx[1]),xx[2]),bf_add(bf_add(bf_mul(xx[0],xx[1]),bf_mul(xx[0],xx[2])),bf_mul(xx[1],xx[2])),bf_mul(bf_mul(xx[0],xx[1]),xx[2])]
verified(cs==[(1545,0),(1906,0),(1261,0)])
verified(all(c[1]==0 and bf_pow(c,16)==c and bf_pow(c,4)!=c for c in cs))
verified(bf_pow(dst[1][0],16)==dst[3][0] and bf_pow(dst[1][1],16)==dst[3][1])
verified((bf_pow(dst[2][0],16),bf_pow(dst[2][1],16))==dst[6])  # one-sided Frobenius changes log 2 -> 6
verified((bf_pow(dst[1][0],2),bf_pow(dst[1][1],2)) not in dst[1:7])
# Over F_2 the degree-eight s1 polynomial is inseparable modulo the chosen power-basis order,
# but its *chosen residue* has degree four. Do not infer arithmetic ramification from this alone.
verified(modpoly(G8,2)==[1,0,1,0,0,0,0,0,1])  # (Z^4+Z+1)^2
verified((gf_pow(1545,4)^1545^1)==0)
print('CT86 PASS: K_aut=6 L_aut=3 kernel_degree=8 src=(27,30,33) binary=(1545,1906,1261) residue_degree=4 frob4=3 wrong_Q=6')
