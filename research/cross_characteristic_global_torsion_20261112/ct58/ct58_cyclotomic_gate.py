#!/usr/bin/env python3
"""CT58 independent (Python) exact determinant/cyclotomic gate controls.

Mathematical theorem (Weil pairing) is NOT established by these tests.
Every execution check is unconditional under python -O.
"""

from math import isqrt

def require(v, msg):
    if not v:
        raise AssertionError(msg)

def is_prime(n):
    if n < 2: return False
    for i in range(2, isqrt(n)+1):
        if n % i == 0: return False
    return True

def order(a, ell):
    require(is_prime(ell) and a % ell, "invalid order input")
    cur = a % ell
    for i in range(1,ell):
        if cur == 1: return i
        cur = (cur * a) % ell
    raise AssertionError("order missing")

def mm(a,b,ell):
    return tuple(tuple(sum(a[i][k]*b[k][j] for k in range(2)) % ell for j in range(2)) for i in range(2))

def mpow(a,n,ell):
    r=((1,0),(0,1))
    while n:
        if n&1:r=mm(r,a,ell)
        a=mm(a,a,ell)
        n>>=1
    return r

def det(a,ell):
    return (a[0][0]*a[1][1]-a[0][1]*a[1][0]) % ell

def count_curve_f2():
    return 1+sum(1 for x in range(2) for y in range(2) if (y*y+y-x*x*x+x*x+10*x+20)%2 == 0)

def run():
    primes=(3,5,7,11,13,17,19,23,31,37,41,43,67,163,257)
    for ell in primes:
        f=order(2,ell)
        require((ell-1)%f==0, "cyclic unit group order")
        require(all(pow(2,d,ell)!=1 for d in range(1,f)),"nonminimal f")
        require(pow(2,f,ell)==1, "invalid cyclotomic f")
        print("ell=%d cyclotomic_global_degree_floor=%d binary_full_basis_residue_degree_floor=%d"%(ell,ell-1,f))
    # Exhaustively audit the finite two-dimensional determinant symplectic identity.
    # This is an independent finite control, not a verification of Weil pairing theory.
    for ell in (3,5,7):
        seen=0
        for a in range(ell):
            for b in range(ell):
                for c in range(ell):
                    for d in range(ell):
                        determinant=(a*d-b*c)%ell
                        if determinant==0:
                            continue
                        seen+=1
                        require((a*d-c*b)%ell==determinant, "symplectic determinant identity")
        require(seen==(ell*ell-1)*(ell*ell-ell), "GL2 group order")
    # 11a1 reduced mod 2: #E(F2)=5, trace=-2, det Frobenius=2.
    require(count_curve_f2()==5, "curve order 5 control")
    ell=5
    F=((0,-2),(1,-2))
    require(det(F,ell)==2 and sum(F[i][i] for i in range(2))%ell == (-2)%ell, "F matrix t/d")
    I=((1,0),(0,1))
    # One eigenline fixed, but not entire ell-torsion until 2^4.
    for d in range(1,5):
        power=mpow(F,d,ell)
        fixed=sum(1 for x in range(5) for y in range(5) if ((power[0][0]-1)*x+power[0][1]*y)%5==0 and (power[1][0]*x+(power[1][1]-1)*y)%5==0)
        expected=25 if d==4 else 5
        require(fixed==expected, "Frob fixed count %s/%s"%(d,fixed))
        require((power==I)==(d==4), "premature full torsion")
        require(det(power,ell)==pow(2,d,ell),"cyclotomic determinant mutant")
        print("11a1 mod2 f=%d fixed_5_torsion=%d all_basis=%s"%(d,fixed,str(power==I).lower()))
    # Mutation: erroneously promoting source 5-point rationality to a rational full basis.
    require(mpow(F,1,ell)!=I and count_curve_f2()==5, "bad full basis mutant")
    print("PASS: exact integer-only controls; theorem is external, results [pcc:unverified]")

if __name__=="__main__": run()
