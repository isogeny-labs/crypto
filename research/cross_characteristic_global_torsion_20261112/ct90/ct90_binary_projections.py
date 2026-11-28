#!/usr/bin/env python3
"""CT90: self-contained exact 2-local CRT projection and CT80 place selection."""
from fractions import Fraction
import json

G = [-2102915,3828478,-6423347,-1696464,-39690,-29862,-840,-8,1]
A = [[0]*8 for _ in range(8)]
for j in range(4): A[j][j] = 4
for j, col in enumerate(((1,3,0,0,1,0,0,0),(0,1,3,0,0,1,0,0),
                         (0,0,1,3,0,0,1,0),(1,3,0,1,0,0,0,1)), 4):
    for i in range(8): A[i][j] = col[i]
B = [[Fraction(A[i][j],4) for j in range(8)] for i in range(8)]

def invmat(M):
    m = len(M)
    T = [[M[i][j] for j in range(m)] + [Fraction(i==j) for j in range(m)] for i in range(m)]
    for k in range(m):
        t = next(i for i in range(k,m) if T[i][k])
        T[k],T[t] = T[t],T[k]
        d=T[k][k]; T[k] = [v/d for v in T[k]]
        for i in range(m):
            if i!=k:
                d=T[i][k];T[i]=[v-d*w for v,w in zip(T[i],T[k])]
    return [r[m:] for r in T]

Bi=invmat(B)
def prod(a,b):
    z=[Fraction(0)]*15
    for i,x in enumerate(a):
        for j,y in enumerate(b): z[i+j]+=x*y
    for k in range(14,7,-1):
        t=z[k]
        for j in range(8):z[k-8+j]-=t*G[j]
    return z[:8]
def coords(a): return [sum(Bi[i][j]*a[j] for j in range(8)) for i in range(8)]
def bits(v): return sum((int(t)&1)<<i for i,t in enumerate(v))
bs=[[B[i][j] for i in range(8)] for j in range(8)]
M=[]
for i in range(8):
    M.append([])
    for j in range(8):
        c=coords(prod(bs[i],bs[j]))
        if not all(v.denominator==1 for v in c): raise ArithmeticError('nonintegral basis multiplication')
        M[i].append(bits(c))

def mul(a,b):
    z=0
    for i in range(8):
        if a>>i&1:
            for j in range(8):
                if b>>j&1: z^=M[i][j]
    return z

E=(240,241)
assert mul(240,240)==240 and mul(241,241)==241 and 240^241==1
cols=[[e,mul(e,2),mul(mul(e,2),2),mul(mul(mul(e,2),2),2)] for e in E]
lookup=[]
for e,bs4 in zip(E,cols):
    d={}
    for i in range(16):
        m=0
        for j in range(4):
            if i>>j&1:m^=bs4[j]
        if m in d:raise ArithmeticError('non-injective field basis')
        d[m]=i
    assert len(d)==16
    lookup.append(d)
projections=[[lookup[j][mul(E[j],v)] for j in range(2)] for v in range(256)]

def mul4(a,b):
    p=0
    for i in range(4):
        if a>>i&1:
            for j in range(4):
                if b>>j&1:p^=1<<(i+j)
    for k in range(6,3,-1):
        if p>>k&1:p^=0b10011<<(k-4)
    return p

for a in range(256):
    for b in range(256):
        v=projections[mul(a,b)]
        if any(v[i]!=mul4(projections[a][i],projections[b][i]) for i in (0,1)):
            raise ArithmeticError('projection not multiplicative')
        assert all(projections[a^b][i]==(projections[a][i]^projections[b][i]) for i in (0,1))
assert projections[1]==[1,1] and projections[2]==[2,2]

q2=23624165350186
c2=[51746818064829,-330291288560641,-95029425174429,-1711075375204,
    -1596513316848,-44914399152,-455319459,54044109]
q3=10252887761980724
c3=[-201756977283948362,563360952055619683,149521192983511524,3495575179577110,
    2638006865239263,74263835359077,711827220769,-88442189137]

def local_mask(c,q):
    v=coords([Fraction(x,q) for x in c])
    if any(x.denominator%2==0 for x in v):raise ArithmeticError('nonintegral at 2')
    return bits([x.numerator%2 for x in v])
e2=local_mask(c2,q2);e3=local_mask(c3,q3)

def mul12(a,b):
    z=0
    while b:
        if b&1:z^=a
        b>>=1;a<<=1
        if a&4096:a^=0x1225
    return z&4095

def at_target(x,u=1545):
    z=0;t=1
    for i in range(4):
        if x>>i&1:z^=t
        t=mul12(t,u)
    return z
assert mul12(mul12(mul12(1545,1545),1545),1545)^1545^1==0
v2=[at_target(k) for k in projections[e2]]
v3=[at_target(k) for k in projections[e3]]
assert (e2,e3,v2,v3)==(4,254,[1906,1906],[1261,1260])
assert projections[240]==[1,0] and projections[241]==[0,1]
res={"basis_images":[projections[1<<i] for i in range(8)],
     "e2_basis_mask":e2,"e2_binary_values":v2,"e2_images":projections[e2],
     "e3_basis_mask":e3,"e3_binary_values":v3,"e3_images":projections[e3],
     "idempotents":list(E),"selected_idempotent":240,"target_s":1545,
     "tested_products":65536,"test":"ct90-binary-residue-selection"}
print(json.dumps(res,sort_keys=True,separators=(',',':')))
