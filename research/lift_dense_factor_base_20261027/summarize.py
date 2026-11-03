#!/usr/bin/env python3
"""Score a run against PROTOCOL.md §4 and write summary.json.

Usage: python3 summarize.py reports/run-<UTC>/
Reads rows.jsonl (harness), c2pnb_lines.json and c2pnb_delta.json.
Std-lib only. The Mann-Whitney p-value is exact by permutation
(5e4 resamples, fixed seed) and is cross-checked against the normal
approximation.
"""
import json
import math
import random
import statistics as st
import sys
from pathlib import Path

run = Path(sys.argv[1])
rows = [json.loads(line) for line in (run / "rows.jsonl").read_text().splitlines()]
kmax = [r for r in rows if r["kind"] == "kmax"]
lines = [r for r in rows if r["kind"] == "line"]
ym2 = [r for r in rows if r["kind"] == "yield_m2"]


def gbin(n, k):
    g = 1.0
    for i in range(k):
        g *= (2 ** (n - i) - 1) / (2 ** (k - i) - 1)
    return g


def expect(n, k, size):
    """Expected number of k-dim subspaces with all nonzero elements in a
    uniform random size-`size` subset of F_q^*."""
    tot = 2 ** n - 1
    p = gbin(n, k)
    for i in range(2 ** k - 1):
        p *= max(size - i, 0) / (tot - i)
    return p


def mann_whitney_less(x, y, reps=50_000, seed=20261027):
    """One-sided P(U <= observed) for H1: x < y; permutation p and normal p."""
    def u_stat(a, b):
        u = 0.0
        for xi in a:
            for yj in b:
                u += 1.0 if xi < yj else (0.5 if xi == yj else 0.0)
        return u  # large U = x tends to be smaller
    obs = u_stat(x, y)
    pool = list(x) + list(y)
    rng = random.Random(seed)
    ge = 0
    for _ in range(reps):
        rng.shuffle(pool)
        if u_stat(pool[: len(x)], pool[len(x):]) >= obs:
            ge += 1
    n1, n2 = len(x), len(y)
    mu, sd = n1 * n2 / 2, math.sqrt(n1 * n2 * (n1 + n2 + 1) / 12)
    z = (obs - mu) / sd
    p_norm = 0.5 * math.erfc(z / math.sqrt(2))
    return obs, (ge + 1) / (reps + 1), p_norm


S = {"run": run.name}

# ---- H-A / H-B: lines -------------------------------------------------------
ha_fail, hb_fail, cells = [], [], {}
for r in lines:
    key = f"N{r['N']}_d{r['d']}"
    c = cells.setdefault(key, {"N": r["N"], "d": r["d"], "m": r["m"], "curves": {}})
    cur = c["curves"].setdefault(r["curve"], {"Tr_a": r["Tr_a"], "u_count": r["u_count"]})
    q1 = 2 ** r["d"] - 1
    n = r["N"] // r["d"]
    # u-count rule
    uc = r["u_count"]
    ok_u = True
    if n == 2:
        ok_u = uc in (0, q1)
    elif n == 3:
        ok_u = uc in (0, 2 * q1)
    elif n == 4:
        ok_u = uc % q1 == 0
    if not ok_u:
        hb_fail.append((key, r["curve"], "u_count", uc))
    if r.get("u") is None:
        cur["twisted"] = None
        continue
    cur[r["line"]] = {"lifting": r["lifting"], "nonzero": r["line_nonzero"],
                      "yield": r["yield_ordered_mean"], "pred": r["yield_pred"], "targets": r["targets"]}
    if r["line"] == "twisted":
        want = r["line_nonzero"] if r["Tr_a"] == 0 else 0
        if r["lifting"] != want:
            hb_fail.append((key, r["curve"], "lifting", r["lifting"], want))
    if r["fb_points"] > 0:
        se = math.sqrt(max(r["yield_pred"], 1e-12) / r["targets"])
        if abs(r["yield_ordered_mean"] - r["yield_pred"]) > 4 * se:
            ha_fail.append((key, r["curve"], r["line"], r["yield_ordered_mean"], r["yield_pred"], se))
for c in cells.values():
    ratios = []
    for cur in c["curves"].values():
        tw, sd = cur.get("twisted"), cur.get("standard")
        if tw and sd and cur["Tr_a"] == 0 and sd["yield"] > 0:
            ratios.append({"measured": tw["yield"] / sd["yield"], "predicted": tw["pred"] / sd["pred"],
                           "lift_ratio_pow_m": (tw["lifting"] / sd["lifting"]) ** c["m"]})
    c["yield_ratio_tr0"] = ratios
    c["u_exists_fraction"] = sum(1 for cur in c["curves"].values() if cur["u_count"] > 0) / len(c["curves"])
S["lines"] = cells
S["H_A"] = {"failures": ha_fail, "verdict": "pass" if not ha_fail else "FAIL"}
S["H_B"] = {"failures": hb_fail, "verdict": "pass" if not hb_fail else "FAIL"}

# ---- Amendment 1: H-A' with empirical per-target variance --------------------
am = run / "lines_amend1" / "rows.jsonl"
if am.exists():
    A = [json.loads(x) for x in am.read_text().splitlines()]
    A = [r for r in A if r["kind"] == "line"]
    orig = {(r["N"], r["d"], r["curve"], r["line"]): r for r in lines}
    reproduced = all(
        r.get("u") is None or (orig[(r["N"], r["d"], r["curve"], r["line"])]["yield_ordered_mean"] == r["yield_ordered_mean"])
        for r in A)
    fails, pooled = [], {}
    for r in A:
        if r.get("u") is None or r["fb_points"] == 0:
            continue
        t = r["targets"]
        var = max(r["yield_ordered_m2"] - r["yield_ordered_mean"] ** 2, 0.0)
        se = math.sqrt(var / t) if var > 0 else 1e-12
        z = (r["yield_ordered_mean"] - r["yield_pred"]) / se
        r["z_emp"] = z
        if abs(z) > 4:
            fails.append((r["N"], r["d"], r["curve"], r["line"], round(z, 2)))
        key = f"N{r['N']}_d{r['d']}"
        P = pooled.setdefault(key, {"meas": 0.0, "pred": 0.0, "var": 0.0})
        P["meas"] += r["yield_ordered_mean"] * t
        P["pred"] += r["yield_pred"] * t
        P["var"] += var * t
    pool_fail = []
    for key, P in pooled.items():
        P["ratio"] = P["meas"] / P["pred"]
        P["z"] = (P["meas"] - P["pred"]) / math.sqrt(P["var"])
        if abs(P["z"]) > 4:
            pool_fail.append((key, round(P["z"], 2)))
    zs = [r["z_emp"] for r in A if "z_emp" in r]
    S["H_A_prime"] = {"reproduces_confirmatory_means": reproduced, "row_failures": fails,
                      "pooled": pooled, "pooled_failures": pool_fail,
                      "z_mean": st.mean(zs), "z_sd": st.pstdev(zs), "rows": len(zs),
                      "verdict": "pass" if (reproduced and not fails and not pool_fail) else "FAIL"}

# ---- m = 2 yield check on prime n (instrument) -----------------------------
# post-hoc (not a decision rule): sampled targets exclude O and T, while the
# identity k(2k+1)/#E includes the k pairs {P, -P} summing to O; removing
# them gives 2k^2 / (#E - 2).
for r in ym2:
    for arm in [r["dense"]] + r["random"]:
        arm["pred_excl_O"] = 2 * arm["k"] ** 2 / (r["order"] - 2)
        arm["ratio_excl_O"] = arm["mean"] / arm["pred_excl_O"]
S["yield_m2"] = ym2

# ---- kmax / counts ----------------------------------------------------------
byn = {}
for r in kmax:
    byn.setdefault(r["n"], []).append(r)
S["kmax"] = {}
for n, R in sorted(byn.items()):
    out = {}
    for arm in ("curve", "null", "planted"):
        A = [r for r in R if r["arm"] == arm]
        out[arm] = {
            "k_exact": [r["k_exact"] for r in A],
            "k_budget": [r["k_budget"] for r in A],
            "struct_k": [r["struct_k"] for r in A] if arm == "curve" else None,
        }
        if arm == "planted":
            out[arm]["planted_k"] = A[0]["planted_k"]
            out[arm]["budget_detect"] = sum(r["k_budget"] >= r["planted_k"] for r in A) / len(A)
            if A[0]["k_exact"] is not None:
                out[arm]["exact_detect"] = sum(r["k_exact"] >= r["planted_k"] for r in A) / len(A)
        # density above kmax: best rho at kmax+1..+6, and implied m=3 gain
        out[arm]["rho_star_first"] = [float(r["rho_star"][0]) if r["rho_star"] else None for r in A]
        if A and A[0]["subspace_counts"]:
            ratios = {}
            for k in range(2, 6):
                ratios[k] = [r["subspace_counts"][k] / expect(n, k, r["subspace_counts"][1])
                             if len(r["subspace_counts"]) > k else 0.0 for r in A]
            out[arm]["count_ratio"] = ratios
    curves_kb = out["curve"]["k_budget"]
    nulls_kb = out["null"]["k_budget"]
    out["kb_median_diff"] = st.median(curves_kb) - st.median(nulls_kb)
    S["kmax"][n] = out

# H-C
hc = {}
for n in sorted(S["kmax"]):
    o = S["kmax"][n]
    if "count_ratio" not in o["curve"]:
        continue
    x = o["curve"]["count_ratio"][4]
    y = o["null"]["count_ratio"][4]
    u, p_perm, p_norm = mann_whitney_less(x, y)
    hc[n] = {"n_curves": len(x), "n_nulls": len(y), "median_curve": st.median(x), "median_null": st.median(y),
             "mean_curve": st.mean(x), "mean_null": st.mean(y),
             "sd_curve": st.pstdev(x), "sd_null": st.pstdev(y),
             "U": u, "p_perm": p_perm, "p_norm": p_norm}
S["H_C"] = hc
if 11 in hc:
    S["H_C_verdict_N11"] = "repulsion confirmed" if hc[11]["p_perm"] < 0.01 else "not confirmed"

# ---- c2pnb ------------------------------------------------------------------
c2 = run / "c2pnb_lines.json"
if c2.exists():
    L = json.loads(c2.read_text())
    for r in L:
        tw = r.get("sampled_lift_twisted", [None, None])
        if r["Tr_a"] == 0 and r["u_all_lift"] > 0 and tw[0] == tw[1]:
            r["verdict"] = "applicable"
        elif r["Tr_a"] == 1 and tw[0] == 0:
            r["verdict"] = "inapplicable (trace obstruction: line lies on the twist)"
        else:
            r["verdict"] = "UNEXPECTED"
    S["c2pnb_lines"] = L
dl = run / "c2pnb_delta.json"
if dl.exists():
    D = json.loads(dl.read_text())
    S["c2pnb_delta_label"] = "cost-model output (upstream C1 model, H0-H3), not a measurement"
    S["c2pnb_delta"] = [r for r in D if r["metric"] in ("time", "TxM-a-p0", "mesh-a", "MM1-a")]

(run / "summary.json").write_text(json.dumps(S, indent=1, default=str))
print("H_A", S["H_A"]["verdict"], len(ha_fail), "| H_B", S["H_B"]["verdict"], len(hb_fail))
if "H_A_prime" in S:
    h = S["H_A_prime"]
    print("H_A'", h["verdict"], "reproduced", h["reproduces_confirmatory_means"], "row fails", h["row_failures"],
          "pooled fails", h["pooled_failures"], "z mean %.2f sd %.2f over %d rows" % (h["z_mean"], h["z_sd"], h["rows"]))
for n, h in hc.items():
    print("H_C n=%d curves %.3f±%.3f nulls %.3f±%.3f p_perm %.2g p_norm %.2g" % (
        n, h["mean_curve"], h["sd_curve"], h["mean_null"], h["sd_null"], h["p_perm"], h["p_norm"]))
for n, o in S["kmax"].items():
    pl = o["planted"]
    print("n=%d kb curve %s null %s | planted k=%s budget_detect %.2f exact_detect %s | struct %s" % (
        n, sorted(o["curve"]["k_budget"]), sorted(o["null"]["k_budget"]), pl["planted_k"], pl["budget_detect"],
        pl.get("exact_detect"), o["curve"]["struct_k"]))
