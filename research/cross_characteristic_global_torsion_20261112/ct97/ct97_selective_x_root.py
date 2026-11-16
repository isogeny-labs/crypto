#!/usr/bin/env python3
"""CT97: scalar-blind selection of *x-coordinate* global torsion lift.

Uses a fixed degree-8 subgroup cubic and discriminant resolvent. No input
scalar, generator-multiple table, or target label is given to the recognizer.
The *precomputed* cubic comes from CT87 (whose setup used [2],[3] maps).
The y-orientation and full end-to-end DLP cost remain unresolved.
Scientific status: [pcc:unverified].
Requires SymPy 1.14.0 for exact QQ polynomial linear algebra.
"""
import json
from functools import reduce
from math import gcd, lcm
import sympy as sp

x=sp.Symbol("x")
QQ=lambda expr:sp.Poly(expr,x,domain=sp.QQ)
X=QQ(x); I=QQ(1)
D=QQ(4*x**3-4*x**2-40*x-79)
P3=QQ(3*x**4-4*x**3-60*x**2-237*x-21)
F4=QQ(2*x**6-4*x**5-100*x**4-790*x**3-210*x**2-1496*x-5821)
F=(D**2*F4-P3**3)*P3**3-D**2*F4**3
check=lambda b,msg: b or (_ for _ in ()).throw(ArithmeticError(msg))
check(F.degree()==24 and F.LC()==7,"7-division polynomial degree")
rem=lambda a:a.rem(F)
inv=lambda a:sp.invert(a,F)

# Frozen CT87 structure constants (input-independent, but charge their setup).
A2=QQ((3*x*x-2*x-10)**2)+(QQ(1-2*x)*D)
A3=X*P3**2-D*F4
B=D*P3**2
N=X*B+A2*P3**2+A3*D
q2=23624165350186
C2=[51746818064829,-330291288560641,-95029425174429,
    -1711075375204,-1596513316848,-44914399152,-455319459,54044109]
q3=10252887761980724
C3=[-201756977283948362,563360952055619683,149521192983511524,
    3495575179577110,2638006865239263,74263835359077,711827220769,
    -88442189137]
# Discriminant square root in S=Q(s). Precomputed and checked independently by
# the CT97 JS integer certificate; not a map tagged by the input target Q.
Cd=[1330115088493357156,656532317695454585,17598156120098064,
    12706441996691436,225325011610699,-309225601391,
    -443542601663,4007355953]

s=rem(N*inv(B))
spowers=[I]
for _ in range(1,8):spowers.append(rem(spowers[-1]*s))
def coefficient(coeffs,den):
    return rem(sum((coeffs[i]*spowers[i] for i in range(len(coeffs))),QQ(0))
               .mul_ground(sp.Rational(1,den)))
e2,e3,dlt=coefficient(C2,q2),coefficient(C3,q3),coefficient(Cd,q3)
hp=rem(3*X**2-2*s*X+e2)
v=rem(dlt*inv(hp))
plus=rem((s-X+v).mul_ground(sp.Rational(1,2)))
minus=rem((s-X-v).mul_ground(sp.Rational(1,2)))
# Exact splitting of the relative cubic, no source scalar selected.
check(rem(X+plus+minus)==s,"global cubic trace")
check(rem(X*plus+X*minus+plus*minus)==e2,"global cubic pair sum")
check(rem(X*plus*minus)==e3,"global cubic product")
roots=[X,plus,minus]

# Online arithmetic: convert an exact algebraic root to a low-degree
# rational expression by QQ nullspace, increasing TOTAL degrees (not [m]).
Xp=[I]
for _ in range(19):Xp.append(rem(Xp[-1]*X))
def normalize(A,B):
    terms=A.all_coeffs()+B.all_coeffs()
    common=lcm(*(int(z.q) for z in terms))
    A=A.mul_ground(common);B=B.mul_ground(common)
    g=reduce(gcd,(abs(int(z)) for z in A.all_coeffs()+B.all_coeffs()))
    A=A.mul_ground(sp.Rational(1,g)); B=B.mul_ground(sp.Rational(1,g))
    if B.LC()<0:A=-A;B=-B
    return A,B
def rational_reconstruct(beta):
    prod=[rem(beta*w) for w in Xp]
    for budget in range(1,18):
        for b in range(budget+1):
            a=budget-b
            if a==0:continue
            cols=Xp[:a+1]+[-w for w in prod[:b+1]]
            M=sp.Matrix([[col.nth(i) for col in cols] for i in range(24)])
            for v in M.nullspace():
                A=QQ(sum(v[i]*x**i for i in range(a+1)))
                B=QQ(sum(v[a+1+j]*x**j for j in range(b+1)))
                if not B.is_zero and rem(A-beta*B).is_zero:
                    return normalize(A,B)
    raise ArithmeticError("no exact expression within searched degree budget")

# Fixed target prime: CT80 polynomial basis F_(2^12), alpha->binary 2.
def m12(a,b):
    c=0
    while b:
        if b&1:c^=a
        a<<=1
        if a&4096:a^=0x1225
        b>>=1
    return c
def p12(a,n):
    c=1
    while n:
        if n&1:c=m12(c,a)
        a=m12(a,a); n>>=1
    return c
def binary_eval(A):
    z=0
    for a in A.all_coeffs():z=m12(z,2)^(int(a)&1)
    return z
def source_eval(A):
    return int(A.eval(9))%37
def recognize(public_x):
    matches=[]
    # No scalar or target-side root is read. Precomputed candidate *algebraic*
    # roots are filtered by the single fixed source-place residue.
    for beta in roots:
        A,B=rational_reconstruct(beta)
        den=source_eval(B)
        check(den!=0,"source denominator nonunit")
        residue=source_eval(A)*pow(den,-1,37)%37
        if residue==public_x:
            bin_den=binary_eval(B)
            check(bin_den!=0,"binary denominator nonunit")
            matches.append(dict(public_x=public_x,target_binary_x=
                m12(binary_eval(A),p12(bin_den,4094)),
                numerator_degree=A.degree(),denominator_degree=B.degree(),
                global_identity=True))
    check(len(matches)==1,"source residue is not a unique lift")
    return matches[0]

# Public x-coordinates constitute ALL nonzero x-classes of E(F37)[7].
# The y-orientation is deliberately *not* implied by this x-only function.
output=[recognize(a) for a in (9,27,28)]
check([r["target_binary_x"] for r in output]==[2,17,1562],
      "independent CT80/91 binary controls")
try:
    recognize(10)
except ArithmeticError:
    negative_control=True
else:raise ArithmeticError("invalid source root accepted")
print(json.dumps(dict(experiment="CT97-cubic-discriminant-scalar-blind-x",
    exact_three_global_roots=True,nonmember_rejected=negative_control,
    oriented_y_reconstruction=False,source_targets=output),
    sort_keys=True,separators=(',',':')))
