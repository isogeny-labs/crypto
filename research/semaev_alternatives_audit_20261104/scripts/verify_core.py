import sympy as sp

a, b, x1, x2, x3, xR, e1, e2, T = sp.symbols('a b x1 x2 x3 xR e1 e2 T')

# Semaev S_3
S3 = ((x1 - x2)**2 * x3**2
      - 2*((x1 + x2)*(x1*x2 + a) + 2*b)*x3
      + (x1*x2 - a)**2 - 4*b*(x1 + x2))

# 1) Vieta form: substitute x2 = e1 - x1 and reduce mod x1^2 - e1 x1 + e2
f = sp.expand(S3.subs(x2, e1 - x1).subs(x3, xR))
rem = sp.rem(sp.Poly(f, x1), sp.Poly(x1**2 - e1*x1 + e2, x1))
Q = sp.expand(rem.as_expr())
print("Vieta remainder degree in x1:", sp.degree(Q, x1))
print("Q =", sp.collect(sp.expand(Q), [e1, e2]))
target = (xR**2*(e1**2 - 4*e2) - 2*xR*(e1*e2 + a*e1 + 2*b) + e2**2 - 2*a*e2 + a**2 - 4*b*e1)
print("matches claimed conic:", sp.simplify(Q - target) == 0)
print("total degree in (e1,e2):", sp.Poly(Q, e1, e2).total_degree())
print("monomials:", sp.Poly(Q, e1, e2).monoms())

# 2) discriminant in T
g = sp.expand(S3.subs(x3, T))
disc = sp.discriminant(sp.Poly(g, T))
claim = 16*(x1**3 + a*x1 + b)*(x2**3 + a*x2 + b)
print("disc - 16 f(x1) f(x2) =", sp.simplify(sp.expand(disc - claim)))
print("leading coeff:", sp.Poly(g, T).LC())

# 3) numeric: roots of S_3(x1,x2,T) are x(P1+P2), x(P1-P2)
p = 1000003
A, B = -1, 1
def add(P, Q):
    if P is None: return Q
    if Q is None: return P
    (X1, Y1), (X2, Y2) = P, Q
    if X1 == X2 and (Y1 + Y2) % p == 0: return None
    if P == Q:
        lam = (3*X1*X1 + A) * pow(2*Y1, -1, p) % p
    else:
        lam = (Y2 - Y1) * pow(X2 - X1, -1, p) % p
    X3 = (lam*lam - X1 - X2) % p
    return (X3, (lam*(X1 - X3) - Y1) % p)

def neg(P): return None if P is None else (P[0], (-P[1]) % p)

pts = [P for P in ((X, Y) for X in range(p) for Y in [0]) ][:0]
# build some points
def randpts(n, seed=1):
    import random
    random.seed(seed)
    out = []
    while len(out) < n:
        X = random.randrange(p)
        rhs = (X**3 + A*X + B) % p
        Y = pow(rhs, (p+1)//4, p) if p % 4 == 3 else None
        if Y is not None and Y*Y % p == rhs:
            out.append((X, Y))
    return out

P1, P2 = randpts(2)
P3 = add(P1, P2); Pm = add(P1, neg(P2))
x1v, x2v = P1[0], P2[0]
s3 = lambda u, v, w: ((u-v)**2*w**2 - 2*((u+v)*(u*v+A)+2*B)*w + (u*v-A)**2 - 4*B*(u+v)) % p
print("S3(x1,x2,x(P1+P2)) =", s3(x1v, x2v, P3[0]))
print("S3(x1,x2,x(P1-P2)) =", s3(x1v, x2v, Pm[0]))
print("x(P1+P2) root:", sp.Poly(g.subs({a: A, b: B, x1: x1v, x2: x2v}), T).eval(p) % p == 0)
print("x(P1-P2) root:", sp.Poly(g.subs({a: A, b: B, x1: x1v, x2: x2v}), T).eval(p) % p == 0)
