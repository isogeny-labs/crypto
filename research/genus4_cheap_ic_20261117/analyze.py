#!/usr/bin/env python3
"""Readings, gates and statistics of PROTOCOL.md (genus4_cheap_ic_20261117).

Written before any data of this study existed; it implements the frozen
definitions only. Every number it prints is [pcc:unverified].
"""
import glob
import json
import os
import random

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.join(HERE, "..")
NB, SEED = 2000, 20261122
WS = [("0", 0.0), ("0.1", 0.1), ("1", 1.0)]
PRIMES = [8209, 16411]
ARMS = {
    "w": "C_double_lp+pre+coord+unitT",
    "s": "C_double_lp+pre+coord+unitT+sgecoord",
    "f": "C_double_lp+pre+coord+unitT+ferrari",
    "x": "C_double_lp+pre+coord+unitT+ferrari+sgecoord",
    "z": "Z_sieve_lp2",
}


def rows(paths):
    out = []
    for p in paths:
        out += [json.loads(l) for l in open(p) if l.strip()]
    return out


def key(r):
    return (r["p"], r["curve_seed"], r.get("target_seed", r["curve_seed"]), r["seed"])


def by_arm(rs, name, prefix=False):
    d = {}
    for r in rs:
        if r.get("kind") != "run" or r.get("g") != 4:
            continue
        a = r["arm"]
        if (a.startswith(name) if prefix else a == name):
            k = key(r)
            if k in d:
                x = {f: v for f, v in d[k].items() if f != "wall_ns"}
                y = {f: v for f, v in r.items() if f != "wall_ns"}
                if x != y:
                    raise SystemExit(f"duplicate rows differ for {name} {k}")
            d[k] = r
    return d


new = rows(sorted(glob.glob(os.path.join(HERE, "p*.jsonl"))))
lean = rows(sorted(glob.glob(os.path.join(RES, "genus4_lean_rho_20261116", "p*.jsonl"))))
older = rows([os.path.join(RES, "genus4_rho_refs_20261116", "run.jsonl")]
             + sorted(glob.glob(os.path.join(RES, "genus4_scale_20261116", "p*.jsonl"))))

IC = {a: by_arm(new, n) for a, n in ARMS.items()}
# arm labels of the ext arms may carry an sge suffix; fall back to prefix match
for a in ("s", "x"):
    if not IC[a]:
        IC[a] = by_arm(new, ARMS[a], prefix=True)
W_lean = by_arm(lean, ARMS["w"])
RHO = {"B": by_arm(lean, "R_rho_neg_brent"), "Q": by_arm(lean, "R_rho_plain_nowin"),
       "R": by_arm(older, "R_rho_neg")}

gates = {"G1": [], "G2": []}
for a, d in IC.items():
    for k, r in d.items():
        if not (r.get("certified") and r.get("ok")):
            gates["G1"].append(f"{a} {k}")
for k, o in W_lean.items():
    n = IC["w"].get(k)
    if n is None:
        continue
    for f in ["group_ops", "rel_mul", "rel_add", "la_mul", "la_add", "trials", "relations"]:
        if n.get(f) != o.get(f):
            gates["G2"].append(f"{k} {f}: {n.get(f)} != {o.get(f)}")
gate_ok = {g: not v for g, v in gates.items()}


def native(r, q):
    if q == "Q1":
        return r["rel_mul"] + r["la_mul"]
    return r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"]


def ic_cost(r, q, w):
    return r["group_ops"] + w * native(r, q)


def rho_cost(r, q, w):
    G = r["group_ops"]
    C = 0 if r["arm"].startswith("R_rho_plain") else r["native"]
    K, H, D = r["compares_uncharged"], r["hash_uncharged"], r["coef_adds_uncharged"]
    if q in ("Q1", "Q2"):
        return G + w * C
    if q == "Q3":
        return G + w * (C + K)
    return G + w * (C + K + H + D)


def mean(v):
    return sum(v) / len(v)


def resample(rnd, keys):
    tg = sorted({k[:3] for k in keys})
    by = {t: [k for k in keys if k[:3] == t] for t in tg}
    ks = []
    for t in [rnd.choice(tg) for _ in tg]:
        ks += [rnd.choice(by[t]) for _ in by[t]]
    return ks


def interval(fn, keys, seed):
    rnd = random.Random(seed)
    bs = sorted(fn(resample(rnd, keys)) for _ in range(NB))
    return fn(keys), bs[int(0.025 * NB)], bs[int(0.975 * NB) - 1]


out = {"gates": gate_ok, "gate_failures": {g: v[:20] for g, v in gates.items()}, "per_prime": {},
       "claims": "[pcc:unverified]"}
for p in PRIMES:
    keys = sorted(k for k in IC["w"] if k[0] == p)
    if not keys:
        continue
    refs = {n: d for n, d in RHO.items() if all(k in d for k in keys)}
    cell = {"runs": len(keys), "refs": sorted(refs), "arms": {}}
    for a, d in IC.items():
        ks = [k for k in keys if k in d]
        if len(ks) != len(keys):
            cell["arms"][a] = {"missing": len(keys) - len(ks)}
            continue
        ac = {}
        for q in ("Q3", "Q4"):
            for n, w in WS:
                def stat(kk, q=q, w=w):
                    ic = mean([ic_cost(d[k], q, w) for k in kk])
                    return ic / min(mean([rho_cost(r[k], q, w) for k in kk]) for r in refs.values())
                est, lo, hi = interval(stat, keys, SEED)
                cheapest = min(refs, key=lambda n: mean([rho_cost(refs[n][k], q, w) for k in keys]))
                ac[f"{q}_w{n}"] = {"ratio": est, "lo": lo, "hi": hi, "cheapest": cheapest}
        for part, fn in [("native", lambda r: native(r, "Q3")),
                         ("rel", lambda r: r["rel_mul"] + r["rel_add"]),
                         ("la", lambda r: r["la_mul"] + r["la_add"]),
                         ("group", lambda r: r["group_ops"])]:
            def rel_w(kk, fn=fn):
                return mean([fn(d[k]) for k in kk]) / mean([fn(IC["w"][k]) for k in kk])
            est, lo, hi = interval(rel_w, keys, SEED + 1)
            ac[f"{part}_vs_w"] = {"ratio": est, "lo": lo, "hi": hi}
        ac["below_rho_w1"] = ac["Q3_w1"]["hi"] < 1 and ac["Q4_w1"]["hi"] < 1
        ac["below_rho_w01"] = ac["Q3_w0.1"]["hi"] < 1 and ac["Q4_w0.1"]["hi"] < 1
        cell["arms"][a] = ac
    scored = {a: v["Q3_w1"]["ratio"] for a, v in cell["arms"].items() if "Q3_w1" in v}
    cell["best_q3_w1"] = min(scored, key=scored.get) if scored else None
    out["per_prime"][str(p)] = cell
json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1)

f3 = lambda v: f"{v['ratio']:.3f} [{v['lo']:.3f}, {v['hi']:.3f}]"
L = ["# Tables (generated by analyze.py; every number [pcc:unverified])", "",
     "Gates: " + ", ".join(f"{g} {'pass' if ok else 'FAIL'}" for g, ok in gate_ok.items()), ""]
for p, c in out["per_prime"].items():
    L += [f"## p = {p}: {c['runs']} runs per arm; rho references {c['refs']}; best arm (Q3, w = 1): {c['best_q3_w1']}",
          "", "| arm | Q3 w=0.1 | Q3 w=1 | Q4 w=1 | native / w | relation / w | LA / w |",
          "|--|--|--|--|--|--|--|"]
    for a, v in c["arms"].items():
        if "missing" in v:
            L.append(f"| {a} | missing {v['missing']} runs | | | | | |")
            continue
        L.append(f"| {a} | {f3(v['Q3_w0.1'])} | {f3(v['Q3_w1'])} | {f3(v['Q4_w1'])} | {f3(v['native_vs_w'])}"
                 f" | {f3(v['rel_vs_w'])} | {f3(v['la_vs_w'])} |")
    L.append("")
open(os.path.join(HERE, "tables.md"), "w").write("\n".join(L) + "\n")

import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

fig, ax = plt.subplots(figsize=(8.5, 4.4))
order = [a for a in ARMS]
for i, (p, c) in enumerate(out["per_prime"].items()):
    xs, ys, lo, hi = [], [], [], []
    for j, a in enumerate(order):
        v = c["arms"].get(a, {}).get("Q3_w1")
        if not v:
            continue
        xs.append(j + 0.18 * (i - 0.5)); ys.append(v["ratio"])
        lo.append(v["ratio"] - v["lo"]); hi.append(v["hi"] - v["ratio"])
    ax.errorbar(xs, ys, yerr=[lo, hi], fmt="o", capsize=3, label=f"p = {p}")
ax.set_xticks(range(len(order)), order)
ax.axhline(1, color="k", lw=0.8, ls="--")
ax.set(ylabel="IC arm / cheapest rho, Q3, w = 1", title="Genus 4: cheaper IC arms vs window-free plain rho  [pcc:unverified]")
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
