"""Exhaustive (all-target) test of hypothesis G and of the proven tower floors.  [pcc:unverified]

Instance: prime-order E: y^2 = x^3 + a x + b over F_p, p = 1 mod 2^k, tower base W = g*mu_{2^k}, arity m.
Every target abscissa x_R of E(F_p)\\{O} is processed (each stands for the 2 targets +-R), so every
quantity below is an exact count over the whole group, not a sample.

Per target:  V_R (grid zeros of S_R, over F_pbar-abscissas in W), |V_R|, decomposable (a zero with all
coordinates in B_x), degenerate (pi_1(V_R) = W), delta_det (least t with L_t cap F_{V_R} != 0),
delta_weak (least t of Lemma 4's weak form; infinity for degenerate targets), Delta_R.
Per t (= theta - Delta):
  original G (PROOF.md Thm C):  good  <=> delta_weak >= t, delta_det >= t, |V_R| <= H(t)/2
      eps1 = Pr_R[not good],  nu = #{R decomposable, not good, delta_weak < inf},  bound = 4/H min(Ndec, n)
  amended G' (this follow-up):  good' <=> delta_weak >= t+1 and delta_det >= t+1   (cost >= H(t))
Proven floors checked on every target (Lemma L / Prop T of the follow-up note):
  delta_det  >= k - ceil(log2 s_line) >= k - m + 1     (all targets with V_R nonempty)
  delta_weak >= k - m + 1                              (all non-degenerate targets)
  H(delta_det) >= |W|^m / |V_R|                        (uncertainty principle)
Also recorded: the conjectured Reed-Muller analogue delta_det >= m k - log2|V_R| (PROOF.md section 3)."""
import sys, json, math, random, time, os
import numpy as np
import icmodel as M
import fastdelta as F
from check_G_hypothesis import semaev_dict, S_pres_degree

def prime_order_curve(p, rng, j0=False):
    from sympy import isprime
    for _ in range(20000):
        a = 0 if j0 else rng.randrange(p); b = rng.randrange(1, p)
        if (4 * a ** 3 + 27 * b ** 2) % p == 0: continue
        n = len(M.curve_points(p, a, b)) + 1
        if isprime(n): return a, b, n
    raise RuntimeError("no prime-order curve")

def det_degree(Vidx, Vinv, Wt, p, d, m):
    if not Vidx: return None
    cols = []
    for v in Vidx:
        T = np.ones((1,) * m, dtype=np.int64)
        for ax in range(m):
            sh = [1] * m; sh[ax] = d
            T = (T * Vinv[:, v[ax]].reshape(sh)) % p
        cols.append(T)
    tmax = int(Wt.max()); det = tmax
    for t in range(tmax - 1, -2, -1):
        mask = Wt > t
        A = np.stack([c[mask] for c in cols], axis=1)
        if F._rank(A, p) < len(Vidx): det = t
        else: break
    return det

def main():
    m, k, p, cseed = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
    gmode = sys.argv[5] if len(sys.argv) > 5 else "rand"       # rand | one
    j0 = len(sys.argv) > 6 and sys.argv[6] == "j0"
    rng = random.Random(f"hypG-{m}-{k}-{p}-{cseed}-{gmode}-{j0}")
    a, b, n = prime_order_curve(p, rng, j0)
    d = 2 ** k
    g = 1 if gmode == "one" else rng.randrange(2, p)
    base = M.tower_base(p, k, g)
    roots = [w[0] for w in base.pts]
    Vm = F.vandermonde(roots, p); Vinv = F.mat_inv_mod(Vm, p)
    Wt = F.weight_grid(d, m, "tower"); tmax = int(Wt.max())
    onE = lambda x: (x ** 3 + a * x + b) % p == 0 or pow((x ** 3 + a * x + b) % p, (p - 1) // 2, p) == 1
    Bx = [i for i, x in enumerate(roots) if onE(x)]
    Bxs = set(Bx)
    t0 = time.time()
    rows = []
    for xR in range(p):
        if not onE(xR): continue
        SR = semaev_dict(m, p, a, b, xR)
        G = F.semaev_grid(SR, m, roots, p)
        Vidx = [tuple(int(i) for i in ix) for ix in zip(*np.nonzero(G == 0))]
        dec = any(all(c in Bxs for c in v) for v in Vidx)
        degen = len({v[0] for v in Vidx}) == d
        hinv = F.inv_mod_array(G, p)
        dS = S_pres_degree(SR, "tower", d, p)
        dw = None if degen else F.delta_weak(hinv, roots, Vinv, Vidx, Wt, p, dS, "tower", Vand=Vm)
        dd = det_degree(Vidx, Vinv, Wt, p, d, m)
        # largest number of V_R points on one axis-parallel line (direction x_m)
        lines = {}
        for v in Vidx: lines[v[:-1]] = lines.get(v[:-1], 0) + 1
        sline = max(lines.values()) if lines else 0
        rows.append({"xR": xR, "nV": len(Vidx), "dec": dec, "degen": degen, "dweak": dw, "ddet": dd,
                     "dS": dS, "sline": sline})
    sec = time.time() - t0
    # Hilbert function of the grid
    vals, cnt = np.unique(Wt, return_counts=True)
    H = {t: int(cnt[vals <= t].sum()) for t in range(-1, tmax + 1)}
    grid = d ** m
    INF = 10 ** 9
    ndec_targets = 2 * sum(r["dec"] for r in rows)          # each abscissa = 2 targets (+-R)
    ntar = n - 1
    floor = k - m + 1
    # proven-floor checks
    chk = {"det_line_floor_ok": all(r["ddet"] is None or r["ddet"] >= k - math.ceil(math.log2(r["sline"])) if r["sline"] > 1 else (r["ddet"] is None or r["ddet"] >= k) for r in rows),
           "det_floor_k_m_1_ok": all(r["ddet"] is None or r["ddet"] >= floor for r in rows),
           "weak_floor_k_m_1_ok": all(r["degen"] or r["dweak"] >= floor for r in rows),
           "uncertainty_ok": all(r["ddet"] is None or H[r["ddet"]] * r["nV"] >= grid for r in rows),
           "RM_analogue_ok": all(r["ddet"] is None or r["ddet"] >= m * k - math.log2(r["nV"]) - 1e-9 for r in rows),
           "RM_analogue_violations": sum(1 for r in rows if r["ddet"] is not None and r["ddet"] < m * k - math.log2(r["nV"]) - 1e-9),
           "min_dweak_nondegen": min((r["dweak"] for r in rows if not r["degen"]), default=None),
           "min_dweak_dec_nondegen": min((r["dweak"] for r in rows if r["dec"] and not r["degen"]), default=None),
           "min_ddet": min((r["ddet"] for r in rows if r["ddet"] is not None), default=None),
           "min_ddet_nondegen": min((r["ddet"] for r in rows if r["ddet"] is not None and not r["degen"]), default=None),
           "Delta_values": sorted({r["dS"] for r in rows}),
           "max_nV": max(r["nV"] for r in rows), "max_nV_nondegen": max((r["nV"] for r in rows if not r["degen"]), default=0),
           "n_degenerate_abscissas": sum(r["degen"] for r in rows),
           "n_dec_abscissas": sum(r["dec"] for r in rows)}
    per_t = []
    for t in range(0, tmax + 1):
        Ht = H[t]
        bad_o = [r for r in rows if (r["dweak"] if r["dweak"] is not None else INF) < t or
                 (r["ddet"] if r["ddet"] is not None else INF) < t or r["nV"] > Ht / 2]
        eps1 = 2 * len(bad_o) / ntar
        nu = 2 * sum(1 for r in bad_o if r["dec"] and r["dweak"] is not None)
        bound = 4.0 / Ht * min(ndec_targets, n)
        bad_a = [r for r in rows if (r["dweak"] if r["dweak"] is not None else INF) < t + 1 or
                 (r["ddet"] if r["ddet"] is not None else INF) < t + 1]
        eps1a = 2 * len(bad_a) / ntar
        nua = 2 * sum(1 for r in bad_a if r["dec"] and r["dweak"] is not None)
        nu_V = 2 * sum(1 for r in rows if r["dec"] and r["dweak"] is not None and r["nV"] > Ht / 2)
        per_t.append({"t": t, "H": Ht, "eps1": round(eps1, 6), "nu": nu, "bound_ii": round(bound, 4),
                      "G_i": eps1 <= 0.5, "G_ii": nu <= bound, "G": eps1 <= 0.5 and nu <= bound,
                      "nu_from_V_only": nu_V,
                      "amended_eps1": round(eps1a, 6), "amended_nu": nua, "amended_G": eps1a <= 0.5 and nua <= bound})
    tG = max([q["t"] for q in per_t if q["G"]], default=None)
    tA = max([q["t"] for q in per_t if q["amended_G"] and q["amended_nu"] == 0 and q["amended_eps1"] == 0], default=None)
    summ = {"m": m, "k": k, "p": p, "a": a, "b": b, "n": n, "g": g, "gmode": gmode, "j0": j0, "grid": grid, "rho": tmax,
            "Bx": len(Bx), "B": 2 * len(Bx), "Ndec_targets": ndec_targets, "n_abscissas": len(rows),
            "proven_floor_k_m_1": floor, "sec": round(sec, 1), **chk,
            "max_t_original_G": tG, "max_t_original_G_over_rho": round(tG / tmax, 3) if tG is not None else None,
            "max_t_all_targets_good_amended": tA}
    print(json.dumps({"summary": summ, "per_t": per_t, "rows": rows}))

if __name__ == "__main__":
    main()
