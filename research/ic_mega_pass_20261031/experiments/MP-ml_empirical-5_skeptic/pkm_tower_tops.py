"""PKM Kummer-tower presentation over F_p: what is F^top, what is d_reg(F^top),
and what does the Salizzoni envelope d_reg(F^top)+1 say against the measured
F4 step degree D (msolve -v 2)?

Presentation (treatise falldegree.tex, sec. fd-pkm): for each summand i,
  y_{i,0} = x_i/g,  y_{i,j+1} = y_{i,j}^2 (j=0..t-2),  y_{i,t-1}^2 = 1,
plus S_{m+1}(g y_{1,0}, ..., g y_{m,0}, x_R) = 0 on E: y^2 = x^3 + a x + b.
Variables N = m t.  Field equations y^p - y are NOT included (degree p).

Claim tested: F^top = (y_{i,j}^2 : all i,j) exactly, because the top of the
Semaev polynomial in the m variables y_{i,0} has total degree >= m+1 and so
every one of its monomials has an exponent >= 2 (pigeonhole).  Then
HS(R/F^top) = (1+z)^N, d_reg(F^top) = N+1, and the Salizzoni bound reads
D <= N+2 = m t + 2, i.e. exactly the treatise's Theorem pkm(4) bound 2t+2
at m=2: it carries no information about the staircase / plateau.
"""
import sys, os, json, random, subprocess, re, time, itertools
sys.path.insert(0, "/Volumes/SSD990-2/ml-cryptanalysis")
from csd import semaev as sm

P = 2013265921
OUT = os.path.dirname(os.path.abspath(__file__))
TMP = os.environ.get("TMPDIR", "/tmp")
rng = random.Random(20261031)


def spec(poly, nvars, subs):
    """poly: dict{exp tuple over (x1..xm, X, a, b)} -> dict over the N tower variables.
    subs: list of polys (dict{exp tuple: coeff}) for each original variable."""
    out = {}
    for ex, c in poly.items():
        term = {tuple([0] * nvars): c % P}
        for v, e in enumerate(ex):
            for _ in range(e):
                term = pmul(term, subs[v], nvars)
        for k, v in term.items():
            out[k] = (out.get(k, 0) + v) % P
    return {k: v for k, v in out.items() if v}


def pmul(A, B, nvars):
    C = {}
    for ea, ca in A.items():
        for eb, cb in B.items():
            e = tuple(x + y for x, y in zip(ea, eb))
            C[e] = (C.get(e, 0) + ca * cb) % P
    return {k: v for k, v in C.items() if v}


def const(c, nvars):
    return {tuple([0] * nvars): c % P}


def var(i, nvars, coeff=1):
    e = [0] * nvars
    e[i] = 1
    return {tuple(e): coeff % P}


def top(poly):
    d = max(sum(e) for e in poly)
    return {e: c for e, c in poly.items() if sum(e) == d}, d


def build(m, t, a, b, g, xR):
    N = m * t
    idx = lambda i, j: i * t + j
    eqs = []
    for i in range(m):
        for j in range(t - 1):
            eqs.append({**var(idx(i, j + 1), N), **{tuple(2 if k == idx(i, j) else 0 for k in range(N)): P - 1}})
        e = tuple(2 if k == idx(i, t - 1) else 0 for k in range(N))
        eqs.append({e: 1, tuple([0] * N): P - 1})
    S = sm.semaev(m + 1)  # variables (x_1..x_m, X, a, b)
    subs = [var(idx(i, 0), N, g) for i in range(m)] + [const(xR, N), const(a, N), const(b, N)]
    eqs.append(spec(S, N, subs))
    return eqs, N


def fmt(poly, N):
    names = [f"y{k}" for k in range(N)]
    terms = []
    for e, c in poly.items():
        mon = "*".join(f"{names[k]}^{ek}" for k, ek in enumerate(e) if ek)
        terms.append(f"{c}*{mon}" if mon else f"{c}")
    return "+".join(terms)


def run_msolve(eqs, N, tag, timeout=600):
    path = os.path.join(TMP, f"pkm_{tag}.ms")
    with open(path, "w") as f:
        f.write(",".join(f"y{k}" for k in range(N)) + "\n" + str(P) + "\n")
        f.write(",\n".join(fmt(e, N) for e in eqs) + "\n")
    out = os.path.join(TMP, f"pkm_{tag}.out")
    t0 = time.time()
    try:
        r = subprocess.run(["msolve", "-v", "2", "-g", "2", "-f", path, "-o", out], capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return {"timeout": True}
    degs = [int(m.group(1)) for m in re.finditer(r"^\s*(\d+)\s+\d+\s+\d+\s+\d+ x \d+", r.stdout, re.M)]
    gb = open(out).read()
    unsat = bool(re.search(r"\[\s*1\s*\]|\n1\]|\[1\]", gb)) or ("#length of basis:      1 " in gb and re.search(r"\[\s*1\s*\]", gb) is not None)
    # robust unsat check: basis of length 1 whose single element is a constant
    mm = re.search(r"\[(.*?)\]:", gb, re.S)
    body = mm.group(1).strip() if mm else ""
    unsat = ("y" not in body) and body != ""
    return {"max_step_deg": max(degs) if degs else None, "steps": degs, "unsat": unsat, "wall_s": round(time.time() - t0, 2)}


results = []
cells = [(2, t) for t in range(1, 8)] + [(3, t) for t in range(1, 4)]
for (m, t) in cells:
    a, b, g = rng.randrange(1, P), rng.randrange(1, P), rng.randrange(1, P)
    xR = rng.randrange(1, P)
    eqs, N = build(m, t, a, b, g, xR)
    tops = [top(e) for e in eqs]
    sem_top, sem_deg = tops[-1]
    all_sq = all(max(e) >= 2 for e in sem_top)  # every top monomial has an exponent >= 2
    tower_tops_are_squares = all(len(tp) == 1 and sum(next(iter(tp))) == 2 and max(next(iter(tp))) == 2 for tp, _ in tops[:-1])
    dreg = N + 1 if (all_sq and tower_tops_are_squares) else None
    rec = {"m": m, "t": t, "N": N, "n_eqs": len(eqs), "semaev_top_degree_in_tower_vars": sem_deg,
           "semaev_top_monomials": [list(e) for e in sem_top][:6], "semaev_top_in_square_ideal": all_sq,
           "tower_tops_are_pure_squares": tower_tops_are_squares,
           "d_reg_Ftop": dreg, "salizzoni_bound": (dreg + 1) if dreg else None}
    if N <= 12:
        rec["msolve"] = run_msolve(eqs, N, f"m{m}t{t}", timeout=900 if m == 2 else 1200)
        if rec["msolve"].get("max_step_deg") is not None and dreg:
            rec["gap_bound_minus_D"] = dreg + 1 - rec["msolve"]["max_step_deg"]
    print(json.dumps(rec), flush=True)
    results.append(rec)

json.dump(results, open(os.path.join(OUT, "pkm_tower_results.json"), "w"), indent=1)
