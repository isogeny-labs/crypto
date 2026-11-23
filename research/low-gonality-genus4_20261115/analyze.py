#!/usr/bin/env python3
"""Aggregates results/*.jsonl and results/dlp_*.json into RESULTS.json and RESULTS.md.
Pure Python. Every number is [pcc:unverified]. usage: python3 analyze.py"""
import glob, json, math, os, re, statistics as st

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.join(HERE, "results")
HASH_NS = 188.0  # fnv over a 46-word key, best of 7, standalone rustc -O run (/tmp/hashbench.rs), loaded host
GIT = "3c24520f7cdc1c47bac26cc7778d058e655b79a8"


def mean_ci(xs):
    n = len(xs)
    if n == 0:
        return None
    m = sum(xs) / n
    if n < 2:
        return [m, m, m]
    sd = st.stdev(xs)
    tcrit = {2: 12.71, 3: 4.30, 4: 3.18, 5: 2.78, 6: 2.57, 7: 2.45, 8: 2.36}.get(n, 2.1 if n < 20 else 1.97)
    h = tcrit * sd / math.sqrt(n)
    return [m, m - h, m + h]


def slope(xs, ys):
    """OLS slope of log y on log x with standard error; needs >= 3 points for the SE."""
    lx = [math.log(x) for x in xs]
    ly = [math.log(y) for y in ys]
    n = len(lx)
    mx, my = sum(lx) / n, sum(ly) / n
    sxx = sum((a - mx) ** 2 for a in lx)
    b = sum((a - mx) * (c - my) for a, c in zip(lx, ly)) / sxx
    a0 = my - b * mx
    if n < 3:
        return [b, None, a0]
    res = sum((c - (a0 + b * a)) ** 2 for a, c in zip(lx, ly))
    se = math.sqrt(res / (n - 2) / sxx)
    return [b, se, a0]


R = {"commands": [
    "genus4_pencil_ic yield P K 20261115 results/yield_pP.jsonl  for (P,K) in (31,16) (61,16) (127,16) (251,12) (509,8)",
    "genus4_pencil_ic dlp 61 S 8 results/dlp_p61_sS.json (S=1,2,3); dlp 127 S 8 (S=1,2,3); dlp 251 S 6 (S=1,2,3); dlp 509 1 3",
    "cargo test --release --bin genus4_pencil_ic -j2 (results/unit_tests.log)"],
    "host": "shared 4-CPU Linux VM (another agent and up to three of these jobs ran concurrently); timings are best-of-N under load",
    "binary": "target/release/genus4_pencil_ic built from the final source with CARGO_TARGET_DIR=/home/user/crypto/target --release",
    "git_head": GIT, "date": "2026-11-15", "label": "[pcc:unverified]", "yield": {}, "dlp": [], "fits": {}, "predictions": []}

# ---------------- yield study ----------------
for f in sorted(glob.glob(os.path.join(RES, "yield_p*.jsonl"))):
    p = int(re.search(r"yield_p(\d+)", f).group(1))
    rows = [json.loads(l) for l in open(f)]
    fr = []
    hist = [0, 0, 0, 0]
    tot = 0
    for r in rows:
        h1, h2 = r["fib_hist_pencil1"], r["fib_hist_pencil2"]
        fr.append((h1[3] + h2[3]) / (2 * r["fibres"]))
        for i in range(4):
            hist[i] += h1[i] + h2[i]
        tot += 2 * r["fibres"]
    cells = {}
    for r in rows:
        n = r["npts"]
        for c in r["cells"]:
            cells.setdefault(c["phi"], []).append((n, c))
    ph = []
    for phi, lst in sorted(cells.items()):
        ys, ratios, ratios_h, dup, mm = [], [], [], [], []
        for n, c in lst:
            m = c["m"]
            y = c["hits_a"] / c["pairs"] if c["pairs"] else 0.0
            ys.append(y)
            pred = (m / n) ** 3 / 6
            predh = (m - 3) * (m - 4) * (m - 5) / ((n - 3) * (n - 4) * (n - 5)) / 6
            ratios.append(y / pred)
            ratios_h.append(y / predh)
            if c["hits_a"]:
                dup.append(c["unique_a"] / c["hits_a"])
            mm.append(m)
        tot_hits = sum(c["hits_a"] for _, c in lst)
        tot_pairs = sum(c["pairs"] for _, c in lst)
        tot_unique = sum(c["unique_a"] for _, c in lst)
        pooled_pred = sum(c["pairs"] * (c["m"] / n) ** 3 / 6 for n, c in lst) / tot_pairs
        pooled_pred_h = sum(c["pairs"] * (c["m"] - 3) * (c["m"] - 4) * (c["m"] - 5) / ((n - 3) * (n - 4) * (n - 5)) / 6 for n, c in lst) / tot_pairs
        f3 = [c["fibres_full_in_f"] / (2 * (p + 1)) for _, c in lst]
        ph.append({
            "phi": phi,
            "m_mean": sum(mm) / len(mm),
            "pairs_total": tot_pairs,
            "hits_total": tot_hits,
            "unique_total": tot_unique,
            "yield_pooled": tot_hits / tot_pairs,
            "pred_naive_pooled": pooled_pred,
            "pred_hyper_pooled": pooled_pred_h,
            "ratio_naive_pooled": (tot_hits / tot_pairs) / pooled_pred,
            "ratio_naive_curves": mean_ci(ratios),
            "ratio_hyper_pooled": (tot_hits / tot_pairs) / pooled_pred_h,
            "unique_over_hits": tot_unique / tot_hits if tot_hits else None,
            "fibre_rows_full_in_F_frac": mean_ci(f3),
            "pred_fibre_full_in_F": (sum(c["m"] / n for n, c in lst) / len(lst)) ** 3 / 6,
        })
    R["yield"][str(p)] = {
        "curves": len(rows),
        "npts_mean": sum(r["npts"] for r in rows) / len(rows),
        "split3_fraction": mean_ci(fr),
        "split3_pooled": hist[3] / tot,
        "hist": [x / tot for x in hist],
        "phi_cells": ph,
    }

# ---------------- DLP cells ----------------
S_W = [0.0, 0.1, 1.0]
for f in sorted(glob.glob(os.path.join(RES, "dlp_p*_s*.json"))):
    d = json.load(open(f))
    if "error" in d:
        continue
    c = d["calib"]
    wp, wl = c["w_cal_p"], c["w_cal_ell"]
    w_hash = HASH_NS / c["t_add_ns"]
    enum = d["enum_native_p"]
    cell = {"p": d["p"], "seed": d["seed"], "file": os.path.basename(f), "N": d["N"], "ell": d["ell"], "npts": d["npts"], "factors": d["factors"],
            "calib": c, "w_cal_hash": w_hash, "fallbacks": d["fallbacks"], "exceptional_keys": d["exceptional_keys"], "keyfail": d["keyfail"],
            "curve_seed": d["curve_seed"], "trial": d["trial"], "g": d["g"], "x_true": d["x_true"], "secs": {"ic": d["ic_secs"], "rho": d["rho_secs"], "total": d["total_secs"]}}
    passes = []
    for k, fin in enumerate(d["ic_final"]):
        pr = {"pass": k, "ok": fin["ok"]}
        if fin["ok"]:
            G, Np, Nl = fin["group_ops"], fin["native_p"] + enum, fin["native_ell"]
            S = {"w=0": G, "w=0.1": G + 0.1 * (Np + Nl), "w=1": G + (Np + Nl), "w=cal": G + wp * Np + wl * Nl, "w=10cal": G + 10 * (wp * Np + wl * Nl)}
            sums = d["ic_sums_all_rungs"][k]
            Gs, Nps, Nls = sums["group_ops"], sums["native_p"] + enum, sums["native_ell"]
            Sall = {"w=0": Gs, "w=0.1": Gs + 0.1 * (Nps + Nls), "w=1": Gs + (Nps + Nls), "w=cal": Gs + wp * Nps + wl * Nls}
            att = [a for a in d["ic_passes"][k] if a["ok"]][0]
            pr.update({"phi": fin["phi"], "m": fin["m"], "c_supplement": fin["c_budget_factor"] > 0, "group_ops": G, "native_p": Np, "native_ell": Nl, "S": S, "S_all_rungs": Sall,
                       "rows": att["rows"], "ncols_used": att["ncols_used"], "unique_a": att["unique_a"], "unique_c": att["unique_c"], "fibre_rows": att["fibre_rows"],
                       "la_ops": att["la_ops"], "la_matvecs": att["la_matvecs"], "indiv_steps": att["indiv_steps"], "pairs": att["pairs"], "hits_a": att["hits_a"]})
            # first rung (phi) at which A+B alone sufficed
            ab = [a for a in d["ic_passes"][k] if a["c_budget_factor"] == 0 and a["ok"]]
            pr["a_plus_b_sufficient"] = bool(ab)
            pr["phi_star_ab"] = ab[0]["phi"] if ab else None
        passes.append(pr)
    cell["ic"] = passes
    runs = [r for r in d["rho"] if r["verified"]]
    cell["rho_runs_total"] = len(d["rho"])
    cell["rho_runs_verified"] = len(runs)
    gae = [r["setup_adds"] + r["walk_adds"] for r in runs]
    nat = [r["native"] for r in runs]
    cell["rho_gae"] = mean_ci(gae)
    cell["rho_gae_minmax"] = [min(gae), max(gae)] if gae else None
    cell["rho_gae_runs"] = gae
    cell["rho_native_mean"] = sum(nat) / len(nat) if nat else None
    cell["rho_expected_steps"] = math.sqrt(math.pi * d["ell"] / 2)
    cell["rho_steps_over_expected"] = (sum(r["steps"] for r in runs) / len(runs)) / cell["rho_expected_steps"] if runs else None
    if len(gae) >= 4:
        h = len(gae) // 2
        a, b = gae[:h], gae[h:]
        se = math.sqrt(st.variance(a) / len(a) + st.variance(b) / len(b)) if len(a) > 1 and len(b) > 1 else None
        cell["rho_aa"] = {"mean_first": sum(a) / len(a), "mean_second": sum(b) / len(b), "z": (sum(a) / len(a) - sum(b) / len(b)) / se if se else None}
    if runs:
        rm = sum(gae) / len(gae)
        rn = sum(nat) / len(nat)
        cell["S_rho"] = {"w=0": rm, "w=0.1": rm + 0.1 * rn, "w=1": rm + rn, "w=cal": rm + w_hash * rn, "w=10cal": rm + 10 * w_hash * rn}
        cell["S_rho_free_negation_assumed"] = {k: v / math.sqrt(2) for k, v in cell["S_rho"].items()}
        for pr in passes:
            if pr.get("ok"):
                pr["ratio"] = {k: pr["S"][k] / cell["S_rho"][k] for k in pr["S"]}
                pr["ratio_all_rungs"] = {k: pr["S_all_rungs"][k] / cell["S_rho"][k] for k in pr["S_all_rungs"]}
                pr["ratio_free_neg"] = {k: pr["S"][k] / cell["S_rho_free_negation_assumed"][k] for k in pr["S"]}
                # sensitivity: a GAE costs only 2.6e3 mulmods (optimised arithmetic): native ops are 100x dearer
                mpa = c["mulmods_per_add"]
                scale = mpa / 2600.0
                pr["ratio_if_gae_2600"] = {"w=cal": (pr["group_ops"] + scale * (wp * pr["native_p"] + wl * pr["native_ell"])) / (rm + scale * w_hash * rn)}
                # break-even w: S_IC(w) = S_rho(w)
                den = (pr["native_p"] + pr["native_ell"]) - rn
                pr["break_even_w"] = (rm - pr["group_ops"]) / den if den > 0 else None
    R["dlp"].append(cell)

# ---------------- fits across cells (per p mean over seeds, first IC pass) ----------------
byp = {}
for cell in R["dlp"]:
    ps = [x for x in cell["ic"] if x.get("ok")]
    if not ps or not cell.get("rho_gae"):
        continue
    byp.setdefault(cell["p"], []).append((cell, ps[0]))
fit = {}
if byp:
    P = sorted(byp)
    def pm(fn):
        return [sum(fn(c, i) for c, i in byp[p]) / len(byp[p]) for p in P]
    series = {
        "rho_gae": pm(lambda c, i: c["rho_gae"][0]),
        "ic_group_ops": pm(lambda c, i: i["group_ops"]),
        "ic_native_p": pm(lambda c, i: i["native_p"]),
        "ic_native_ell": pm(lambda c, i: i["native_ell"]),
        "ic_m_star": pm(lambda c, i: i["m"]),
        "ic_rows": pm(lambda c, i: i["rows"]),
        "npts": pm(lambda c, i: c["npts"]),
    }
    R["series_by_p"] = {"p": P, **series}
    if len(P) >= 2:
        for k, v in series.items():
            fit[k] = slope(P, v)
        fit["rho_gae_vs_ell"] = None
    R["fits"] = {k: {"slope": v[0], "se": v[1], "n": len(P)} for k, v in fit.items() if v}
    P2 = [p for p in P if p >= 127]
    if len(P2) >= 3:
        R["fits_p_ge_127"] = {}
        for k, v in series.items():
            sl = slope(P2, [v[P.index(p)] for p in P2])
            R["fits_p_ge_127"][k] = {"slope": sl[0], "se": sl[1], "n": len(P2)}
    norm = []
    for cell in R["dlp"]:
        ex = cell["rho_expected_steps"]
        norm += [(r["steps"] + 0.0) / ex for r in json.load(open(os.path.join(RES, cell["file"])))["rho"] if r["verified"]]
    R["rho_pooled_steps_over_expected"] = mean_ci(norm) + [len(norm)]
    # break-even w by p (measured implementation) and a model with root-finding per pair
    R["break_even_by_p"] = {str(p): [i["break_even_w"] for c, i in byp[p] if i.get("break_even_w") is not None] for p in P}
    def wstar(Np_fn):
        out = []
        for p in P:
            vals = []
            for c, i in byp[p]:
                rn = c["rho_native_mean"]
                rm = c["rho_gae"][0]
                den = Np_fn(c, i) + i["native_ell"] - rn
                if den > 0:
                    vals.append((rm - i["group_ops"]) / den)
            out.append(sum(vals) / len(vals) if vals else None)
        return out
    meas = wstar(lambda c, i: i["native_p"])
    ideal = wstar(lambda c, i: i["native_p"] - i["pairs"] * (4 * i["m"] + 60) + i["pairs"] * 2000)
    proj = {"p": P, "w_star_measured_impl": meas, "w_star_root_finding_model": ideal, "assumption": "model: each A pair costs 2000 mulmods (cubic root finding) instead of 4m+60 (scan of F); not a measurement"}
    for nm, ws in (("w_star_measured_impl", meas), ("w_star_root_finding_model", ideal)):
        if all(v and v > 0 for v in ws) and len(P) >= 3:
            b, se, a0 = slope(P, ws)
            proj[nm + "_slope"] = [b, se]
            for target in (0.1, 1.0):
                if b != 0:
                    pstar = math.exp((math.log(target) - a0) / b)
                    proj[f"{nm}_p_at_w{target}"] = pstar
    R["projection"] = proj

# ---------------- predictions ----------------
PRED = R["predictions"]
Yd = R["yield"]
tot = sum(2 * y["curves"] * (int(p) + 1) for p, y in Yd.items() if int(p) >= 61)
s3 = sum(y["split3_pooled"] * 2 * y["curves"] * (int(p) + 1) for p, y in Yd.items() if int(p) >= 61)
f = s3 / tot if tot else None
PRED.append({"id": "P1", "claim": "fibres with 3 rational points = 1/6 +-25% (pooled p >= 61)", "outcome": "confirmed" if f and 0.125 <= f <= 0.2083 else "refuted", "evidence": f"pooled fraction {f:.4f}; per p: " + ", ".join(f"{p}:{y['split3_pooled']:.3f}" for p, y in sorted(Yd.items(), key=lambda kv: int(kv[0])))})
ratios = [(int(p), c["phi"], c["ratio_naive_pooled"]) for p, y in Yd.items() for c in y["phi_cells"] if c["m_mean"] >= 60]
okr = all(0.7 <= r <= 1.3 for _, _, r in ratios)
PRED.append({"id": "P2", "claim": "A-line yield per pair = phi^3/6 +-30% for |F| >= 60", "outcome": "confirmed" if okr else "refuted", "evidence": f"{len(ratios)} cells, measured/naive in [{min(r for _,_,r in ratios):.2f}, {max(r for _,_,r in ratios):.2f}]; outside +-30%: " + (", ".join(f"p={p} phi={ph} ratio {r:.2f}" for p, ph, r in ratios if not 0.7 <= r <= 1.3) or "none")})
dups = [c["unique_over_hits"] for y in Yd.values() for c in y["phi_cells"] if c["hits_total"] >= 100]
PRED.append({"id": "P3", "claim": "unique A relations = hits/10 +-15%", "outcome": "confirmed" if all(0.085 <= d <= 0.115 for d in dups) else "refuted", "evidence": f"unique/hits in [{min(dups):.3f}, {max(dups):.3f}] over {len(dups)} cells"})
c0 = [(c, c["ic"][0]) for c in R["dlp"] if c["ic"] and c["ic"][0].get("ok")]
by = {}
for c, i in c0:
    by.setdefault(c["p"], []).append(i)
ev = []
for p in sorted(by):
    ev.append(f"p={p}: " + ", ".join(("phi*=%s" % i["phi_star_ab"]) if i["a_plus_b_sufficient"] else "A+B insufficient (needed C)" for i in by[p]) + f"; m used {[i['m'] for i in by[p]]}")
ok61 = all(not i["a_plus_b_sufficient"] for i in by.get(61, []))
pred_phi = {127: 1.0, 251: 0.8, 509: 0.67}
dev = []
for p, pv in pred_phi.items():
    for i in by.get(p, []):
        if i["a_plus_b_sufficient"]:
            dev.append(abs(i["phi_star_ab"] - pv))
        else:
            dev.append(9)
mstar = [(p, sum(i["m"] for i in by[p]) / len(by[p])) for p in sorted(by) if p >= 127 and all(i["a_plus_b_sufficient"] for i in by[p])]
sl = slope([p for p, _ in mstar], [m for _, m in mstar]) if len(mstar) >= 2 else [None, None, None]
PRED.append({"id": "P4", "claim": "A+B insufficient at p=61; phi* ~ 1.0, 0.8, 0.67 at p=127, 251, 509 (+-0.1); exponent of m* in [0.6, 0.9]", "outcome": ("confirmed" if ok61 and dev and max(dev) <= 0.1 + 1e-9 and sl[0] is not None and 0.6 <= sl[0] <= 0.9 else "refuted" if (not ok61 or (dev and max(dev) > 0.1 + 1e-9)) else "untested"), "evidence": "; ".join(ev) + f"; exponent of m* over the p >= 127 values where every cell was solved by A+B alone (p = {', '.join(str(p) for p, _ in mstar)}; {len(mstar)} points only): {sl[0] if sl[0] is None else round(sl[0], 2)}; all-cell fit over p = 61..509 is in the scaling table"})
tf = sum(c["fallbacks"] for c in R["dlp"])
tops = sum((sum(r["setup_adds"] + r["walk_adds"] for r in json.load(open(os.path.join(RES, c["file"])))["rho"])) for c in R["dlp"])
PRED.append({"id": "P5", "claim": "level-3 fallback in < 1% of combine operations", "outcome": "confirmed" if tops and tf / tops < 0.01 else "refuted", "evidence": f"{tf} fallbacks in at least {tops} combine calls (rho only counted)"})
unsolved = [(c["p"], c["seed"], k) for c in R["dlp"] for k, i in enumerate(c["ic"]) if not i.get("ok")]
rho_bad = [(c["p"], c["seed"]) for c in R["dlp"] if c["rho_runs_verified"] != c["rho_runs_total"]]
PRED.append({"id": "P6", "claim": "IC and rho solve and verify (x g = h) at every cell", "outcome": "confirmed" if not unsolved and not rho_bad else "refuted", "evidence": f"{len(R['dlp'])} cells; IC unsolved passes {unsolved}; rho cells with unverified runs {rho_bad}"})
rat = [(c["p"], c["seed"], i["ratio"]) for c in R["dlp"] for i in c["ic"] if i.get("ok") and "ratio" in i]
cond = bool(rat) and all(r["w=0"] < 1 and r["w=cal"] < 1 and r["w=0.1"] > 1 and r["w=1"] > 1 for _, _, r in rat)
PRED.append({"id": "P7", "claim": "S_IC/S_rho < 1 at w=0 and w_cal, > 1 at w=0.1 and w=1", "outcome": "confirmed" if cond else "refuted", "evidence": "; ".join(f"p={p} s{sd}: {r['w=0']:.3g}/{r['w=cal']:.3g}/{r['w=0.1']:.3g}/{r['w=1']:.3g}" for p, sd, r in rat)})
aa = []
for c in R["dlp"]:
    ok2 = [i for i in c["ic"] if i.get("ok")]
    if len(ok2) == 2:
        a, b2 = ok2
        r0 = b2["S"]["w=0"] / a["S"]["w=0"]
        rn = (b2["native_p"] + b2["native_ell"]) / (a["native_p"] + a["native_ell"])
        aa.append((c["p"], c["seed"], r0, rn, c.get("rho_aa", {}).get("z")))
good = all(0.7 <= r0 <= 1.3 and 0.7 <= rn <= 1.3 and (z is None or abs(z) < 2) for _, _, r0, rn, z in aa)
PRED.append({"id": "P8", "claim": "A/A within +-30% (IC pass 2 vs 1, GAE and native); rho halves within 2 s.e.", "outcome": "confirmed" if aa and good else "refuted", "evidence": "; ".join(f"p={p} s{sd}: GAE x{r0:.2f}, native x{rn:.2f}, rho z={z if z is None else round(z,2)}" for p, sd, r0, rn, z in aa)})
fs = R.get("fits", {})
npf = fs.get("ic_native_p", {}).get("slope")
gf = fs.get("ic_group_ops", {}).get("slope")
PRED.append({"id": "P9", "claim": "native F_p cost of IC ~ p^[2.0,2.6]; IC group-op count roughly flat in p", "outcome": "confirmed" if npf is not None and 2.0 <= npf <= 2.6 and gf is not None and abs(gf) < 0.5 else "refuted" if npf is not None else "untested", "evidence": f"native_p slope {npf}, group_ops slope {gf}; rho GAE slope {fs.get('rho_gae', {}).get('slope')}"})

json.dump(R, open(os.path.join(HERE, "RESULTS.json"), "w"), indent=1)

# ---------------- RESULTS.md ----------------
L = []
L.append("# RESULTS (all numbers [pcc:unverified]; local development check)\n")
L.append(f"git HEAD at run time `{GIT}`. Generated by `analyze.py` from `results/`.\n")
L.append("## Fibre splitting of the two trigonal pencils (P1)\n")
L.append("| p | curves | mean #points | fraction of fibres with 3 rational points [95% CI over curves] | pooled | hist (0,1,2,3 points) | verdict (1/6 +-25%) |")
L.append("|--:|--:|--:|:--|--:|:--|:--|")
for p, y in sorted(R["yield"].items(), key=lambda kv: int(kv[0])):
    s3 = y["split3_fraction"]
    ok = 0.125 <= y["split3_pooled"] <= 0.2083
    L.append(f"| {p} | {y['curves']} | {y['npts_mean']:.1f} | {s3[0]:.4f} [{s3[1]:.4f}, {s3[2]:.4f}] | {y['split3_pooled']:.4f} | " + ", ".join(f"{h:.3f}" for h in y["hist"]) + f" | {'inside' if ok else 'OUTSIDE'} |")
L.append("")
L.append("## Plane-quintic line yield (A relations: plane through Pa, P1, P2; 3 further rational points all in F) (P2, P3)\n")
L.append("| p | phi | mean m | pairs | hits | yield/pair | naive phi^3/6 | measured/naive | hypergeom. pred. | measured/hyper | unique/hits (pred 0.1) | fibre rows full in F per fibre | pred phi^3/6 |")
L.append("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
for p, y in sorted(R["yield"].items(), key=lambda kv: int(kv[0])):
    for c in y["phi_cells"]:
        fa = c["fibre_rows_full_in_F_frac"]
        L.append(f"| {p} | {c['phi']} | {c['m_mean']:.0f} | {c['pairs_total']} | {c['hits_total']} | {c['yield_pooled']:.4f} | {c['pred_naive_pooled']:.4f} | {c['ratio_naive_pooled']:.2f} | {c['pred_hyper_pooled']:.4f} | {c['ratio_hyper_pooled']:.2f} | {('-' if c['unique_over_hits'] is None else format(c['unique_over_hits'], '.3f'))} | {fa[0]:.4f} | {c['pred_fibre_full_in_F']:.4f} |")
L.append("")
L.append("## DLP cells\n")
L.append("| p | seed | #points | N | ell | IC pass | phi* (A+B) | m | rows/cols | A uniq | C uniq | GAE (group ops) | native F_p | native mod ell | rho GAE mean (min, max; runs) | rho steps/expected |")
L.append("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
for cell in R["dlp"]:
    for pr in cell["ic"]:
        rg = cell.get("rho_gae")
        mm_ = cell.get("rho_gae_minmax")
        rgs = f"{rg[0]:.0f} (min {mm_[0]:.0f}, max {mm_[1]:.0f}; {cell['rho_runs_verified']}/{cell['rho_runs_total']} runs)" if rg else "-"
        if pr.get("ok"):
            L.append(f"| {cell['p']} | {cell['seed']} | {cell['npts']} | {cell['N']} | {cell['ell']} | {pr['pass']} | {pr['phi']}{' +C' if pr['c_supplement'] else ''} | {pr['m']} | {pr['rows']}/{pr['ncols_used']} | {pr['unique_a']} | {pr['unique_c']} | {pr['group_ops']} | {pr['native_p']:.3g} | {pr['native_ell']:.3g} | {rgs} | {cell['rho_steps_over_expected']:.2f} |")
        else:
            L.append(f"| {cell['p']} | {cell['seed']} | {cell['npts']} | {cell['N']} | {cell['ell']} | {pr['pass']} | UNSOLVED | | | | | | | | {rgs} | |")
L.append("")
L.append("## S_IC / S_rho (final rung; plain rho measured; GAE = one combine)\n")
L.append("| p | seed | pass | w=0 | w=w_cal | w=10 w_cal | w=0.1 | w=1 | break-even w | all rungs w=0 | all rungs w=w_cal | free-negation rho assumed, w=0 | if GAE=2.6e3 mulmods, w_cal |")
L.append("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
def g3(v):
    return "-" if v is None else f"{v:.3g}"
for cell in R["dlp"]:
    for pr in cell["ic"]:
        if pr.get("ok") and "ratio" in pr:
            r = pr["ratio"]
            L.append(f"| {cell['p']} | {cell['seed']} | {pr['pass']} | {g3(r['w=0'])} | {g3(r['w=cal'])} | {g3(r['w=10cal'])} | {g3(r['w=0.1'])} | {g3(r['w=1'])} | {g3(pr['break_even_w'])} | {g3(pr['ratio_all_rungs']['w=0'])} | {g3(pr['ratio_all_rungs']['w=cal'])} | {g3(pr['ratio_free_neg']['w=0'])} | {g3(pr['ratio_if_gae_2600']['w=cal'])} |")
L.append("")
L.append("## Calibration (per cell, measured in the same binary under shared-host load)\n")
L.append("| p | seed | combine (us) | mulmods per combine | mulmod mod p (ns) | mulmod mod ell (ns) | w_cal (p) | w_cal (ell) | w_cal (rho hash) | level-3 fallbacks | exceptional keys | key failures |")
L.append("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
for cell in R["dlp"]:
    c = cell["calib"]
    L.append(f"| {cell['p']} | {cell['seed']} | {c['t_add_ns']/1000:.0f} | {c['mulmods_per_add']:.0f} | {c['t_mulmod_p_ns']:.2f} | {c['t_mulmod_ell_ns']:.2f} | {c['w_cal_p']:.2e} | {c['w_cal_ell']:.2e} | {cell['w_cal_hash']:.2e} | {cell['fallbacks']} | {cell['exceptional_keys']} | {cell['keyfail']} |")
L.append("")
if "series_by_p" in R:
    L.append("## Scaling across p (mean over seeds, first IC pass)\n")
    sp = R["series_by_p"]
    L.append("| p | " + " | ".join(k for k in sp if k != "p") + " |")
    L.append("|--:|" + "--:|" * (len(sp) - 1))
    for i, p in enumerate(sp["p"]):
        L.append(f"| {p} | " + " | ".join(f"{sp[k][i]:.4g}" for k in sp if k != "p") + " |")
    L.append("")
    L.append("| quantity | log-log slope vs p | s.e. | points |")
    L.append("|:--|--:|--:|--:|")
    for k, v in R["fits"].items():
        L.append(f"| {k} | {v['slope']:.2f} | {'-' if v['se'] is None else format(v['se'], '.2f')} | {v['n']} |")
L.append("\n## Preregistered predictions\n")
L.append("| id | claim | outcome | evidence |")
L.append("|:--|:--|:--|:--|")
for pr in R["predictions"]:
    L.append(f"| {pr['id']} | {pr['claim']} | **{pr['outcome']}** | {pr['evidence']} |")
if "fits_p_ge_127" in R:
    L.append("")
    L.append("Same fits restricted to p >= 127 (3 values of p):\n")
    L.append("| quantity | slope | s.e. |")
    L.append("|:--|--:|--:|")
    for k, v in R["fits_p_ge_127"].items():
        L.append(f"| {k} | {v['slope']:.2f} | {'-' if v['se'] is None else format(v['se'], '.2f')} |")
if "rho_pooled_steps_over_expected" in R:
    pr_ = R["rho_pooled_steps_over_expected"]
    L.append(f"\nRho steps / sqrt(pi ell / 2), all verified runs pooled: mean {pr_[0]:.3f} [{pr_[1]:.3f}, {pr_[2]:.3f}] over {pr_[3]} runs.")
if "projection" in R:
    pj = R["projection"]
    L.append("")
    L.append("## Break-even price and extrapolation (labelled model/extrapolation, not measurement)\n")
    L.append("| p | w* measured implementation | w* with root-finding model (2000 mulmods per A pair) |")
    L.append("|--:|--:|--:|")
    for i, p in enumerate(pj["p"]):
        L.append(f"| {p} | {g3(pj['w_star_measured_impl'][i])} | {g3(pj['w_star_root_finding_model'][i])} |")
    for k, v in pj.items():
        if k.endswith("_slope") or "_p_at_w" in k:
            L.append(f"- `{k}`: {v}")
L.append("")
open(os.path.join(HERE, "RESULTS.md"), "w").write("\n".join(L))
print("wrote RESULTS.json / RESULTS.md")
