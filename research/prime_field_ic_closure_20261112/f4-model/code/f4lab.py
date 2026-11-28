"""F4 (msolve) versus XL degree and cost on the closure's index-calculus routes.  [pcc:unverified]

One cell = (family, m, p-bits, size parameter, seed).  For each cell:
  * a prime-order curve E/F_p (cached), a uniform target R in E(F_p) (seeded),
  * the unsplit Semaev system S_{m+1}(x_1..x_m, x_R) plus the base generators in each block,
  * msolve 0.6.5 F4 over F_p, DRL, -g 2 (reduced GB only), one thread,
  * parsed: F4 max pair degree D_F4, per-round matrices, sum of rows, sum rows*cols, nnz, time,
  * XL side (no-aux families): Delta = deg S_R, exact XL refutation degree D_XL = Delta + delta(1/S_R)
    (Lemma 4, valid when V_R is empty), XL column count C(m+D_XL, m), Theorem B rank bound
    C(m+d-Delta, m), and the first fall degree D_ff of {f(x_i), S_R} (top forms, exact).

families
  uni      f = prod (x - r), r random distinct in F_p          (A5, Theorem B (U))
  interval f = prod_{v=1..d} (x - v)                            (A6-APS, Theorem B (U))
  graph2   x = t^2 + t,         f_T(t) = prod (t - r)           (A4 parametrised, Theorem B (Gamma), e=2)
  graphr   x = (t^2+1)/(t+3),   f_T(t) = prod (t - r)           (A4 Moebius-type rational, e=2)
  bits     x = sum 2^j b_j, b_j^2 = b_j                         (A6 Boolean bits, Theorem C, aux vars)
"""
import sys, os, json, random, subprocess, re, time, math, pickle, resource, itertools
from math import comb
import numpy as np
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import icmodel as M, fastdelta as F
import flint

PRIMES = {10: 1021, 12: 4093, 16: 65521, 20: 1048573}
OUT = os.path.join(HERE, "..", "out")
RUNS = os.path.join(HERE, "..", "runs")

# ------------------------------------------------------------------ curves
def get_curve(bits):
    fn = os.path.join(OUT, "curves.json")
    cache = json.load(open(fn)) if os.path.exists(fn) else {}
    if str(bits) in cache:
        c = cache[str(bits)]
        return c["p"], c["a"], c["b"], c["n"]
    p = PRIMES[bits]
    rng = random.Random(f"f4-model-curve-{bits}")
    a, b, pts, n = M.rand_curve(p, rng, prime_order=True, tries=5000)
    cache[str(bits)] = {"p": p, "a": a, "b": b, "n": n}
    json.dump(cache, open(fn, "w"), indent=1)
    return p, a, b, n

def lift_x(p, a, b, x):
    r = (x ** 3 + a * x + b) % p
    if r == 0: return (x, 0)
    if pow(r, (p - 1) // 2, p) != 1: return None
    y = int(flint.nmod(r, p).sqrt())
    return (x, y)

def rand_point(p, a, b, rng):
    while True:
        P = lift_x(p, a, b, rng.randrange(p))
        if P: return P if rng.random() < 0.5 else M.ec_neg(P, p)

# ------------------------------------------------------------------ Semaev
_S5 = None
def semaev(m, p, a, b, xR):
    global _S5
    if m in (2, 3):
        return M.semaev_special(m, p, a, b, xR)
    if m == 4:
        if _S5 is None:
            _S5 = pickle.load(open(os.path.join(HERE, "semaev_polys.pkl"), "rb"))["S5"]
        out = {}
        for e, c in _S5.items():
            assert e[7] == 0
            v = int(c) * pow(a, e[0], p) * pow(b, e[1], p) * pow(xR, e[6], p) % p
            if v:
                k = tuple(int(t) for t in e[2:6])
                out[k] = (out.get(k, 0) + v) % p
        return {k: v for k, v in out.items() if v}
    raise ValueError(m)

def semaev_selfcheck(m, p, a, b, trials=20, seed=0):
    """S_R(x(P_1..P_m)) == 0 whenever R = P_1+...+P_m, and != 0 for a random unrelated x_R (mostly)."""
    rng = random.Random(f"selfcheck-{m}-{p}-{seed}")
    ok = 0; neg = 0
    for _ in range(trials):
        Ps = [rand_point(p, a, b, rng) for _ in range(m)]
        R = None
        for P in Ps: R = M.ec_add(R, P, p, a)
        if R is None: continue
        S = semaev(m, p, a, b, R[0])
        ok += M.eval_poly(S, [P[0] for P in Ps], p) == 0
        S2 = semaev(m, p, a, b, rand_point(p, a, b, rng)[0])
        neg += M.eval_poly(S2, [P[0] for P in Ps], p) != 0
    return ok, neg, trials

# ------------------------------------------------------------------ bases (local generators as dicts)
def polfromroots(roots, p):
    f = [1]
    for r in roots: f = M.polmul1(f, [(-r) % p, 1], p)
    return f

def peval(c, x, p):
    s = 0
    for k in range(len(c) - 1, -1, -1): s = (s * x + c[k]) % p
    return s

def make_base(fam, size, p, rng):
    """returns (local var suffixes, generator list of dicts over local exps, x-abscissa list of W, info)"""
    if fam in ("uni", "interval"):
        roots = rng.sample(range(1, p), size) if fam == "uni" else list(range(1, size + 1))
        f = polfromroots(roots, p)
        return ["x"], [{(k,): c for k, c in enumerate(f) if c}], roots, {"d": size}
    if fam in ("graph2", "graphr"):
        if fam == "graph2": num, den = [0, 1, 1], [1]
        else: num, den = [1, 0, 1], [3, 1]
        T = []
        while len(T) < size:
            r = rng.randrange(1, p)
            if r in T or peval(den, r, p) == 0: continue
            T.append(r)
        fT = polfromroots(T, p)
        g1 = {}
        for k, c in enumerate(den):
            if c: g1[(1, k)] = (g1.get((1, k), 0) + c) % p
        for k, c in enumerate(num):
            if c: g1[(0, k)] = (g1.get((0, k), 0) - c) % p
        g1 = {e: c for e, c in g1.items() if c}
        g2 = {(0, k): c for k, c in enumerate(fT) if c}
        xs = [peval(num, r, p) * pow(peval(den, r, p), -1, p) % p for r in T]
        return ["x", "t"], [g1, g2], xs, {"dT": size, "e": 2, "distinct_x": len(set(xs))}
    if fam == "bits":
        s = size
        nloc = 1 + s
        def mono(i, e):
            v = [0] * nloc; v[i] = e; return tuple(v)
        g0 = {mono(0, 1): 1}
        for j in range(s): g0[mono(1 + j, 1)] = (-(2 ** j)) % p
        gens = [g0] + [{mono(1 + j, 2): 1, mono(1 + j, 1): p - 1} for j in range(s)]
        return ["x"] + [f"b{j}_" for j in range(s)], gens, list(range(2 ** s)), {"s": s}
    raise ValueError(fam)

# ------------------------------------------------------------------ msolve
STEP = re.compile(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s*x\s*(\d+)\s+([\d.]+)%", re.M)

def to_ms(polys, names, p):
    def ps(P):
        t = []
        for e, c in P.items():
            mon = "*".join(f"{names[v]}^{q}" if q > 1 else names[v] for v, q in enumerate(e) if q)
            t.append(f"{c}" + (f"*{mon}" if mon else ""))
        return "+".join(t) if t else "0"
    return ",".join(names) + "\n" + str(p) + "\n" + ",\n".join(ps(P) for P in polys) + "\n"

def limit_mem():
    lim = int(os.environ.get("F4_MEM_GB", "5")) * 2 ** 30
    resource.setrlimit(resource.RLIMIT_AS, (lim, lim))

def run_msolve(fn, timeout):
    t0 = time.time()
    try:
        out = subprocess.run(["msolve", "-v", "2", "-g", "2", "-t", "1", "-f", fn, "-o", fn + ".out"],
                             capture_output=True, text=True, timeout=timeout, preexec_fn=limit_mem)
        txt = out.stdout + out.stderr; rc = out.returncode; status = "ok" if rc == 0 else f"rc{rc}"
    except subprocess.TimeoutExpired as ex:
        txt = (ex.stdout or b"").decode() if isinstance(ex.stdout, bytes) else (ex.stdout or "")
        rc = None; status = "timeout"
    sec = time.time() - t0
    open(fn + ".log", "w").write(txt)
    steps = [(int(a), int(b_), int(c), int(r), int(cl), float(dn)) for a, b_, c, r, cl, dn in STEP.findall(txt)]
    res = {"status": status, "sec": round(sec, 3), "rounds": len(steps)}
    if steps:
        res.update({
            "D_F4": max(s[0] for s in steps),
            "max_rows": max(s[3] for s in steps), "max_cols": max(s[4] for s in steps),
            "max_rc": max(s[3] * s[4] for s in steps),
            "sum_rows": sum(s[3] for s in steps), "sum_rc": sum(s[3] * s[4] for s in steps),
            "sum_nnz": int(sum(s[3] * s[4] * s[5] / 100 for s in steps)),
            "deg_hist": {str(k): sum(1 for s in steps if s[0] == k) for k in sorted({s[0] for s in steps})},
        })
    m_ = re.search(r"#rows reduced\s+(\d+)", txt); res["rows_reduced"] = int(m_.group(1)) if m_ else None
    m_ = re.search(r"msolve overall time\s+([\d.]+) sec \(elapsed\)", txt); res["ms_time"] = float(m_.group(1)) if m_ else None
    m_ = re.search(r"size of basis\s+(\d+)", txt); res["gb_size"] = int(m_.group(1)) if m_ else None
    res["unit_ideal"] = "No solution" in txt or "single element" in txt
    return res

# ------------------------------------------------------------------ XL side and first fall degree (no-aux families)
def topform(S):
    D = M.tdeg(S)
    return {e: c for e, c in S.items() if sum(e) == D}, D

def first_fall_degree(S, m, d, p, tmax_extra=64):
    """least D such that  S^h * A_{D-Delta} -> A_D  has a nonzero kernel, A = k[x]/(x_1^d..x_m^d)
    (top-form syzygy that is not Koszul in the f's).  Exact rank over F_p."""
    Sh, Dl = topform(S)
    def mons(t):
        return [e for e in itertools.product(range(d), repeat=m) if sum(e) == t] if t >= 0 else []
    # mons() by product is fine for the toy d^m sizes used here
    allm = list(itertools.product(range(d), repeat=m))
    bydeg = {}
    for e in allm: bydeg.setdefault(sum(e), []).append(e)
    # for t + Delta < d, A_{t+Delta} is all of k[x]_{t+Delta} and S^h* is injective (domain): start there
    for t in range(max(0, d - Dl), m * (d - 1) + 1):
        src = bydeg.get(t, [])
        if not src: continue
        tgt = bydeg.get(t + Dl, [])
        if not tgt:
            return t + Dl, t
        idx = {e: i for i, e in enumerate(tgt)}
        A = flint.nmod_mat(len(src), len(tgt), p)
        for i, u in enumerate(src):
            for e, c in Sh.items():
                v = tuple(a + b for a, b in zip(u, e))
                if max(v) < d:
                    A[i, idx[v]] = (int(A[i, idx[v]]) + c) % p
        if A.rank() < len(src):
            return t + Dl, t
    return None, None

def xl_side(S, roots, m, p, want_ff=True):
    d = len(roots)
    Dl = M.tdeg(S)
    out = {"Delta": Dl, "grid": d ** m, "thmB_rank_bound": comb(m + d - Dl, m) if d >= Dl else 0}
    if d ** m <= 3_000_000:
        G = F.semaev_grid(S, m, roots, p)
        nz = int((G % p == 0).sum())
        out["V_R"] = nz
        if nz == 0:
            Vinv = F.mat_inv_mod(F.vandermonde(roots, p), p)
            C = F.coeffs(F.inv_mod_array(G, p), Vinv, p)
            delta = F.delta_of(C, F.weight_grid(d, m, "uni"))
            out["delta"] = delta
            out["D_XL"] = Dl + delta
            out["XL_cols"] = comb(m + Dl + delta, m)
    if want_ff and d ** m <= 400_000:
        Dff, t = first_fall_degree(S, m, d, p)
        out["D_ff"] = Dff
        out["ff_cols"] = comb(m + Dff, m) if Dff else None
    return out

# ------------------------------------------------------------------ one cell
def cell(fam, m, bits, size, seed, timeout=900):
    p, a, b, n = get_curve(bits)
    rng = random.Random(f"f4-model|{fam}|{m}|{bits}|{size}|{seed}")
    R = rand_point(p, a, b, rng)
    S = semaev(m, p, a, b, R[0])
    suff, gens, xs, info = make_base(fam, size, p, rng)
    nloc = len(suff)
    names = []
    for i in range(1, m + 1):
        names += [f"{s}{i}" for s in suff]
    N = m * nloc
    polys = []
    Sg = {}
    for e, c in S.items():
        v = [0] * N
        for i in range(m): v[i * nloc] = e[i]
        Sg[tuple(v)] = c
    polys.append(Sg)
    for i in range(m):
        for g in gens:
            P = {}
            for e, c in g.items():
                v = [0] * N
                for j, k in enumerate(e): v[i * nloc + j] = k
                P[tuple(v)] = c
            polys.append(P)
    tag = f"{fam}_m{m}_p{bits}_s{size}_r{seed}"
    fn = os.path.join(RUNS, tag + ".ms")
    open(fn, "w").write(to_ms(polys, names, p))
    rec = {"tag": tag, "family": fam, "m": m, "bits": bits, "p": p, "a": a, "b": b, "n": n,
           "size": size, "seed": seed, "xR": R[0], "N": N, "info": info,
           "Bx": sum(1 for x in set(xs) if lift_x(p, a, b, x % p))}
    rec["ms"] = run_msolve(fn, timeout)
    # grid zeros for every family (abscissa grid, with multiplicity of the presentation)
    if len(xs) ** m <= 3_000_000:
        G = F.semaev_grid(S, m, [x % p for x in xs], p)
        rec["V_R_grid"] = int((G % p == 0).sum())
    if fam in ("uni", "interval"):
        rec["xl"] = xl_side(S, xs, m, p)
    return rec

if __name__ == "__main__":
    if sys.argv[1] == "selfcheck":
        res = {}
        for bits in (10, 12, 16, 20):
            p, a, b, n = get_curve(bits)
            for m in (2, 3, 4):
                res[f"p{bits}_m{m}"] = semaev_selfcheck(m, p, a, b, trials=10 if m == 4 else 30)
        print(json.dumps(res))
        sys.exit()
    fam, m, bits, size, seed = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5])
    timeout = int(sys.argv[6]) if len(sys.argv) > 6 else 900
    rec = cell(fam, m, bits, size, seed, timeout)
    line = json.dumps(rec)
    with open(os.path.join(OUT, "cells.jsonl"), "a") as fh: fh.write(line + "\n")
    print(line)
