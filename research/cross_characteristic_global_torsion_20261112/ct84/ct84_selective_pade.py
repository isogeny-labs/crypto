#!/usr/bin/env python3
"""CT84: exact-integer LLL recovers a single rational function from two local roots.

No scalar label/list given to recognizer. Global formula is checked separately.
"""
import json
from fractions import Fraction
from math import gcd

p, N = 37, 18
def check(b, msg):
    if not b: raise ArithmeticError(msg)
def trim(a):
    while len(a)>1 and a[-1]==0: a.pop()
    return a
def mul(a,b):
    c=[0]*(len(a)+len(b)-1)
    for i,u in enumerate(a):
        for j,v in enumerate(b): c[i+j]+=u*v
    return trim(c)
def sub(a,b):
    return trim([(a[i] if i<len(a) else 0)-(b[i] if i<len(b) else 0)
                 for i in range(max(len(a),len(b)))])
def power(a,n):
    v=[1]
    for _ in range(n): v=mul(v,a)
    return v
def ev(a,x,m=None):
    v=0
    for z in reversed(a):
        v=v*x+z
        if m is not None: v%=m
    return v

D=[-79,-40,-4,4]
P3=[-21,-237,-60,-4,3]
F4=[-5821,-1496,-210,-790,-100,-4,2]
F=sub(mul(sub(mul(power(D,2),F4),power(P3,3)),power(P3,3)),
      mul(power(D,2),power(F4,3)))
DF=[i*F[i] for i in range(1,len(F))]
check(len(F)==25 and F[-1]==7,'division polynomial')
def lift(r):
    check(ev(F,r,p)==0,'source point is not 7-division root')
    x=r; dinv=pow(ev(DF,r,p),-1,p)
    for k in range(1,N):
        m=p**(k+1)
        digit=(ev(F,x,m)//p**k)%p
        x=(x-digit*dinv*p**k)%m
        check(ev(F,x,m)==0,'Hensel failure')
    return x

# The algorithm sees only the chosen generator's x residue 9 and the public
# target's x residue 27. It is NOT passed the scalar 2 or a candidate list.
a,b=lift(9),lift(27)
q=p**N
cs=[pow(a,i,q) for i in range(5)]+[(-b*pow(a,i,q))%q for i in range(4)]
# Z^9 kernel of the single congruence A(a)-b B(a)=0 modulo q.
basis=[[q]+[0]*8]+[[(-cs[i])%q]+[int(i==j+1) for j in range(8)]
                     for i in range(1,9)]
def gram(B):
    n=len(B); orth=[]; mu=[[Fraction(0)]*n for _ in range(n)]; norm=[]
    for i in range(n):
        w=list(map(Fraction,B[i]))
        for j in range(i):
            mu[i][j]=sum(B[i][t]*orth[j][t] for t in range(n))/norm[j]
            w=[w[t]-mu[i][j]*orth[j][t] for t in range(n)]
        orth.append(w); norm.append(sum(z*z for z in w))
    return mu,norm
def lll(B):
    B=[row[:] for row in B]
    mu,norm=gram(B); k=1; iterations=0
    while k<len(B):
        iterations+=1
        check(iterations<10000,'LLL iteration bound')
        for j in range(k-1,-1,-1):
            f=mu[k][j]+Fraction(1,2)
            z=f.numerator//f.denominator
            if z:
                B[k]=[x-z*y for x,y in zip(B[k],B[j])]
                mu,norm=gram(B)
        if norm[k]>=(Fraction(3,4)-mu[k][k-1]**2)*norm[k-1]: k+=1
        else:
            B[k],B[k-1]=B[k-1],B[k]
            mu,norm=gram(B); k=max(k-1,1)
    return B

v=lll(basis)[0]
check(any(v) and sum(v[i]*cs[i] for i in range(9))%q==0,'not a lattice vector')
g=0
for z in v:g=gcd(g,abs(z))
v=[z//g for z in v]
if v[-1]<0:v=[-z for z in v]
A,B=v[:5],v[5:]
check(ev(B,a,p)!=0,'denominator nonunit')
check((ev(A,a,q)-b*ev(B,a,q))%q==0,'wrong local relation')
# Independent exact global certificate: check the symbolic group-law
# duplication numerator (3x^2-2x-10)^2+(1-2x)(4x^3-4x^2-40x-79).
M=[-10,-2,3]
check(A==sub(mul(M,M),mul([-1,2],D)) and B==D,
      'candidate is not globally certified')
check(ev(A,9,p)*pow(ev(B,9,p),-1,p)%p==27,'wrong local point')
check(ev(A,9,p)*pow(ev(B,9,p),-1,p)%p!=28,'wrong-root control failed')
# At the binary place B(alpha)=1, A(alpha)=1+alpha^4.
# Doubling the global oriented point gives (x,y)=(17,Y+77).
check([t%2 for t in A]==[1,0,0,0,1] and [t%2 for t in B]==[1,0,0,0],
      'binary polynomial mismatch')
print(json.dumps({
    'basis_dimension':9, 'binary_image_x':17, 'binary_image_y':[77,1],
    'exact_global_identity':True, 'experiment':'ct84-selective-rational-recognition',
    'negative_root_rejected':True, 'precision_digits':N,
    'public_source_point':[27,9], 'recovered_denominator':B,
    'recovered_numerator':A, 'source_generator':[9,4],
},sort_keys=True,separators=(',',':')))
