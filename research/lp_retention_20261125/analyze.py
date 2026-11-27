#!/usr/bin/env python3
"""Apply PROTOCOL.md's frozen decision rules to results/*.json. Writes results/verdict.json and RESULTS_TABLES.md."""
import json, os
import numpy as np

R = "results"
POL = ["random", "smallest_lp", "degree", "spectral", "core_then_random"]
NAME = {"Random": "random", "SmallestLp": "smallest_lp", "Degree": "degree", "Spectral": "spectral",
        "CoreThenRandom": "core_then_random"}


def ci(v, seed=7, n=10000):
    v = np.asarray(v, float)
    m = v[np.random.default_rng(seed).integers(0, len(v), (n, len(v)))].mean(1)
    return [float(v.mean()), float(np.percentile(m, 2.5)), float(np.percentile(m, 97.5))]


def load(name):
    # REAL was collected after the root crate built; it lives with the Addendum A runs.
    path = os.path.join("results_confirm" if name == "real" else R, name + ".json")
    return json.load(open(path)) if os.path.exists(path) else None


def table(rep):
    """{fraction: {policy: [left_nullity per repeat]}}"""
    t = {}
    for r in rep["repeats"]:
        for b in r["budgets"]:
            d = t.setdefault(b["fraction"], {p: [] for p in POL})
            for o in b["outcomes"]:
                if o["policy"] in NAME:  # the frozen five; Addendum A's sixth is scored by confirm_h7.py
                    d[NAME[o["policy"]]].append(o["rank"]["left_nullity"])
    return t


def ratio(t, f, a, b):
    return ci(np.array(t[f][a]) / np.array(t[f][b]))


fmt = lambda c: f"{c[0]:.3f} [{c[1]:.3f}, {c[2]:.3f}]"
beats = lambda rows: all(r[1] > 1 for r in rows)
ties = lambda rows: all(0.97 <= r[1] and r[2] <= 1.03 for r in rows)

streams = {s: load(s) for s in ["fp0", "fp1", "fp2", "fp0r", "fp1r", "fp2r", "syn", "synr", "real"]}
T = {s: table(v) for s, v in streams.items() if v}
fracs = sorted(next(iter(T.values())).keys(), reverse=True)
V, L = {}, []

# Internal check: relabel leaves degree/spectral/core selections' counts identical.
inv = True
for a, b in [("fp0", "fp0r"), ("fp1", "fp1r"), ("fp2", "fp2r"), ("syn", "synr")]:
    for f in fracs:
        for p in ["degree", "spectral", "core_then_random", "random"]:
            inv &= T[a][f][p] == T[b][f][p]
V["relabel_invariance_ok"] = inv
L.append(f"Relabel invariance (degree, spectral, core_then_random, random unchanged): {'ok' if inv else 'VIOLATED -> INVALID'}\n")

# Tables: mean left nullity and ratio to random, per stream and budget.
for s in T:
    L.append(f"### {s} (pool {streams[s]['repeats'][0]['pool']}, full-pool left nullity {streams[s]['repeats'][0]['full']['rank']['left_nullity']})\n")
    L.append("| budget | " + " | ".join(POL) + " | smallest_lp/degree | smallest_lp/spectral | spectral/degree |")
    L.append("|--:|" + "--:|" * (len(POL) + 3))
    for f in fracs:
        means = " | ".join(f"{np.mean(T[s][f][p]):.0f}" for p in POL)
        L.append(f"| {f} | {means} | {fmt(ratio(T[s], f, 'smallest_lp', 'degree'))} | "
                 f"{fmt(ratio(T[s], f, 'smallest_lp', 'spectral'))} | {fmt(ratio(T[s], f, 'spectral', 'degree'))} |")
    L.append("")

FP, FPr = ["fp0", "fp1", "fp2"], ["fp0r", "fp1r", "fp2r"]
# H1
h1 = {c: [ratio(T[s], 0.25, "smallest_lp", c) for s in FP] for c in ["degree", "spectral"]}
V["H1"] = dict(supported=beats(h1["degree"]) and beats(h1["spectral"]), ratios=h1)
# H3 budget sweep
V["H3"] = {str(f): beats([ratio(T[s], f, "smallest_lp", "degree") for s in FP]) and
           beats([ratio(T[s], f, "smallest_lp", "spectral") for s in FP]) for f in fracs}
# H4 size information
h4 = {}
for fam, ss in [("FPr", FPr), ("SYNr", ["synr"])]:
    sl = [ratio(T[s], 0.25, "smallest_lp", "random") for s in ss]
    best = max(["degree", "spectral"], key=lambda p: np.mean([np.mean(T[s][0.25][p]) for s in ss]))
    bc = [ratio(T[s], 0.25, best, "random") for s in ss]
    h4[fam] = dict(smallest_lp_ties_random=ties(sl), smallest_lp_over_random=sl,
                   best_centrality=best, best_beats_random=beats(bc), best_over_random=bc)
V["H4"] = h4
# H5 synthetic
V["H5"] = {c: ratio(T["syn"], 0.25, "smallest_lp", c) for c in ["degree", "spectral", "random"]}
# H6 real
if "real" in T:
    V["H6"] = {str(f): {p: ratio(T["real"], f, p, "random") for p in POL if p != "random"} for f in fracs}
else:
    V["H6"] = "blocked: no REAL stream (root crate did not produce lp_graph_collect)"

L.append("## Decisions\n")
L.append(f"- H1 (FP, 0.25: smallest_lp beats degree and spectral): {'supported' if V['H1']['supported'] else 'not supported'}")
L.append("- H3 (smallest_lp beats both at each budget, FP): " + ", ".join(f"{k}: {'yes' if v else 'no'}" for k, v in V["H3"].items()))
for fam, d in h4.items():
    L.append(f"- H4 {fam}: smallest_lp ties random: {'yes' if d['smallest_lp_ties_random'] else 'no'} "
             f"(smallest_lp/random {', '.join(fmt(c) for c in d['smallest_lp_over_random'])}); best centrality {d['best_centrality']} "
             f"beats random: {'yes' if d['best_beats_random'] else 'no'}")
L.append("- H5 (SYN, 0.25): " + "; ".join(f"smallest_lp/{k} {fmt(v)}" for k, v in V["H5"].items()))
L.append(f"- H6: {'see real table' if isinstance(V['H6'], dict) else V['H6']}")
json.dump(V, open(os.path.join(R, "verdict.json"), "w"), indent=1)
open("RESULTS_TABLES.md", "w").write("\n".join(L) + "\n")
print("\n".join(L))
