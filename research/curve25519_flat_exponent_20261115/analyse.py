#!/usr/bin/env python3
"""Flat-presentation exponent study: the parent's section-5 estimator on T (counted field_ops) and on W (per-target
wall ms), two-point local slopes, the P1-P6 decisions of PROTOCOL.md, chain/flat ratios against the recorded chain
cells (exponent study + extension), the wall-time regime split at the 2026-11-15 container restart, and the load
flags. Writes analysis.json and analysis.md next to this file. stdlib only."""
import glob, json, math, os, re

H = os.path.dirname(os.path.abspath(__file__))
R = os.path.join(H, "..")
SHA = "5abc70ba47b1192df71a5e93d697f095e59a0e5f0132cf47db1d0d34d34e3746"
RESTART = "2026-11-15T08:00:00Z"   # cells started before this ran in the first container; D = 57 and later in the second
T975 = {1: 12.706, 2: 4.303, 3: 3.182, 4: 2.776, 5: 2.571, 6: 2.447, 7: 2.365, 8: 2.306, 9: 2.262, 10: 2.228,
        11: 2.201, 12: 2.179, 13: 2.160, 14: 2.145, 15: 2.131, 16: 2.120, 17: 2.110, 18: 2.101}


def fit(xs, ys):
    n = len(xs); mx, my = sum(xs) / n, sum(ys) / n; sxx = sum((x - mx) ** 2 for x in xs)
    s = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx; c = my - s * mx
    rss = sum((y - c - s * x) ** 2 for x, y in zip(xs, ys))
    se = math.sqrt(rss / (n - 2) / sxx) if n > 2 else float("nan"); t = T975.get(n - 2, 1.96)
    return {"n": n, "slope": s, "lo": s - t * se, "hi": s + t * se, "intercept": c, "se": se}


# ---- load this study's cells ----
cells = []
for f in sorted(glob.glob(os.path.join(H, "cells", "*.json"))):
    j = json.load(open(f)); e = j["ext"]
    mini = "20" if (j.get("p") == 1048573 or j.get("miniature") == "20") else "24"
    c = {"file": os.path.basename(f), "miniature": mini, "m": j["m"], "D": j["d_or_t"],
         "capped": bool(j.get("capped")), "failure": j.get("failure"), "wall_s": e["wall_s"],
         "started": e["started_utc"], "load_before": e["load_before"], "load_after": e["load_after"],
         "sha_ok": e["bin_sha256"] == SHA, "exit": e["exit"], "seed": j.get("seed"),
         "regime": "first container" if e["started_utc"] < RESTART else "second container"}
    c["load_flag"] = max(e["load_before"], e["load_after"]) > 1.5
    pt = j.get("per_target") or []
    if not c["capped"] and pt:
        c["n_targets"] = len(pt)
        c["T"] = j["mean_field_ops"]
        c["W_ms"] = sum(t["ms"] for t in pt) / len(pt)
        c["W_min_ms"] = min(t["ms"] for t in pt); c["W_max_ms"] = max(t["ms"] for t in pt)
        c["SD"] = max(t["solving_degree_max"] for t in pt)
        c["ns_per_op"] = 1e6 * c["W_ms"] / c["T"]
        c["complete_all"] = all(t.get("complete") for t in pt)
        c["inconsistent_all"] = all(t.get("inconsistent") for t in pt)
        c["max_rows"] = max(t["max_rows"] for t in pt); c["max_cols"] = max(t["max_cols"] for t in pt)
    cells.append(c)

k20 = [c for c in cells if c["miniature"] == "20"]
k24 = [c for c in cells if c["miniature"] != "20"]

# ---- recorded chain cells (frozen binary dc2c30a4) ----
rec = {}
for f in glob.glob(os.path.join(R, "curve25519_exponent_study_20261108/cells/k20_m[34]_sym_d*_g0.json")) + \
         glob.glob(os.path.join(R, "curve25519_exponent_study_ext_20261110/cells/k20_m[34]_sym_d*_g0.json")):
    j = json.load(open(f)); key = (j["m"], j["d_or_t"])
    if not j["capped"] and j.get("per_target"):
        pt = j["per_target"]
        rec[key] = {"T": j["mean_field_ops"], "W_ms": sum(t["ms"] for t in pt) / len(pt),
                    "SD": max(t["solving_degree_max"] for t in pt), "src": os.path.relpath(f, R)}
    elif j["capped"] and key not in rec:
        rec[key] = {"T": None, "censored": True, "src": os.path.relpath(f, R)}

out = {"binary_sha256": SHA, "restart_boundary_utc": RESTART, "cells": cells, "fits": {}, "local_slopes": {},
       "chain_vs_flat": [], "predictions": {}, "controls": {}}

# ---- estimator per (m, unit) ----
def estimator(m, unit, pool, label):
    sel = sorted((c["D"], c[unit]) for c in pool if c["m"] == m and not c["capped"] and c["D"] >= 9 and unit in c
                 and not (unit == "W_ms" and c["load_flag"]))
    res = {"n": len(sel), "cells": [d for d, _ in sel]}
    if len(sel) >= 4 and sel[-1][0] / sel[0][0] >= 4:
        res.update(fit([math.log(d) for d, _ in sel], [math.log(v) for _, v in sel]))
        res["span"] = sel[-1][0] / sel[0][0]; res["applicable"] = True
        res["balanced_total_exponent_derived"] = 2 / (m + 1 - res["slope"])
    else:
        res["applicable"] = False
    out["fits"][label] = res
    return res

fT3 = estimator(3, "T", k20, "m3_T_all")
fW3 = estimator(3, "W_ms", k20, "m3_W_all")
fT4 = estimator(4, "T", k20, "m4_T_all")
fW4 = estimator(4, "W_ms", k20, "m4_W_all")
# wall-time regime sensitivity: first container only (D <= 54) and second container only
fW3_first = estimator(3, "W_ms", [c for c in k20 if c["regime"] == "first container"], "m3_W_first_container")
fT3_first = estimator(3, "T", [c for c in k20 if c["regime"] == "first container"], "m3_T_first_container")

# ---- local slopes ----
for m in (3, 4):
    for unit in ("T", "W_ms"):
        comp = sorted((c["D"], c[unit]) for c in k20 if c["m"] == m and not c["capped"] and unit in c)
        out["local_slopes"][f"m{m}_{unit}"] = [
            {"D_lo": a[0], "D_hi": b[0], "slope": math.log(b[1] / a[1]) / math.log(b[0] / a[0])}
            for a, b in zip(comp, comp[1:])]

# ---- chain / flat ratios ----
for c in sorted(k20, key=lambda c: (c["m"], c["D"])):
    r = rec.get((c["m"], c["D"]))
    row = {"m": c["m"], "D": c["D"], "flat_T": c.get("T"), "flat_SD": c.get("SD"), "flat_W_ms": c.get("W_ms"),
           "flat_censored": c["capped"], "chain_T": r["T"] if r else None, "chain_SD": r.get("SD") if r else None,
           "chain_W_ms": r.get("W_ms") if r else None, "chain_censored": bool(r and r.get("censored")),
           "chain_src": r["src"] if r else None}
    if r and r.get("T") and c.get("T"):
        row["chain_over_flat_T"] = r["T"] / c["T"]; row["chain_over_flat_W"] = r["W_ms"] / c["W_ms"]
    out["chain_vs_flat"].append(row)

# ---- predictions ----
P = out["predictions"]
m3_complete = sorted(c["D"] for c in k20 if c["m"] == 3 and not c["capped"])
m3_censored = sorted(c["D"] for c in k20 if c["m"] == 3 and c["capped"])
m4_complete = sorted(c["D"] for c in k20 if c["m"] == 4 and not c["capped"])
m4_censored = sorted(c["D"] for c in k20 if c["m"] == 4 and c["capped"])

P["P1"] = {"text": "m = 3 estimator applies, s_T in [4.0, 4.8], lower bound > 3; verdict no lever",
           "applicable": fT3["applicable"], "s_T": fT3.get("slope"), "lo": fT3.get("lo"), "hi": fT3.get("hi"),
           "held": bool(fT3["applicable"] and 4.0 <= fT3["slope"] <= 4.8 and fT3["lo"] > 3),
           "lever_m3": bool(fT3["applicable"] and fT3["slope"] < 0 and fT3["hi"] < 0)}
P["P2"] = {"text": "m = 3: s_W >= s_T + 0.5 and s_W in [4.5, 7]",
           "s_W": fW3.get("slope"), "lo": fW3.get("lo"), "hi": fW3.get("hi"), "s_T": fT3.get("slope"),
           "held": bool(fW3["applicable"] and fT3["applicable"] and fW3["slope"] >= fT3["slope"] + 0.5
                        and 4.5 <= fW3["slope"] <= 7),
           "first_container_only": {"s_W": fW3_first.get("slope"), "lo": fW3_first.get("lo"), "hi": fW3_first.get("hi"),
                                    "s_T": fT3_first.get("slope")}}
P["P3"] = {"text": "m = 3 chain reaches D = 57 and censors at or before D = 84", "max_complete": max(m3_complete),
           "first_censored": m3_censored[0] if m3_censored else None,
           "held": bool(max(m3_complete) >= 57 and m3_censored and m3_censored[0] <= 84)}
ls4 = [x for x in out["local_slopes"]["m4_T"] if x["D_lo"] >= 9]
P["P4"] = {"text": "m = 4: 3, 7, 9 complete; 12 completes; censors at or before 28; estimator inapplicable; local slopes (D >= 9) > 2",
           "complete": m4_complete, "first_censored": m4_censored[0] if m4_censored else None,
           "estimator_applicable": fT4["applicable"], "local_slopes_D_ge_9": ls4,
           "held": bool({3, 7, 9, 12} <= set(m4_complete) and m4_censored and m4_censored[0] <= 28
                        and not fT4["applicable"] and all(x["slope"] > 2 for x in ls4))}
# P5: controls
c20_12 = next(c for c in k20 if c["m"] == 3 and c["D"] == 12)
c24_12 = next(c for c in k24 if c["D"] == 12)
def sig4(x): return float(f"{x:.4g}")
ctrl_rows = []
for c in sorted(k24, key=lambda c: c["D"]):
    pred = math.exp(fT3["intercept"] + fT3["slope"] * math.log(c["D"])) if fT3["applicable"] else None
    ctrl_rows.append({"D": c["D"], "T": c["T"], "SD": c["SD"], "W_ms": c["W_ms"], "fitted_k20_T": pred,
                      "ratio_to_fit": (c["T"] / pred) if pred else None, "seed": c["seed"]})
out["controls"] = {"rows": ctrl_rows, "k20_D12_T": c20_12["T"], "k24_D12_T": c24_12["T"],
                   "four_digit_equal": sig4(c20_12["T"]) == sig4(c24_12["T"]),
                   "rel_diff_D12": abs(c20_12["T"] - c24_12["T"]) / c20_12["T"]}
P["P5"] = {"text": "k = 24 controls equal k = 20 at D = 12 to four digits; all within 1.5x of the k = 20 m = 3 law",
           "four_digit_equal": out["controls"]["four_digit_equal"],
           "max_ratio_dev": max(max(r["ratio_to_fit"], 1 / r["ratio_to_fit"]) for r in ctrl_rows) if fT3["applicable"] else None}
P["P5"]["held"] = bool(P["P5"]["four_digit_equal"] and P["P5"]["max_ratio_dev"] is not None and P["P5"]["max_ratio_dev"] <= 1.5)
# P6: sha in every cell and in both launch lines
launch = [l for l in open(os.path.join(H, "run.log")) if l.startswith("binary ")]
P["P6"] = {"text": "binary sha256 matches at launch and in every cell record",
           "launch_lines": len(launch), "launch_ok": all(SHA in l for l in launch) and bool(launch),
           "cells_ok": all(c["sha_ok"] for c in cells)}
P["P6"]["held"] = P["P6"]["launch_ok"] and P["P6"]["cells_ok"]

out["load_flags"] = [c["file"] for c in cells if c["load_flag"]]
out["regimes"] = {r: sorted(c["D"] for c in k20 if c["m"] == 3 and c["regime"] == r) for r in ("first container", "second container")}
out["ns_per_op_m3"] = [{"D": c["D"], "ns_per_op": c["ns_per_op"], "regime": c["regime"]} for c in sorted(k20, key=lambda c: c["D"]) if c["m"] == 3 and "T" in c]

json.dump(out, open(os.path.join(H, "analysis.json"), "w"), indent=1)

# ---- markdown ----
L = []
L.append("## Cells (k = 20, flat presentation, N = 6 targets per cell)\n")
L.append("| m | D | T = mean field_ops | W = mean ms/target | ns per op | SD | wall s | load before/after | container | chain T (recorded) | chain/flat T |")
L.append("|---|---|---|---|---|---|---|---|---|---|---|")
for c in sorted(k20, key=lambda c: (c["m"], c["D"])):
    r = next(x for x in out["chain_vs_flat"] if x["m"] == c["m"] and x["D"] == c["D"])
    if c["capped"]:
        ct = "censored" if r["chain_censored"] else (f"{r['chain_T']:.4g}" if r["chain_T"] else "n/a")
        L.append(f"| {c['m']} | {c['D']} | censored ({c['failure']}) | | | | {c['wall_s']:.0f} | {c['load_before']}/{c['load_after']} | {c['regime']} | {ct} | |")
    else:
        ct = "censored" if r["chain_censored"] else (f"{r['chain_T']:.4g}" if r["chain_T"] else "n/a")
        cr = f"{r['chain_over_flat_T']:.1f}" if r.get("chain_over_flat_T") else ""
        L.append(f"| {c['m']} | {c['D']} | {c['T']:.4g} | {c['W_ms']:.4g} | {c['ns_per_op']:.2f} | {c['SD']} | {c['wall_s']:.0f} | "
                 f"{c['load_before']}/{c['load_after']} | {c['regime']} | {ct} | {cr} |")
L.append("\n## Estimator (parent section 5; OLS of ln unit on ln D, complete cells D >= 9, 95 % t interval)\n")
L.append("| fit | n | D range | slope | 95 % interval | balanced total exponent 2/(m+1-s) (derived) |")
L.append("|---|---|---|---|---|---|")
for k, f in out["fits"].items():
    if f["applicable"]:
        L.append(f"| {k} | {f['n']} | {f['cells'][0]}..{f['cells'][-1]} | {f['slope']:.3f} | [{f['lo']:.3f}, {f['hi']:.3f}] | {f['balanced_total_exponent_derived']:.3f} |")
    else:
        L.append(f"| {k} | {f['n']} | {f['cells']} | not applicable | | |")
L.append("\n## Two-point local slopes\n")
for k, v in out["local_slopes"].items():
    L.append(f"- {k}: " + ", ".join(f"{x['D_lo']}->{x['D_hi']}: {x['slope']:.2f}" for x in v))
L.append("\n## k = 24 controls (p = 16777141, A = 7842; m = 3)\n")
L.append("| D | T | SD | W ms | k = 20 fitted T | ratio |")
L.append("|---|---|---|---|---|---|")
for r in ctrl_rows:
    L.append(f"| {r['D']} | {r['T']:.4g} | {r['SD']} | {r['W_ms']:.4g} | {r['fitted_k20_T']:.4g} | {r['ratio_to_fit']:.3f} |")
L.append(f"\nk = 20 vs k = 24 at D = 12: T = {c20_12['T']:.7g} vs {c24_12['T']:.7g} (relative difference {out['controls']['rel_diff_D12']:.2e}).\n")
L.append("## Predictions\n")
for k, p in P.items():
    L.append(f"- **{k}** ({p['text']}): **{'held' if p['held'] else 'FAILED'}**. " +
             ", ".join(f"{a} = {b}" for a, b in p.items() if a not in ("text", "held", "first_container_only", "local_slopes_D_ge_9")))
L.append(f"\nLoad flags (any 1-minute load > 1.5): {out['load_flags'] or 'none'}.")
L.append(f"Wall-time regimes: first container D = {out['regimes']['first container']}, second container D = {out['regimes']['second container']}.")
open(os.path.join(H, "analysis.md"), "w").write("\n".join(L) + "\n")
print("\n".join(L))
