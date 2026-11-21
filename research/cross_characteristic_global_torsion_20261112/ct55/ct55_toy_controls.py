#!/usr/bin/env python3
"""CT55: exact rational positive and naive-lift negative controls."""
from fractions import Fraction as F

O = None


def check(cond, message):
    if not cond:raise AssertionError(message)


def inv(t, p):
    if p is None:return 1/t
    return pow(int(t)%p,-1,p)


def on(T, p, A, B, q=None):
    if T is None:return True
    x,y=T
    z=y*y+p*y-x*x*x-A*x-B
    return z==0 if q is None else z%q==0


def add(P,Q,p,A,B,q=None):
    if P is None:return Q
    if Q is None:return P
    x,y=P;u,v=Q
    if q is not None:
        x,y,u,v=(int(t)%q for t in (x,y,u,v))
    if x==u and (y+v+p if q is None else (y+v+p)%q)==0:return None
    if x==u and y==v:
        lam=(3*x*x+A)*inv(2*y+p,q)
    else:
        lam=(v-y)*inv(u-x,q)
    if q is not None:lam%=q
    nu=y-lam*x
    X=lam*lam-x-u
    Y=-lam*X-nu-p
    if q is not None:X,Y=X%q,Y%q
    T=(X,Y)
    check(on(T,p,A,B,q),"sum off curve")
    return T


def mult(P,n,p,A,B,q=None):
    out=None
    while n:
        if n&1:out=add(out,P,p,A,B,q)
        P=add(P,P,p,A,B,q);n//=2
    return out


def reduced(P,q):
    if P is None:return None
    return tuple(int(t.numerator*pow(t.denominator,-1,q))%q for t in P)


def main():
    p,A,B=(5,0,6)
    P=(F(0),F(1))
    check(on(P,p,A,B),"bad rational source")
    global3=[mult(P,n,p,A,B) for n in range(3)]
    check(mult(P,3,p,A,B) is None,"global rational 3-torsion failed")
    for src,dst in [(5,2),(5,5)]:
        src_pts=[reduced(T,src) for T in global3]
        dst_pts=[reduced(T,dst) for T in global3]
        check(len(set(src_pts))==len(set(dst_pts))==3,"reduction lost points")
        t=dict(zip(src_pts,dst_pts))
        for i in range(3):
            for j in range(3):
                lhs=t[src_pts[(i+j)%3]]
                rhs=add(t[src_pts[i]],t[src_pts[j]],p,A,B,dst)
                check(lhs==rhs,"positive transfer not a homomorphism")
        for i in range(3):
            trial=None
            for k in range(3):
                if trial==dst_pts[i]:break
                trial=add(trial,dst_pts[1],p,A,B,dst)
            else:raise AssertionError("toy log not found")
            check(i==k,"toy log mismatch")
    check(global3==[None,(F(0),F(1)),(F(0),F(-6))],"unexpected global points")
    print("positive p5 to p2: rational 3-torsion, all 9 group pairs, all 3 public logs")

    # Same CRT construction with p7, ell5; literal point coordinates lift, torsion does not.
    p,A,B=(7,8,8)
    R=(F(0),F(1))
    check(on(R,p,A,B),"bad rational naive lift")
    check(mult(R,5,p,A,B) is not None,"negative mutant unexpectedly 5-torsion over Q")
    R7=reduced(R,7);R2=reduced(R,2)
    check(mult(R7,5,p,A,B,7) is None,"source is not 5-torsion")
    check(mult(R2,3,p,A,B,2) is None,"binary image is not 3-torsion")
    check(mult(R2,1,p,A,B,2) is not None,"binary image is zero")
    check(mult(R2,5,p,A,B,2)==mult(R2,2,p,A,B,2),"mod2 order mutant missed")
    print("negative p7 to p2: naive rational lift has order 5 at source but 3 at target")
    print("PASS: exact rational algebra and independent finite reductions; no scalable map")

if __name__=='__main__':main()
