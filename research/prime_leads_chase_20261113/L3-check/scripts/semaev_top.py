# Independent check of (a) Semaev S_4, S_5 vanishing on genuine decompositions and
# (b) the claim f_top = c*(x_1...x_m)^(2^(m-1)) for the x-top homogeneous part of S_{m+1}(x_1..x_m, r).
# Uses sympy resultants over GF(p) (independent of pcdeg's hand-written Sylvester code).
import random, sys
from sympy import symbols, Poly, GF, resultant
p = 1048583  # prime ~2^20
random.seed(7)
U, V = symbols('U V')
def inv(a): return pow(a % p, p - 2, p)
def s3poly(x1, x2, var, a, b):
    # S3(x1,x2,X) = (x1-x2)^2 X^2 - 2((x1+x2)(x1 x2 + a) + 2b) X + (x1 x2 - a)^2 - 4b(x1+x2)
    return Poly(((x1-x2)**2) * var**2 - 2*((x1+x2)*(x1*x2+a) + 2*b)*var + (x1*x2-a)**2 - 4*b*(x1+x2), var, modulus=p)
def S3(x1,x2,x3,a,b):
    return int(s3poly(x1,x2,U,a,b).eval(x3)) % p
def S4(x1,x2,x3,x4,a,b):
    # Res_U(S3(x1,x2,U), S3(x3,x4,U))
    return int(resultant(s3poly(x1,x2,U,a,b), s3poly(x3,x4,U,a,b))) % p
def S4poly_in_U(x3,x4,r,a,b):
    # S4(U,x3,x4,r) as polynomial in U: Res_V(S3(U,x3,V), S3(x4,r,V)) symbolic in U
    f = Poly((U-x3)**2*V**2 - 2*((U+x3)*(U*x3+a)+2*b)*V + (U*x3-a)**2 - 4*b*(U+x3), V, U, modulus=p)
    g = Poly(((x4-r)**2)*V**2 - 2*((x4+r)*(x4*r+a)+2*b)*V + (x4*r-a)**2 - 4*b*(x4+r), V, U, modulus=p)
    R = resultant(f, g, V)
    return Poly(R.as_expr(), U, modulus=p)
def S5(x1,x2,x3,x4,r,a,b):
    return int(resultant(s3poly(x1,x2,U,a,b), S4poly_in_U(x3,x4,r,a,b))) % p
# curve
while True:
    a = random.randrange(p); b = random.randrange(1,p)
    if (4*a**3 + 27*b*b) % p: break
def rhs(x): return (x*x*x + a*x + b) % p
def sqrt(v):
    v %= p
    if v == 0: return 0
    if pow(v,(p-1)//2,p) != 1: return None
    # p = 1048583: p % 4 == 3?
    assert p % 4 == 3
    return pow(v,(p+1)//4,p)
def add(P,Q):
    if P is None: return Q
    if Q is None: return P
    (x1,y1),(x2,y2) = P,Q
    if x1==x2 and (y1+y2)%p==0: return None
    if x1==x2: lam=(3*x1*x1+a)*inv(2*y1)%p
    else: lam=(y2-y1)*inv(x2-x1)%p
    x3=(lam*lam-x1-x2)%p; return (x3,(lam*(x1-x3)-y1)%p)
def rpt():
    while True:
        x=random.randrange(p); y=sqrt(rhs(x))
        if y is not None: return (x,y)
def neg(P): return (P[0],(-P[1])%p)
ok=True
for trial in range(3):
    P=[rpt() for _ in range(4)]
    R3=neg(add(add(P[0],P[1]),P[2]))   # P1+P2+P3+R3 = O
    v4=S4(P[0][0],P[1][0],P[2][0],R3[0],a,b)
    R4=neg(add(add(add(P[0],P[1]),P[2]),P[3]))
    v5=S5(P[0][0],P[1][0],P[2][0],P[3][0],R4[0],a,b)
    print('S4 on decomposition:',v4,' S5 on decomposition:',v5, ' S4 on random:', S4(*(random.randrange(p) for _ in range(4)),a,b)!=0)
    ok = ok and v4==0 and v5==0
print('Semaev vanishing check:', ok)
# top-form check: coefficient of t^{me} in S_{m+1}(t*y, r) vs (prod y)^e ; also t^{me+1..me+2} must vanish
def interp_mod(xs, ys):
    # coefficients of the unique poly of degree < len(xs) through (xs, ys) mod p (Newton divided differences)
    n=len(xs); c=list(ys)
    for j in range(1,n):
        for i in range(n-1,j-1,-1):
            c[i]=(c[i]-c[i-1])*inv(xs[i]-xs[i-j])%p
    poly=[0]*n
    for i in range(n-1,-1,-1):
        # poly = poly*(x - xs[i]) + c[i]
        new=[0]*n
        for k in range(n-1):
            new[k+1]=(new[k+1]+poly[k])%p
        for k in range(n):
            new[k]=(new[k]-xs[i]*poly[k])%p
        new[0]=(new[0]+c[i])%p
        poly=new
    return {k:v for k,v in enumerate(poly)}
def topcheck(m, S, ntrials=4):
    e = 2**(m-1); N = m*e
    rs = [random.randrange(p) for _ in range(2)]
    ratios=set(); extra=set()
    for r in rs:
        for _ in range(ntrials):
            y=[random.randrange(1,p) for _ in range(m)]
            ts=list(range(1,N+4))
            vals=[S(*[t*yi % p for yi in y], r) for t in ts]
            # interpolate degree <= N+2 polynomial through N+3 points
            co=interp_mod(ts,vals)
            lead=co.get(N,0); extra.add((co.get(N+1,0), co.get(N+2,0)))
            prod=1
            for yi in y: prod=prod*yi%p
            ratios.add(lead*inv(pow(prod,e,p))%p)
    return ratios, extra
print('m=2 (S3): ratio set', *topcheck(2, lambda x1,x2,r: S3(x1,x2,r,a,b)))
print('m=3 (S4): ratio set', *topcheck(3, lambda x1,x2,x3,r: S4(x1,x2,x3,r,a,b), 3))
print('m=4 (S5): ratio set', *topcheck(4, lambda x1,x2,x3,x4,r: S5(x1,x2,x3,x4,r,a,b), 2))
