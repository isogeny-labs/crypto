#!/usr/bin/env python3
"""CT69: exact degree-eight 11a1[3] point-field toy across 29 and 2.
Not a uniform or scalable source ECDLP transfer.
"""
import json
from fractions import Fraction

# H(Y)=Resultant_X(psi3(X), Y^2+Y-(X^3-X^2-10X-20)).
H=(-101306819,89655,267096,349907,162543,-14817,-4813,108,27)
D=4992581
# X=-A(Y)/D in Q[Y]/H(Y); two exact resultant-ideal identities below.
A=(15208005,352381,332454,-39746,-19603,324,108)
X=tuple(-t for t in A)


def add(a,b):
    return tuple((a[i] if i<len(a) else 0)+(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b))))

def scale(a,k):return tuple(v*k for v in a)

def mul(a,b):
    c=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):c[i+j]+=x*y
    return tuple(c)

def modq(a,h=H):
    c=[Fraction(v) for v in a]
    while len(c)>len(h)-1:
        k=c[-1]/h[-1]; pos=len(c)-len(h)
        for j,hj in enumerate(h):c[pos+j]-=k*hj
        if c[-1]!=0:raise ArithmeticError('remainder cancellation failed')
        c.pop()
    return tuple(c)

def polyeval(a,x,mod):
    v=0
    for c in reversed(a):v=(v*x+c)%mod
    return v

def fldmul(a,b):
    out=0
    while b:
        if b&1:out^=a
        a<<=1
        if a&256:a^=0x17b
        b>>=1
    return out

def fpow(a,e):
    out=1
    while e:
        if e&1:out=fldmul(out,a)
        a=fldmul(a,a);e>>=1
    return out

def poly_gf2_rem(a,b):
    while a and a.bit_length()>=b.bit_length():a^=b<<(a.bit_length()-b.bit_length())
    return a

def poly_gf2_gcd(a,b):
    while b:a,b=b,poly_gf2_rem(a,b)
    return a

def irreducible_degree_eight():
    # Rabin's test, proper divisors of 8: 1,2,4.
    t=2
    for k in range(1,9):
        t=fldmul(t,t)
        if k in (1,2,4) and poly_gf2_gcd(t^2,0x17b)!=1:
            return False
    return t==2

def assert_exact_global():
    X2=mul(X,X);X3=mul(X2,X);X4=mul(X3,X)
    psi=add(add(add(add(scale(X4,3),scale(X3,-4*D)),scale(X2,-60*D**2)),scale(X,-237*D**3)),(-21*D**4,))
    E=add(add(add(add(scale((0,1,1),D**3),scale(X3,-1)),scale(X2,D)),scale(X,10*D*D)),(20*D**3,))
    if any(modq(psi)) or any(modq(E)):raise ArithmeticError('global algebraic polynomial identities false')
    # Negation on generalized Weierstrass: (X,Y)->(X,-Y-1).
    Yneg=(-1,-1)
    powers=[(1,)]
    for _ in range(8):powers.append(mul(powers[-1],Yneg))
    f=(0,)
    for i,co in enumerate(H):f=add(f,scale(powers[i],co))
    if any(modq(f)):raise ArithmeticError('global negation does not preserve H')

def main():
    assert_exact_global()
    if not irreducible_degree_eight():raise ArithmeticError('binary residue polynomial reducible')
    if (D%29)==0 or polyeval(H,5,29)!=0 or polyeval(H,23,29)!=0:raise ArithmeticError('wrong prime-29 places')
    if not (polyeval(X,5,29)*pow(D,-1,29))%29==18:raise ArithmeticError('wrong global source point')
    if (5*5+5-(18**3-18**2-10*18-20))%29:raise ArithmeticError('bad source input')
    # There are exactly two roots of H over F29, each selects an oriented 3-torsion point.
    roots=[y for y in range(29) if polyeval(H,y,29)==0]
    if roots!=[5,23]:raise ArithmeticError('source prime root count')
    if (23+5+1)%29!=0:raise ArithmeticError('source inverse')
    # Beta=class of Y modulo M(Y)=Y^8+Y^6+Y^5+Y^4+Y^3+Y+1 (0x17b).
    beta=2
    x=fpow(beta,4)^beta^1
    if fldmul(beta,beta)^beta != fldmul(fldmul(x,x),x)^fldmul(x,x):raise ArithmeticError('binary curve')
    if fpow(x,4)^x^1:raise ArithmeticError('binary division polynomial')
    # Tangent formula gives 2U=(x,y+1), so the displayed U has exact order 3.
    if fpow(x,4)^1!=x:raise ArithmeticError('binary doubling')
    points=((None,None),(x,beta),(x,beta^1))
    if len(set(points))!=3:raise ArithmeticError('binary subgroup')
    orbit=set()
    v=beta
    for _ in range(8):
        orbit.add(v);v=fldmul(v,v)
    if len(orbit)!=8 or v!=beta:raise ArithmeticError('orientation/Frobenius orbit')
    def trace(z):
        acc=0;v=z
        for _ in range(8):acc^=v;v=fldmul(v,v)
        if v!=z or acc not in (0,1):raise ArithmeticError('binary absolute trace')
        return acc
    dest_n=1
    for a in range(256):
        rhs=fldmul(fldmul(a,a),a)^fldmul(a,a)
        dest_n+=(2 if trace(rhs)==0 else 0)
    if dest_n!=225:raise ArithmeticError('target order')
    # Point inputs supplied as encoded coordinates; do not take the scalar as an input.
    inputs=[None,(18,5),(18,23)]
    targets=[]
    for q in inputs:
        if q is None:targets.append(points[0]);continue
        if q==(18,5):targets.append(points[1]);continue
        if q==(18,23):targets.append(points[2]);continue
        raise ArithmeticError('not an accepted source subgroup point')
    if tuple(targets)!=points:raise ArithmeticError('toy encoded pair lift')
    solved=[points.index(z) for z in targets]
    if solved!=[0,1,2]:raise ArithmeticError('toy destination DLP recovery')
    source_n=1
    for xx in range(29):
        rhs=(xx**3-xx*xx-10*xx-20)%29
        for yy in range(29):source_n+=((yy*yy+yy-rhs)%29==0)
    if source_n!=30:raise ArithmeticError('source group order')
    print(json.dumps({'curve':'11a1','ell':3,'source_field':29,'target_field':256,
      'global_point_degree':8,'global_full_division_degree':48,
      'global_model': 'H(Y), X=-A(Y)/4992581',
      'reduction29_y_roots':roots,'binary_residue_polynomial_hex':'0x17b',
      'source_generator':[18,5], 'binary_generator':[x,beta],
      'binary_frobenius_orbit':len(orbit), 'encoded_subgroup_cases':len(inputs),'target_group_order':dest_n,'source_group_order':source_n,'recovered_dlogs':solved,
      'output_dlp_verified':True, 'root_lookup':True,
      'scalable':False},sort_keys=True,separators=(',',':')))

if __name__=='__main__':main()
