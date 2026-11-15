import random, time, collections
import sympy as sp

# --- reproduce section 6.2 carry table on p = nextprime(2^40), y^2 = x^3 + 7
p = int(sp.nextprime(2**40))
a, b = 0, 7
random.seed(20261104)

def add(P, Q):
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

def base_points(B):
    out = []
    for x in range(1, B+1):
        rhs = (x*x*x + b) % p
        if pow(rhs, (p-1)//2, p) == 1:
            y = sp.sqrt_mod(rhs, p, all_roots=False)
            if y is not None:
                out.append((x, int(y) % p))
    return out

def S3(x1, x2, x3, a=0, b=7):
    return ((x1-x2)**2*x3**2 - 2*((x1+x2)*(x1*x2+a)+2*b)*x3 + (x1*x2-a)**2 - 4*b*(x1+x2))

print("p bits:", p.bit_length())
for B in (16, 64, 256, 1024, 4096, 16384):
    pts = base_points(B)
    if len(pts) < 2:
        continue
    P, Q = random.sample(pts, 2)
    R = add(P, Q)
    if R is None: continue
    val = S3(P[0], Q[0], R[0])
    print(f"B={B:6d} x1={P[0]:6d} x2={Q[0]:6d} carry_bits={abs(val//p).bit_length():3d} "
          f"coef_bits={max(abs(val//p), 1).bit_length()} S3/p~B^2p? "
          f"pred={(p*B*B).bit_length()}")
