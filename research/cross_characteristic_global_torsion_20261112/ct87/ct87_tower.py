#!/usr/bin/env python3
"""CT87: integer-only certificate of relative cubic L/F for CT86."""

def trim(a):
 a=list(a)
 while len(a)>1 and a[-1]==0:a.pop()
 return a
def add(a,b,sgn=1):return trim([(a[i] if i<len(a) else 0)+sgn*(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))])
def mul(a,b):
 v=[0]*(len(a)+len(b)-1)
 for i,c in enumerate(a):
  for j,d in enumerate(b):v[i+j]+=c*d
 return trim(v)
def powp(a,n):
 z=[1]
 while n:
  if n&1:z=mul(z,a)
  a=mul(a,a);n>>=1
 return z
def rem(a,b):
 r=trim(a)
 while len(r)>=len(b) and r!=[0]:
  d=len(r)-len(b);v=r[-1]
  r=add([b[-1]*c for c in r],[0]*d+[v*c for c in b],-1)
 return r

def safe(ok):
 if not ok:raise ArithmeticError('CT87 exact polynomial certificate failure')
X=[0,1]; D=[-79,-40,-4,4];p3=[-21,-237,-60,-4,3];f4=[-5821,-1496,-210,-790,-100,-4,2]
p5=add(mul(powp(D,2),f4),powp(p3,3),-1)
p7=add(mul(p5,powp(p3,3)),mul(powp(D,2),powp(f4,3)),-1)
a2=add(powp([-10,-2,3],2),mul([1,-2],D))
b3=powp(p3,2)
a3=add(mul(X,b3),mul(D,f4),-1)
B=mul(D,b3)
N=add(add(mul(X,B),mul(a2,b3)),mul(a3,D))
E2=add(add(mul(mul(X,a2),b3),mul(mul(X,a3),D)),mul(a2,a3))
E3=mul(mul(X,a2),a3)
q2=23624165350186
c2=[51746818064829,-330291288560641,-95029425174429,-1711075375204,-1596513316848,-44914399152,-455319459,54044109]
q3=10252887761980724
c3=[-201756977283948362,563360952055619683,149521192983511524,3495575179577110,2638006865239263,74263835359077,711827220769,-88442189137]
def evhom(c):
 z=[0]
 for i,t in enumerate(c):z=add(z,[t*k for k in mul(powp(N,i),powp(B,7-i))])
 return z
H2=evhom(c2);H3=evhom(c3)
safe(rem(add([q2*k for k in mul(E2,powp(B,6))],H2,-1),p7)==[0])
safe(rem(add([q3*k for k in mul(E3,powp(B,6))],H3,-1),p7)==[0])
# Explicit relative cubic alpha³ - (N/B)alpha² + (C2(N/B)/q2)alpha - C3(N/B)/q3.
Z=add(add([q2*q3*k for k in mul(powp(B,7),powp(X,3))],[q2*q3*k for k in mul(mul(N,powp(B,6)),powp(X,2))],-1),[q3*k for k in mul(H2,X)])
Z=add(Z,[q2*k for k in H3],-1)
safe(rem(Z,p7)==[0])
print('CT87 PASS: relative_cubic_degree=3 base_degree=8 galois_x=3 denominator_e2=23624165350186 denominator_e3=10252887761980724')
