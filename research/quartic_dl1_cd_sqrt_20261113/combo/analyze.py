#!/usr/bin/env python3
"""Follow-up 2 (c_D = 1.7 + square-root chain), scored by combo/PROTOCOL.md. usage: analyze.py RESULTS_DIR
Writes analysis.json and tables.md next to this file."""
import json, math, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
SWEEP = os.path.join(HERE, "..", "cd_sweep", "runs", "units")
PH = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
QS = [262111, 524287, 1048507, 2097091, 4194271]
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
aa = {"compared": 0, "mismatches": []}
for arm, sweep_arm in (("BASE", "C200"), ("C170", "C170")):
    for q in QS:
        for s in (1, 2, 3):
            a = {r["arm"]: r for r in unit(arm, q, s)}
            b = {r["arm"]: r for r in recs(os.path.join(SWEEP, f"{sweep_arm}_{q}_{s}.jsonl"))}
            for y in YS:
                aa["compared"] += 1
                ca, cb = counted(a[y]), counted(b[y])
                d = [k for k in ca if k in cb and ca[k] != cb[k]]
                if d:
                    aa["mismatches"].append(f"{arm} {q}_{s}/{y}: {d[:5]}")
chain = {"pairs": 0, "violations": []}
for q in QS:
    for s in (1, 2, 3):
        for y in YS:
            a, b = unit("C170", q, s, y), unit("B170", q, s, y)
            chain["pairs"] += 1
            if len(a) != len(b):
                chain["violations"].append(f"{q}_{s}/{y}: attempts differ")
                continue
            for x, z in zip(a, b):
                cx, cz = counted(x), counted(z)
                d = [k for k in cx if k in cz and cx[k] != cz[k] and k != "relations.mul"]
                if d or not z["relations.mul"] < x["relations.mul"]:
                    chain["violations"].append(f"{q}_{s}/{y}: {d[:5]}")
fails, retried = [], []
for arm in ("BASE", "C170", "B170"):
    for q in QS:
        for s in (1, 2, 3):
            for y in YS:
                rs = unit(arm, q, s, y)
                if not rs[-1]["ok"] or rs[-1]["k_found"] != rs[-1]["k"]:
                    fails.append(f"{arm} {q}_{s}/{y}")
                if len(rs) > 1:
                    retried.append(f"{arm} {q}_{s}/{y}")

ratios, cpu = {}, {}
for num, den in (("B170", "BASE"), ("C170", "BASE"), ("B170", "C170")):
    for y in YS:
        for q in QS:
            for w in WADD:
                rr = [meq(unit(num, q, s, y), w) / meq(unit(den, q, s, y), w) for s in (1, 2, 3)]
                ratios[f"{num}/{den}/{y}/{q}/{w}"] = {"geomean": gm(rr), "max": max(rr), "all_below_1": max(rr) < 1}
            cp = lambda a, s: sum(r.get(p + ".cpu", 0) for r in unit(a, q, s, y) for p in PH)
            cpu[f"{num}/{den}/{y}/{q}"] = [cp(num, s) / cp(den, s) for s in (1, 2, 3)]
robust = {f"{y}/{q}": ratios[f"B170/BASE/{y}/{q}/0.0"]["all_below_1"] and ratios[f"B170/BASE/{y}/{q}/1.0"]["all_below_1"]
          for y in YS for q in QS}
headline = all(robust[f"DL/{q}"] for q in QS)
out = {"gates": {"aa": aa, "chain": chain, "verify_failures": fails, "retried": retried}, "ratios": ratios,
       "robust": robust, "headline_DL_robust_all_k": headline, "cpu_diagnostic": cpu}
json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1)

L = ["# c_D = 1.7 + square-root chain: tables (analyze.py)", "",
     f"A/A: {aa['compared']} records against the sweep, {len(aa['mismatches'])} mismatches. Chain identity: "
     f"{chain['pairs']} pairs, {len(chain['violations'])} violations. Verification failures: {len(fails)}. Retried: {len(retried)}.", "",
     f"Headline (DL, B170 / BASE robust at every k = 18..22): **{'yes' if headline else 'no'}**.", "",
     "| ratio | IC arm | w_add | k = 18 | 19 | 20 | 21 | 22 |", "|:--|:--|--:|--:|--:|--:|--:|--:|"]
for num, den in (("B170", "BASE"), ("C170", "BASE"), ("B170", "C170")):
    for y in YS:
        for w in WADD:
            cells = [f"{ratios[f'{num}/{den}/{y}/{q}/{w}']['geomean']:.3f}" + ("" if ratios[f'{num}/{den}/{y}/{q}/{w}']["all_below_1"] else " ✗") for q in QS]
            L.append(f"| {num} / {den} | {y} | {w} | " + " | ".join(cells) + " |")
L += ["", "✗: not below 1 at every seed.", "", "CPU diagnostic, total CPU ratio (seeds 1, 2, 3; one unit at a time, L0):", "",
      "| ratio | IC arm | k = 18 | 19 | 20 | 21 | 22 |", "|:--|:--|:--|:--|:--|:--|:--|"]
for num, den in (("B170", "BASE"), ("B170", "C170")):
    for y in ("DL", "DL1"):
        L.append(f"| {num} / {den} | {y} | " + " | ".join(" ".join(f"{x:.2f}" for x in cpu[f'{num}/{den}/{y}/{q}']) for q in QS) + " |")
open(os.path.join(HERE, "tables.md"), "w").write("\n".join(L) + "\n")
print("\n".join(L[:5]))
