#!/usr/bin/env python3
"""CT85: exact non-CM and scalar-leaking endomorphism x-map controls."""
import json
from math import isqrt

def mul(a,b):
    z=[0]*(len(a)+len(b)-1)
    for i,v in enumerate(a):
        for j,w in enumerate(b):z[i+j]+=v*w
    return z
def sub(a,b):return [(a[i] if i<len(a) else 0)-(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))]
def ev(a,x,p=None):
    z=0
    for v in a[::-1]:
        z=z*x+v
        if p:z%=p
    return z

def check(p):
    if not p:raise ArithmeticError('CT85 certificate failed')

# Curve a1=0,a2=-1,a3=1,a4=-10,a6=-20: c4=496,Delta=-11^5.
c4=496;delta=-161051
check(c4**3%11!=0 and delta==-11**5)  # nonintegral j => non-CM
D=[-79,-40,-4,4];psi3=[-21,-237,-60,-4,3]
f4=[-5821,-1496,-210,-790,-100,-4,2]
A2=sub(mul([-10,-2,3],[-10,-2,3]),mul([-1,2],D))
A3=sub(mul([0,1],mul(psi3,psi3)),mul(D,f4))
B3=mul(psi3,psi3)
functions=[([0,1],[1]),(A2,D),(A3,B3)]
MOD=4096+512+32+4+1
def fm(u,v):
    z=0
    while v:
        if v&1:z^=u
        v>>=1;u<<=1
        if u&4096:u^=MOD
    return z&4095
def fpower(u,n):
    z=1
    while n:
        if n&1:z=fm(z,u)
        u=fm(u,u);n>>=1
    return z
def binary(c):
    z=0
    for v in c[::-1]:z=fm(z,2)^(v&1)
    return z

rows=[];ximages=[2,17,1562]
for n,(A,B) in enumerate(functions,1):
    check((len(A)-1,len(B)-1,A[-1],B[-1])==(n*n,n*n-1,1,n*n))
    check(ev(B,9,37)!=0 and ev(A,9,37)*pow(ev(B,9,37),-1,37)%37==[9,27,28][n-1])
    bb=binary(B);check(bb!=0 and fm(binary(A),fpower(bb,4094))==ximages[n-1])
    check(isqrt(B[-1]//A[-1])==n)
    rows.append([n,n*n,n*n-1,n*n])
print(json.dumps({'binary_x':[2,17,1562],'c4':c4,'delta':delta,'j_num':c4**3,'map_degrees':rows,'non_cm':True,'source_x':[9,27,28]},sort_keys=True,separators=(',',':')))
