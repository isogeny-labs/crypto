#!/usr/bin/env python3
"""CT106: exact primitive C10-invariant in the 11-torsion line field.
Independent of CT101/105 scripts; no scalar-conditioned public-point transfer.
Python uses an integer division polynomial and Fermat inverses mod 13.
"""
import json

def ck(c, message):
    if not c:
        raise ArithmeticError(message)

def trim(v):
    v = list(v)
    while len(v) > 1 and not v[-1]:
        v.pop()
    return v

def add(a,b):
    return trim([(a[i] if i<len(a) else 0) + (b[i] if i<len(b) else 0)
                 for i in range(max(len(a),len(b)))])

def neg(a): return [-x for x in a]
def sub(a,b): return add(a,neg(b))
def mul(a,b):
    out=[0]*(len(a)+len(b)-1)
    for i,v in enumerate(a):
        for j,w in enumerate(b):
            out[i+j] += v*w
    return trim(out)

def power(a,n):
    z=[1]
    while n:
        if n&1:z=mul(z,a)
        a=mul(a,a);n//=2
    return z

D=[-79,-40,-4,4]
f3=[-21,-237,-60,-4,3]
f4=[-5821,-1496,-210,-790,-100,-4,2]
f5=sub(mul(power(D,2),f4),power(f3,3))
f6=mul(f3,sub(f5,power(f4,2)))
f7=sub(mul(f5,power(f3,3)),mul(power(D,2),power(f4,3)))
f11=sub(mul(f7,power(f5,3)),mul(mul(power(D,2),f4),power(f6,3)))
ck(len(f11)==61 and f11[-1]==11,"eleven division polynomial")

p=13
F=trim([i%p for i in f11]);X=[0,1]
def rem(a,b):
    a=trim([x%p for x in a]);b=trim([x%p for x in b]);bi=pow(b[-1],-1,p)
    while len(a)>=len(b) and a!=[0]:
        c=a[-1]*bi%p;k=len(a)-len(b)
        for i,v in enumerate(b):a[k+i]=(a[k+i]-c*v)%p
        a=trim(a)
    return a

def red(a):return rem(a,F)
def fieldadd(a,b):return red(add(a,b))
def fieldmul(a,b):return red(mul(a,b))
def fieldneg(a):return red(neg(a))
def fieldpow(a,n):
    z=[1]
    while n:
        if n&1:z=fieldmul(z,a)
        a=fieldmul(a,a);n//=2
    return z

def gcd(a,b):
    a=trim([v%p for v in a]);b=trim([v%p for v in b])
    while b!=[0]:a,b=b,rem(a,b)
    return trim([(v*pow(a[-1],-1,p))%p for v in a])

# The complete modulo-13 irreducibility proof is recomputed here (CT102 premise).
z=X
frobs={}
for k in range(1,61):
    z=fieldpow(z,p)
    if k in (12,20,30,60): frobs[k]=z
ck(frobs[60]==X and all(gcd(F,sub(frobs[d],X))==[1] for d in (12,20,30)),
   "division polynomial is not irreducible mod 13")

def fieldinv(a):
    ck(a!=[0],"zero divisor")
    y=fieldpow(a,p**60-2)
    ck(fieldmul(a,y)==[1],"Fermat inversion")
    return y

# x([n]T) = phi_n(X)/psi_n(X)^2; denominators are units on E[11].
num=[X,
     sub(mul(X,D),f3),
     sub(mul(X,power(f3,2)),mul(D,f4)),
     sub(mul(X,mul(D,power(f4,2))),mul(f5,f3)),
     sub(mul(X,power(f5,2)),mul(D,mul(f6,f4)))]
den=[[1],D,power(f3,2),mul(D,power(f4,2)),power(f5,2)]
s=[0]
for a,b in zip(num,den):
    s=fieldadd(s,fieldmul(red(a),fieldinv(red(b))))
ck(len(s)==60,"nontrivial invariant representation")

# Build exact characteristic-13 minimal polynomial of s by Frobenius conjugacy.
conjugates=[s]
for d in range(1,13):
    z=fieldpow(conjugates[-1],13)
    if d<12:
        ck(z!=s,"early Frobenius return")
        conjugates.append(z)
    else: ck(z==s,"not fixed by Frobenius 12")

Q=[[1]]
for r in conjugates:
    nxt=[[0] for _ in range(len(Q)+1)]
    for j,c in enumerate(Q):
        nxt[j]=fieldadd(nxt[j],fieldmul(c,fieldneg(r)))
        nxt[j+1]=fieldadd(nxt[j+1],c)
    Q=nxt
ck(all(len(c)==1 for c in Q),"minimal polynomial not over F13")
q13=[c[0] for c in Q]
ck(q13==[2,11,0,1,0,3,4,8,6,2,12,6,1],"unexpected degree-12 polynomial")

# Independent source subgroup polynomial obtained from roots, not from scalar labels.
p59=59
def ev(a,x,m):
    z=0
    for c in reversed(a):z=(z*x+c)%m
    return z
roots=[x for x in range(p59) if ev(f11,x,p59)==0]
ck(roots==[19,20,22,43,46],"source subgroup roots")
H=[1]
for x in roots:H=[v%p59 for v in mul(H,[-x,1])]
ck(H==[27,19,45,31,27,1],"source kernel polynomial")

def rhs(x):return (x*x*x-x*x-10*x-20)%p59
def add59(P,Q):
    if P is None:return Q
    if Q is None:return P
    x1,y1=P;x2,y2=Q
    if x1==x2:
        if (y1+y2+1)%p59==0:return None
        slope=((3*x1*x1-2*x1-10)*pow((2*y1+1)%p59,-1,p59))%p59
    else:slope=((y2-y1)*pow((x2-x1)%p59,-1,p59))%p59
    xx=(slope*slope+1-x1-x2)%p59
    yy=(-y1-slope*(xx-x1)-1)%p59
    return xx,yy

def multiples(P):
    out=[None]
    for i in range(11):out.append(add59(out[-1],P))
    ck(out[11] is None and len(set(out[:11]))==11,"order eleven source")
    return out
source_points=[(x,y) for x in roots for y in range(p59) if (y*y+y-rhs(x))%p59==0]
ck(len(source_points)==10,"source point count")
for P in source_points:
    group=multiples(P)
    chkroots=sorted({R[0] for R in group[1:11]})
    ck(chkroots==roots,"subgroup polynomial depends on source orientation")

# Binary target: construct actual five cyclic-subgroup x-roots in F_(2^20).
mod=0x130027
mask=(1<<20)-1
def kmul(a,b):
    out=0
    while b:
        if b&1:out^=a
        b>>=1;a<<=1
        if a&(1<<20):a^=mod
    return out&mask

def kpow(a,n):
    z=1
    while n:
        if n&1:z=kmul(z,a)
        a=kmul(a,a);n//=2
    return z

def kadd(P,Q):
    if P is None:return Q
    if Q is None:return P
    x,y=P;xx,yy=Q
    if x==xx:
        if y^yy==1:return None
        ck(y==yy,"binary coincident points")
        slope=kmul(x,x)
    else:
        slope=kmul(y^yy,kpow(x^xx,(1<<20)-2))
    z=kmul(slope,slope)^1^x^xx
    w=y^kmul(slope,x^z)^1
    return z,w

U=(2,0);points=[None]
for i in range(11):points.append(kadd(points[-1],U))
ck(points[11] is None and len(set(points[:11]))==11,"binary order 11")
binroots=[points[i][0] for i in range(1,6)]
B=[1]
for x in binroots:
    C=[0]*(len(B)+1)
    for j,z in enumerate(B):
        C[j]^=kmul(z,x);C[j+1]^=z
    B=C
ck(B==[0x73383,0x1358,0x10117,1,0x10116,1],"target kernel polynomial")
ck(all(kpow(c,16)==c for c in B),"binary polynomial not in F16")
ck(any(kpow(c,4)!=c for c in B),"binary invariant has degree less than four")
for x in binroots:
    z=0
    for c in reversed(B):z=kmul(z,x)^c
    ck(z==0,"binary root does not lie on polynomial")
# The binary point-field F_(2^40) sits over x-field F_(2^20),
# which sits over the explicit F16 line-field residue. Frobenius 16
# permutes the five x-roots transitively; thus H is irreducible over F16.
orb=[binroots[0]]
for j in range(1,5):
    z=kpow(orb[-1],16)
    ck(z not in orb,"relative quintic Frobenius orbit shortened")
    orb.append(z)
ck(kpow(orb[-1],16)==orb[0] and set(orb)==set(binroots),
   "relative quintic did not have degree five")
sbin=B[4]
conjs=[kpow(sbin,2**j) for j in range(4)]
min4=[1]
for root in conjs:
    z=[0]*(len(min4)+1)
    for i,c in enumerate(min4):
        z[i]^=kmul(c,root);z[i+1]^=c
    min4=z
ck(min4==[1,1,0,0,1],"unexpected primitive F16 polynomial")

print(json.dumps({
    'case':'ct106-primitive-line-field',
    'global_x_degree':60, 'global_oriented_degree':120,
    'fixed_line_field_degree':12,'source_p':59,
    'source_subgroup_polynomial':H,'source_all_oriented_same_polynomial':True,
    'source_invariant_s':32,
    'target_p':2,'target_kernel_coefficients_f2_20':B,'target_coefficient_degree':4,
    'target_invariant_s_f2_20':sbin,'target_s_minpoly_f2':min4,
    'relative_quintic_binary_frobenius_orbit':5,
    'mod13_primitive_invariant_frobenius_degree':12,
    'mod13_primitive_invariant_minpoly':q13,
    'global_char0_minpoly_constructed':False,
    'online_global_public_lift_constructed':False,
    'end_to_end_advantage_measured':False,
},sort_keys=True,separators=(',',':')))
