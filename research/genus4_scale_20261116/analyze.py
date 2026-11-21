#!/usr/bin/env python3
"""Readings Q1-Q4 and Qcal of PROTOCOL.md section 5 plus ANALYSIS_ADDENDUM.md
(window-free plain references P0f, P0u). Every number is [pcc:unverified].

Reference arms per prime and reading:
  R   negation rho, measured on every target;
  P   plain rho (window), measured on target 1 of each curve (seeds 1-2);
  P0f P with compares 0, canonicalisation floored (exact: same walk, B13);
  P0u P with compares 0 and canonicalisation 0.
P, P0f and P0u reach all targets by the per-curve ratio to R on target 1
(protocol section 3). The frozen verdict uses min(R, P); the addendum verdict
uses min(R, P, P0f); P0u is a sensitivity. Bootstrap over targets then seeds;
per-curve ratios held fixed (disclosed in RESULTS).
"""
import json
import os
import random

HERE = os.path.dirname(os.path.abspath(__file__))
REFS = os.path.join(HERE, "..", "genus4_rho_refs_20261116", "run.jsonl")
NB, SEED = 2000, 20261120
WS = [("0", 0.0), ("0.1", 0.1), ("1", 1.0)]
READ = ["Q1", "Q2", "Q3", "Q4"]
WARM = "C_double_lp+pre+coord+unitT"


def load(path):
    return [json.loads(l) for l in open(path) if l.strip()] if os.path.exists(path) else []


def key(r):
    return (r["p"], r["curve_seed"], r["target_seed"], r["seed"])


rows_new = []
for f in ["p8209.jsonl", "p16411a.jsonl", "p16411b.jsonl", "p16411a_resume.jsonl",
          "p8209_plain.jsonl", "p16411_plain.jsonl"]:
    rows_new += load(os.path.join(HERE, f))
rows_refs = load(REFS)


def arm(rows, name):
    d = {}
    for r in rows:
        if r.get("kind") == "run" and r.get("g") == 4 and r["arm"] == name:
            d[key(r)] = r
    return d


W, R, P = arm(rows_new, WARM), arm(rows_new, "R_rho_neg"), arm(rows_new, "R_rho_plain")
Wr, Rr, Pr = arm(rows_refs, WARM), arm(rows_refs, "R_rho_neg"), arm(rows_refs, "R_rho_plain")
cal = [r for r in rows_new if r.get("kind") == "calibration"]
inst = [r for r in rows_new if r.get("kind") == "instance"]
wm_p = sum(c["w_cal_mul_p"] for c in cal) / len(cal)
wa_p = sum(c["w_cal_add_p"] for c in cal) / len(cal)
wm_r = sum(c["w_cal_mul_r"] for c in cal) / len(cal)

gates = {"G1": [], "G2": []}
for d in (W, R, P):
    for k, r in d.items():
        if not (r.get("certified") and r.get("ok")):
            gates["G1"].append(f"{r['arm']} {k}")
for r in inst:
    if r.get("h", 99) > 2:
        gates["G2"].append(f"h {r['curve_seed']}")
    if r["p"] == 8209 and r["curve_seed"] in (363, 393, 407):
        gates["G2"].append(f"reused {r['curve_seed']}")
gate_ok = {g: not v for g, v in gates.items()}


def ic_cost(r, q, w):
    G, M, A = r["group_ops"], r["rel_mul"] + r["la_mul"], r["rel_add"] + r["la_add"]
    if q == "Q1":
        return G + w * M
    if q == "Qcal":
        return G + wm_p * r["rel_mul"] + wm_r * r["la_mul"] + wa_p * A
    return G + w * (M + A)


def rho_cost(r, q, w, variant=None):
    G, C = r["group_ops"], r["native"]
    K, H, D = r["compares_uncharged"], r["hash_uncharged"], r["coef_adds_uncharged"]
    if variant in ("P0f", "P0u"):
        K = 0
    if variant == "P0u":
        C = 0
    if q in ("Q1", "Q2"):
        return G + w * C
    if q == "Q3":
        return G + w * (C + K)
    if q == "Q4":
        return G + w * (C + K + H + D)
    return G


def mean(v):
    return sum(v) / len(v)


def curve_ratios(Wd, Rd, Pd, q, w):
    """Per curve: mean cost of P / P0f / P0u over R on target 1 (same keys)."""
    out = {}
    for c in sorted({k[1] for k in Pd}):
        ks = [k for k in Pd if k[1] == c and k in Rd]
        if not ks:
            continue
        rr = mean([rho_cost(Rd[k], q, w) for k in ks])
        out[c] = {v: mean([rho_cost(Pd[k], q, w, None if v == "P" else v) for k in ks]) / rr
                  for v in ("P", "P0f", "P0u")}
    return out


def stat(ks, Wd, Rd, ratios, q, w, refset):
    ic = mean([ic_cost(Wd[k], q, w) for k in ks])
    costs = {"R": mean([rho_cost(Rd[k], q, w) for k in ks])}
    if ratios and all(k[1] in ratios for k in ks):
        for v in ("P", "P0f", "P0u"):
            costs[v] = mean([rho_cost(Rd[k], q, w) * ratios[k[1]][v] for k in ks])
    avail = {n: c for n, c in costs.items() if n in refset}
    best = min(avail, key=avail.get)
    return ic / avail[best], best, costs


def boot(keys, Wd, Rd, Pd, q, w, refset):
    ratios = curve_ratios(Wd, Rd, Pd, q if q != "Qcal" else "Q1", w)
    est, best, costs = stat(keys, Wd, Rd, ratios, q, w, refset)
    rnd = random.Random(SEED)
    tg = sorted({k[:3] for k in keys})
    by = {t: [k for k in keys if k[:3] == t] for t in tg}
    bs = []
    for _ in range(NB):
        ks = []
        for t in [rnd.choice(tg) for _ in tg]:
            ks += [rnd.choice(by[t]) for _ in by[t]]
        bs.append(stat(ks, Wd, Rd, ratios, q, w, refset)[0])
    bs.sort()
    return {"ratio": est, "lo": bs[int(0.025 * NB)], "hi": bs[int(0.975 * NB) - 1], "cheapest": best,
            "rho_means": costs, "per_curve_ratios": {str(c): v for c, v in ratios.items()}}


cells = {
    "8209_new": (8209, W, R, P, False),
    "8209_pool": (8209, {**Wr, **W}, {**Rr, **R}, {**Pr, **P}, True),
    "16411": (16411, W, R, P, False),
}
FROZEN, ADD, SENS = {"R", "P"}, {"R", "P", "P0f"}, {"R", "P", "P0f", "P0u"}
out = {"gates": gate_ok, "gate_failures": gates, "calibration": {"w_mul_p": wm_p, "w_add_p": wa_p, "w_mul_r": wm_r},
       "cells": {}, "claims": "[pcc:unverified]"}
for name, (p, Wd, Rd, Pd, _) in cells.items():
    keys = sorted(k for k in Wd if k[0] == p and k in Rd)
    c = {"runs": len(keys), "targets": len({k[:3] for k in keys}), "curves": sorted({k[1] for k in keys}),
         "frozen": {}, "addendum": {}, "p0u": {}}
    for q in READ:
        c["frozen"][q] = {n: boot(keys, Wd, Rd, Pd, q, w, FROZEN) for n, w in WS}
        c["addendum"][q] = {n: boot(keys, Wd, Rd, Pd, q, w, ADD) for n, w in WS}
        c["p0u"][q] = boot(keys, Wd, Rd, Pd, q, 1.0, SENS)
    c["frozen"]["Qcal"] = boot(keys, Wd, Rd, Pd, "Qcal", 0.0, FROZEN)
    c["verdict_frozen_Q3"] = all(c["frozen"]["Q3"][n]["hi"] < 1 for n, _ in WS)
    c["verdict_addendum_Q3"] = all(c["addendum"]["Q3"][n]["hi"] < 1 for n, _ in WS)
    out["cells"][name] = c
json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1)

f3 = lambda v: f"{v['ratio']:.3f} [{v['lo']:.3f}, {v['hi']:.3f}] ({v['cheapest']})"
L = ["# Tables (generated by analyze.py; every number [pcc:unverified])", "",
     "Gates: " + ", ".join(f"{g} {'pass' if ok else 'FAIL'}" for g, ok in gate_ok.items()),
     f"Calibration (mean of {len(cal)}): mul p {wm_p:.5f}, add p {wa_p:.5f}, mul r {wm_r:.5f} GAE", ""]
for name, c in out["cells"].items():
    L += [f"## {name}: curves {c['curves']}, {c['targets']} targets, {c['runs']} runs", "",
          "| reading | ref set | w = 0 | w = 0.1 | w = 1 |", "|--|--|--|--|--|"]
    for q in READ:
        for lab, key_ in [("R,P (frozen)", "frozen"), ("R,P,P0f (addendum)", "addendum")]:
            cc = c[key_][q]
            L.append(f"| {q} | {lab} | {f3(cc['0'])} | {f3(cc['0.1'])} | {f3(cc['1'])} |")
        L.append(f"| {q} | +P0u (sensitivity) | | | {f3(c['p0u'][q])} |")
    L += ["", f"Qcal (frozen refs): {f3(c['frozen']['Qcal'])}",
          f"Below rho at w = 0, 0.1, 1 in Q3: frozen refs **{c['verdict_frozen_Q3']}**, "
          f"addendum refs **{c['verdict_addendum_Q3']}**", ""]
open(os.path.join(HERE, "tables.md"), "w").write("\n".join(L) + "\n")

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

fig, ax = plt.subplots(figsize=(8.5, 4.6))
x = 0
ticks = []
for name, c in out["cells"].items():
    for j, (lab, key_) in enumerate([("frozen", "frozen"), ("addendum", "addendum")]):
        v = c[key_]["Q3"]["1"]
        ax.errorbar([x], [v["ratio"]], yerr=[[v["ratio"] - v["lo"]], [v["hi"] - v["ratio"]]], fmt="o",
                    color=["#1f77b4", "#d62728"][j], capsize=3, label=f"refs {lab}" if x < 2 else None)
        x += 1
    v = c["p0u"]["Q3"]
    ax.errorbar([x], [v["ratio"]], yerr=[[v["ratio"] - v["lo"]], [v["hi"] - v["ratio"]]], fmt="s", color="k",
                capsize=3, label="+P0u" if x < 3 else None)
    ticks.append((x - 1, name))
    x += 2
ax.set_xticks([t for t, _ in ticks], [n for _, n in ticks])
ax.axhline(1, color="k", lw=0.8, ls="--")
ax.set(yscale="log", ylabel="arm w / cheapest rho, Q3, w = 1", title="Genus 4 scale study  [pcc:unverified]")
ax.legend()
fig.tight_layout()
fig.savefig(os.path.join(HERE, "figure.svg"))
fig.savefig(os.path.join(HERE, "figure.png"), dpi=110)
with PdfPages(os.path.join(HERE, "REPORT.pdf")) as pdf:
    pdf.savefig(fig)
    for src in ["RESULTS.md", "tables.md"]:
        pth = os.path.join(HERE, src)
        if os.path.exists(pth):
            t = plt.figure(figsize=(11, 8.5))
            t.text(0.03, 0.97, open(pth).read()[:7000], va="top", family="monospace", fontsize=5.4)
            pdf.savefig(t)
print("\n".join(L))
