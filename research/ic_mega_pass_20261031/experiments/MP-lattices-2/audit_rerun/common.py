import sys, math, itertools, random
sys.path.insert(0, "/Volumes/SSD990-2/ml-cryptanalysis")
from csd import semaev as sm
import cypari2
pari = cypari2.Pari(); pari.allocatemem(2*10**9)

def S3_specialised(a, b, xR, p):
    """f(x1,x2)=S3(x1,x2,xR) mod p as dict {(i,j):c}; monic in x1^2x2^2."""
    f = {}
    def add(e, c): f[e] = (f.get(e, 0) + c) % p
    X = xR
    add((2,0), X*X); add((1,1), -2*X*X); add((0,2), X*X)
    add((2,1), -2*X); add((1,2), -2*X); add((1,0), -2*X*a); add((0,1), -2*X*a); add((0,0), -4*X*b)
    add((2,2), 1); add((1,1), -2*a); add((0,0), a*a); add((1,0), -4*b); add((0,1), -4*b)
    return {e: c for e, c in f.items() if c}

def S3_int(a, b, x1, x2, X):
    return (x1-x2)**2*X*X - 2*((x1+x2)*(x1*x2+a)+2*b)*X + (x1*x2-a)**2 - 4*b*(x1+x2)

_S4 = None
def S4_specialised(a, b, xR, p):
    global _S4
    if _S4 is None: _S4 = sm.S4()
    f = {}
    for (e1,e2,e3,e4,ea,eb), c in _S4.items():
        k = (e1,e2,e3); f[k] = (f.get(k,0) + c*pow(xR,e4,p)*pow(a,ea,p)*pow(b,eb,p)) % p
    return {e: c for e, c in f.items() if c}

def pmul(f, g, p=None):
    h = {}
    for e1, c1 in f.items():
        for e2, c2 in g.items():
            e = tuple(x+y for x, y in zip(e1, e2)); h[e] = h.get(e, 0) + c1*c2
    if p: h = {e: c % p for e, c in h.items()}
    return {e: c for e, c in h.items() if c}

def build_rows(f, p, l, d):
    """JM box lattice: monomials alpha in [0..d*l]^n; for each, k=min(floor(alpha_i/d), l);
    row = x^(alpha-d*k*1) f^k p^(l-k). Returns list of (alpha,k,polydict)."""
    n = len(next(iter(f)))
    fpows = [{(0,)*n: 1}]
    for k in range(1, l+1): fpows.append(pmul(fpows[-1], f, p**l))
    rows = []
    for alpha in itertools.product(range(d*l+1), repeat=n):
        k = min(min(a//d for a in alpha), l)
        sh = tuple(a - d*k for a in alpha)
        g = {tuple(s+e for s, e in zip(sh, ee)): c*p**(l-k) for ee, c in fpows[k].items()}
        rows.append((alpha, k, g))
    return rows

def deg_order(n, D):
    mons = sorted(itertools.product(range(D+1), repeat=n), key=lambda a: (sum(a), a))
    return mons, {m: i for i, m in enumerate(mons)}
