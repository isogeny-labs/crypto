#!/usr/bin/env python3
"""CT105: scalar-blind public-point local 11-torsion lift at p=59.

Does not globalize into the fixed 120-dimensional number field or evaluate at p=2.
"""
import json
import sys

P = 59
N = 8

def chk(ok, msg):
    if not ok:
        raise ArithmeticError(msg)

def mul(a, b):
    out = [0] * (len(a)+len(b)-1)
    for i, ai in enumerate(a):
        for j,bj in enumerate(b):
            out[i+j] += ai*bj
    return out

def sub(a,b):
    return [(a[i] if i < len(a) else 0)-(b[i] if i<len(b) else 0)
            for i in range(max(len(a), len(b)))]

def power(a,n):
    z=[1]
    while n:
        if n&1: z=mul(z,a)
        a=mul(a,a)
        n//=2
    return z

D=[-79,-40,-4,4]
A=[-21,-237,-60,-4,3]
B=[-5821,-1496,-210,-790,-100,-4,2]
f5=sub(mul(power(D,2),B),power(A,3))
f6=mul(A,sub(f5,power(B,2)))
f7=sub(mul(f5,power(A,3)),mul(power(D,2),power(B,3)))
f11=sub(mul(f7,power(f5,3)),mul(mul(power(D,2),B),power(f6,3)))
chk(len(f11)==61 and f11[-1]==11,"division polynomial degree")

def eval_poly(a,x,m=None):
    z=0
    for c in reversed(a):
        z=z*x+c
        if m is not None:z%=m
    return z

def rhs(x):
    return x*x*x-x*x-10*x-20

def f_deriv(x):
    return sum(i*c*pow(x,i-1,P) for i,c in enumerate(f11) if i)%P

def admit(x,y):
    return (0<=x<P and 0<=y<P and
        eval_poly(f11,x,P)==0 and (y*y+y-rhs(x))%P==0 and
        f_deriv(x)!=0 and (2*y+1)%P!=0)

def lift(x,y):
    if not admit(x,y):
        raise ValueError("not an admissible nonzero 11-torsion source point")
    X,Y=x,y
    dxi=pow(f_deriv(x),-1,P)
    dyi=pow((2*y+1)%P,-1,P)
    modulus=P
    for k in range(1,N):
        new_modulus=modulus*P
        fx=eval_poly(f11,X,new_modulus)
        chk(fx%modulus==0,"x not lifted to prior precision")
        t=(-(fx//modulus)*dxi)%P
        X+=t*modulus
        residual=(Y*Y+Y-rhs(X))%new_modulus
        chk(residual%modulus==0,"y not lifted to prior precision")
        u=(-(residual//modulus)*dyi)%P
        Y+=u*modulus
        modulus=new_modulus
    chk(eval_poly(f11,X,modulus)==0,"lifted x violates division polynomial")
    chk((Y*Y+Y-rhs(X))%modulus==0,"lifted point off curve")
    chk(X%P==x and Y%P==y,"source orientation lost")
    return {'input':[x,y], 'x59adic':str(X), 'y59adic':str(Y)}

# Enumerate source residue places algebraically, without any scalar table.
roots=[x for x in range(P) if eval_poly(f11,x,P)==0]
chk(roots==[19,20,22,43,46],"unexpected eleven-division rational x roots")
all_points=[(x,y) for x in roots for y in range(P) if (y*y+y-rhs(x))%P==0]
chk(len(all_points)==10 and all(admit(x,y) for x,y in all_points),"oriented degree-one places")
chk(not admit(19,23) and not admit(0,0),"invalid input accepted")
chk(sum((y*y+y-rhs(x))%P==0 for x in range(P) for y in range(P))==54,
    "source E(F59) order 55")

# Accept one public Q, or independently sweep all ten public points.
if len(sys.argv)==3 and sys.argv[1]=='--point':
    parts=sys.argv[2].split(',')
    if len(parts)!=2:raise ValueError('point requires x,y')
    points=[lift(int(parts[0]),int(parts[1]))]
elif len(sys.argv)==1:
    points=[lift(x,y) for x,y in all_points]
else:raise ValueError('usage: ct105_source_hensel.py [--point x,y]')

print(json.dumps({'case':'ct105-scalar-blind-local-hensel','count':len(points),
    'lift_precision_digits':N,'modulus':str(P**N),'points':points,
    'fixed_binary_specialization_evaluated':False,
    'global_K_element_reconstructed':False,
    'source_scalar_recovered':False},sort_keys=True,separators=(',',':')))
