from sage.all import *
# Semaev-style index calculus on a small Curve25519-like curve and its isogenous curves.
# Fixed generator G on E0; on E' = phi(E0) use G' = phi(G), Q' = phi(kG) (same scalars).
# Factor base FB = {P : x(P) < B}, B = floor(p^(1/m)), short-Weierstrass x unless noted.
# Decomposition R = sum of m signed FB points == root of S_{m+1}(x1..xm, x(R)) with x_i < B;
# over a prime field the only way to find these is enumeration, so we precompute all
# m-sums (table T_m) and do O(1) lookups. Cross-checked against explicit S3/S4 below.
import time, csv, json, random, itertools, math, sys
from collections import defaultdict

p = 1048589; A_M = 487554
F = GF(p)
EM = EllipticCurve(F, [0, A_M, 0, 1, 0])
N = EM.order(); h = 8; q = N // h
E0 = EM.short_weierstrass_model()
iso0 = EM.isomorphism_to(E0)
# usage: sage -python ic_isogeny.py NSCAL M "curve:mode,curve:mode,..." OUTDIR
NSCAL = int(sys.argv[1]); M_ARG = int(sys.argv[2])
JOBS = [tuple(j.split(":")) for j in sys.argv[3].split(",")]
OUT = sys.argv[4]
import os; os.makedirs(OUT, exist_ok=True)

# ---------- pure-python affine arithmetic on y^2 = x^3 + a x + b ----------
def mk(a):
    def add(P, Q):
        if P is None: return Q
        if Q is None: return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if (y1 + y2) % p == 0: return None
            l = (3*x1*x1 + a) * pow(2*y1, -1, p) % p
        else:
            l = (y2 - y1) * pow(x2 - x1, -1, p) % p
        x3 = (l*l - x1 - x2) % p
        return (x3, (l*(x1 - x3) - y1) % p)
    def neg(P): return None if P is None else (P[0], (-P[1]) % p)
    def mul(k, P):
        R = None
        while k:
            if k & 1: R = add(R, P)
            P = add(P, P); k >>= 1
        return R
    return add, neg, mul

def tup(P): return None if P.is_zero() else (int(P[0]), int(P[1]))

# ---------- curves ----------
rng = random.Random(1105); set_random_seed(1105)
GM = EM.random_point() * h
while GM.is_zero(): GM = EM.random_point() * h
G0 = iso0(GM)
assert q * G0 == 0 and not G0.is_zero()
scalars = [rng.randrange(1, q) for _ in range(NSCAL)]

curves = []  # (name, degree, kind, E, phi_total(callable on E0 points), xmap for FB)
curves.append(dict(name="E0", deg=1, kind="base", E=E0, phi=lambda P: P))
def add_iso(name, kind, phis):
    def phi(P):
        for f in phis: P = f(P)
        return P
    curves.append(dict(name=name, deg=prod(f.degree() for f in phis), kind=kind,
                       E=phis[-1].codomain(), phi=phi))
def walk(l, steps):
    chain = []; E = E0; prevj = None
    for _ in range(steps):
        opts = [f for f in E.isogenies_prime_degree(l) if f.codomain().j_invariant() != prevj] \
               or E.isogenies_prime_degree(l)
        f = opts[0]; chain.append(f); prevj = E.j_invariant(); E = f.codomain()
    return chain
CURVE_SPECS = {f"l={l}": ("low prime", lambda l=l: [E0.isogenies_prime_degree(l)[0]]) for l in [2, 5, 13, 17]}
CURVE_SPECS.update({f"l={l}": ("high prime", lambda l=l: [E0.isogenies_prime_degree(l)[0]]) for l in [101, 197]})
CURVE_SPECS["2^12 chain"] = ("high smooth", lambda: walk(2, 12))
CURVE_SPECS["5^6 chain"] = ("high smooth", lambda: walk(5, 6))
for name in sorted({c for c, _ in JOBS} - {"E0"}):
    kind, mkphis = CURVE_SPECS[name]; add_iso(name, kind, mkphis())
byname = {c["name"]: c for c in curves}
# extra rows: model sensitivity (Montgomery x on E0) and pushed-forward FB (phi(FB_E0)) on l=101
for c in curves:
    c["fbmode"] = "smallx"

def semaev_S3(a, b, x1, x2, x3):
    return ((x1-x2)**2*x3**2 - 2*((x1+x2)*(x1*x2+a)+2*b)*x3 + (x1*x2-a)**2 - 4*b*(x1+x2))

# ---------- engine ----------
def build_fb(E, m, mode, c):
    a, b = int(E.a4()), int(E.a6())
    B = int(floor(RR(p)**(1.0/m)))
    if mode == "smallx":
        xs = [x for x in range(B) if E.is_x_coord(F(x))]
        fb = [tup(E.lift_x(F(x))) for x in xs]
    elif mode in ("fixedK", "montfixedK"):
        # same |FB| on every curve: K = |FB_E0| under the standard bound, then take the K smallest x on this curve
        K = sum(1 for x in range(B) if E0.is_x_coord(F(x)))
        C = EM if mode == "montfixedK" else E
        xs = []; x = 0
        while len(xs) < K:
            if C.is_x_coord(F(x)): xs.append(x)
            x += 1
        B = xs[-1] + 1
        fb = [tup(iso0(EM.lift_x(F(x)))) if mode == "montfixedK" else tup(E.lift_x(F(x))) for x in xs]
    elif mode == "montgomery":   # x_M < B on the Montgomery model, mapped into E0
        fb = [tup(iso0(EM.lift_x(F(x)))) for x in range(B) if EM.is_x_coord(F(x))]
    elif mode == "pushforward":  # phi(FB_E0) for E0's small-x factor base
        fb0 = [E0.lift_x(F(x)) for x in range(B) if E0.is_x_coord(F(x))]
        fb = list({tup(c["phi"](P)) for P in fb0} - {None})
    return a, b, B, fb

def build_table(fb, m, add, neg):
    signed = []
    for j, P in enumerate(fb):
        signed.append((P, (j, 1)))
        if P[1] != 0: signed.append((neg(P), (j, -1)))
    T = {}; cnt = defaultdict(int)
    # multisets of m signed FB points (repetition allowed)
    for combo in itertools.combinations_with_replacement(range(len(signed)), m):
        S = None
        for i in combo: S = add(S, signed[i][0])
        if S is None: continue
        cnt[S] += 1
        if S not in T: T[S] = [signed[i][1] for i in combo]
    return T, cnt

def solve_dlp(rels, nfb):
    # rows: a + b k = sum e_j L_j  ->  sum e_j L_j - b k = a   (mod q), unknowns (L_used, k)
    used = sorted({j for (_, _, d) in rels for (j, _) in d})
    col = {j: i for i, j in enumerate(used)}
    n = len(used) + 1
    M = matrix(GF(q), len(rels), n); v = vector(GF(q), len(rels))
    for r, (a, b, d) in enumerate(rels):
        for (j, s) in d: M[r, col[j]] += s
        M[r, n-1] = -b; v[r] = a
    try:
        x = M.solve_right(v)
    except ValueError:
        return None
    K = M.right_kernel()
    if any(w[n-1] != 0 for w in K.basis()): return None
    return int(x[n-1])

def run_curve(c, m, mode, scal_list, nbits=300):
    E = c["E"]; coset = mode == "coset"
    a, b, B, fb = build_fb(E, m, "smallx" if coset else mode, c)
    add, neg, mul = mk(a)
    t0 = time.time(); T, cnt = build_table(fb, m, add, neg); tbuild = time.time() - t0
    G = tup(c["phi"](G0)); assert mul(q, G) is None and G is not None
    tors = [None]
    if coset:  # accept R + T for every rational 2-power torsion T (killed by the cofactor h)
        tors = sorted({tup(q * E.random_point()) for _ in range(200)}, key=str)
        assert len(tors) == h, len(tors)
    def look(R):
        for Tt in tors:
            S = add(R, Tt)
            if S in T: return S
        return None
    rows = []; bits = []
    for si, k in enumerate(scal_list):
        Q = tup(c["phi"](k * G0))
        r = random.Random(10_000*m + si)   # same (a,b) stream on every curve -> paired R' = phi(R)
        rels = []; trials = 0; ndec = 0; t1 = time.time(); kk = None
        def used(): return len({j for (_, _, d) in rels for (j, _) in d})
        bl = []
        while trials < 400000:
            while len(rels) < used() + 5:
                aa, bb = r.randrange(q), r.randrange(1, q)
                R = look(add(mul(aa, G), mul(bb, Q))); trials += 1
                if len(bl) < nbits and trials == len(bl) + 1: bl.append(int(R is not None))
                if R is not None:
                    rels.append((aa, bb, T[R])); ndec += cnt[R]
            kk = solve_dlp(rels, len(fb))
            if kk is not None: break
            for _ in range(5):
                aa, bb = r.randrange(q), r.randrange(1, q)
                R = look(add(mul(aa, G), mul(bb, Q))); trials += 1
                if len(bl) < nbits and trials == len(bl) + 1: bl.append(int(R is not None))
                if R is not None: rels.append((aa, bb, T[R])); ndec += cnt[R]
        rows.append(dict(curve=c["name"], kind=c["kind"], deg=c["deg"], fbmode=mode, m=m, B=B,
                         fb=len(fb), table=len(T), scalar_idx=si, k=k, trials=trials,
                         relations=len(rels), yield_=len(rels)/trials,
                         dec_per_trial=ndec/trials, secs=time.time()-t1,
                         recovered=int(kk == k)))
        bits.append(bl)
    return rows, dict(curve=c["name"], m=m, fbmode=mode, B=B, fb=len(fb), table=len(T),
                      table_build_s=tbuild, bits=bits, T=T, cnt=cnt, add=add, mul=mul, G=G, a=a, b=b, fbpts=fb)

# ---------- S3 / S4 cross-check on E0 ----------
def semaev_check(info, m, ntest=20):
    a, b, fb = info["a"], info["b"], info["fbpts"]
    R_ = PolynomialRing(GF(p), "x1,x2,x3,x4,X")
    x1, x2, x3, x4, X = R_.gens()
    S3 = lambda u, v, w: semaev_S3(a, b, u, v, w)
    xs = sorted({P[0] for P in fb})
    r = random.Random(7); ok = 0; mism = 0
    if m == 2:
        poly = None
    else:
        S4 = S3(x1, x2, X).resultant(S3(x3, X, x4), X)  # S4(x1,x2,x3,x4)
    hits = 0
    for _ in range(ntest * 50):
        R = info["mul"](r.randrange(1, q), info["G"])
        xr = R[0]
        # count x-multisets with S_{m+1}=0 directly from the polynomial
        if m == 2:
            sx = {tuple(sorted((u, v))) for u in xs for v in xs if u <= v and S3(u, v, xr) % p == 0}
        else:
            P1 = S4.subs(x4=xr)
            sx = set()
            for u, v in itertools.combinations_with_replacement(xs, 2):
                g = P1.subs(x1=u, x2=v).univariate_polynomial()
                if g == 0: continue
                for w, _ in g.roots():
                    if int(w) in xs: sx.add(tuple(sorted((u, v, int(w)))))
        xset = set(xs); signed = []
        for P in fb:
            signed.append(P)
            if P[1] != 0: signed.append((P[0], (-P[1]) % p))
        negR = (R[0], (-R[1]) % p); dx = set()
        if m == 2:
            for P in signed:
                D = info["add"](R, (P[0], (-P[1]) % p))
                if D is not None and D[0] in xset: dx.add(tuple(sorted((P[0], D[0]))))
        else:
            for i, P in enumerate(signed):
                for P2 in signed[i:]:
                    S = info["add"](P, P2)
                    D = info["add"](R, None if S is None else (S[0], (-S[1]) % p))
                    if D is not None and D[0] in xset: dx.add(tuple(sorted((P[0], P2[0], D[0]))))
        if sx == dx: ok += 1
        else: mism += 1
        hits += bool(dx)
        if ok + mism >= ntest and hits >= ntest // 2: break
    return dict(m=m, tested=ok + mism, agree=ok, mismatch=mism, with_decomp=hits)

# ---------- main ----------
allrows = []; meta = []; bitsets = {}
m = M_ARG
for name, mode in JOBS:
    c = byname[name]
    rows, info = run_curve(c, m, mode, scalars)
    allrows += rows; bitsets[f"{m}|{name}|{mode}"] = info["bits"]
    meta.append({k: v for k, v in info.items() if k in ("curve","m","fbmode","B","fb","table","table_build_s")})
    print(f"m={m} {name:11s} [{mode}] B={info['B']} |FB|={info['fb']} |T|={info['table']} "
          f"yield={sum(r['yield_'] for r in rows)/len(rows):.4f} rec={sum(r['recovered'] for r in rows)}/{len(rows)}",
          flush=True)
    if name == "E0" and mode == "smallx" and m in (2, 3):
        sc = semaev_check(info, m, ntest=12 if m == 3 else 20)
        print("  Semaev cross-check", sc, flush=True); meta.append(dict(semaev=sc))
json.dump(bitsets, open(f"{OUT}/bits.json", "w"))

with open(f"{OUT}/per_scalar.csv", "w", newline="") as f:
    w = csv.DictWriter(f, fieldnames=list(allrows[0].keys())); w.writeheader(); w.writerows(allrows)
json.dump(dict(p=p, A=A_M, N=int(N), q=int(q), h=h, G_mont=[int(GM[0]), int(GM[1])],
               curves=[dict(name=c["name"], deg=int(c["deg"]), kind=c["kind"],
                            j=int(c["E"].j_invariant()), a4=int(c["E"].a4()), a6=int(c["E"].a6()))
                       for c in curves], meta=meta),
          open(f"{OUT}/meta.json", "w"), indent=1, default=str)
print("done")
