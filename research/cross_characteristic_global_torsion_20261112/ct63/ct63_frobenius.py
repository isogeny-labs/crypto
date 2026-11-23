#!/usr/bin/env python3
"""CT63: direct F_(4^n) point counts and cyclotomic Frobenius checks, n=1..5."""
import json

def deg(a): return a.bit_length()-1
def poly_mod(a,b):
    while a and deg(a)>=deg(b): a^=b << (deg(a)-deg(b))
    return a
def poly_gcd(a,b):
    while b: a,b=b,poly_mod(a,b)
    return a
def poly_square(a):
    out=0; bit=0
    while a:
        if a&1:out|=1<<(2*bit)
        a>>=1;bit+=1
    return out
def verify_irreducible(k,f):
    x=2
    for i in range(1,k+1):
        x=poly_mod(poly_square(x),f)
        if i<=k//2 and poly_gcd(x^2,f)!=1:
            raise ValueError(('reducible field',k,f,i))
    if x!=2:raise ValueError(('wrong Frobenius period',k,f))
def fmul(a,b,k,irr):
    mask=(1<<k)-1
    out=0
    while b:
        if b&1:out^=a
        b>>=1
        a<<=1
        if a&(1<<k):a^=irr
    return out&mask
def fpow(a,n,k,irr):
    y=1
    while n:
        if n&1:y=fmul(y,a,k,irr)
        a=fmul(a,a,k,irr);n>>=1
    return y
def point_counts(n):
    k=2*n;q=1<<k;irr={2:7,4:19,6:67,8:285,10:1033}[k]
    verify_irreducible(k,irr)
    roots=[a for a in range(q) if fmul(a,a,k,irr)^a^1==0]
    if len(roots)!=2:raise ValueError(('omega roots',k,roots))
    omega=roots[0]
    if omega in (0,1) or fpow(omega,4,k,irr)!=omega:
        raise ValueError('omega embedding')
    nC=nE=1
    for x in range(q):
        x2=fmul(x,x,k,irr)
        x3=fmul(x2,x,k,irr)
        x5=fmul(x3,x2,k,irr)
        rhsC=x5^fmul(omega,x3,k,irr)
        rhsE=x3^omega
        for rhs,kind in ((rhsC,'C'),(rhsE,'E')):
            tr=0;z=rhs
            for _ in range(k):
                tr^=z
                z=fmul(z,z,k,irr)
            if tr not in (0,1):raise ValueError('trace outside F2')
            if tr==0:
                if kind=='C':nC+=2
                else:nE+=2
    expectedC=q+1+(1<<n) if n%5 else q+1-4*(1<<n)
    expectedE=(1-(1<<n))**2
    if (nC,nE)!=(expectedC,expectedE):
        raise AssertionError((n,nC,expectedC,nE,expectedE))
    return {'n':n,'curve_points':nC,'elliptic_points':nE}
def frob_product(n):
    z=1<<n
    return sum(z**i for i in range(5)) if n%5 else (1-z)**4
def main():
    entries=[point_counts(n) for n in range(1,6)]
    for e in entries:e['jacobian_points']=frob_product(e['n'])
    if entries[0]['jacobian_points']!=31 or entries[4]['jacobian_points']!=31**4:
        raise AssertionError('wrong Jacobian group')
    print(json.dumps({'model':'C: y^2+y=x^5+omega*x^3; E: y^2+y=x^3+omega',
      'frobenius':'X^4+2X^3+4X^2+8X+16=16 Phi5(X/2)',
      'minimal_geometric_split_degree':5,'entries':entries},separators=(',',':')))
if __name__=='__main__':main()
