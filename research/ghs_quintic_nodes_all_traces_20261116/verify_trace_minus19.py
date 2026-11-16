#!/usr/bin/env sage -python
"""Independent exact source/carrier certificate for trace minus nineteen."""
from sage.all import GF, PolynomialRing, ProjectiveSpace, Curve, EllipticCurve, ZZ
from pathlib import Path
import hashlib
import json

F3=GF(3)
R=PolynomialRing(F3,names=("x","y","z"));x,y,z=R.gens()
f=(x**6-x**5*y+x**4*y**2+x**2*y**4+y**6+x**5*z+x**4*y*z
   +x**3*y**2*z+x**2*y**3*z+x**4*z**2-x**3*y*z**2+x**3*z**3
   -x**2*y*z**3+y*z**5+z**6)
P2=ProjectiveSpace(F3,2,names=("x","y","z"))
C=Curve(P2.subscheme([f]))
assert C.is_irreducible() and C.geometric_genus()==5
assert len(f.change_ring(GF(9,name="a")).factor())==1
assert len(f.change_ring(GF(27,name="b")).factor())==1

K=GF(243,name="u");u=K.gen()
assert str(K.modulus())=="x^5 + 2*x + 1"
node=(K(1),u**127,u**143)
nodes=[tuple(v**(3**j) for v in node) for j in range(5)]
assert len(set(nodes))==5
for p in nodes:
    assert f.change_ring(K)(*p)==0
    assert f.derivative(y).change_ring(K)(*p)==0
    assert f.derivative(z).change_ring(K)(*p)==0
a=K(2)*f.derivative(y).derivative(y).change_ring(K)(*node)
b=f.derivative(y).derivative(z).change_ring(K)(*node)
c=K(2)*f.derivative(z).derivative(z).change_ring(K)(*node)
disc=b*b-a*c
assert disc!=0 and disc**121==1

counts=[]
for r in (1,2,3,4,5):
    L=GF(3**r,name="v");es=list(L);one=L(1);zero=L(0)
    pts=[(one,s,t) for s in es for t in es]
    pts.extend((zero,one,s) for s in es)
    pts.append((zero,zero,one))
    q=f.change_ring(L)
    plane=sum(q(*p)==0 for p in pts)
    counts.append(int(plane)+(5 if r==5 else 0))
assert counts==[4,10,28,82,339]

power_sums=[0]+[3**r+1-counts[r-1] for r in (1,2,3,4,5)]
coeff=[ZZ(1)]
for k in (1,2,3,4,5):
    numerator=sum(coeff[k-i]*power_sums[i] for i in range(1,k+1))
    assert numerator%k==0
    coeff.append(ZZ(-numerator//k))
coeff.extend([ZZ(0)]*5)
for k in range(5):coeff[10-k]=ZZ(3**(5-k))*coeff[k]
S=PolynomialRing(ZZ,name="T");T=S.gen()
Lpoly=sum(coeff[i]*T**i for i in range(11))
assert Lpoly==1+19*T**5+243*T**10

E=EllipticCurve(K,[0,u,0,0,u**4+u**2+2])
points=E.points()
assert len(points)==263
base_point=E(0,u**3+2*u**2)
assert base_point.order()==263
target=7*base_point
assert target==E(2*u**4+2*u,u**4+u**3+2)
assert Lpoly==1+(len(points)-244)*T**5+243*T**10

print(json.dumps({"schema":"ghs.quintic-nodes-trace-minus19-certificate/v1",
                  "source_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  "sextic":str(f),"genus":int(C.geometric_genus()),
                  "absolutely_irreducible":True,"ordinary_node_orbit":5,
                  "plane_counts":[4,10,28,82,334],"normalized_counts":counts,
                  "L_polynomial":str(Lpoly),"jacobian_F3_order":int(Lpoly(1)),
                  "elliptic_field_modulus":str(K.modulus()),
                  "elliptic_ainvs":[str(v) for v in E.a_invariants()],
                  "elliptic_F243_order":len(points),"source_point":str(base_point),
                  "source_point_order":int(base_point.order()),
                  "validation_target_7P":str(target)},sort_keys=True))
