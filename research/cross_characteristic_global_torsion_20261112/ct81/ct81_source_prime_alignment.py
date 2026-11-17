#!/usr/bin/env python3
"""CT81: independent F37 group-law and 6! orientation control."""
from itertools import permutations
p=37
def check(b): 
    if not b: raise RuntimeError("CT81 failure")
def add(P,Q):
    if P is None:return Q
    if Q is None:return P
    x,y=P;u,v=Q
    if x==u and (y+v+1)%p==0:return None
    if P==Q:
        d=pow((2*y+1)%p,-1,p)
        m=(3*x*x-2*x-10)*d%p
        n=(-x**3-10*x-40-y)*d%p
    else:
        d=pow((u-x)%p,-1,p)
        m=(v-y)*d%p;n=(y*u-v*x)*d%p
    X=(m*m+1-x-u)%p;Y=(-m*X-n-1)%p
    check((Y*Y+Y-X**3+X*X+10*X+20)%p==0)
    return (X,Y)
S=[None];Q=None
for j in range(1,8):
    Q=add(Q,(9,4))
    if j<7:S.append(Q)
check(Q is None and len(set(S))==7)
check(set(x for x,y in S[1:])=={9,27,28})
total=good=fixed=0
for perm in permutations(range(1,7)):
    total+=1;f=(0,)+perm
    if all(f[(i+j)%7]==(f[i]+f[j])%7 for i in range(7) for j in range(7)):
        good+=1
        if f[1]==1:fixed+=1
check((total,good,fixed)==(720,6,1))
print("CT81 PASS: 7 points; x=9,27,28; bijections=720; homomorphisms=6; fixed-generator=1")
