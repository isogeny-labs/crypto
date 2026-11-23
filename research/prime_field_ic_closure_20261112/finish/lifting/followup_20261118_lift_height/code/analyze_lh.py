#!/usr/bin/env python3
"""lift-height analysis: per-cell rho_G statistics, kill rule K-B/C/D, 256-bit margins, Lang-constant gap.
[pcc:unverified]   usage: analyze_lh.py <out_json>"""
import sys, os, json, glob, math, statistics as st
import numpy as np
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
os.environ.setdefault("LIFT_CODE", HERE)
import bounds_hermite as BH
import lh_heights

D = os.path.dirname(HERE)
PREV = os.environ.get("LH_PREV", "/nonexistent")   # pass 2: 20261115 data (5 doublings, scaled) not mixed in
RHO_MAX = {7: 32.6, 8: 10.8, 9: 4.94}


def q(xs, f):
    xs = sorted(xs)
    if not xs:
        return None
    k = (len(xs) - 1) * f
    lo, hi = math.floor(k), math.ceil(k)
    return xs[lo] + (xs[hi] - xs[lo]) * (k - lo)


def load(pattern, src):
    out = []
    for f in sorted(glob.glob(pattern)):
        for l in open(f):
            d = json.loads(l)
            d["_src"] = src
            d.setdefault("origin", "O")
            out.append(d)
    return out


def alt_rhos(d):
    """tangent construction: rho in the fixed difference basis w1 = P1 - O', wi = Pi - P1, and the
    (data-dependent, not union-bound-usable) LLL-reduced-basis value."""
    h = d["heights"]
    G = np.array(h.get("G") or [])
    if G.size == 0 or not h.get("lambda_G"):
        return None, None
    r = G.shape[0]
    T = np.eye(r)
    for i in range(1, r):
        T[0, i] = -1.0
    Gd = T.T @ G @ T
    rho_diff = float(max(np.diag(Gd)) / h["lambda_G"])
    # greedy pairwise reduction (Gram-only LLL surrogate): repeat size reduction until stable
    B = np.eye(r)
    Gb = G.copy()
    for _ in range(200):
        changed = False
        order = np.argsort(np.diag(Gb))
        for a in order:
            for b in range(r):
                if a == b:
                    continue
                mu = round(Gb[a, b] / Gb[b, b])
                if mu and Gb[a, a] - 2 * mu * Gb[a, b] + mu * mu * Gb[b, b] < Gb[a, a] - 1e-9:
                    B[:, a] -= mu * B[:, b]
                    Gb = B.T @ G @ B
                    changed = True
        if not changed:
            break
    rho_red = float(max(np.diag(Gb)) / h["lambda_G"])
    # fixed unimodular basis T3 (data-independent): {w2+w3-w1, w2-w1, ..., wr-w1}
    T3 = np.zeros((r, r))
    T3[0, 0], T3[1, 0], T3[2, 0] = -1, 1, 1
    for i in range(1, r):
        T3[0, i] = -1
        T3[i, i] = 1
    assert abs(abs(np.linalg.det(T3)) - 1) < 1e-9
    G3 = T3.T @ G @ T3
    rho_T3 = float(max(np.diag(G3)) / h["lambda_G"])
    return rho_diff, rho_red, rho_T3


def cell_stats(recs):
    ok = [d for d in recs if d["status"] == "ok"]
    rel = [d for d in ok if d.get("rel")]
    rr = [d for d in ok if not d.get("rel") and d.get("heights") and d["heights"].get("rho_G")
          and d["heights"].get("lambda_vec") not in ("singular", "enumeration-cap")]
    rhos = [d["heights"]["rho_G"] for d in rr]
    epsrel = [d["heights"]["eps"] / d["heights"]["lambda_G"] for d in rr]
    s = dict(n=len(recs), n_ok=len(ok), n_rel=len(rel), n_rho=len(rhos),
             rho_med=q(rhos, .5), rho_p90=q(rhos, .9), rho_p99=q(rhos, .99), rho_max=max(rhos) if rhos else None,
             eps_over_lambda_max=max(epsrel) if epsrel else None,
             H_med=q([d["heights"]["H"] for d in rr], .5),
             lam_med=q([d["heights"]["lambda_G"] for d in rr], .5),
             logDW_med=q([d["heights"]["log_absDW"] for d in rr], .5),
             lang_lamG_min=min((d["heights"]["lambda_G"] / d["heights"]["log_absDW"] for d in rr), default=None),
             sec_med=q([d["sec"] for d in ok], .5),
             caps=sum(1 for d in ok if d.get("heights", {}).get("lambda_vec") == "enumeration-cap"))
    ml = [d["ml"] for d in rr if d.get("ml")]
    if ml:
        s["rho5_max"] = max(m["5"]["rho_G"] for m in ml)
        s["rho5_med"] = q([m["5"]["rho_G"] for m in ml], .5)
        s["rich_consistency_max"] = max(abs(m["rich"]["rho_G"] - m["rich_prev"]["rho_G"]) / m["rich"]["rho_G"] for m in ml)
    if recs and recs[0].get("origin") == "tangent":
        ad = [alt_rhos(d) for d in rr]
        ad = [a for a in ad if a[0] is not None]
        if ad:
            s["rho_diff_max"] = max(a[0] for a in ad)
            s["rho_diff_med"] = q([a[0] for a in ad], .5)
            s["rho_red_max"] = max(a[1] for a in ad)
            s["rho_red_med"] = q([a[1] for a in ad], .5)
            s["rho_T3_max"] = max(a[2] for a in ad)
            s["rho_T3_med"] = q([a[2] for a in ad], .5)
    return s, rhos


def margin(r, rho):
    return BH.margin_from_log2N(BH.log2_N_hermite(r, rho), 256)


def main(out_json):
    new = load(os.environ.get("LH_RUNS", D + "/out/runs") + "/*.jsonl", "new")
    prev = [d for d in load(PREV + "/*_b*_r*.jsonl", "pass1") if d["r"] in (7, 8, 9)]
    dep = load(os.environ.get("LH_DEP", D + "/out/depD") + "/*.jsonl", "depD")
    res = {"cells": [], "verdicts": {}}
    cells = {}
    for d in new + prev:
        key = (d["origin"], d["arm"], d["r"], d["bits"], d["_src"])
        cells.setdefault(key, []).append(d)
    rho_by = {}
    for key in sorted(cells):
        s, rhos = cell_stats(cells[key])
        s.update(origin=key[0], arm=key[1], r=key[2], bits=key[3], src=key[4])
        if key[1] == "DET":
            ok = [d for d in cells[key] if d["status"] == "ok"]
            s["det_hits"] = sum(1 for d in ok if d.get("rel") and sorted(map(abs, d["rel"][:3])) == [1, 1, 1]
                                and not any(d["rel"][3:]))
        res["cells"].append(s)
        if key[1] == "X":
            rho_by.setdefault((key[0], key[2]), {}).setdefault(key[3], []).extend(rhos)
    # kill rule per r
    for (origin, r), byb in sorted(rho_by.items()):
        if r not in RHO_MAX:
            continue
        bits = sorted(byb)
        bmax = max(b for b in bits if byb[b])
        mx = max(byb[bmax])
        pts = [(b, q(byb[b], .9)) for b in bits if b >= 12 and len(byb[b]) >= 10]
        slope = icpt = ext = None
        if len(pts) >= 2:
            xs = np.array([p[0] for p in pts], float)
            ys = np.array([p[1] for p in pts], float)
            slope, icpt = np.polyfit(xs, ys, 1)
            ext = float(icpt + slope * 256)
        n20 = sum(len(byb[b]) for b in bits if b >= 20)
        allr = [x for b in bits for x in byb[b]]
        k_i = mx > RHO_MAX[r]
        k_ii = bool(slope is not None and slope > 0 and ext > RHO_MAX[r])
        if n20 < 20:
            v = "INCONCLUSIVE (<20 ok trials at bits>=20)"
        elif k_i or k_ii:
            v = "KILLED"
        else:
            v = "SUPPORTED-EMPIRICAL"
        res["verdicts"][f"{origin}_r{r}"] = dict(
            origin=origin, r=r, rho_max_thr=RHO_MAX[r], bits_max=bmax, rho_max_at_bmax=mx, K_i=k_i,
            p90_slope=None if slope is None else float(slope), p90_ext256=ext, K_ii=k_ii, n_bits_ge20=n20,
            p90_points=pts, rho_max_all=max(allr), margin256_at_max_all=margin(r, max(allr)),
            margin256_at_max_bmax=margin(r, mx), verdict_pre_KD=v)
    # arm D: dependent-conditional
    depc = {}
    for d in dep:
        if d["status"] != "ok":
            continue
        key = (d["origin"], d["r"], d["bits"])
        depc.setdefault(key, {"indep": [], "dep": []})
        h = d.get("heights", {})
        if d.get("rel"):
            c = d.get("circuit", {})
            if d.get("verified") == "exact" and c.get("rho_L"):
                depc[key]["dep"].append(dict(rho_L=c["rho_L"], rho_LJ=c["rho_LJ"], k=c["k"], rel=d["rel"],
                                             resid_rel=c["resid"] / h["H"]))
        elif h.get("rho_G") and h.get("lambda_vec") not in ("singular", "enumeration-cap"):
            depc[key]["indep"].append(h["rho_G"])
    dres = []
    for origin in ("O", "tangent"):
        keys = [k for k in depc if k[0] == origin]
        dl = [x for k in keys for x in depc[k]["dep"]]
        ind = [x for k in keys for x in depc[k]["indep"]]
        # same-cell pooled comparison: only cells that produced >=1 dependent trial
        keys_d = [k for k in keys if depc[k]["dep"]]
        ind_same = [x for k in keys_d for x in depc[k]["indep"]]
        row = dict(origin=origin, n_dep=len(dl), n_indep=len(ind), dep_rho_L=[round(x["rho_L"], 3) for x in dl],
                   dep_rho_L_med=q([x["rho_L"] for x in dl], .5), dep_rho_L_max=max((x["rho_L"] for x in dl), default=None),
                   dep_resid_rel_max=max((abs(x["resid_rel"]) for x in dl), default=None),
                   dep_k=[x["k"] for x in dl],
                   indep_same_cells_n=len(ind_same), indep_same_cells_med=q(ind_same, .5),
                   indep_same_cells_p90=q(ind_same, .9), indep_same_cells_p99=q(ind_same, .99),
                   indep_same_cells_max=max(ind_same) if ind_same else None,
                   frac_dep_above_indep_p90=(sum(1 for x in dl if x["rho_L"] > q(ind_same, .9)) / len(dl)) if dl and ind_same else None)
        if len(dl) >= 5:
            row["K_D_contradicted"] = row["dep_rho_L_med"] > row["indep_same_cells_p99"]
        else:
            row["K_D_contradicted"] = None
        per = []
        for k in sorted(keys):
            per.append(dict(r=k[1], bits=k[2], n_indep=len(depc[k]["indep"]), n_dep=len(depc[k]["dep"]),
                            indep_med=q(depc[k]["indep"], .5), indep_max=max(depc[k]["indep"], default=None),
                            dep=[round(x["rho_L"], 3) for x in depc[k]["dep"]]))
        row["per_cell"] = per
        dres.append(row)
    res["armD"] = dres
    # fits of H and log|Delta_W| vs bits (new + prev, honest, relation-free) per (origin, r)
    fits = {}
    for origin in ("O", "tangent"):
        for r in (7, 8, 9):
            xs, hs, ds = [], [], []
            for d in new + prev:
                if d["origin"] == origin and d["r"] == r and d["arm"] == "X" and d["status"] == "ok" \
                        and d.get("heights") and d["heights"].get("H") and not d.get("rel"):
                    xs.append(d["bits"])
                    hs.append(d["heights"]["H"])
                    ds.append(d["heights"]["log_absDW"])
            if len(set(xs)) >= 2:
                hb, ha = np.polyfit(xs, hs, 1)
                db, da = np.polyfit(xs, ds, 1)
                H256, D256 = ha + hb * 256, da + db * 256
                lam_req = H256 / RHO_MAX[r]
                fits[f"{origin}_r{r}"] = dict(H=[float(ha), float(hb)], logDW=[float(da), float(db)], n=len(xs),
                                              bits=sorted(set(xs)), H256=float(H256), logDW256=float(D256),
                                              lambda_req256=float(lam_req), c_req256=float(lam_req / D256))
    res["fits"] = fits
    # explicit Lang-type constants at sigma = 1 (their most favourable value)   [L-recalled]
    sig = 1.0
    c_HS = 2.0 / ((20 * sig) ** 8 * 10 ** (4 * sig + 1.1))           # Hindry-Silverman 1988 shape, d = 1
    c_P = 1.0 / (1e15 * sig ** 6 * math.log(104613 * sig ** 2) ** 2)  # Petsche 2006 Thm 2
    res["lang"] = dict(sigma=sig, c_HS=c_HS, c_Petsche=c_P,
                       gap_factor={k: dict(HS=v["c_req256"] / c_HS, Petsche=v["c_req256"] / c_P) for k, v in fits.items()})
    json.dump(res, open(out_json, "w"), indent=1, default=float)
    return res


if __name__ == "__main__":
    r = main(sys.argv[1])
    for c in r["cells"]:
        if c["r"] >= 7:
            print({k: (round(v, 4) if isinstance(v, float) else v) for k, v in c.items()})
    print()
    for k, v in r["verdicts"].items():
        print(k, {kk: (round(vv, 3) if isinstance(vv, float) else vv) for kk, vv in v.items() if kk != "p90_points"})
    print()
    for row in r["armD"]:
        print({k: v for k, v in row.items() if k != "per_cell"})
    print()
    for k, v in r["fits"].items():
        print(k, {kk: (round(vv, 6) if isinstance(vv, float) else vv) for kk, vv in v.items()})
    print(r["lang"])
