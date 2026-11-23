"""rootfam closure toy [pcc:unverified].

Factor base B_f = {P in E(F_p) : f(x(P)) = 0} for a fixed polynomial f of degree d.
Families: kummer  f = X^d - g^d        (roots g*mu_d, d | p-1; the "naive" PKM presentation)
          split   f = prod (X - r_i)   (d random abscissas of E(F_p))
          height  f = prod_{i=1..d}(X - i)  (Diem-style low-height / small-integer abscissas)

Instrument (ordering-free, Macaulay/XL truncated ideal):
  I = <S_{m+1}(x_1..x_m, x_R), f(x_1), ..., f(x_m)>  over F_p.
  D_ref  = least D with 1 in span(Macaulay_D)              (refuting a non-decomposable target)
  D_elim = least D with g(x_1) in span(Macaulay_D), g = prod over solution x_1-values (x_1 - r)
           (decomposable/planted targets).  Membership is monotone in D -> binary search.
  width  = number of monomials of degree <= D in m variables at D_ref / D_elim.

KILL RULE (fixed before running; same accounting as ratmap / arity bridge
 crypto:research/prime_ic_lead_review_20261031/PDP_ARITY_CORRECTION.md, need e < m/2 - 1):
  K1 size: one f of degree d gives |B_x| <= d; a useful base needs |B| ~ p^{1/m}.
  K2 equations: net equations minus unknowns per relation on E(F_p) must exceed 1.
  K3 query: if D_ref grows at least linearly in d (= |B_x|) for every family, then
     width ~ C(m+D,m) >= |B|^m/m!, i.e. query exponent e >= m > m/2 - 1 for all m -> CLOSED.
     If D_ref stays bounded (<= m+3) as d doubles twice, the route stays OPEN.
"""
import sys, json, random, time, itertools
from math import comb
import flint
from sympy import sqrt_mod, symbols, Poly, resultant, expand

p = int(sys.argv[1]); m = int(sys.argv[2]); fam = sys.argv[3]; d = int(sys.argv[4])
ntar = int(sys.argv[5]); seed = int(sys.argv[6])
rng = random.Random(seed * 1000003 + d * 97 + m * 13 + hash(fam) % 1000)
rng = random.Random(f"{seed}-{fam}-{d}-{m}")

# curve
while True:
    a = rng.randrange(1, p); b = rng.randrange(1, p)
    if (4*a**3 + 27*b**2) % p: break
def rhs(x): return (x*x*x + a*x + b) % p
def on_E(x):
    r = rhs(x); return r == 0 or pow(r, (p-1)//2, p) == 1
def lift(x):
    y = sqrt_mod(rhs(x), p); return (x % p, y)
def add(P, Q):
    if P is None: return Q
    if Q is None: return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3*x1*x1 + a) * pow(2*y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l*l - x1 - x2) % p
    return (x3, (l*(x1 - x3) - y1) % p)

# base polynomial (univariate coefficient list, low->high) and its root set
def polmul(A, B):
    C = [0]*(len(A)+len(B)-1)
    for i, u in enumerate(A):
        for j, v in enumerate(B): C[i+j] = (C[i+j] + u*v) % p
    return C
if fam == "kummer":
    assert (p-1) % d == 0
    while True:
        g0 = rng.randrange(2, p); c = pow(g0, d, p)
        z = pow(rng.randrange(2, p), (p-1)//d, p)
        roots = sorted({g0*pow(z, k, p) % p for k in range(d)})
        if len(roots) == d: break
    f = [(-c) % p] + [0]*(d-1) + [1]
elif fam == "split":
    roots = set()
    while len(roots) < d:
        x = rng.randrange(p)
        if on_E(x): roots.add(x)
    roots = sorted(roots)
    f = [1]
    for r in roots: f = polmul(f, [(-r) % p, 1])
elif fam == "height":
    roots = list(range(1, d+1)); f = [1]
    for r in roots: f = polmul(f, [(-r) % p, 1])
Eroots = [r for r in roots if on_E(r)]

# Semaev polynomial S_{m+1}(x1..xm, xR) as dict exponent-tuple -> coeff
X = symbols("x1:%d" % (m+1)); T = symbols("T")
def S3(u, v, w):
    return (u-v)**2*w**2 - 2*((u+v)*(u*v+a) + 2*b)*w + (u*v-a)**2 - 4*b*(u+v)
def semaev(xR):
    if m == 2: e = expand(S3(X[0], X[1], xR))
    elif m == 3: e = resultant(S3(X[0], X[1], T), S3(T, X[2], xR), T)
    else: raise SystemExit("m<=3 only")
    P = Poly(e, *X, modulus=p)
    return {mon: int(cf) % p for mon, cf in P.terms() if int(cf) % p}
def S_eval(xs, xR):
    if m == 2: return S3(xs[0], xs[1], xR) % p
    # m=3: exists T with S3(x1,x2,T)=0 and S3(T,x3,xR)=0 -> use resultant numerically
    return None

def monos(D):
    out = []
    for tot in range(D+1):
        for c in itertools.combinations_with_replacement(range(m), tot):
            e = [0]*m
            for i in c: e[i] += 1
            out.append(tuple(e))
    return out
def gens(xR):
    G = [semaev(xR)]
    for i in range(m):
        h = {}
        for k, cf in enumerate(f):
            if cf:
                e = [0]*m; e[i] = k; h[tuple(e)] = cf
        G.append(h)
    return G
def deg(h): return max(sum(e) for e in h)
def in_span(G, target, D):
    M = monos(D); idx = {e: j for j, e in enumerate(M)}; n = len(M)
    rows = []
    for h in G:
        dh = deg(h)
        if dh > D: continue
        for u in monos(D - dh):
            r = [0]*n
            for e, cf in h.items():
                r[idx[tuple(x+y for x, y in zip(e, u))]] = cf
            rows.append(r)
    if not rows: return False, n, 0
    A = flint.nmod_mat(rows, p)
    rk = A.rank()
    t = [0]*n
    for e, cf in target.items(): t[idx[e]] = cf % p
    B = flint.nmod_mat(rows + [t], p)
    return B.rank() == rk, n, len(rows)
def least_D(G, target, Dlo, Dhi, tlimit):
    t0 = time.time()
    ok, n, r = in_span(G, target, Dhi)
    while not ok:
        Dhi = Dhi + max(4, Dhi // 2)
        if time.time() - t0 > tlimit: return None, Dhi, n, r
        ok, n, r = in_span(G, target, Dhi)
    lo, hi = Dlo, Dhi   # invariant: in_span at hi
    while lo < hi:
        mid = (lo + hi) // 2
        if in_span(G, target, mid)[0]: hi = mid
        else: lo = mid + 1
    ok, n, r = in_span(G, target, hi)
    return hi, hi, n, r

def solutions(xR):
    """all x-tuples in roots^m with S_{m+1}=0 (over F_p values)."""
    sols = []
    if m == 2:
        for x1 in roots:
            for x2 in roots:
                if S3(x1, x2, xR) % p == 0: sols.append((x1, x2))
    else:
        S = semaev(xR)
        for xs in itertools.product(roots, repeat=3):
            v = 0
            for e, cf in S.items():
                v = (v + cf*pow(xs[0], e[0], p)*pow(xs[1], e[1], p)*pow(xs[2], e[2], p)) % p
            if v == 0: sols.append(xs)
    return sols

out = {"p": p, "m": m, "family": fam, "d": d, "seed": seed, "a": a, "b": b,
       "nroots": len(roots), "nroots_on_E": len(Eroots), "targets": []}
kinds = ["random"] * ntar + (["planted"] * ntar if len(Eroots) >= 1 else [])
for kind in kinds:
    if kind == "random":
        while True:
            xR = rng.randrange(p)
            if on_E(xR): break
    else:
        P = None
        for _ in range(m): P = add(P, lift(rng.choice(Eroots)) if rng.random() < .5 else
                                     (lambda q: (q[0], (-q[1]) % p))(lift(rng.choice(Eroots))))
        if P is None: continue
        xR = P[0]
    sols = solutions(xR)
    x1vals = sorted({s[0] for s in sols})
    if not sols: target = {tuple([0]*m): 1}; what = "one"
    else:
        gpoly = [1]
        for r in x1vals: gpoly = polmul(gpoly, [(-r) % p, 1])
        target = {tuple([k] + [0]*(m-1)): cf for k, cf in enumerate(gpoly) if cf}
        what = "elim_x1"
    G = gens(xR)
    t0 = time.time()
    import os
    if os.environ.get("FIXED_D"):
        for Dt in map(int, os.environ["FIXED_D"].split(",")):
            ok, ncols, nrows = in_span(G, target, Dt)
            rec = {"kind": kind, "xR": xR, "n_solutions": len(sols), "measure": what,
                   "D_tested": Dt, "in_span": ok, "macaulay_cols": ncols, "macaulay_rows": nrows,
                   "sec": round(time.time() - t0, 2)}
            out["targets"].append(rec); print(json.dumps(rec), flush=True)
        continue
    D, _, ncols, nrows = least_D(G, target, 1, max(2*d + 4, 8) if m == 2 else 2*d + 6, 240)
    out["targets"].append({"kind": kind, "xR": xR, "n_solutions": len(sols),
        "n_x1": len(x1vals), "measure": what, "D": D, "macaulay_cols": ncols,
        "macaulay_rows": nrows, "sec": round(time.time() - t0, 2)})
    print(json.dumps(out["targets"][-1]), flush=True)
print("RESULT " + json.dumps(out))
