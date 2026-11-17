#!/usr/bin/env python3
"""Readings, gates and statistics of PROTOCOL.md (genus4_lean_rho_20261116).

Written after the run was launched and before any of its data was read; it
implements the frozen definitions only. Every number it prints is
[pcc:unverified].
"""
import json
import math
import os
import random

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.join(HERE, "..")
NB = 2000
SEED = 20261121
WS = [("0", 0.0), ("0.1", 0.1), ("1", 1.0)]
READ = ["Q1", "Q2", "Q3", "Q4"]
PRIMES = [8209, 16411, 32771]
WARM = "C_double_lp+pre+coord+unitT"


def load(*paths):
    out = []
    for p in paths:
        p = os.path.join(RES, p)
        if os.path.exists(p):
            out += [json.loads(l) for l in open(p) if l.strip()]
    return out


def key(r):
    return (r["p"], r["curve_seed"], r.get("target_seed", r["curve_seed"]), r["seed"])


DUPLICATES = {"compared": 0}


def arm(rows, name):
    d = {}
    for r in rows:
        if r.get("kind") == "run" and r.get("g", r.get("genus")) == 4 and r["arm"] == name:
            k = key(r)
            if k in d:
                a = {f: v for f, v in d[k].items() if f != "wall_ns"}
                b = {f: v for f, v in r.items() if f != "wall_ns"}
                if a != b:
                    raise SystemExit(f"determinism check failed for {name} {k}")
                DUPLICATES["compared"] += 1
            d[k] = r
    return d


import glob
# Every data file of this study: run.sh outputs, resume.sh / percurve.sh re-runs after the
# container restarts (session log 2026-11-17, declared before reading results).
new = load(*sorted(os.path.relpath(f, RES) for f in glob.glob(os.path.join(HERE, "p*.jsonl"))))
old = load("genus4_rho_refs_20261116/run.jsonl", "genus4_scale_20261116/p8209.jsonl",
           "genus4_scale_20261116/p16411a.jsonl", "genus4_scale_20261116/p16411b.jsonl",
           "genus4_scale_20261116/p16411a_resume.jsonl", "genus4_scale_20261116/p8209_plain.jsonl",
           "genus4_scale_20261116/p16411_plain.jsonl")
W, Bw, Q = arm(new, WARM), arm(new, "R_rho_neg_brent"), arm(new, "R_rho_plain_nowin")
Wold, Rold, Pold = arm(old, WARM), arm(old, "R_rho_neg"), arm(old, "R_rho_plain")
cal = [r for r in new if r.get("kind") == "calibration"]

# Gates (protocol section 4)
gates = {"G1": [], "G2": [], "G3": []}
for d in (W, Bw, Q):
    for k, r in d.items():
        if not (r.get("certified") and r.get("ok")):
            gates["G1"].append(f"{r['arm']} {k}")
for k, o in Wold.items():
    n = W.get(k)
    if n is None:
        continue
    for f in ["group_ops", "rel_mul", "la_mul", "rel_add", "la_add", "trials", "relations"]:
        if f in o and n.get(f) != o[f]:
            gates["G2"].append(f"{k} {f}: {n.get(f)} != {o[f]}")
g3_checked = 0
for k, o in Pold.items():
    n = Q.get(k)
    if n is None or k[0] != 8209:
        continue
    g3_checked += 1
    for f in ["steps", "group_ops"]:
        if n[f] != o[f]:
            gates["G3"].append(f"{k} {f}: {n[f]} != {o[f]}")
gate_ok = {g: not v for g, v in gates.items()}


def ic_cost(r, reading, w):
    G, M, A = r["group_ops"], r["rel_mul"] + r["la_mul"], r["rel_add"] + r["la_add"]
    if reading == "Q1":
        return G + w * M
    return G + w * (M + A)


def canon(r, floored):
    """Actual canonicalisations: 0 for the plain walk (none performed); the row's
    `native` is floored at steps by run_rho_walk. Floored = the row's value."""
    if floored:
        return r["native"]
    return 0 if r["arm"].startswith("R_rho_plain") else r["native"]


def rho_cost(r, reading, w, floored=False):
    G, C = r["group_ops"], canon(r, floored)
    K, H, D = r["compares_uncharged"], r["hash_uncharged"], r["coef_adds_uncharged"]
    if reading in ("Q1", "Q2"):
        return G + w * C
    if reading == "Q3":
        return G + w * (C + K)
    return G + w * (C + K + H + D)


def mean(v):
    return sum(v) / len(v)


def refs_for(p, keys):
    """Rho arms that cover every key at this prime (protocol section 5)."""
    out = {}
    for name, d in [("B", Bw), ("Q", Q), ("R", Rold)]:
        if all(k in d for k in keys):
            out[name] = d
    return out


def stat(ks, refs, reading, w, floored):
    ic = mean([ic_cost(W[k], reading, w) for k in ks])
    costs = {n: mean([rho_cost(d[k], reading, w, floored) for k in ks]) for n, d in refs.items()}
    best = min(costs, key=costs.get)
    return ic / costs[best], best, costs


def resample(rnd, keys):
    targets = sorted({k[:3] for k in keys})
    by_t = {t: [k for k in keys if k[:3] == t] for t in targets}
    ks = []
    for t in [rnd.choice(targets) for _ in targets]:
        ks += [rnd.choice(by_t[t]) for _ in by_t[t]]
    return ks


def boot(p, keys, reading, w, floored=False):
    refs = refs_for(p, keys)
    est, best, costs = stat(keys, refs, reading, w, floored)
    rnd = random.Random(SEED)
    bs = sorted(stat(resample(rnd, keys), refs, reading, w, floored)[0] for _ in range(NB))
    return {"ratio": est, "lo": bs[int(0.025 * NB)], "hi": bs[int(0.975 * NB) - 1], "cheapest": best,
            "refs": sorted(refs), "rho_means": costs, "n_runs": len(keys)}


out = {"duplicate_rows_identical": DUPLICATES["compared"], "gates": gate_ok, "gate_failures": {g: v[:20] for g, v in gates.items()}, "g3_checked": g3_checked,
       "per_prime": {}, "slope": {}, "claims": "[pcc:unverified]"}
keys_by_p = {p: sorted(k for k in W if k[0] == p and k in Bw and k in Q) for p in PRIMES}
for p in PRIMES:
    keys = keys_by_p[p]
    if not keys:
        continue
    cell = {"runs": len(keys), "targets": len({k[:3] for k in keys}),
            "B_compares_per_step": mean([Bw[k]["compares_uncharged"] / Bw[k]["steps"] for k in keys]),
            "Q_compares_per_step": mean([Q[k]["compares_uncharged"] / Q[k]["steps"] for k in keys]),
            "readings": {}, "floored": {}}
    for reading in READ:
        cell["readings"][reading] = {n: boot(p, keys, reading, w) for n, w in WS}
        cell["floored"][reading] = boot(p, keys, reading, 1.0, floored=True)
    v3, v4 = cell["readings"]["Q3"]["1"], cell["readings"]["Q4"]["1"]
    cell["below_rho_w1"] = v3["hi"] < 1 and v4["hi"] < 1
    cell["below_rho_w0_w01_all"] = all(cell["readings"][q][n]["hi"] < 1 for q in READ for n in ("0", "0.1"))
    out["per_prime"][str(p)] = cell


def fit(xs, ys):
    mx, my = mean(xs), mean(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)


ps = [p for p in PRIMES if keys_by_p[p]]
if len(ps) >= 2:
    for reading in ["Q3", "Q4"]:
        refs = {p: refs_for(p, keys_by_p[p]) for p in ps}
        xs = [math.log(p) for p in ps]
        est = fit(xs, [math.log(stat(keys_by_p[p], refs[p], reading, 1.0, False)[0]) for p in ps])
        rnd = random.Random(SEED + 1)
        bs = sorted(fit(xs, [math.log(stat(resample(rnd, keys_by_p[p]), refs[p], reading, 1.0, False)[0])
                             for p in ps]) for _ in range(NB))
        out["slope"][reading] = {"slope": est, "lo": bs[int(0.025 * NB)], "hi": bs[int(0.975 * NB) - 1],
                                 "primes": ps}
json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1)

f3 = lambda v: f"{v['ratio']:.3f} [{v['lo']:.3f}, {v['hi']:.3f}] ({v['cheapest']})"
L = ["# Tables (generated by analyze.py; every number [pcc:unverified])", "",
     "Gates: " + ", ".join(f"{g} {'pass' if ok else 'FAIL'}" for g, ok in gate_ok.items())
     + f" (G3 keys checked: {g3_checked})", ""]
for p, c in out["per_prime"].items():
    L += [f"## p = {p}: {c['targets']} targets, {c['runs']} runs; reference arms "
          f"{c['readings']['Q3']['1']['refs']}; compares/step B {c['B_compares_per_step']:.2f}, "
          f"Q {c['Q_compares_per_step']:.2f}", "",
          "| reading | w = 0 | w = 0.1 | w = 1 | w = 1, plain C floored |", "|--|--|--|--|--|"]
    for reading in READ:
        cc = c["readings"][reading]
        L.append(f"| {reading} | {f3(cc['0'])} | {f3(cc['0.1'])} | {f3(cc['1'])} | {f3(c['floored'][reading])} |")
    L += ["", f"Below rho at w = 1 (Q3 and Q4 upper < 1): **{c['below_rho_w1']}**; "
          f"below at w = 0 and 0.1 in every reading: **{c['below_rho_w0_w01_all']}**", ""]
for reading, s in out["slope"].items():
    L.append(f"Slope of log(arm w / cheapest rho) on log p, {reading}, w = 1: "
             f"{s['slope']:.3f} [{s['lo']:.3f}, {s['hi']:.3f}] (primes {s['primes']})")
open(os.path.join(HERE, "tables.md"), "w").write("\n".join(L) + "\n")

import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"  # keep figures under the 64 KB artifact threshold
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

fig, axs = plt.subplots(1, 2, figsize=(11, 4.4))
cols = {"0": "#1f77b4", "0.1": "#ff7f0e", "1": "#d62728"}
for reading, ax in zip(["Q3", "Q4"], axs):
    for n, _ in WS:
        xs, ys, lo, hi = [], [], [], []
        for p, c in out["per_prime"].items():
            v = c["readings"][reading][n]
            xs.append(int(p)); ys.append(v["ratio"]); lo.append(v["ratio"] - v["lo"]); hi.append(v["hi"] - v["ratio"])
        ax.errorbar(xs, ys, yerr=[lo, hi], fmt="o-", color=cols[n], capsize=3, label=f"w = {n}")
    ax.axhline(1, color="k", lw=0.8, ls="--")
    ax.set(xscale="log", yscale="log", xlabel="p", ylabel="arm w / cheapest rho (B, Q, R)",
           title=f"Genus 4, reading {reading}  [pcc:unverified]")
    ax.legend()
fig.tight_layout()
fig.savefig(os.path.join(HERE, "figure.svg"))
fig.savefig(os.path.join(HERE, "figure.png"), dpi=90)
with PdfPages(os.path.join(HERE, "REPORT.pdf")) as pdf:
    pdf.savefig(fig)
    for src in ["RESULTS.md", "tables.md"]:
        pth = os.path.join(HERE, src)
        if os.path.exists(pth):
            t = plt.figure(figsize=(11, 8.5))
            t.text(0.03, 0.97, open(pth).read()[:7000], va="top", family="monospace", fontsize=5.6)
            pdf.savefig(t)
print("\n".join(L))
