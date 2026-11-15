#!/usr/bin/env python3
"""Fits, decisions, tables, figure.  usage: analyze.py  (reads raw/*.jsonl, writes results.json, tables.md, fig_*)"""
import json, glob, math, random, sys
from collections import defaultdict
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

rows, mins = [], []
for fn in sorted(glob.glob("raw/*.jsonl")):
    for ln in open(fn):
        r = json.loads(ln)
        (mins if "best_clauses" in r else rows).append(r)

PRIMARY_N = 17
SECONDARY = {3: 11, 4: 13, 5: 17, 6: 19, 7: 23}


def metric(r, name):
    a = r["arm"]
    if name == "size":
        if a in ("cold", "warm", "free", "xproof"):
            return r["final"]["clauses"] if r.get("checked") else None
        if a == "xor":
            return r["translation_clauses"]
    if name == "conflicts" and a in ("cold", "warm", "free"):
        return r["conflicts"] if not r.get("censored") else None
    if name == "raw" and a in ("cold", "warm", "free"):
        return r["raw"]["clauses"]
    if name == "raw" and a == "xproof":
        return r["raw_clauses"]
    if name == "lits" and a in ("cold", "warm", "free") and r.get("checked"):
        return r["final"]["lits"]
    if name == "steps" and a == "xor":
        return r["steps"]
    if name == "leaf_steps" and a == "xor":
        return r["steps"] - (r["root"]["rowops"] + r["root"]["props"] + r["root"]["nodes"])
    if name == "leaf_translation" and a == "xor":
        rt = r["root"]
        return r["translation_clauses"] - (8 * rt["wsum"] + rt["rowops"] + 2 * rt["props"] + rt["nodes"])
    return None


def cells(arm, name, series):
    d = defaultdict(list)
    for r in rows:
        if r["arm"] != arm:
            continue
        if series == "primary" and r["n"] != PRIMARY_N:
            continue
        if series == "secondary" and SECONDARY.get(r["l"]) != r["n"]:
            continue
        if series == "fixed13" and r["n"] != 13:
            continue
        v = metric(r, name)
        if v:
            d[r["l"]].append(math.log2(v))
    return dict(d)


def slope(d):
    xs = [l for l, v in d.items() for _ in v]
    ys = [y for v in d.values() for y in v]
    return float(np.polyfit(xs, ys, 1)[0])


def fit(arm, name, series, lmin=None, lmax=None, need=6):
    d = {l: v for l, v in cells(arm, name, series).items()
         if (lmin is None or l >= lmin) and (lmax is None or l <= lmax)}
    full = {l: v for l, v in d.items() if len(v) >= need}
    if len(full) < 3:
        return {"arm": arm, "metric": name, "series": series, "sizes": sorted(d), "complete_sizes": sorted(full),
                "slope": None, "note": "fewer than 3 complete sizes"}
    rng = random.Random(1)
    bs = []
    for _ in range(2000):
        bs.append(slope({l: [rng.choice(v) for _ in v] for l, v in full.items()}))
    bs.sort()
    return {"arm": arm, "metric": name, "series": series, "sizes": sorted(full), "rows": sum(map(len, full.values())),
            "slope": round(slope(full), 3), "ci95": [round(bs[49], 3), round(bs[1949], 3)],
            "geomean": {l: round(2 ** (sum(v) / len(v)), 1) for l, v in sorted(full.items())}}


FITS = []
for series in ("primary", "fixed13", "secondary"):
    for arm, names in (("cold", ("size", "conflicts", "raw", "lits")), ("warm", ("size", "conflicts", "raw", "lits")),
                       ("free", ("size", "conflicts")), ("xproof", ("size", "raw")),
                       ("xor", ("steps", "size", "leaf_steps", "leaf_translation"))):
        for nm in names:
            FITS.append(fit(arm, nm, series))
FITS.append(dict(fit("xproof", "size", "primary", 2, 4), note="same l range as cold/warm"))
FITS.append(dict(fit("xproof", "size", "primary", 3, 6), note="without l = 2"))
FITS.append(dict(fit("xor", "leaf_translation", "primary", 3, 6), note="without l = 2"))


def get(arm, name, series):
    return next(f for f in FITS if f["arm"] == arm and f["metric"] == name and f["series"] == series and "note" not in f or
                (f["arm"] == arm and f["metric"] == name and f["series"] == series and f.get("slope") is None))


# ---- checking status
status = defaultdict(lambda: defaultdict(int))
for r in rows:
    status[r["arm"]][r.get("status")] += 1
fwd = defaultdict(lambda: defaultdict(int))
for r in rows:
    if r["arm"] in ("cold", "warm", "free"):
        fwd[r["arm"]][str(r.get("forward_check"))] += 1

# ---- decisions
dec = {}
fc, fw, fx = get("cold", "size", "primary"), get("warm", "size", "primary"), get("xproof", "size", "primary")
allv = lambda arm: all(r.get("status") == "VERIFIED" for r in rows if r["arm"] == arm and r["n"] == PRIMARY_N and not r.get("censored"))
cands = [f for f in (fc, fw) if f.get("slope") is not None]
if cands:
    b = min(cands, key=lambda f: f["geomean"][max(f["sizes"])])
    dec["step2_rup_only"] = {"arm": b["arm"], "slope": b["slope"], "ci95": b["ci95"], "sizes": b["sizes"],
                             "all_verified": allv("cold") and allv("warm"),
                             "decision": "short proofs exist" if b["slope"] <= 1.5 and allv("cold") and allv("warm") else "not shown"}
else:
    dec["step2_rup_only"] = {"decision": "not shown", "why": "fewer than 3 complete sizes"}
if fx.get("slope") is not None:
    dec["step2_rup_plus_definitions"] = {"arm": "xproof", "slope": fx["slope"], "ci95": fx["ci95"], "sizes": fx["sizes"],
                                         "all_verified": allv("xproof"),
                                         "defs_syntax_all_ok": all(r.get("defs_syntax_ok") for r in rows if r["arm"] == "xproof"),
                                         "decision": "short proofs exist" if fx["slope"] <= 1.5 and allv("xproof") else "not shown"}
for arm in ("cold", "warm", "xproof", "free"):
    f = get(arm, "size", "fixed13")
    if f.get("slope") is not None:
        v = all(r.get("status") == "VERIFIED" for r in rows if r["arm"] == arm and r["n"] == 13)
        dec["posthoc_fixed13_" + arm] = {"slope": f["slope"], "ci95": f["ci95"], "sizes": f["sizes"], "all_verified": v,
                                         "rule_value": ("<= 1.5" if f["slope"] <= 1.5 else "> 1.5")}
# partial (17, 4) cell of the RUP-only arms: two-point step from l = 3
part = {}
for arm in ("cold", "warm", "free"):
    r4 = [r for r in rows if r["arm"] == arm and r["n"] == 17 and r["l"] == 4 and r.get("checked")]
    r3 = [r for r in rows if r["arm"] == arm and r["n"] == 17 and r["l"] == 3 and r.get("checked") and r4 and r["target"] == r4[0]["target"]]
    g3 = [math.log2(r["final"]["clauses"]) for r in rows if r["arm"] == arm and r["n"] == 17 and r["l"] == 3 and r.get("checked")]
    if r4:
        part[arm] = {"targets_at_l4": len(r4), "size_l4": r4[0]["final"]["clauses"], "conflicts_l4": r4[0]["conflicts"],
                     "geomean_l3": round(2 ** (sum(g3) / len(g3)), 1),
                     "bits_from_l3_geomean": round(math.log2(r4[0]["final"]["clauses"]) - sum(g3) / len(g3), 2)}
dec["partial_17_4_rup_only"] = part
# step 3 at the largest l with 6 complete cold/warm targets
def share_for(N):
  share = {}
  for l in sorted({r["l"] for r in rows if r["n"] == N}):
      per = {}
      for r in rows:
          if r["n"] == N and r["l"] == l and r["arm"] in ("cold", "warm") and r.get("checked") and r.get("sharing"):
              k = r["target"]
              if k not in per or r["final"]["clauses"] < per[k]["final"]["clauses"]:
                  per[k] = r
      for m in mins:
          if m["n"] == N and m["l"] == l:
              k = m["target"]
              if k not in per or m["best_clauses"] < per[k]["final"]["clauses"]:
                  per[k] = {"final": {"clauses": m["best_clauses"]}, "sharing": m["sharing"], "arm": "min3:" + m["best_label"], "target": k}
      if len(per) == 6:
          med = sorted(p["sharing"]["median"] for p in per.values())
          share[l] = {"per_target_median": [per[k]["sharing"]["median"] for k in sorted(per)],
                      "per_target_mean": [per[k]["sharing"]["mean"] for k in sorted(per)],
                      "per_target_share_ge4": [per[k]["sharing"]["share_ge4"] for k in sorted(per)],
                      "per_target_source": [per[k]["arm"] for k in sorted(per)],
                      "per_target_size": [per[k]["final"]["clauses"] for k in sorted(per)],
                      "median_of_medians": (med[2] + med[3]) / 2}
  return share


share = share_for(PRIMARY_N)
share13 = share_for(13)
if share:
    L = max(share)
    dec["step3"] = {"l": L, "leaves": 1 << L, "median_of_medians": share[L]["median_of_medians"],
                    "decision": "cross-leaf sharing" if share[L]["median_of_medians"] >= 4 else "no cross-leaf sharing shown"}
if 4 in share13:
    dec["step3_posthoc_n13_l4"] = {"l": 4, "leaves": 16, "median_of_medians": share13[4]["median_of_medians"],
                                   "rule_value": ">= 4" if share13[4]["median_of_medians"] >= 4 else "< 4"}
min_sum = {}
for (nn, l) in sorted({(m["n"], m["l"]) for m in mins}):
    ms = [m for m in mins if m["l"] == l and m["n"] == nn]
    g = lambda k: round(2 ** (sum(math.log2(m[k]) for m in ms) / len(ms)), 1)
    min_sum["n%d_l%d" % (nn, l)] = {"targets": len(ms), "geomean_best": g("best_clauses"), "geomean_noshare_reference": g("noshare_reference"),
                  "ratio_best_to_reference": [m["ratio_best_to_reference"] for m in ms],
                  "best_labels": [m["best_label"] for m in ms],
                  "best_leaf_equivalents": [round(m["ratio_best_to_reference"] * m["leaves"], 2) for m in ms],
                  "sharing_median": [m["sharing"]["median"] for m in ms],
                  "sharing_mean": [m["sharing"]["mean"] for m in ms],
                  "sharing_share_ge4": [m["sharing"]["share_ge4"] for m in ms],
                  "sharing_share_all_leaves": [m["sharing"]["share_all_leaves"] for m in ms],
                  "all_forward_checked": all(m["best_forward_check"] for m in ms)}

# descriptive two-variable fit for xproof over every complete cell (post hoc): log2 size = a + b l + c log2 n
pts = [(r["l"], math.log2(r["n"]), math.log2(r["final"]["clauses"])) for r in rows if r["arm"] == "xproof" and r.get("checked") and r["l"] <= 5]
A = np.array([[1, p[0], p[1]] for p in pts]); y = np.array([p[2] for p in pts])
coef, res_, _, _ = np.linalg.lstsq(A, y, rcond=None)
rng = random.Random(1); bsb = []; bsc = []
for _ in range(2000):
    idx = [rng.randrange(len(pts)) for _ in pts]
    cf = np.linalg.lstsq(A[idx], y[idx], rcond=None)[0]; bsb.append(cf[1]); bsc.append(cf[2])
bsb.sort(); bsc.sort()
dec["xproof_two_variable_fit_posthoc"] = {"model": "log2(size) = a + b*l + c*log2(n)", "rows": len(pts), "b_bits_per_l": round(float(coef[1]), 3),
    "b_ci95": [round(bsb[49], 3), round(bsb[1949], 3)], "c_power_of_n": round(float(coef[2]), 3), "c_ci95": [round(bsc[49], 3), round(bsc[1949], 3)],
    "max_abs_residual_bits": round(float(np.max(np.abs(A @ coef - y))), 3)}
x6 = [r for r in rows if r["arm"] == "xproof" and r["l"] == 6 and r.get("checked")]
g5 = [math.log2(r["final"]["clauses"]) for r in rows if r["arm"] == "xproof" and r["l"] == 5 and r["n"] == 17]
if x6:
    dec["xproof_partial_17_6"] = {"targets": len(x6), "size": x6[0]["final"]["clauses"], "bits_from_l5_geomean": round(math.log2(x6[0]["final"]["clauses"]) - sum(g5) / len(g5), 2), "seconds": x6[0]["solve_s"]}
REF = {"pilot01_cdcl_found_conflicts": {"slope": 3.34, "ci95": [3.24, 3.42], "n": 17, "l": "2..5", "source": "../../sat_smt_leads_20261114/01-proof-size-slope/README.md"},
       "pilot01_cdcl_found_trimmed_core": {"slope": 3.27, "ci95": [3.14, 3.38], "n": 17, "l": "2..5",
                                           "geomean": {2: 296, 3: 1802, 4: 22063, 5: 246904}},
       "scan_count_2^l": {"slope": 1.0}}
json.dump({"standing": "[pcc:unverified] local pilot", "fits": FITS, "decisions": dec, "sharing_by_l": share, "sharing_by_l_n13": share13,
           "step3_minimisation": min_sum, "status_counts": {a: dict(v) for a, v in status.items()},
           "forward_check_counts": {a: dict(v) for a, v in fwd.items()}, "references": REF,
           "rows": rows, "min_rows": mins}, open("results.json", "w"), indent=1)

# ---- tables
T = ["# Tables (generated by analyze.py) [pcc:unverified]", "", "## Slopes, bits per l", "",
     "| series | arm | metric | sizes l | rows | slope | 95 % interval | geometric means | note |", "|---|---|---|---|---|---|---|---|---|"]
for f in FITS:
    if f.get("slope") is None:
        T.append("| %s | %s | %s | %s | | not fitted | | | %s |" % (f["series"], f["arm"], f["metric"], f["complete_sizes"], f["note"]))
    else:
        T.append("| %s | %s | %s | %s | %d | %.2f | %.2f..%.2f | %s | %s |" % (
            f["series"], f["arm"], f["metric"], "%d..%d" % (f["sizes"][0], f["sizes"][-1]), f["rows"], f["slope"],
            f["ci95"][0], f["ci95"][1], ", ".join("%s: %s" % kv for kv in f["geomean"].items()), f.get("note", "")))
T += ["", "## Check status per arm", ""] + ["- %s: %s; forward check: %s" % (a, dict(v), dict(fwd.get(a, {}))) for a, v in status.items()]
T += ["", "## Decisions", "", "```", json.dumps(dec, indent=1), "```", "", "## Sharing by l (smallest verified cold/warm/minimised proof per target)", "",
      "```", json.dumps({"n17": share, "n13": share13}, indent=1), "```", "", "## Step 3 minimisation", "", "```", json.dumps(min_sum, indent=1), "```"]
open("tables.md", "w").write("\n".join(T) + "\n")

# ---- figure
COL = {"cold": "#0072B2", "warm": "#56B4E9", "free": "#D55E00", "xproof": "#009E73", "xor": "#CC79A7"}
LAB = {"cold": "scan-guided, CDCL per leaf, fresh solver (RUP, checked)", "warm": "scan-guided, CDCL per leaf, one solver (RUP, checked)",
       "free": "CDCL-found, no guidance, same CNF (checked)", "xproof": "scan certificate, RUP + definitions (checked)",
       "xor": "XOR engine, translation count (unchecked)"}
fig, axs = plt.subplots(1, 3, figsize=(19, 6), sharey=True)
for ax, series, title in ((axs[0], "primary", "Primary: fixed n = 17 (CDCL arms: 1 target at l = 4)"), (axs[1], "fixed13", "Fixed n = 13 (added after the n = 17 cost was seen)"), (axs[2], "secondary", "n grows with l: (l, n) = (3, 11), (4, 13), (5, 17)")):
    for arm in ("free", "cold", "warm", "xproof", "xor"):
        d = cells(arm, "size", series)
        if not d:
            continue
        ls = sorted(d)
        for l in ls:
            ax.scatter([l] * len(d[l]), d[l], s=10, color=COL[arm], alpha=0.35, linewidths=0)
        f = next((f for f in FITS if f["arm"] == arm and f["metric"] == "size" and f["series"] == series and "note" not in f), None)
        lab = LAB[arm] + (": %.2f bits/l" % f["slope"] if f and f.get("slope") is not None else "")
        ax.plot(ls, [sum(d[l]) / len(d[l]) for l in ls], "-o", color=COL[arm], lw=2, ms=5, label=lab)
    if series == "primary":
        g = REF["pilot01_cdcl_found_trimmed_core"]["geomean"]
        ax.plot(list(g), [math.log2(v) for v in g.values()], "--s", color="#555555", lw=1.5, ms=4,
                label="pilot 01 stored: CDCL-found trimmed core, other CNF: 3.27 bits/l")
    xs = np.array([2, 6]) if series == "primary" else (np.array([2, 4]) if series == "fixed13" else np.array([3, 5]))
    ax.plot(xs, xs + 6, ":", color="#000000", lw=1.2, label="reference 2^l (times 64): 1 bit/l")
    ax.set_title(title, fontsize=10)
    ax.set_xlabel("l = dimension of the factor-base subspace")
    ax.set_xticks(range(2, 7))
    ax.grid(True, color="#dddddd", lw=0.6)
    ax.legend(fontsize=7.2, loc="upper left", frameon=False)
axs[0].set_ylabel("log2(proof clauses after trimming), per target and geometric mean")
fig.suptitle("Size of refutations of non-decomposable m = 3 targets by proof kind  [pcc:unverified, local pilot]", fontsize=11)
fig.tight_layout()
fig.savefig("fig_proof_size_vs_l.svg")
fig.savefig("fig_proof_size_vs_l.png", dpi=150)
print(open("tables.md").read()[:6000])
