#!/usr/bin/env python3
"""Follow-up c_D sweep, scored by cd_sweep/PROTOCOL.md. usage: analyze.py RESULTS_DIR
Writes analysis.json and tables.md next to this file."""
import json, math, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
MAIN_RUNS = os.path.join(HERE, "..", "runs", "local-20261113", "units")
PH = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
QS = [262111, 524287, 1048507, 2097091, 4194271]
CS = {"C155": 1.55, "C170": 1.7, "C180": 1.8}
YS = ["DL", "DL1", "D"]
WADD = [0.0, 0.5, 1.0]
TIMING = ("wall", "cpu")
C = {}
for line in open(os.path.join(HERE, "..", "..", "quartic_n4v2_20261020", "results", "summary.json")):
    r = json.loads(line)
    if r.get("kind") == "ic_size" and r["arm"] == "DL":
        C[r["q"]] = (r["r_inv"], r["r_lmul"])
res = sys.argv[1]


def recs(path, y=None):
    out = [json.loads(l) for l in open(path) if l.strip()]
    return [r for r in out if r.get("rec") == "ic" and (y is None or r["arm"] == y)]


def unit(arm, q, s, y=None):
    return recs(os.path.join(res, "units", f"{arm}_{q}_{s}.jsonl"), y)


def meq(rs, wadd):
    ri, rl = C[rs[0]["q"]]
    return sum(r.get(p + ".mul", 0) + ri * r.get(p + ".inv", 0) + rl * r.get(p + ".lmul", 0)
               + wadd * (r.get(p + ".add", 0) + r.get(p + ".ladd", 0)) for r in rs for p in PH)


def counted(r):
    return {k: v for k, v in r.items() if not k.endswith(TIMING) and k != "load1" and not k.startswith("s.controls")}


gm = lambda v: math.exp(sum(map(math.log, v)) / len(v))
# gates
aa = {"compared": 0, "mismatches": []}
for arm, main_arm in (("C200", "BASE"), ("C155", "CRULE")):
    for q in QS:
        for s in (1, 2, 3):
            a = {r["arm"]: r for r in unit(arm, q, s)}
            b = {r["arm"]: r for r in recs(os.path.join(MAIN_RUNS, f"{main_arm}_{q}_{s}.jsonl"))}
            for y in YS:
                aa["compared"] += 1
                ca, cb = counted(a[y]), counted(b[y])
                d = [k for k in ca if k in cb and ca[k] != cb[k]]
                if d:
                    aa["mismatches"].append(f"{arm} {q}_{s}/{y}: {d[:5]}")
fails, retried = [], []
for arm in ["C200"] + list(CS):
    for q in QS:
        for s in (1, 2, 3):
            for y in YS:
                rs = unit(arm, q, s, y)
                if not rs[-1]["ok"] or rs[-1]["k_found"] != rs[-1]["k"]:
                    fails.append(f"{arm} {q}_{s}/{y}")
                if len(rs) > 1:
                    retried.append(f"{arm} {q}_{s}/{y}: {len(rs)} attempts")

ratios, cpu = {}, {}
for arm in CS:
    for y in YS:
        for q in QS:
            for w in WADD:
                rr = [meq(unit(arm, q, s, y), w) / meq(unit("C200", q, s, y), w) for s in (1, 2, 3)]
                ratios[f"{arm}/{y}/{q}/{w}"] = {"geomean": gm(rr), "max": max(rr), "all_below_1": max(rr) < 1}
            cp = lambda a, s, ph=PH: sum(r.get(p + ".cpu", 0) for r in unit(a, q, s, y) for p in ph)
            cpu[f"{arm}/{y}/{q}"] = {"total": [cp(arm, s) / cp("C200", s) for s in (1, 2, 3)],
                                    "relations": [cp(arm, s, ["relations"]) / cp("C200", s, ["relations"]) for s in (1, 2, 3)],
                                    "linear_algebra": [cp(arm, s, ["linear_algebra"]) / cp("C200", s, ["linear_algebra"]) for s in (1, 2, 3)]}
robust = {f"{arm}/{y}/{q}": ratios[f"{arm}/{y}/{q}/0.0"]["all_below_1"] and ratios[f"{arm}/{y}/{q}/1.0"]["all_below_1"]
          for arm in CS for y in YS for q in QS}
headline = [CS[a] for a in CS if all(robust[f"{a}/DL/{q}"] for q in QS)]
best = {}
for y in YS:
    for w in WADD:
        at22 = min(CS, key=lambda a: ratios[f"{a}/{y}/4194271/{w}"]["geomean"])
        avg = min(CS, key=lambda a: gm([ratios[f"{a}/{y}/{q}/{w}"]["geomean"] for q in QS]))
        best[f"{y}/{w}"] = {"at_k22": CS[at22], "over_k18_22": CS[avg]}
rowW = {f"{arm}/{q}": gm([unit(arm, q, s, "DL")[-1]["s.cycle_row_weight_mean"] for s in (1, 2, 3)]) for arm in ["C200"] + list(CS) for q in QS}

out = {"gates": {"aa": aa, "verify_failures": fails, "retried": retried}, "ratios": ratios, "robust": robust,
       "headline_DL_robust_c": headline, "best_c": best, "cpu_diagnostic": cpu, "cycle_row_weight_DL": rowW}
json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1)

L = ["# c_D sweep: tables (analyze.py)", "",
     f"A/A: {aa['compared']} records compared with the main study, {len(aa['mismatches'])} mismatches. "
     f"Verification failures: {len(fails)}. Units with a retried arm: {len(retried)} {retried[:6]}.", "",
     f"Headline (DL, robust at every k = 18..22: below 1 at every seed with additions free and at 1 M-eq): "
     f"**{', '.join(str(c) for c in headline) or 'none'}**.", "",
     "| c_D | IC arm | w_add | k = 18 | 19 | 20 | 21 | 22 |", "|--:|:--|--:|--:|--:|--:|--:|--:|"]
for y in YS:
    for arm in CS:
        for w in WADD:
            cells = []
            for q in QS:
                r = ratios[f"{arm}/{y}/{q}/{w}"]
                cells.append(f"{r['geomean']:.3f}" + ("" if r["all_below_1"] else " ✗"))
            L.append(f"| {CS[arm]} | {y} | {w} | " + " | ".join(cells) + " |")
L += ["", "✗: not below 1 at every seed.", "", "Best c_D (lowest geometric-mean ratio):", "",
      "| IC arm | w_add | at k = 22 | over k = 18..22 |", "|:--|--:|--:|--:|"]
for k, v in best.items():
    y, w = k.split("/")
    L.append(f"| {y} | {w} | {v['at_k22']} | {v['over_k18_22']} |")
L += ["", "Mean cycle-row weight (DL record, geometric mean over seeds):", "", "| c_D | k = 18 | 19 | 20 | 21 | 22 |", "|--:|--:|--:|--:|--:|--:|"]
for arm, c in [("C200", 2.0)] + list(CS.items()):
    L.append(f"| {c} | " + " | ".join(f"{rowW[f'{arm}/{q}']:.1f}" for q in QS) + " |")
L += ["", "CPU diagnostic, DL arm total CPU / c_D = 2.0 (seeds 1, 2, 3; one unit at a time, L0):", "",
      "| c_D | k = 18 | 19 | 20 | 21 | 22 |", "|--:|:--|:--|:--|:--|:--|"]
for arm in CS:
    L.append(f"| {CS[arm]} | " + " | ".join(" ".join(f"{x:.2f}" for x in cpu[f'{arm}/DL/{q}']['total']) for q in QS) + " |")
open(os.path.join(HERE, "tables.md"), "w").write("\n".join(L) + "\n")
print("\n".join(L[:6]))
