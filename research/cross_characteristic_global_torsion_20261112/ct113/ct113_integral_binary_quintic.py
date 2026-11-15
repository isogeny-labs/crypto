#!/usr/bin/env python3
"""CT113: exact 2-integral specialization of CT112's relative quintic.

No source scalar, no finite lookup of DLPs, and no assertion-only checks.
Uses integer polynomial arithmetic and finite-field tests independent of CT112.
"""
import json
from pathlib import Path

def require(ok, why):
    if not ok:
        raise ArithmeticError(why)

fixture=json.loads((Path(__file__).parent.parent/"ct112"/"reconstructed.json").read_text())
L=int(fixture["common_denominator"])
C=[[int(x) for x in row] for row in fixture["numerators_low_to_high"]]
H=[1,1,0,0,1]
require(L%4==0 and (L//4)%2==1, "denominator must have v2 exactly two")
require(len(C)==4 and all(len(row)==12 for row in C),"bad CT112 shape")

def trim(x):
    x=x[:]
    while len(x)>1 and x[-1]==0:x.pop()
    return x

def divmonic(a,b):
    require(b[-1]==1,"not monic")
    a=trim(a);q=[0]*max(1,len(a)-len(b)+1)
    while len(a)>=len(b) and a!=[0]:
        d=len(a)-len(b);z=a[-1];q[d]=z
        for k,v in enumerate(b):a[d+k]-=z*v
        a=trim(a)
    return trim(q),a

def hlevels(p):
    out=[]
    while p!=[0]:
        p,r=divmonic(p,H)
        out.append(r)
    return out

parity=[]
for j,poly in enumerate(C):
    levels=hlevels(poly)
    require(len(levels)==3 and all(len(x)<=4 for x in levels),"wrong h-adic basis")
    require(all(v%4==0 for v in levels[0]),"unintegral constant part")
    require(all(v%2==0 for v in levels[1]),"unintegral U part")
    # C_j/L = ((A0/4)+(A1/2)U+A2 U^2)/(L/4).
    # This is an exact identity, not just reduction modulo 2.
    parity.append([[(v//(4 if k==0 else 2 if k==1 else 1))&1
                    for v in level] for k,level in enumerate(levels)])

def mul16(x,y):
    r=0
    while y:
        if y&1:r^=x
        y>>=1;x<<=1
        if x&16:x^=0x13
    return r
def pow16(x,n):
    r=1
    while n:
        if n&1:r=mul16(r,x)
        x=mul16(x,x);n>>=1
    return r
def inv16(x):
    require(x!=0,"inverting zero in F16")
    v=pow16(x,14)
    require(mul16(x,v)==1,"F16 inverse failed")
    return v
def polyeval(a,x):
    v=0
    for c in a[::-1]:v=mul16(v,x)^c
    return v
def polytrim(a):
    a=a[:]
    while len(a)>1 and a[-1]==0:a.pop()
    return a
def polyplus(a,b):
    c=[0]*max(len(a),len(b))
    for i,x in enumerate(a):c[i]^=x
    for i,x in enumerate(b):c[i]^=x
    return polytrim(c)
def polymul(a,b):
    c=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):c[i+j]^=mul16(x,y)
    return polytrim(c)
def polymod(a,b):
    a=a[:]
    while len(a)>=len(b) and a!=[0]:
        v=mul16(a[-1],inv16(b[-1]))
        d=len(a)-len(b)
        for j,x in enumerate(b):a[d+j]^=mul16(v,x)
        a=polytrim(a)
    return a
def polygcd(a,b):
    while b!=[0]:a,b=b,polymod(a,b)
    return [mul16(c,inv16(a[-1])) for c in a]
def polypowmod(a,n,p):
    v=[1]
    while n:
        if n&1:v=polymod(polymul(v,a),p)
        a=polymod(polymul(a,a),p);n>>=1
    return v

s=2
require(pow16(s,4)^s^1==0,"wrong F16 primitive element")
w=mul16(s,s)^s
w2=mul16(w,w)
require([w,w2]==[6,7] and mul16(w,w2)==1,"bad cubic roots")
values=[]
for u in (1,w,w2):
    coeffs=[]
    for levels in parity:
        acc=0;uk=1
        for row in levels:
            acc ^= mul16(polyeval(row,s),uk)
            uk=mul16(uk,u)
        coeffs.append(acc)
    H_u=coeffs+[s,1]  # -S=S in characteristic two
    require(polygcd(H_u,polyplus(polypowmod([0,1],16,H_u),[0,1]))==[1],
            "degree-1 binary factor")
    require(polygcd(H_u,polyplus(polypowmod([0,1],256,H_u),[0,1]))==[1],
            "degree-2 binary factor")
    require(polypowmod([0,1],16**5,H_u)==[0,1],"binary quintic not split in F16^5")
    values.append(H_u)
require(values==[[9,6,3,1,2,1],[4,10,15,7,2,1],
                 [15,15,13,6,2,1]],"binary prime specialization mismatch")
require(values[0]!=values[1] and values[1]!=values[2],"prime label collision")
require((C[0][0]+1)%4 != 0 or all(x%4==0 for x in hlevels([C[0][0]+1]+C[0][1:])[0]) is False,
        "mutation negative control failed")
# A one-unit mutation of any coefficient of C0 breaks level-zero /4 divisibility.
require(any(x%4 for x in hlevels([C[0][0]+1]+C[0][1:])[0]),"mutation accepted")

# Fixed CT103 target cross-check: derive x([m]T) directly in F_(2^20)
# from the classical integer division-polynomial recurrences. This computation
# is independent of the rational interpolation CT112 used.
g=0x130027
def gf20mul(a,b):
    z=0
    while b:
        if b&1:z^=a
        b>>=1;a<<=1
        if a&(1<<20):a^=g
    return z
def gf20pow(a,n):
    r=1
    while n:
        if n&1:r=gf20mul(r,a)
        a=gf20mul(a,a);n>>=1
    return r
def gf20eval(poly,x):
    a=0
    for c in poly[::-1]:a=gf20mul(a,x)^(c&1)
    return a
def intmul(a,b):
    z=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):z[i+j]+=x*y
    return trim(z)
def intsub(a,b):
    return trim([(a[k] if k<len(a) else 0)-(b[k] if k<len(b) else 0)
                 for k in range(max(len(a),len(b)))])
def ipow(a,n):
    p=[1]
    while n:
        if n&1:p=intmul(p,a)
        a=intmul(a,a);n>>=1
    return p
D=[-79,-40,-4,4]
f3=[-21,-237,-60,-4,3]
f4=[-5821,-1496,-210,-790,-100,-4,2]
f5=intsub(intmul(ipow(D,2),f4),ipow(f3,3))
f6=intmul(f3,intsub(f5,ipow(f4,2)))
X=[0,1]
numerators=[
    X,
    intsub(intmul(X,D),f3),
    intsub(intmul(X,ipow(f3,2)),intmul(D,f4)),
    intsub(intmul(X,intmul(D,ipow(f4,2))),intmul(f5,f3)),
    intsub(intmul(X,ipow(f5,2)),intmul(D,intmul(f6,f4)))
]
denominators=[[1],D,ipow(f3,2),intmul(D,ipow(f4,2)),ipow(f5,2)]
x=2 # root of the fixed binary polynomial 0x130027, corresponding to CT103
xvals=[]
for a,b in zip(numerators,denominators):
    denom=gf20eval(b,x)
    require(denom!=0,"pole at binary torsion root")
    inv=gf20pow(denom,(1<<20)-2)
    require(gf20mul(denom,inv)==1,"bad F2^20 inverse")
    xvals.append(gf20mul(gf20eval(a,x),inv))
sym=[1]
for root in xvals:
    nxt=[0]*(len(sym)+1)
    for i,v in enumerate(sym):
        nxt[i]^=gf20mul(v,root)
        nxt[i+1]^=v
    sym=nxt
s_actual=0
for v in xvals:s_actual^=v
require(sym[4]==s_actual and (gf20pow(s_actual,4)^s_actual^1)==0,
        "not in binary line field")
# The CT111 fixed prime is U=1. Evaluate F16 coefficients under s -> s_actual.
expect=[]
for levels in parity:
    acc=0;upow=1
    for row in levels:
        y=0
        for bit in row[::-1]:y=gf20mul(y,s_actual)^bit
        acc^=gf20mul(y,upow)
        upow=gf20mul(upow,1)
    expect.append(acc)
require(sym[:4]==expect and sym[5]==1,"fixed binary CT103 prime mismatch")
print(json.dumps({
    "kind":"ct113-two-integral-relative-quintic",
    "exact_integral_h_adic_certificate":True,
    "mutated_coefficient_rejected":True,
    "binary_primes":["1","w","w2"],
    "binary_quintics":values,
    "all_three_irreducible_over_f16":True,
    "fixed_ct103_residue_checked":True,
    "global_orientation_selected":False,
    "complete_dlp_cost_measured":False
},sort_keys=True,separators=(",",":")))
