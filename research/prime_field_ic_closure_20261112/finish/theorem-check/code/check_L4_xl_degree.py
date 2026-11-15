"""Check Lemma L4 (exact XL degree = interpolation degree), L5 (base-blindness D >= d), L6 (rank bounds)
on toy instances, against the literal Macaulay matrix.  [pcc:unverified]

For each instance (presentation, m) and target x_R:
  V_R   = zeros of S_R on the grid W^m;  g_R = prod_{r in pi_1(V_R)} (x_1 - r)  (g_R = 1 if V_R empty)
  delta = interp_degree(h) on X = W^m \\ V_R with h = g_R(x_1)/S_R           (all monomials in N vars)
  pred  = max(deg g_R, deg S_input + delta)
  XL    : g_R notin Mac_{pred-1}  and  g_R in Mac_pred                        (=> D_XL == pred exactly)
          (non-H-basis presentations: only D_XL >= pred is claimed; we search D_XL upward)
  L5    : D_XL >= witness degree d_B (uni: deg f; graph: deg f_T)
  L6    : rank Mac_{D_XL} >= C(N + D_XL - dS, N);  reduced rank = rank(E_{D_XL-dS} on X) in [H - |V_R|, H]
Writes one JSON line per target to stdout.
"""
import sys, json, random, time, itertools
from math import comb
import flint
import icmodel as M

def reduce_mod_tower_x(SR, base, m, p):
    """Normal form of S_R(x_1..x_m) modulo the tower relations, as a polynomial in tower variables
    (x^e -> x^{e0} t_1^{e1} ... with e = sum e_j 2^j, and x^{2^k} -> c)."""
    k = base.info["k"]; c = base.info["c"]; n = 2 ** k; nloc = base.nloc
    N = m * nloc
    out = {}
    for e, coef in SR.items():
        v = [0] * N; cc = coef
        for i, ei in enumerate(e):
            q, r = divmod(ei, n)
            cc = cc * pow(c, q, p) % p
            for j in range(k):
                if (r >> j) & 1: v[i * nloc + j] += 1
        key = tuple(v)
        out[key] = (out.get(key, 0) + cc) % p
    return {kk: vv for kk, vv in out.items() if vv}

def run_instance(tag, base, m, p, a, b, pts, targets, S_mode="x", Dcap=40, hbasis=True, wdeg=None, tlimit=600):
    t_inst = time.time()
    N = m * base.nloc
    W = M.grid(base, m)
    xs_index = [i * base.nloc for i in range(m)]
    for kind, xR in targets:
        t0 = time.time()
        SR = M.semaev_special(m, p, a, b, xR)
        if S_mode == "x":
            Sg = M.S_global(SR, m, base.nloc)
        else:
            Sg = reduce_mod_tower_x(SR, base, m, p)
        dS = M.tdeg(Sg)
        Svals = [M.eval_poly(Sg, w, p) for w in W]
        V = [w for w, s in zip(W, Svals) if s == 0]
        X = [w for w, s in zip(W, Svals) if s != 0]
        sX = [s for s in Svals if s != 0]
        r1 = sorted({w[0] for w in V})
        g = [1]
        for r in r1: g = M.polmul1(g, [(-r) % p, 1], p)
        gR = {}
        for kk, cf in enumerate(g):
            if cf:
                v = [0] * N; v[0] = kk; gR[tuple(v)] = cf
        hv = [M.eval_poly(gR, w, p) * pow(s, -1, p) % p for w, s in zip(X, sX)]
        reg = None
        delta = M.interp_degree(N, X, hv, p, tmax=60)
        pred = max(len(r1), dS + delta) if delta >= 0 else len(r1)
        G = [Sg] + [M.lift_block(gg, i, base.nloc, N) for i in range(m) for gg in base.gens]
        rec = {"tag": tag, "kind": base.kind, "m": m, "p": p, "N": N, "S_mode": S_mode, "dS": dS,
               "target": kind, "xR": xR, "nV": len(V), "deg_g": len(r1), "delta": delta, "pred": pred,
               "hbasis_claim": hbasis, "info": base.info}
        if hbasis:
            below, _, _, _ = M.in_mac(G, N, pred - 1, p, gR) if pred - 1 >= len(r1) else (False, None, None, None)
            at, rk, nr, nc = M.in_mac(G, N, pred, p, gR)
            rec.update({"in_mac_pred_minus_1": below, "in_mac_pred": at, "D_XL": pred if (at and not below) else None,
                        "rank_mac": rk, "rows": nr, "cols": nc})
            D_XL = pred if (at and not below) else None
        else:
            D_XL = None; rk = nr = nc = None
            for D in range(max(pred - 1, len(r1)), Dcap + 1):
                ok, rk, nr, nc = M.in_mac(G, N, D, p, gR)
                if ok: D_XL = D; break
                if time.time() - t0 > tlimit: break
            rec.update({"D_XL": D_XL, "rank_mac": rk, "rows": nr, "cols": nc,
                        "pred_le_DXL": (D_XL is not None and pred <= D_XL)})
        if D_XL is not None:
            bound = comb(N + D_XL - dS, N) if D_XL >= dS else 0
            rec["L6_unreduced_bound"] = bound
            rec["L6_unreduced_ok"] = rk >= bound
            t = D_XL - dS
            if t >= 0:
                mons = M.monomials_upto(N, t)
                Hfull = M.eval_monomials(W, mons, p).rank()
                redX = M.eval_monomials(X, mons, p).rank() if X else 0
            else:
                Hfull = redX = 0
            rec.update({"H_t": Hfull, "reduced_rank": redX, "L6_reduced_ok": Hfull - len(V) <= redX <= Hfull})
            if wdeg is not None:
                rec["witness_deg"] = wdeg
                rec["L5_ok"] = D_XL >= wdeg
        rec["sec"] = round(time.time() - t0, 2)
        print(json.dumps(rec), flush=True)

def pick_targets(base, m, p, a, b, pts, rng, n_rand, n_dec):
    W = M.grid(base, m)
    xset = base.xvals
    out = []
    tries = 0
    while len([t for t in out if t[0] == "nondec"]) < n_rand and tries < 5000:
        tries += 1
        xR = rng.randrange(p)
        SR = M.semaev_special(m, p, a, b, xR)
        if all(M.eval_poly(SR, [w[i * base.nloc] for i in range(m)], p) for w in W):
            out.append(("nondec", xR))
    # decomposable: roots X of S_{m+1}(x_1..x_m, X) for random grid abscissas
    tries = 0
    seen = set()
    while len([t for t in out if t[0] == "dec"]) < n_dec and tries < 200:
        tries += 1
        xs = [rng.choice(xset) for _ in range(m)]
        cand = [X for X in range(p) if M.eval_poly(M.semaev_special(m, p, a, b, X), xs, p) == 0] if p < 3000 else None
        if cand is None:
            # use the point sums of rational lifts when available
            pl = {P[0]: P for P in pts}
            if not all(x in pl for x in xs): continue
            R = None
            for x in xs:
                P = pl[x]
                if rng.random() < .5: P = M.ec_neg(P, p)
                R = M.ec_add(R, P, p, a)
            if R is None: continue
            cand = [R[0]]
        for X in cand:
            if X not in seen:
                seen.add(X); out.append(("dec", X)); break
    return out

if __name__ == "__main__":
    which = sys.argv[1]
    seed = int(sys.argv[2]) if len(sys.argv) > 2 else 1
    rng = random.Random(f"L4-{which}-{seed}")
    if which == "uni2":
        p = 12289
        a, b, pts, n = M.rand_curve(p, rng)
        absc = sorted({P[0] for P in pts})
        for d in (2, 3, 4, 5, 6, 8):
            base = M.uni_base(p, rng.sample(absc, d))
            tg = pick_targets(base, 2, p, a, b, pts, rng, 4, 3)
            run_instance(f"uni2-d{d}", base, 2, p, a, b, pts, tg, wdeg=d)
    elif which == "kummer2":
        p = 12289
        a, b, pts, n = M.rand_curve(p, rng)
        for d in (4, 8, 16):
            base = M.kummer_base(p, d, rng.randrange(2, p))
            tg = pick_targets(base, 2, p, a, b, pts, rng, 3, 2)
            run_instance(f"kummer2-d{d}", base, 2, p, a, b, pts, tg, wdeg=d)
    elif which == "uni3":
        p = 12289
        a, b, pts, n = M.rand_curve(p, rng)
        absc = sorted({P[0] for P in pts})
        for d in (2, 3, 4):
            base = M.uni_base(p, rng.sample(absc, d))
            tg = pick_targets(base, 3, p, a, b, pts, rng, 2, 2)
            run_instance(f"uni3-d{d}", base, 3, p, a, b, pts, tg, wdeg=d)
    elif which == "tower2":
        p = 12289
        a, b, pts, n = M.rand_curve(p, rng)
        for k in (1, 2):
            base = M.tower_base(p, k, rng.randrange(2, p))
            tg = pick_targets(base, 2, p, a, b, pts, rng, 3, 2)
            run_instance(f"tower2-k{k}-Sx", base, 2, p, a, b, pts, tg, S_mode="x")
        for k in (1, 2, 3):
            base = M.tower_base(p, k, rng.randrange(2, p))
            tg = pick_targets(base, 2, p, a, b, pts, rng, 3, 2)
            run_instance(f"tower2-k{k}-Sred", base, 2, p, a, b, pts, tg, S_mode="red")
    elif which == "graph2":
        p = 12289
        a, b, pts, n = M.rand_curve(p, rng)
        for dT in (2, 3, 4, 5):
            troots = rng.sample(range(1, p), dT)
            num = [0, 1, 1]          # x = t^2 + t
            base = M.graph_base(p, num, [1], troots)
            tg = pick_targets(base, 2, p, a, b, pts, rng, 2, 2)
            run_instance(f"graph2-dT{dT}", base, 2, p, a, b, pts, tg, hbasis=False, wdeg=dT, Dcap=4 * dT + 12)

if __name__ == "__main__" and sys.argv[1] == "bits2":
    p = 12289
    rng = random.Random("L4-bits2")
    a, b, pts, n = M.rand_curve(p, rng)
    for s_ in (1, 2, 3):
        base = M.bits_base(p, s_)
        tg = pick_targets(base, 2, p, a, b, pts, rng, 3, 2)
        run_instance(f"bits2-s{s_}-Sx", base, 2, p, a, b, pts, tg, S_mode="x")
