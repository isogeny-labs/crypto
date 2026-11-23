#!/usr/bin/env python3
"""CT23: bounded F101 rational-product transport controls, not genus-2 maps.
Reproduce: python3 -S research/chained_jacobian_transfers_20261110/ct23_rational_product_transport.py
[pcc:unverified]
"""
from math import isqrt
P=101
SRC=((1,46),(44,21))
KERNELS=((72,100,35,1),(77,86,71,1),(59,46,8,1))
TARGETS=((26,58),(13,42),(69,50))
DUALS=((58,26,58,1),(42,13,8,1),(29,88,45,1))

def add(A,B,curve):
    if A is None: return B
    if B is None: return A
    a,_=curve; x,y=A; z,v=B
    if x==z and (y+v)%P==0: return None
    if x==z:
        assert y==v and y!=0
        m=(3*x*x+a)*pow(2*y,-1,P)%P
    else: m=(v-y)*pow((z-x)%P,-1,P)%P
    w=(m*m-x-z)%P
    return w,(m*(x-w)-y)%P

def mul(A,k,curve):
    out=None
    while k:
        if k&1: out=add(out,A,curve)
        A=add(A,A,curve); k//=2
    return out

def points(curve):
    a,b=curve; squares={}
    for y in range(P): squares.setdefault(y*y%P,[]).append(y)
    return [None]+[(x,y) for x in range(P)
                   for y in squares.get((x*x*x+a*x+b)%P,())]

def kohel(A,curve,kernel):
    """Unscaled degree-7 map: x->R(x), y->R'(x)*y."""
    if A is None: return None
    x,y=A; a,b=curve
    c0,c1,c2,c3=kernel; assert c3==1
    h=(x*x*x+c2*x*x+c1*x+c0)%P
    assert h!=0,"unexpected rational order-7 kernel point"
    d=(3*x*x+2*c2*x+c1)%P
    d2=(6*x+2*c2)%P
    i=pow(h,-1,P)
    f=(x*x*x+a*x+b)%P
    df=(3*x*x+a)%P
    A0=(6*x*x+2*a)%P
    B=(d*d-h*d2)%P
    xx=(7*x+2*c2-A0*d*i+4*f*B*i*i)%P
    der=(7-12*x*d*i-A0*(d2*h-d*d)*i*i
         +4*(df*B*i*i+f*(d*d2-6*h)*i*i-2*f*B*d*i*i*i))%P
    return xx,der*y%P

def reverse(A,curve,kernel):
    if A is None: return None
    x,y=kohel(A,curve,kernel)
    u=pow(7,-1,P)
    return x*u*u%P,y*u*u*u%P

def bsgs(G,Q,curve,order):
    """Cold BSGS; counts elliptic additions, not complete field cost."""
    m=isqrt(order)
    if m*m<order:m+=1
    table={};T=None;ops=0
    for i in range(m):
        table[T]=i
        T=add(T,G,curve);ops+=1
    minus=None if T is None else (T[0],(-T[1])%P)
    R=Q
    for j in range(m+1):
        if R in table:
            k=(j*m+table[R])%order
            assert mul(G,k,curve)==Q
            return k,ops
        R=add(R,minus,curve);ops+=1
    raise AssertionError("BSGS failed")

def run():
    e1,e2=SRC
    p1,p2=points(e1),points(e2)
    assert len(p1)==109 and len(p2)==113
    G=(2,37)
    assert G in p1 and mul(G,109,e1) is None
    for label,i in (("B+A",1),("B+B",2)):
        t1,t2=TARGETS[0],TARGETS[i]
        k1,k2=KERNELS[0],KERNELS[i]
        d1,d2=DUALS[0],DUALS[i]
        dest1,dest2=points(t1),points(t2)
        assert (len(dest1),len(dest2))==(109,113)
        m1={A:kohel(A,e1,k1) for A in p1}
        m2={A:kohel(A,e2,k2) for A in p2}
        assert set(m1.values())==set(dest1)
        assert set(m2.values())==set(dest2)
        assert all(reverse(m1[A],t1,d1)==mul(A,7,e1) for A in p1)
        assert all(reverse(m2[A],t2,d2)==mul(A,7,e2) for A in p2)
        for A in p1:
            assert m1[add(A,G,e1)]==add(m1[A],m1[G],t1)
        for A in p2:
            assert m2[add(A,p2[1],e2)]==add(m2[A],m2[p2[1]],t2)
        images={(m1[A],m2[B]) for A in p1 for B in p2}
        assert len(images)==109*113==12317
        # Hom(C109,C113)=Hom(C113,C109)=0 on rational groups.
        assert sum(mul(B,109,t2) is None for B in dest2)==1
        assert sum(mul(A,113,t1) is None for A in dest1)==1
        # Wrong elliptic codomain must fail.
        wrong_t2=TARGETS[3-i]
        assert any(D is not None and
                   (D[1]*D[1]-D[0]**3-wrong_t2[0]*D[0]-wrong_t2[1])%P!=0
                   for D in m2.values())
        target_g=m1[G]
        source_total=target_total=0
        for j in range(109):
            D=mul(G,j,e1)
            assert m1[D]==mul(target_g,j,t1)
            ds,cs=bsgs(G,D,e1,109)
            dt,ct=bsgs(target_g,m1[D],t1,109)
            assert ds==dt==j
            source_total+=cs;target_total+=ct
        assert source_total==target_total
        id1,id2=9266,3052
        assert id1%109==1 and id1%113==0
        assert id2%109==0 and id2%113==1
        for A in p1:
            assert mul(A,id1,e1)==A and mul(A,id2,e1) is None
        for B in p2:
            assert mul(B,id2,e2)==B and mul(B,id1,e2) is None
        print(label,"product=12317, distinct images=12317,"
              " forward/dual[7], CRT=all points, DLP=109/109,"
              " BSGS direct/transport additions=",source_total,target_total,
              "map evaluations=109")
    print("PASS rational product-coordinate control only; "
          "not balanced Cantor/Phi or complete cost [pcc:unverified]")
if __name__=="__main__":run()
