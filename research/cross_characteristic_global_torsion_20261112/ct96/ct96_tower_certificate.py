#!/usr/bin/env python3
"""CT96 exact field-degree and GL2 orbit census. Standing: [pcc:unverified]."""
import json
from itertools import product

def check(b, msg):
    if not b: raise ArithmeticError(msg)
def trim(a):
    while len(a)>1 and a[-1]==0: a.pop()
    return a
def mul(a,b):
    c=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b): c[i+j]+=x*y
    return trim(c)
def sub(a,b):
    return trim([(a[i] if i<len(a) else 0)-(b[i] if i<len(b) else 0)
                 for i in range(max(len(a),len(b)))])
def power(a,k):
    c=[1]
    for _ in range(k): c=mul(c,a)
    return c
def div7():
    # Reconstruct CT84's psi_7 using integer polynomials, not CT84's routines.
    D=[-79,-40,-4,4]
    P3=[-21,-237,-60,-4,3]
    F4=[-5821,-1496,-210,-790,-100,-4,2]
    return sub(mul(sub(mul(power(D,2),F4),power(P3,3)),power(P3,3)),
               mul(power(D,2),power(F4,3)))
def mp(a,p=3): return trim([x%p for x in a])
def rem(a,f,p=3):
    a=mp(a[:],p);f=mp(f[:],p); inv=pow(f[-1],-1,p)
    while len(a)>=len(f) and a!=[0]:
        q=a[-1]*inv%p; shift=len(a)-len(f)
        for i,x in enumerate(f): a[shift+i]=(a[shift+i]-q*x)%p
        trim(a)
    return a
def pgcd(a,b,p=3):
    while b!=[0]: a,b=b,rem(a,b,p)
    return mp([c*pow(a[-1],-1,p) for c in a],p)
def ppow(a,n,f,p=3):
    r=[1]
    while n:
        if n&1:r=rem(mul(r,a),f,p)
        a=rem(mul(a,a),f,p); n>>=1
    return r
def irreducible_mod3():
    f=mp(div7());check(len(f)==25 and f[-1]==1,"degree/LC of psi7")
    x=[0,1]; u=x;gcd_degrees={}
    for k in range(1,25):
        u=ppow(u,3,f)
        if k in (8,12):
            g=pgcd(f,sub(u,x))
            gcd_degrees[str(k)]=len(g)-1
            check(g==[1], "Rabin gcd at "+str(k))
    check(u==x,"Rabin power at 24")
    return gcd_degrees
def census(l):
    gl=s_line=s_x=s_vec=0
    lines=set();xroots=set();oriented=set()
    for a,b,c,d in product(range(l),repeat=4):
        if (a*d-b*c)%l==0:continue
        gl+=1; v=(a,c); oriented.add(v)
        neg=tuple((-z)%l for z in v)
        xroots.add(min(v,neg))
        lead=next(z for z in v if z)
        lines.add(tuple(z*pow(lead,-1,l)%l for z in v))
        if c==0:s_line+=1
        if v in ((1,0),(l-1,0)):s_x+=1
        if v==(1,0):s_vec+=1
    sizes=[len(lines),len(xroots),len(oriented)]
    want=[l+1,(l*l-1)//2,l*l-1]
    check(sizes==want and [gl//s_line,gl//s_x,gl//s_vec]==want,
          "GL2 orbit-stabilizer")
    check(gl==l*(l-1)**2*(l+1),"GL2 size")
    return dict(ell=l,gl2=gl,line_degree=sizes[0],x_degree=sizes[1],
                oriented_degree=sizes[2],x_over_line=(l-1)//2,oriented_over_x=2)
def frob(l):
    # Characteristic-2 Frobenius on seven-torsion: companion of X^2+2X+2.
    A=(0,(-2)%l,1,(-2)%l); T=(1,0,0,1)
    def mm(v,w):
        a,b,c,d=v;e,f,g,h=w
        return ((a*e+b*g)%l,(a*f+b*h)%l,(c*e+d*g)%l,(c*f+d*h)%l)
    first=None; full=0
    for k in range(1,l*l):
        T=mm(T,A)
        if first is None and ((T[0]-1)*(T[3]-1)-T[1]*T[2])%l==0:first=k
        if T==(1,0,0,1):full=k;break
    check(first is not None and full and (l*l-1)%full==0,"binary Frobenius")
    return dict(ell=l,binary_first_nonzero_torsion_degree=first,
                binary_full_torsion_degree=full)
def main():
    gcds=irreducible_mod3()
    towers=[census(l) for l in (3,5,7,11,13)]
    orders=[frob(l) for l in (3,5,7,11,13,17,19,23,31)]
    check((towers[2]["line_degree"],towers[2]["x_degree"],towers[2]["oriented_degree"])==(8,24,48),
          "CT80/87 degree mismatch")
    check(orders[2]["binary_full_torsion_degree"]==24,"CT94 mismatch")
    print(json.dumps(dict(binary_orders=orders,field_7_division_degree=24,
                          mod3_irreducible=True,rabin_gcd_degrees=gcds,
                          towers=towers),sort_keys=True,separators=(',',':')))
if __name__=="__main__":main()
