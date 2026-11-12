#!/usr/bin/env python3
"""Extension analysis: parent section-5 estimator on the union of parent and extension cells,
and the decisions on predictions P1-P6 of this study's PROTOCOL.md.

Union rule (PROTOCOL "Cells and order"): an extension cell replaces the parent cell with the same
(k, m, form, gidx, d); parent cells the extension did not rerun are kept as they are.
"""
import glob, json, math, os
H = os.path.dirname(os.path.abspath(__file__))
D = os.path.join(H, "..")
PARENT = os.path.join(D, "..", "curve25519_exponent_study_20261108")
GROUPS = {(20, 3, "sym", 0), (20, 4, "sym", 0), (22, 3, "sym", 0)}
T975 = {1: 12.706, 2: 4.303, 3: 3.182, 4: 2.776, 5: 2.571, 6: 2.447, 7: 2.365, 8: 2.306, 9: 2.262,
        10: 2.228, 11: 2.201, 12: 2.179, 13: 2.160, 14: 2.145}


def fit(xs, ys):
    n = len(xs)
    mx, my = sum(xs) / n, sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    s = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx
    c = my - s * mx
    rss = sum((y - c - s * x) ** 2 for x, y in zip(xs, ys))
    se = math.sqrt(rss / (n - 2) / sxx)
    t = T975.get(n - 2, 1.96)
    return {"n": n, "slope": s, "lo95": s - t * se, "hi95": s + t * se, "se": se}


def load(dirname, origin):
    out = {}
    for f in sorted(glob.glob(os.path.join(dirname, "cells", "*.json"))):
        j = json.load(open(f))
        key = (j["k"], j["m"], j["form"], j["gidx"])
        if key not in GROUPS:
            continue
        j["_origin"] = origin
        out[key + (j["d_or_t"],)] = j
    return out


cells = load(PARENT, "parent")
ext = load(D, "ext")
replaced = sorted(k for k in ext if k in cells)
cells.update(ext)

out = {"groups": {}, "replaced_parent_cells": [list(k) for k in replaced]}
for key in sorted(GROUPS):
    cs = sorted((j for k, j in cells.items() if k[:4] == key), key=lambda j: j["d_or_t"])
    rows = []
    for j in cs:
        rows.append({"d": j["d_or_t"], "v_size": j.get("v_size") or j["d_or_t"], "capped": j["capped"],
                     "T": j.get("mean_field_ops"), "done": j.get("targets_done"), "origin": j["_origin"],
                     "failure": j.get("failure"), "wall_s": (j.get("ext") or {}).get("wall_s"),
                     "first_target_x": ((j.get("per_target") or [{}])[0]).get("target_x")})
    ok = [r for r in rows if not r["capped"] and r["T"]]
    g = {"rows": rows}
    for name, lo in (("fit_d_ge_9", 9), ("fit_all", 0)):
        sel = [r for r in ok if r["d"] >= lo]
        span = sel[-1]["v_size"] / sel[0]["v_size"] if sel else 0
        if len(sel) >= 4 and span >= 4:
            g[name] = fit([math.log(r["v_size"]) for r in sel], [math.log(r["T"]) for r in sel])
            g[name]["span"] = span
        else:
            g[name] = {"applicable": False, "n": len(sel), "span": span}
    g["local_slopes"] = [{"d": [a["d"], b["d"]],
                          "slope": math.log(b["T"] / a["T"]) / math.log(b["v_size"] / a["v_size"])}
                         for a, b in zip(ok, ok[1:])]
    out["groups"][f"k{key[0]}_m{key[1]}_{key[2]}_g{key[3]}"] = g

# ---- predictions ----------------------------------------------------------------------------
G = out["groups"]
def row(g, d):
    return next((r for r in G[g]["rows"] if r["d"] == d), None)
P = {}
r57 = row("k20_m3_sym_g0", 57)
P["P1"] = {"claim": "k20 m3 d57 censors again at 10,800 s",
           "outcome": "not testable" if r57 and r57["origin"] == "parent" else
           ("pass" if r57 and r57["capped"] else "fail"),
           "basis": "chain k20 m3 stopped at its first extension-censored cell (PROTOCOL chain rule); "
                    "d57 was not rerun" if r57 and r57["origin"] == "parent" else "extension cell"}
even = [row("k20_m3_sym_g0", d) for d in (12, 14, 18, 28, 36, 38, 42, 54)]
even_done = [r for r in even if r and not r["capped"]]
f = G["k20_m3_sym_g0"]["fit_d_ge_9"]
app = f.get("applicable", True)
P["P2"] = {"claim": ">=4 even m=3 cells complete; estimator applies to k20 m3; s>3 with lower bound>0",
           "even_complete": [r["d"] for r in even_done], "estimator": f,
           "outcome": "pass" if (len(even_done) >= 4 and app and f["slope"] > 3 and f["lo95"] > 0) else "fail",
           "basis": ("estimator applicable" if app else
                     f"estimator not applicable: {f['n']} complete cells with d>=9 span {f['span']:.2f}x < 4x")}
r19 = row("k20_m4_sym_g0", 19)
P["P3"] = {"claim": "k20 m4 d19 censors under 12 GB / 10,800 s",
           "outcome": "pass" if r19 and r19["origin"] == "ext" and r19["capped"] else "fail",
           "cell": r19}
m4 = G["k20_m4_sym_g0"]
m4_done = [r["d"] for r in m4["rows"] if not r["capped"]]
m4_loc = [l for l in m4["local_slopes"] if l["d"][0] >= 9]
P["P4"] = {"claim": "m4 d12,14,18 complete; all two-point slopes among complete m4 cells with d>=9 above 2",
           "m4_complete": m4_done, "local_slopes_d_ge_9": m4_loc,
           "outcome": "pass" if all(d in m4_done for d in (12, 14, 18)) and m4_loc and all(l["slope"] > 2 for l in m4_loc) else "fail",
           "basis": "d12 censored (memory) so the chain stopped; d14, d18 not attempted" if 12 not in m4_done else ""}
r55 = row("k22_m3_sym_g0", 55)
P["P5"] = {"claim": "k22 m3 d55 censors again (memory)",
           "outcome": ("pending" if not r55 or r55["origin"] == "parent" else ("pass" if r55["capped"] else "fail")),
           "cell": r55}
# P6: binary hash at launch + first target_x of rerun cells
want = [ln.split()[0] for ln in open(os.path.join(D, "data", "frozen.sha256")) if ln.strip().endswith("curve25519_family")][0]
log = open(os.path.join(D, "data", "run.log")).read()
sha_ok = f"binary ok {want[:16]}" in log
ext_shas = {(j.get("ext") or {}).get("bin_sha256") for j in ext.values()}
rerun_cmp = []
for k in replaced:
    pj, ej = load(PARENT, "parent")[k], ext[k]
    px = ((pj.get("per_target") or [{}])[0]).get("target_x")
    ex = ((ej.get("per_target") or [{}])[0]).get("target_x")
    rerun_cmp.append({"cell": list(k), "parent_target_x": px, "ext_target_x": ex,
                      "comparable": px is not None and ex is not None, "equal": px == ex if px is not None and ex is not None else None})
P["P6"] = {"claim": "binary sha256 matches frozen.sha256 at launch; rerun cells' first target_x equals parent's (610025 for k20 m3 d57)",
           "sha_ok_in_log": sha_ok, "cell_bin_sha256": sorted(s for s in ext_shas if s), "frozen": want,
           "rerun_target_x": rerun_cmp,
           "outcome": ("pass" if sha_ok and ext_shas == {want} and all(c["equal"] for c in rerun_cmp if c["comparable"]) else "fail")
                      + ("; d57 part not testable (not rerun)" if r57 and r57["origin"] == "parent" else "")
                      + ("; rerun cells recorded no target_x (engine aborted before the first target finished)" if rerun_cmp and not any(c["comparable"] for c in rerun_cmp) else "")}
out["predictions"] = P
out["all_done"] = "ALL_DONE" in log
json.dump(out, open(os.path.join(D, "data", "analysis.json"), "w"), indent=1)
for k, g in G.items():
    print(k, "fit_d_ge_9:", g["fit_d_ge_9"])
    print("   local:", [(l["d"], round(l["slope"], 2)) for l in g["local_slopes"]])
for k, p in P.items():
    print(k, p["outcome"], "|", p.get("basis", ""))
print("ALL_DONE" if out["all_done"] else "run not finished")
