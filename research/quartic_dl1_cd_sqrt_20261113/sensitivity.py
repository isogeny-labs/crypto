#!/usr/bin/env python3
"""POST-HOC sensitivity (not pre-registered; PROTOCOL.md fixes the decision rule). Two checks:
1. BOTH / BASE and CRULE / BASE with additions priced: F_q `add` and mod-ell `ladd` at w_add M-eq each
   (the frozen M-eq model prices them at 0).
2. Per-arm CPU seconds by phase (diagnostic only: shared 4-core container, L0, 4 units in parallel).
usage: sensitivity.py RESULTS_DIR  ->  sensitivity.json, sensitivity.md
"""
import json, math, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
PH = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
BIG = [262111, 524287, 1048507, 2097091, 4194271]
C = {}
for line in open(os.path.join(HERE, "..", "quartic_n4v2_20261020", "results", "summary.json")):
    r = json.loads(line)
    if r.get("kind") == "ic_size" and r["arm"] == "DL":
        C[r["q"]] = (r["r_inv"], r["r_lmul"])
res = sys.argv[1]


def recs(arm, q, s, y):
    return [json.loads(l) for l in open(f"{res}/units/{arm}_{q}_{s}.jsonl")
            if '"rec":"ic"' in l and f'"arm":"{y}"' in l]


def meq(rs, wadd):
    ri, rl = C[rs[0]["q"]]
    return sum(r.get(p + ".mul", 0) + ri * r.get(p + ".inv", 0) + rl * r.get(p + ".lmul", 0)
               + wadd * (r.get(p + ".add", 0) + r.get(p + ".ladd", 0)) for r in rs for p in PH)


def cpu(rs, phases=PH):
    return sum(r.get(p + ".cpu", 0) for r in rs for p in phases)


gm = lambda v: math.exp(sum(map(math.log, v)) / len(v))
out = {"post_hoc": True, "additions_priced": {}, "cpu_diagnostic": {}}
L = ["# Post-hoc sensitivity (not pre-registered)", "",
     "## Additions priced (F_q add and mod-ell ladd at w_add M-eq each; frozen model: 0)", "",
     "| arm / BASE | IC arm | w_add | k = 18 | 19 | 20 | 21 | 22 |", "|:--|:--|--:|--:|--:|--:|--:|--:|"]
for x in ("BOTH", "CRULE"):
    for y in ("DL", "DL1"):
        for wadd in (0.0, 0.1, 0.25, 0.5, 1.0):
            row = []
            for q in BIG:
                rr = [meq(recs(x, q, s, y), wadd) / meq(recs("BASE", q, s, y), wadd) for s in (1, 2, 3)]
                out["additions_priced"][f"{x}/{y}/{wadd}/{q}"] = {"geomean": gm(rr), "max": max(rr)}
                row.append(f"{gm(rr):.3f}" + ("" if max(rr) < 1 else " (not all < 1)"))
            L.append(f"| {x} | {y} | {wadd} | " + " | ".join(row) + " |")
L += ["", "## CPU seconds, BOTH / BASE and CHAIN / BASE (diagnostic: L0, 4 units in parallel)", "",
      "| arm / BASE | IC arm | k | total CPU (seeds 1, 2, 3) | relations CPU | linear-algebra CPU |", "|:--|:--|--:|:--|:--|:--|"]
for x in ("BOTH", "CHAIN"):
    for y in ("DL", "DL1", "D"):
        for q in BIG:
            t, rl, la = [], [], []
            for s in (1, 2, 3):
                a, b = recs(x, q, s, y), recs("BASE", q, s, y)
                t.append(cpu(a) / cpu(b))
                rl.append(cpu(a, ["relations"]) / cpu(b, ["relations"]))
                la.append(cpu(a, ["linear_algebra"]) / cpu(b, ["linear_algebra"]))
            out["cpu_diagnostic"][f"{x}/{y}/{q}"] = {"total": t, "relations": rl, "linear_algebra": la}
            f = lambda v: " ".join(f"{z:.2f}" for z in v)
            L.append(f"| {x} | {y} | {q.bit_length()} | {f(t)} | {f(rl)} | {f(la)} |")
json.dump(out, open(os.path.join(HERE, "sensitivity.json"), "w"), indent=1)
open(os.path.join(HERE, "sensitivity.md"), "w").write("\n".join(L) + "\n")
print("ok")
