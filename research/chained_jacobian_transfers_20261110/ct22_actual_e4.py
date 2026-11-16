#!/usr/bin/env python3
"""CT22b: actual E[4] and Kohel x/y maps in F101^3(zeta), zeta^2=2.
Independent stdlib arithmetic oracle, not a genus-2 divisor map. [pcc:unverified]
Run: python3 -S research/chained_jacobian_transfers_20261110/ct22_actual_e4.py
"""
P=101
Q=P**3
class F:
    __slots__=('c',)
    def __init__(self,x=0):
        if isinstance(x,F):self.c=x.c
        elif isinstance(x,int):self.c=(x%P,0,0)
        else:self.c=tuple(v%P for v in x)
    def __repr__(self):return str(self.c)
    def __eq__(self,o):return self.c==F(o).c
    def __hash__(self):return hash(self.c)
    def __add__(self,o):return F([(a+b)%P for a,b in zip(self.c,F(o).c)])
    __radd__=__add__
    def __neg__(self):return F([-a for a in self.c])
    def __sub__(self,o):return self+-F(o)
    def __rsub__(self,o):return F(o)-self
    def __mul__(self,o):
        o=F(o).c;a=[0]*5
        for i in range(3):
            for j in range(3):a[i+j]=(a[i+j]+self.c[i]*o[j])%P
        for i in (4,3):
            for k,c in enumerate((56,13,6)):a[i-3+k]=(a[i-3+k]-c*a[i])%P
        return F(a[:3])
    __rmul__=__mul__
    def __pow__(self,n):
        if n<0:return self**(Q-1+n%(Q-1))
        out=F(1);v=self
        while n:
            if n&1:out=out*v
            v=v*v;n//=2
        return out
    def __truediv__(self,o):
        o=F(o);assert o!=0
        return self*o**(Q-2)
    def __rtruediv__(self,o):return F(o)/self
    def __bool__(self):return self!=0

def is_square(a):
    a=F(a)
    return a==0 or a**((Q-1)//2)==F(1)
def sqrt(a):
    a=F(a)
    assert is_square(a)
    if a==0:return F(0)
    # Q=101^3 == 5 mod 8, and 2 is a nonsquare in this odd extension.
    z=a**((Q+3)//8)
    if z*z!=a:
        z=z*F(2)**((Q-1)//4)
    assert z*z==a
    return z
def cubic(x,a,b):return x*x*x+a*x+b
def h(x,k):return x**3+k[2]*x*x+k[1]*x+k[0]
def h1(x,k):return 3*x*x+2*k[2]*x+k[1]
def h2(x,k):return 6*x+2*k[2]
def phim(x,a,b,k):
    # Full Kohel odd-degree-7 rational x-map; h=monic cubic kernel.
    hh=h(x,k);assert hh!=0
    d=h1(x,k);d2=h2(x,k)
    return 7*x+2*k[2]-(6*x*x+2*a)*d/hh+4*cubic(x,a,b)*(d*d-hh*d2)/(hh*hh)
def phim2(x,a,k):
    hh=h(x,k);assert hh!=0
    return 7*x+2*k[2]-(6*x*x+2*a)*h1(x,k)/hh
def phi_deriv(x,a,b,k):
    hh=h(x,k);assert hh!=0
    d=h1(x,k);d2=h2(x,k)
    A=6*x*x+2*a; A1=12*x
    f=cubic(x,a,b);f1=3*x*x+a
    B=d*d-hh*d2;B1=d*d2-6*hh
    return 7-A1*d/hh-A*(d2*hh-d*d)/(hh*hh)+4*(f1*B/(hh*hh)+f*(B1*hh-2*B*d)/(hh**3))

# All 4-torsion points have x in F101^3; primitive y lies in
# zeta*F101^3 where zeta^2=2 and zeta^(101^3)=-zeta.
# Represent the point (x,zeta*u) by (x,u), identity by None.
def E_add(Pt,Qt,a):
    if Pt is None:return Qt
    if Qt is None:return Pt
    x,u=Pt; z,v=Qt
    if x==z:
        if u+v==0:return None
        assert u==v and u!=0
        ell=(3*x*x+a)/(4*u)
    else:
        ell=(v-u)/(z-x)
    x3=2*ell*ell-x-z
    u3=ell*(x-x3)-u
    return (x3,u3)
def E_mul(point,n,a):
    out=None
    while n:
        if n&1:out=E_add(out,point,a)
        point=E_add(point,point,a);n//=2
    return out
def e4_points(rs,a,b):
    points=[None]
    for r in rs:
        points.append((r,F(0)))
        # Duplication equation x([2]R)=r is the square
        # (X^2 - 2*r*X - a - 2*r^2)^2.
        s=sqrt(3*r*r+a)
        for x in (r+s,r-s):
            u=sqrt(cubic(x,a,b)/2)
            assert 2*u*u==cubic(x,a,b) and u!=0
            assert x*x-2*r*x-a-2*r*r==0
            points.append((x,u));points.append((x,-u))
    assert len(points)==16 and len(set(points))==16
    for z in points:
        if z is not None:
            assert 2*z[1]*z[1]==cubic(z[0],a,b)
        assert E_mul(z,4,a) is None
    for x in points:
        for y in points:
            assert E_add(x,y,a) in points
    return points
def E_map(z,a,b,k):
    if z is None:return None
    x,u=z
    return (phim(x,a,b,k),u*phi_deriv(x,a,b,k))

theta=F((0,1,0))
tt=[theta**(P**i) for i in range(3)]
assert all(t**3+6*t*t+13*t+56==0 for t in tt)
roots=[([t+2 for t in tt],1,46),([56/t+38 for t in tt],44,21)]
fixtures=[('E1A',(9,76,20,1),(5,39)),('E1B',(72,100,35,1),(26,58)),('E2A',(77,86,71,1),(13,42)),('E2B',(59,46,8,1),(69,50))]
duals=[(49,69,62,1),(58,26,58,1),(42,13,8,1),(29,88,45,1)]
for idx,(name,k,target) in enumerate(fixtures):
    group=0 if name.startswith('E1') else 1
    rs,a,b=roots[group]
    E=e4_points(rs,a,b)
    mapped=[]
    for z in E:
        m=E_map(z,a,b,k)
        assert m is None or 2*m[1]*m[1]==cubic(m[0],*target)
        mapped.append(m)
    assert len(set(mapped))==16
    for s in E:
        for t in E:
            assert E_map(E_add(s,t,a),a,b,k)==E_add(E_map(s,a,b,k),E_map(t,a,b,k),target[0])
    dest=e4_points([phim2(r,a,k) for r in rs],*target)
    assert set(mapped)==set(dest)
    def dual(q):
        z=E_map(q,*target,duals[idx])
        if z is None:return None
        return (z[0]/49,z[1]/343)
    for z in E:
        assert dual(E_map(z,a,b,k))==E_mul(z,7,a)
    for z in dest:
        assert E_map(dual(z),a,b,k)==E_mul(z,7,target[0])
    def fp(z):
        if z is None:return None
        return (z[0]**P,-z[1]**P)
    for z in E:
        assert fp(z) in E
        assert fp(fp(fp(z)))==(None if z is None else (z[0],-z[1]))
        assert E_map(fp(z),a,b,k)==fp(E_map(z,a,b,k))
    for u0 in E:
        for v0 in E:
            assert fp(E_add(u0,v0,a))==E_add(fp(u0),fp(v0),a)
    print(name,'actual E[4]: 16 points, 256 homomorphism checks, dual[7], Frobenius, derivative pass')
print('Four Kohel E[4] maps checked with primitive 4-torsion over F101^6')

# Actual 64-lift HLP quotient, not just symbolic F2 matching.
E1=e4_points(*roots[0])
E2=e4_points(*roots[1])
a1,a2=roots[0][1],roots[1][1]
assert (a1,a2)==(1,44)
graph=[(None,None)]+[((roots[0][0][i],F(0)),(roots[1][0][i],F(0))) for i in range(3)]
def prodmul(z,n,ac1=a1,ac2=a2):
    return (E_mul(z[0],n,ac1),E_mul(z[1],n,ac2))
def classes(Ea,Eb,G,ac1,ac2):
    ix={z:i for i,z in enumerate(Ea)};iy={z:i for i,z in enumerate(Eb)}
    def add(z,w):return (E_add(z[0],w[0],ac1),E_add(z[1],w[1],ac2))
    def canon(z):return min((add(z,w) for w in G),key=lambda y:(ix[y[0]],iy[y[1]]))
    return canon,add

C0,plus0=classes(E1,E2,graph,a1,a2)
products=[(x,y) for x in E1 for y in E2]
lifts=[z for z in products if prodmul(z,2) in graph]
Q0=set(C0(z) for z in lifts)
K0={C0(z) for z in lifts if prodmul(z,2)==(None,None)}
assert (len(products),len(graph),len(lifts),len(Q0),len(K0))==(256,4,64,16,4)

for tag,j in [('B+A',2),('B+B',3)]:
    k1=fixtures[1][1];k2=fixtures[j][1]
    t1=fixtures[1][2];t2=fixtures[j][2]
    def transform(z):return (E_map(z[0],a1,46,k1),E_map(z[1],a2,21,k2))
    T1=e4_points([phim2(r,a1,k1) for r in roots[0][0]],*t1)
    T2=e4_points([phim2(r,a2,k2) for r in roots[1][0]],*t2)
    g1=[transform(z) for z in graph]
    C1,plus1=classes(T1,T2,g1,t1[0],t2[0])
    assert {transform(z) for z in products}=={(u,v) for u in T1 for v in T2}
    sl={transform(z) for z in lifts}
    tl={(u,v) for u in T1 for v in T2 if (E_mul(u,2,t1[0]),E_mul(v,2,t2[0])) in g1}
    assert sl==tl and len(sl)==64
    Q1={C1(z) for z in sl}
    assert len(Q1)==16
    K1={C1(transform(z)) for z in K0}
    K1_expected={C1(z) for z in tl if (E_mul(z[0],2,t1[0]),E_mul(z[1],2,t2[0]))==(None,None)}
    assert K1==K1_expected and len(K1)==4
    assert {C1(transform(z)) for z in Q0}==Q1
    for u in Q0:
        for v in Q0:
            assert C1(transform(plus0(u,v)))==C1(plus1(transform(u),transform(v)))
    rotated=[(None,None)]+[(g1[1+i][0],g1[1+(i+1)%3][1]) for i in range(3)]
    assert len(set(rotated)&set(g1))==1
    print(tag,'64 actual product E[4] lifts, 16 quotient classes, full addition and norm-kernel checks pass')
print('PASS actual 4-torsion product transfers; genus-2 divisor evaluator still outstanding [pcc:unverified]')
