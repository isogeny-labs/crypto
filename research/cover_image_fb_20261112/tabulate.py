#!/usr/bin/env python3
"""Tabulate D (solving_degree_max) per arm, kind, control, m, t from runs/*.jsonl."""
import json, glob, os, collections, sys
D = os.path.dirname(os.path.abspath(__file__))
cells = collections.defaultdict(list)
viol = 0
for path in sorted(glob.glob(os.path.join(D, "runs", "m*.jsonl"))):
    arm = os.path.basename(path)[3:-6]  # m2-quad.jsonl -> quad
    for line in open(path):
        r = json.loads(line)
        if "control" not in r:
            viol += r.get("planted_violations", 0)
            continue
        if r["target"] == "planted" and r["planted_ok"] is False:
            viol += 1
        cov = r.get("cover") or {}
        cells[(r["m"], r["kind"], arm, r["control"], r["t"])].append(
            (r["target"], r["solving_degree_max"], r["timed_out"], r["max_cols"],
             cov.get("base_size", 2 ** r["t"]), cov.get("genus")))
out = ["m\tkind\tarm\tcontrol\tt\tF\tgenus\tD_planted\tD_random\tmax_cols\ttimed_out"]
for k in sorted(cells):
    rows = cells[k]
    dp = [d for tg, d, to, *_ in rows if tg == "planted" and not to]
    dr = [d for tg, d, to, *_ in rows if tg == "random" and not to]
    f = ",".join(sorted({str(x[4]) for x in rows}))
    g = ",".join(sorted({str(x[5]) for x in rows}))
    out.append("\t".join(map(str, [*k, f, g,
        max(dp) if dp else "-", max(dr) if dr else "-",
        max(x[3] for x in rows), sum(x[2] for x in rows)])))
open(os.path.join(D, "cells.tsv"), "w").write("\n".join(out) + "\n")
print("\n".join(out)); print("planted violations:", viol, file=sys.stderr)
