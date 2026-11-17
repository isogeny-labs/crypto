#!/usr/bin/env python3
"""Round 3 (m = 3): W, rows, E and median F4 ms per cell, and the PROTOCOL.md gates."""
import csv, glob, json, os, statistics, collections
D = os.path.dirname(os.path.abspath(__file__))
R2 = os.path.join(D, "..", "cover_image_fb_r2_20261112", "cells.tsv")
cells, viol = collections.defaultdict(list), 0
for path in sorted(glob.glob(os.path.join(D, "runs", "m3*-*.jsonl"))):
    arm = os.path.basename(path).split("-", 1)[1][:-6]
    for line in open(path):
        r = json.loads(line)
        if "control" not in r:
            continue
        if r["target"] == "planted" and r["planted_ok"] is False:
            viol += 1
        cells[(arm, r["control"], r["t"])].append(r)
W, out = {}, ["arm\tcontrol\tt\tF\tW\trows\tD\tinput_degree\tE\tmedian_ms\tcensored"]
for k in sorted(cells):
    rs = cells[k]
    cen = any(r["timed_out"] for r in rs)
    w = max(r["max_cols"] for r in rs)
    if not cen:
        W[k] = w
    din = max(max(r["input_degrees"]) for r in rs)
    d = max(r["solving_degree_max"] for r in rs)
    F = (rs[0].get("cover") or {}).get("base_size", 2 ** rs[0]["t"])
    out.append("\t".join(map(str, [*k, F, w, max(r["max_rows"] for r in rs), d, din, d - din,
                                   round(statistics.median(r["ms"] for r in rs)), int(cen)])))
open(os.path.join(D, "cells.tsv"), "w").write("\n".join(out) + "\n")
print("\n".join(out)); print(f"\nplanted violations: {viol}")
r2 = {(r["arm"], int(r["t"])): int(r["W"]) for r in csv.DictReader(open(R2), delimiter="\t")
      if r["m"] == "3" and r["control"] == "tower"}
r0 = [(a, t, W[(a, "tower", t)], r2[(a, t)]) for (a, t) in r2 if (a, "tower", t) in W]
print("R0", "PENDING" if not r0 else "PASS" if all(x[2] == x[3] for x in r0) else "FAIL", r0)
for arm in ("quad", "cubic", "quart", "generic3"):
    ts = sorted(t for (a, c, t) in W if a == arm and c == "tower" and ("none", "tower", t) in W)
    rp = {t: W[(arm, "tower", t)] / W[("none", "tower", t)] for t in ts}
    rn = {t: W[(arm, "tower", t)] / W[(arm, "null", t)] for t in ts if (arm, "null", t) in W}
    if 4 not in rp:
        v = "PENDING/censored at t=4"
    elif rp.get(3, 9) <= 0.8 and rp[4] <= 0.8:
        v = "H1" + (" S(structure)" if rn.get(4, 9) <= 0.8 else " shape (not structure)" if 4 in rn else " S pending")
    elif rp[4] >= 1:
        v = "H0"
    else:
        v = "inconclusive"
    print(f"{arm}: {v}  W/PKM " + " ".join(f"t{t}={rp[t]:.2f}" for t in ts)
          + "  W/null " + " ".join(f"t{t}={rn[t]:.2f}" for t in sorted(rn)))
