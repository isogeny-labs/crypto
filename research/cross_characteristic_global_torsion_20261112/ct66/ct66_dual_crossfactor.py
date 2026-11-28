#!/usr/bin/env python3
"""Independent exhaustive split-fibre verification of dual quotient pairing.
No imported genus-two/Cantor code, and no elliptic DLP coordinate tables.
"""
import json
Q=1024;POL=0x409;L=31

def require(ok,why):
    if not ok: raise AssertionError(why)
def mul(a,b):
    z=0
    while b:
        if b&1:z^=a
        b>>=1;a<<=1
        if a&Q:a^=POL
    return z&(Q-1)
def power(a,n):
    z=1
    while n:
        if n&1:z=mul(z,a)
        a=mul(a,a);n>>=1
    return z
def inverse(a):
    require(a!=0,'inverse zero')
    return power(a,Q-2)
def cube(a):return mul(mul(a,a),a)
def fifth(a):return mul(mul(a,a),cube(a))
def curve_point(P,coef):
    if P is None:return True
    x,y=P;a,b,c=coef
    return mul(y,y)^y==mul(a,cube(x))^mul(b,mul(x,x))^mul(c,x)
def add(P,R,coef):
    if P is None:return R
    if R is None:return P
    x,y=P;z,w=R;a,b,c=coef
    if x==z and y!=w:return None
    slope=mul(a,mul(x,x))^c if x==z else mul(y^w,inverse(x^z))
    xx=x^z^mul(b^mul(slope,slope),inverse(a))
    yy=mul(slope,x^xx)^y^1
    require(curve_point((xx,yy),coef),'on elliptic quotient')
    return xx,yy

def orbit(P,co):
    xs=set();T=None
    for _ in range(L):
        require(T not in xs,'premature torsion collapse')
        xs.add(T);T=add(T,P,co)
    require(T is None and len(xs)==31,'elliptic 31 order')
    return xs
omega=next(x for x in range(2,Q) if mul(x,x)^x^1==0)
require(omega==236,'omega')
sol={}
for y in range(Q):sol.setdefault(mul(y,y)^y,[]).append(y)
parameters=(20,534,619,809,832)
models=[]
for r in parameters:
    r2=mul(r,r);r3=mul(r2,r);r4=mul(r2,r2)
    a=power(r,512);b=r4^mul(omega,r2)
    require(mul(a,r2)==mul(b,r) and mul(b,b)^a==mul(omega,r),'involution')
    cs=sol.get(mul(r4,r)^mul(omega,r3)); require(cs is not None and len(cs)==2,'Artin-Schreier')
    for c in cs:
        A=mul(a,inverse(r));D=mul(c^mul(A,r3),inverse(r))
        co=(inverse(r),r,mul(D,inverse(r)))
        models.append((r,a,b,c,A,D,co))

def projection(P,M):
    x,y=P;r,a,b,c,A,D,co=M
    ans=(mul(x,x)^mul(r,x), y^mul(A,cube(x))^mul(D,x))
    require(curve_point(ans,co),'quotient map equation')
    return ans

def sigma(P,M):
    x,y=P;r,a,b,c,A,D,co=M
    return (x^r,y^mul(a,mul(x,x))^mul(b,x)^c)
C=[(x,y) for x in range(Q) for y in sol.get(fifth(x)^mul(omega,cube(x)),())]
require(len(C)==896,'curve finite points')
# Make sure all 10 quotients contain 961 points, and the image of C generates rank 2.
for M in models:
    co=M[-1]
    pts=sum(len(sol.get(mul(co[0],cube(x))^mul(co[1],mul(x,x))^mul(co[2],x),())) for x in range(Q))
    require(pts==960,'elliptic order')
    proj=[projection(P,M) for P in C]
    primary=next(x for x in proj if x is not None)
    cyc=orbit(primary,co)
    require(any(x not in cyc for x in proj),'source curve projection does not span elliptic 31^2')
for i in range(0,10,2):
    for P in C:
        require(sigma(sigma(P,models[i]),models[i])==P,'involution square')
        require(sigma(sigma(P,models[i+1]),models[i+1])==P,'complement square')
        require(sigma(sigma(P,models[i]),models[i+1])==(P[0],P[1]^1),'product not hyperelliptic')

matrix=[]
for i,M in enumerate(models):
    row=[]
    for j,N in enumerate(models):
        coef=N[-1]
        image=[]
        for P in C:
            T=add(projection(P,N),projection(sigma(P,M),N),coef)
            if i==j:require(T==add(projection(P,M),projection(P,M),coef),'self norm != 2')
            if i//2==j//2 and i!=j:require(T is None,'complement does not vanish')
            if T is not None:image.append(T)
        if i//2==j//2 and i!=j:rank=0
        else:
            require(image,'missing nonzero map')
            cyc=orbit(image[0],coef)
            rank=2 if any(z not in cyc for z in image) else 1
            require(rank==2,'cross-factor map not full rank')
        row.append(rank)
    matrix.append(row)
print(json.dumps({'source_points':len(C),'parameter_count':len(models),'dual_quotient_at_r20':[models[0][-1],models[1][-1]],'rank_matrix':matrix,'noncomplementary_fullrank':sum(matrix[i][j]==2 for i in range(10) for j in range(10) if i//2!=j//2),'complement_zero':sum(matrix[i][j]==0 for i in range(10) for j in range(10) if i!=j and i//2==j//2)},sort_keys=True,separators=(',',':')))
