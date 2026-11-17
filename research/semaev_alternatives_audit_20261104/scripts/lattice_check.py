import random, math, time
import sympy as sp
from sympy import Matrix

random.seed(7)

def curve_points(p, a, b, n):
    out = []
    while len(out) < n:
        x = random.randrange(p)
        rhs = (x*x*x + a*x + b) % p
        if pow(rhs, (p-1)//2, p) != 1 and rhs != 0:
            continue
        # Tonelli-Shanks via sympy
        y = sp.sqrt_mod(rhs, p, all_roots=False)
        if y is None:
            continue
        out.append((x, int(y) % p))
    return out

def add(P, Q, p, a):
    if P is None: return Q
    if Q is None: return P
    X1, Y1 = P; X2, Y2 = Q
    if X1 == X2 and (Y1 + Y2) % p == 0: return None
    if P == Q:
        lam = (3*X1*X1 + a) * pow(2*Y1, -1, p) % p
    else:
        lam = (Y2 - Y1) * pow(X2 - X1, -1, p) % p
    X3 = (lam*lam - X1 - X2) % p
    return (X3, (lam*(X1 - X3) - Y1) % p)

def s3coeffs(x1, x2, a, b):
    # returns (C, B, A) coefficients of T^0, T^1, T^2
    A = (x1 - x2)**2
    B = -2*((x1 + x2)*(x1*x2 + a) + 2*b)
    C = (x1*x2 - a)**2 - 4*b*(x1 + x2)
    return C, B, A

def lattice_basis(xR, p):
    return Matrix([[p, 0, 0], [-xR % p, 1, 0], [-(xR*xR) % p, 0, 1]])

def recovered(x1, x2, a, b, xR, p, delta=sp.Rational(3,4)):
    L = lattice_basis(xR, p)
    R = L.lll(delta=delta)
    tgt = sp.Matrix([s3coeffs(x1, x2, a, b)[i] for i in range(3)]).T
    t = (s3coeffs(x1, x2, a, b)[0], s3coeffs(x1, x2, a, b)[1], s3coeffs(x1, x2, a, b)[2])
    for row in R.tolist():
        r = tuple(int(z) for z in row)
        if r[0]*t[1]-r[1]*t[0] == 0 and r[0]*t[2]-r[2]*t[0] == 0 and r[1]*t[2]-r[2]*t[1] == 0:
            return True
    return False

def run(pbits, deltas, trials=24, a=0, b=7):
    p = int(sp.nextprime(2**pbits))
    pts = curve_points(p, a, b, 4*trials)
    res = {}
    for d in deltas:
        B = int(p**d)
        if B < 32: 
            continue
        ok = 0
        for t in range(trials):
            # sample x1,x2 uniform in [1,B], keep those on curve with a partner (rare) ->
            # instead: sample base points by x<=B directly
            pass
        res[d] = ok
    return res

# Simpler: build a base of points with x in [1,B] by scanning x
def base_points(p, a, b, B):
    pts = []
    for x in range(1, B+1):
        rhs = (x*x*x + a*x + b) % p
        if pow(rhs, (p-1)//2, p) == 1:
            y = sp.sqrt_mod(rhs, p, all_roots=False)
            if y is not None:
                pts.append((x, int(y) % p))
    return pts

def trial(pbits, delta, trials=24):
    p = int(sp.nextprime(2**pbits))
    B = int(p**delta)
    pts = base_points(p, 0, 7, B)
    ok = 0; attempted = 0
    for t in range(trials):
        P, Q = random.sample(pts, 2)
        R = add(P, Q, p, 0)
        if R is None: continue
        attempted += 1
        if recovered(P[0], Q[0], 0, 7, R[0], p):
            ok += 1
    return ok, attempted, len(pts), B

if __name__ == "__main__":
    for pbits in (72, 96):
        for delta in (0.06,0.07,0.08,0.09,0.10,0.11,0.12):
            t0 = time.time()
            ok, att, nb, B = trial(pbits, sp.Rational(str(delta)))
            print(f"p={pbits}b delta={delta} B={B} base={nb} recovered={ok}/{att} ({time.time()-t0:.1f}s)")
