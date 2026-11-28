#!/usr/bin/env python3
"""CT88: exact rational-Euclidean discriminant and Frob_2 orbit audit."""
from fractions import Fraction as F
import json
G = [-2102915, 3828478, -6423347, -1696464, -39690, -29862, -840, -8, 1]
def strip(a):
    while a and not a[-1]: a.pop()
    return a
def rem(a,b):
    a=list(map(F,a)); b=list(map(F,b))
    while len(a)>=len(b) and a:
        q=a[-1]/b[-1]; d=len(a)-len(b)
        for i,v in enumerate(b): a[d+i]-=q*v
        strip(a)
    return a
def res(a,b):
    if not a or not b: return F(0)
    m=len(a)-1; n=len(b)-1
    if n==0: return F(b[0])**m
    r=rem(a,b)
    if not r: return F(0)
    k=len(r)-1
    return (-1 if (m*n)%2 else 1)*F(b[-1])**(m-k)*res(b,r)
dg=[i*G[i] for i in range(1,len(G))]
D=res(G,dg)
assert D.denominator==1
D=int(D)
assert D==-126276666637416719355500270478117660693463886295963968396774334577180672
v=0; d=D
while d%2==0: d//=2; v+=1
assert v==16
# #E(F_2)=5; trace=-2 and determinant=2 on E[7].
points=[(1,i) for i in range(7)]+[(0,1)]
def canon(x,y):
    x%=7; y%=7
    return (1,y*pow(x,-1,7)%7) if x else (0,1)
def frob(point):
    x,y=point
    return canon(-2*y,x-2*y)
seen=set(); cycles=[]
for P in points:
    if P in seen: continue
    orbit=[]; Q=P
    while Q not in seen:
        seen.add(Q); orbit.append(Q); Q=frob(Q)
    assert Q==P
    cycles.append(len(orbit))
assert sorted(cycles)==[4,4]
h=0b10011
f=sum((c&1)<<i for i,c in enumerate(G))
def sq(a): return sum(1<<(2*i) for i in range(a.bit_length()) if a>>i&1)
assert f==sq(h)
q2=23624165350186; q3=10252887761980724
assert q2%2==0 and q2%4!=0 and q3%4==0 and q3%8!=0
result={"test":"ct88-2-adic-index","curve_order_f2":5,"frobenius_trace":-2,
        "projective_orbits":sorted(cycles),"g_mod2":f,"square_root_mod2":h,
        "disc_g":str(D),"disc_v2":v,"order_index_v2":v//2,
        "relative_q2_v2":1,"relative_q3_v2":2}
print(json.dumps(result,sort_keys=True,separators=(',',':')))
