#!/usr/bin/env python3
"""Round 2: W (max_cols) and E (D - input degree) per cell, and the PROTOCOL.md gates."""
import csv, glob, json, os, collections
D = os.path.dirname(os.path.abspath(__file__))
R1 = os.path.join(D, "..", "cover_image_fb_20261112", "cells.tsv")
cells = collections.defaultdict(list)
viol = 0
for path in sorted(glob.glob(os.path.join(D, "runs", "m*.jsonl"))):
    arm = os.path.basename(path)[3:-6]
    for line in open(path):
        r = json.loads(line)
        if "control" not in r:
            continue
        if r["target"] == "planted" and r["planted_ok"] is False:
            viol += 1
        cov = r.get("cover") or {}
        cells[(r["m"], arm, r["control"], r["t"])].append(
            dict(W=r["max_cols"], E=r["solving_degree_max"] - max(r["input_degrees"]),
                 D=r["solving_degree_max"], din=max(r["input_degrees"]), to=r["timed_out"],
                 F=cov.get("base_size", 2 ** r["t"]), g=cov.get("genus")))
W, out = {}, ["m\tarm\tcontrol\tt\tF\tgenus\tW\tD\tinput_degree\tE\tcensored"]
for k in sorted(cells):
    rs = cells[k]
    cen = any(x["to"] for x in rs)
    w = max(x["W"] for x in rs)
    if not cen:
        W[k] = w
    out.append("\t".join(map(str, [*k, rs[0]["F"], rs[0]["g"], w, max(x["D"] for x in rs),
                                   rs[0]["din"], max(x["E"] for x in rs), int(cen)])))
open(os.path.join(D, "cells.tsv"), "w").write("\n".join(out) + "\n")
print("\n".join(out))
print(f"\nplanted violations: {viol}")
# R0: quad replicates round 1 (Kummer rows).
r1 = {(int(r["m"]), r["control"], int(r["t"])): int(r["max_cols"])
      for r in csv.DictReader(open(R1), delimiter="\t") if r["arm"] == "quad" and r["kind"] == "kummer"}
r0 = [(c, t, W[(2, "quad", c, t)], r1[(2, c, t)]) for (m, a, c, t) in W
      if m == 2 and a == "quad" and (2, c, t) in r1]
print("R0", "PENDING" if not r0 else "PASS" if all(x[2] == x[3] for x in r0) else "FAIL", r0)
# P0: exact positive controls.
p0 = []
for arm, shift in (("square", 1), ("fourth", 2)):
    for (m, a, c, t) in list(W):
        if m == 2 and a == arm and c == "tower" and (2, "none", "tower", t - shift) in W:
            p0.append((arm, t, W[(2, arm, "tower", t)], W[(2, "none", "tower", t - shift)]))
print("P0", "PENDING" if not p0 else "PASS" if all(x[2] == x[3] for x in p0) else "FAIL", p0)
# H1 / H0 per main arm.
for arm in ("cubic", "quart", "generic3"):
    ts = sorted(t for (m, a, c, t) in W if m == 2 and a == arm and c == "tower"
                and (2, arm, "null", t) in W and (2, "none", "tower", t) in W)
    rn = {t: W[(2, arm, "tower", t)] / W[(2, arm, "null", t)] for t in ts}
    rp = {t: W[(2, arm, "tower", t)] / W[(2, "none", "tower", t)] for t in ts}
    h1 = sum(rn[t] <= 0.8 for t in ts if t in (5, 6, 7)) >= 2 or (
        len(ts) >= 2 and all(rp[t] < 1 for t in ts[-2:]))
    big = [t for t in ts if t >= 5]
    h0 = bool(big) and all(0.8 <= rn[t] <= 1.25 and rp[t] > 1 for t in big)
    verdict = "H1" if h1 else "H0" if h0 else "inconclusive"
    print(f"{arm}: {verdict}  W/null " + " ".join(f"t{t}={rn[t]:.2f}" for t in ts)
          + "  W/PKM " + " ".join(f"t{t}={rp[t]:.2f}" for t in ts))
