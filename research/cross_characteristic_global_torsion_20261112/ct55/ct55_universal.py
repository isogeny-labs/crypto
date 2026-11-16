#!/usr/bin/env python3
"""CT55: Exact universal good-reduction CRT lift and binary Frobenius gates."""
from math import isqrt


def require(ok, message):
    if not ok:
        raise AssertionError(message)


def isprime(n):
    return n>=2 and all(n%d for d in range(2,isqrt(n)+1))


def count_short(p,a,b):
    count=1
    for x in range(p):
        rhs=(x*x*x+a*x+b)%p
        count += 1 if rhs==0 else (2 if pow(rhs,(p-1)//2,p)==1 else 0)
    return count


def short_points(p,a,b):
    return [(x,y) for x in range(p) for y in range(p) if (y*y-x*x*x-a*x-b)%p==0]


def add(p,a,u,v):
    if u is None: return v
    if v is None: return u
    x,y=u; s,t=v
    if x==s and (y+t)%p==0: return None
    if u==v:
        m=((3*x*x+a)*pow(2*y,-1,p))%p
    else:
        m=((t-y)*pow(s-x,-1,p))%p
    X=(m*m-x-s)%p
    return (X,(m*(x-X)-y)%p)


def mul(p,a,u,n):
    r=None
    while n:
        if n&1:r=add(p,a,r,u)
        u=add(p,a,u,u)
        n//=2
    return r


def crt_even(x,p):
    return x if x%2==0 else x+p


def source_case(p,ell):
    for a in range(p):
        for b in range(p):
            if (4*a*a*a+27*b*b)%p==0:continue
            count=count_short(p,a,b)
            if count%ell:continue
            for P in short_points(p,a,b):
                if mul(p,a,P,ell) is None:
                    return a,b,count,P
    raise AssertionError((p,ell,"no sample"))


def full_degree(ell):
    require(isprime(ell) and ell>2,"ell must be odd prime")
    x=(-2)%ell
    for k in range(1,ell*ell+1):
        if pow(x,k,ell)==1:return 2*k
    raise AssertionError("bad order")


def point_count_binary(f):
    # Frobenius polynomial X^2 + 2; t0=2,t1=0,t_f=-2*t_(f-2)
    t0,t1=2,0
    if f==0:return 1 # unused
    if f==1:return 3
    for i in range(2,f+1):
        t0,t1=t1,-2*t0
    return (1<<f)+1-t1


def first_some_degree(ell):
    fmax=full_degree(ell)
    for f in range(1,fmax+1):
        if point_count_binary(f)%ell==0:return f
    raise AssertionError("full degree must yield rational ell torsion")


def run():
    source=[(5,3),(7,5),(11,7),(13,3),(17,5),(19,7),(23,11),(29,5),(31,7)]
    for p,ell in source:
        a,b,order,P=source_case(p,ell)
        A,B=crt_even(a,p),crt_even(b,p)
        # global E:y^2+p y=x^3 + A x+B, discriminant =-64 A^3 -27(p^2+4B)^2
        delta=-64*A**3-27*(p*p+4*B)**2
        require(delta%p!=0 and delta%2!=0,"bad global reduction")
        require(A%p==a and B%p==b and A%2==B%2==0,"CRT mismatch")
        require(mul(p,a,P,ell) is None,"source order mismatch")
        q=point_count_binary(full_degree(ell))
        require(q%ell==0,"target full torsion not possible")
        print("source",p,"ell",ell,"a,b",a,b,"group",order,"P",P,"lift A,B",A,B,"binary_f",full_degree(ell))
    print("ell : f_some, f_full, #E(F_2^f_full) divisible by ell^2")
    for ell in [3,5,7,11,13,17,19,23,31,37,41,43]:
        fs,ff=first_some_degree(ell),full_degree(ell)
        require(ff%fs==0,"fixed-point subspace order not divide full action")
        require(point_count_binary(ff)%(ell*ell)==0,"full torsion order check")
        require(all(point_count_binary(j)%ell for j in range(1,fs)),"nonminimal f_some")
        # F^2=-2 I. First even k with F^k=I is precisely 2*ord_ell(-2).
        require(pow(-2,ff//2,ell)==1,"Frob full order fails")
        require(all(pow(-2,j,ell)!=1 for j in range(1,ff//2)),"Frob minimal order fails")
        print(ell,fs,ff)
    # Wrong CRT parity mutates binary fiber: y^2+y=x^3+x has 5, not 3, points.
    def count_binary_mutant(c):
        return 1+sum((y*y+y-x*x*x-c*x)%2==0 for x in range(2) for y in range(2))
    require(count_binary_mutant(0)==3 and count_binary_mutant(1)==5,
            "binary parity mutant was not detected")
    print("PASS: CRT good reduction, source orders, Hasse/Frobenius target degrees; parity mutant rejected; theorem requires division field")

if __name__=='__main__': run()
