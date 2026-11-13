"""Test of the typical-degree hypothesis (G) behind Theorem C, plus controls.  [pcc:unverified]

For a base W (uni: d random abscissas of E; tower: g*mu_{2^k}) and m in {2,3,4}:
  non-decomposable targets x_R (S_R != 0 on W^m):  delta(1/S_R)                (max = m(d-1) or m k)
  decomposable targets (zeros planted):            delta(g_R/S_R ; V_R)
  generic prediction for |V| free values:          least t with #{a : wt(a) > t} <= |V|
  controls: random functions (negative control: expect delta = max w.p. 1 - 1/p);
            planted low-degree functions (positive control: expect delta <= t0, = t0 generically).
Implied XL^J per-attempt rank lower bound (Lemma L6):  H(delta) - |V|, reported as a fraction of |W|^m.
"""
import sys, json, random, time, pickle, os
import numpy as np
import flint
import icmodel as M
import fastdelta as F

HERE = os.path.dirname(os.path.abspath(__file__))
WEAK = os.environ.get("WEAK", "1") == "1"

def semaev_dict(m, p, a, b, xR):
    if m in (2, 3):
        return M.semaev_special(m, p, a, b, xR)
    if m == 4:
        G = S5()
        out = {}
        for e, c in G.items():
            v = c * pow(a, e[0], p) * pow(b, e[1], p) * pow(xR, e[6], p) % p
            if v:
                k = tuple(e[2:6]); out[k] = (out.get(k, 0) + v) % p
        return {k: v for k, v in out.items() if v}
    raise ValueError

_S5 = None
def S5():
    global _S5
    if _S5 is None:
        _S5 = pickle.load(open(os.path.join(HERE, "..", "out", "S5_generic.pkl"), "rb"))
    return _S5

def roots_in_X(m, p, a, b, xs):
    """roots X in F_p of S_{m+1}(xs, X)."""
    # S_{m+1} is symmetric: S(xs, X) = S(X, xs...) ; build univariate in X by evaluating coefficients
    if m in (2, 3):
        gen = M.semaev_generic(m)
        coef = {}
        for e, c in gen.items():
            v = c * pow(a, e[0], p) * pow(b, e[1], p) % p
            for i in range(m):
                v = v * pow(xs[i], e[2 + i], p) % p
            coef[e[2 + m]] = (coef.get(e[2 + m], 0) + v) % p
    else:
        coef = {}
        for e, c in S5().items():
            v = c * pow(a, e[0], p) * pow(b, e[1], p) % p
            for i in range(4):
                v = v * pow(xs[i], e[2 + i], p) % p
            coef[e[6]] = (coef.get(e[6], 0) + v) % p
    deg = max(k for k, v in coef.items() if v) if any(coef.values()) else -1
    if deg < 1: return []
    P = flint.nmod_poly([coef.get(i, 0) for i in range(deg + 1)], p)
    return sorted({int(r) for r, _ in P.roots()})

def weight_cdf(Wt):
    vals, cnt = np.unique(Wt, return_counts=True)
    cdf = {}
    s = 0
    for v, c in zip(vals, cnt):
        s += int(c); cdf[int(v)] = s
    return cdf

def H_of(cdf, t):
    best = 0
    for v, s in cdf.items():
        if v <= t: best = max(best, s)
    return best

def generic_delta_free(Wt, nV):
    tmax = int(Wt.max())
    for t in range(-1, tmax + 1):
        if int((Wt > t).sum()) <= nV:
            return t
    return tmax

def S_pres_degree(SR, kind, d, p):
    if kind == "uni":
        return M.tdeg(SR)
    if kind == "bits":
        s_ = d.bit_length() - 1
        return max(sum(min(ei, s_) for ei in e) for e in SR)
    k = d.bit_length() - 1
    red = {}
    for e, c in SR.items():
        key = tuple(bin(ei % d).count("1") * 0 + (ei % d) for ei in e)
        red[key] = (red.get(key, 0) + c) % p        # x^{2^k} -> const merges exponents mod 2^k
    return max(sum(bin(ei).count("1") for ei in key) for key, c in red.items() if c)

def generic_weak(Wt, w, npiV, nV):
    tmax = int(Wt.max())
    for t in range(-1, tmax + 1):
        if int((Wt > t).sum()) < w - npiV + nV:
            return t
    return tmax

def run(kind, m, size, p, ntar, ndec, nrand, seed, curve_seed=None):
    rng = random.Random(f"G-{kind}-{m}-{size}-{p}-{seed}")
    crng = random.Random(curve_seed if curve_seed is not None else f"curve-{p}-{seed}")
    t0 = time.time()
    a, b, pts, n = M.rand_curve(p, crng) if p < 200000 else rand_curve_big(p, crng)
    if kind == "uni":
        d = size
        absc = abscissas(p, a, b, rng, d)
        roots = absc
    elif kind == "tower":
        k = size; d = 2 ** k
        base = M.tower_base(p, k, rng.randrange(2, p))
        roots = [w[0] for w in base.pts]
    else:                                   # bits: interval {0..2^s-1}, Boolean-bit presentation
        k = size; d = 2 ** k
        roots = list(range(d))
    if kind == "bits":
        Vinv, Vm = F.mobius_mats(size, p)
    else:
        Vm = F.vandermonde(roots, p)
        Vinv = F.mat_inv_mod(Vm, p)
    Wt = F.weight_grid(d, m, "uni" if kind == "uni" else "tower")
    tmaxw = int(Wt.max())
    cdf = weight_cdf(Wt)
    grid_size = d ** m
    rows = []
    # --- non-decomposable targets
    got = 0; tries = 0
    while got < ntar and tries < 50 * ntar:
        tries += 1
        xR = rng.randrange(p)
        SR = semaev_dict(m, p, a, b, xR)
        G = F.semaev_grid(SR, m, roots, p)
        if (G == 0).any(): continue
        h = F.inv_mod_array(G, p)
        C = F.coeffs(h, Vinv, p)
        dl = F.delta_of(C, Wt)
        dS = S_pres_degree(SR, kind, d, p)
        dw = F.delta_weak(h, roots, Vinv, [], Wt, p, dS, "uni" if kind == "uni" else "tower", Vand=Vm) if WEAK else None
        rows.append({"target": "nondec", "xR": xR, "delta": dl, "max": tmaxw, "nV": 0,
                     "generic_pred": tmaxw, "H_frac": (H_of(cdf, dl)) / grid_size,
                     "dS": dS, "delta_weak": dw, "generic_weak": generic_weak(Wt, d, 0, 0),
                     "H_frac_weak": (H_of(cdf, dw) / grid_size) if dw is not None else None})
        got += 1
    # --- decomposable targets (planted zeros)
    got = 0; tries = 0
    pos = {r: i for i, r in enumerate(roots)}
    while got < ndec and tries < 50 * ndec:
        tries += 1
        xs = [rng.choice(roots) for _ in range(m)]
        Xs = roots_in_X(m, p, a, b, xs)
        if not Xs: continue
        xR = rng.choice(Xs)
        SR = semaev_dict(m, p, a, b, xR)
        G = F.semaev_grid(SR, m, roots, p)
        Vidx = [tuple(int(i) for i in ix) for ix in zip(*np.nonzero(G == 0))]
        r1 = sorted({roots[v[0]] for v in Vidx})
        # g_R(x_1) values on grid
        gvals = np.ones(d, dtype=np.int64)
        for r in r1:
            gvals = gvals * ((np.array(roots, dtype=np.int64) - r) % p) % p
        sh = [1] * m; sh[0] = d
        hnum = np.broadcast_to(gvals.reshape(sh), (d,) * m).copy()
        hinv = F.inv_mod_array(G, p)
        h = (hnum * hinv) % p            # zero on V (free there anyway)
        C = F.coeffs(h, Vinv, p)
        dl = F.delta_free(C, Vinv, Vidx, Wt, p)
        gp = generic_delta_free(Wt, len(Vidx))
        dS = S_pres_degree(SR, kind, d, p)
        dw = F.delta_weak(hinv, roots, Vinv, Vidx, Wt, p, dS, "uni" if kind == "uni" else "tower", Vand=Vm) if WEAK else None
        rows.append({"target": "dec", "xR": xR, "delta": dl, "max": tmaxw, "nV": len(Vidx),
                     "deg_g": len(r1), "generic_pred": gp,
                     "H_frac": (H_of(cdf, dl) - len(Vidx)) / grid_size,
                     "dS": dS, "delta_weak": dw, "generic_weak": generic_weak(Wt, d, len(r1), len(Vidx)),
                     "H_frac_weak": ((H_of(cdf, dw) - len(Vidx)) / grid_size) if dw is not None else None})
        got += 1
    # --- negative control: random functions
    for _ in range(nrand):
        h = np.array([rng.randrange(p) for _ in range(grid_size)], dtype=np.int64).reshape((d,) * m)
        C = F.coeffs(h, Vinv, p)
        rows.append({"target": "ctrl_random", "delta": F.delta_of(C, Wt), "max": tmaxw, "nV": 0,
                     "generic_pred": tmaxw})
    # --- positive control: planted low-degree functions (random coefficients on weight <= t0)
    for tz in sorted({max(0, tmaxw // 4), max(0, tmaxw // 2), max(0, tmaxw - 1)}):
        Cp = np.zeros((d,) * m, dtype=np.int64)
        mask = Wt <= tz
        Cp[mask] = np.array([rng.randrange(p) for _ in range(int(mask.sum()))], dtype=np.int64)
        h = Cp.copy()
        for ax in range(m):
            h = F.mode_product(h, Vm, ax, p)       # values = V x C
        C = F.coeffs(h, Vinv, p)
        rows.append({"target": "ctrl_planted", "t0": tz, "delta": F.delta_of(C, Wt), "max": tmaxw,
                     "nV": 0, "generic_pred": tz, "recovered_exact": bool((C == Cp).all())})
    summ = {"kind": kind, "m": m, "size": size, "d": d, "p": p, "a": a, "b": b, "seed": seed,
            "grid": grid_size, "max": tmaxw, "sec": round(time.time() - t0, 1)}
    for tg in ("nondec", "dec", "ctrl_random", "ctrl_planted"):
        rr = [r for r in rows if r["target"] == tg]
        if not rr: continue
        summ[tg] = {"n": len(rr),
                    "eq_generic_pred": sum(r["delta"] == r["generic_pred"] for r in rr),
                    "delta_min": min(r["delta"] for r in rr), "delta_max": max(r["delta"] for r in rr),
                    "below_generic": sum(r["delta"] < r["generic_pred"] for r in rr)}
        if tg in ("nondec", "dec"):
            summ[tg]["H_frac_min"] = round(min(r["H_frac"] for r in rr), 6)
            if WEAK:
                rw = [r for r in rr if r["delta_weak"] is not None]
                summ[tg]["weak_degenerate_none"] = len(rr) - len(rw)   # pi_1(V_R) = whole base: no univariate info at any degree
                if rw:
                    summ[tg]["weak_n"] = len(rw)
                    summ[tg]["weak_eq_generic"] = sum(r["delta_weak"] == r["generic_weak"] for r in rw)
                    summ[tg]["weak_below_generic_minus1"] = sum(r["delta_weak"] < r["generic_weak"] - 1 for r in rw)
                    summ[tg]["weak_min_minus_generic"] = min(r["delta_weak"] - r["generic_weak"] for r in rw)
                    summ[tg]["weak_max_minus_generic"] = max(r["delta_weak"] - r["generic_weak"] for r in rw)
                    summ[tg]["H_frac_weak_min"] = round(min(r["H_frac_weak"] for r in rw), 6)
                summ[tg]["dS_max"] = max(r["dS"] for r in rr)
    return summ, rows

def abscissas(p, a, b, rng, d):
    out = set()
    while len(out) < d:
        x = rng.randrange(p)
        r = (x * x * x + a * x + b) % p
        if r == 0 or pow(r, (p - 1) // 2, p) == 1:
            out.add(x)
    return sorted(out)

def rand_curve_big(p, rng):
    while True:
        a = rng.randrange(p); b = rng.randrange(1, p)
        if (4 * a ** 3 + 27 * b ** 2) % p: return a, b, None, None

if __name__ == "__main__":
    kind, m, size, p = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
    ntar, ndec, nrand, seed = int(sys.argv[5]), int(sys.argv[6]), int(sys.argv[7]), int(sys.argv[8])
    summ, rows = run(kind, m, size, p, ntar, ndec, nrand, seed)
    print(json.dumps({"summary": summ, "rows": rows}))
