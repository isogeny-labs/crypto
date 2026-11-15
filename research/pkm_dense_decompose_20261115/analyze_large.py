#!/usr/bin/env python3
"""EXP-PKM-DENSE-DEC-001 Amendment 1 (larger S, RunPod): validity and L1-L4. Reads runs/large/ (and, for the m = 3
growth fit, the frozen runs/m3-t5-dense.jsonl). Writes results_large.json. Counted field_ops only."""
import glob
import json
import math
import os
from statistics import median

HERE = os.path.dirname(os.path.abspath(__file__))
P1, P3 = 1399680001, 3221225473


def load(pattern):
    rows = []
    for f in sorted(glob.glob(os.path.join(HERE, pattern))):
        rows += [json.loads(l) for l in open(f) if l.strip()]
    return rows


def ok(s):
    return s is not None and not s["undetermined"]


def summarize(rows):
    cells = {}
    for r in rows:
        cells.setdefault(r["cell"], []).append(r)
    out = []
    for name, rs in cells.items():
        r0 = rs[0]
        pl = [r for r in rs if r["kind"] == "planted"]
        rec = {"cell": name, "p": r0["p"], "m": r0["m"], "S": r0["S"], "vkind": r0["vkind"],
               "membership": r0["membership"], "rows": len(rs), "enum_ops": r0["enum_field_ops"],
               "censored": [(r["index"], a) for r in rs for a in ("tower", "dense") if r.get(a) and r[a]["undetermined"]]}
        for arm in ("tower", "dense"):
            good = [r for r in pl if ok(r.get(arm))]
            if good:
                rec[f"{arm}_planted_ops_median"] = median(r[arm]["field_ops"] for r in good)
                rec[f"{arm}_over_enum_median"] = median(r[arm]["ratio_over_enum"] for r in good)
                rec[f"{arm}_ms_median"] = median(r[arm]["ms"] for r in good)
        pr = [r["ratio_dense_over_tower"] for r in pl if r["ratio_dense_over_tower"] is not None]
        if pr:
            rec["dense_over_tower_median"] = median(pr)
        rss = os.path.join(HERE, "runs", "large", f"{name}.rss.json")
        if os.path.exists(rss):
            rec["maxrss_gb"] = json.load(open(rss))["maxrss_kb"] / 2**20
        out.append(rec)
    return sorted(out, key=lambda c: (c["p"], c["m"], c["S"], c["vkind"]))


def fit(points):
    """Least-squares slope of log(ops) on log(S)."""
    if len(points) < 2:
        return None
    xs = [math.log2(s) for s, _ in points]
    ys = [math.log2(v) for _, v in points]
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)


def main():
    rows = load("runs/large/*.jsonl")
    v1 = [(r["cell"], r["index"], a) for r in rows for a in ("tower", "dense")
          if ok(r.get(a)) and not r[a]["agree"]]
    v2 = [(r["cell"], r["index"]) for r in rows if r["kind"] == "planted" and r["enum_relations"] == 0]
    cells = summarize(rows)
    frozen = summarize(load("runs/m3-t5-dense.jsonl"))
    L1 = [(c["S"], c["dense_over_tower_median"]) for c in cells
          if c["p"] == P1 and c["m"] == 2 and "dense_over_tower_median" in c]
    L2 = [(c["cell"], c["dense_over_enum_median"]) for c in cells if "dense_over_enum_median" in c]
    m2 = [(c["S"], c["dense_planted_ops_median"]) for c in cells
          if c["p"] == P3 and c["m"] == 2 and c["vkind"] == "coset" and "dense_planted_ops_median" in c]
    m3 = [(c["S"], c["dense_planted_ops_median"]) for c in frozen + cells
          if c["p"] == P1 and c["m"] == 3 and c["vkind"] == "coset" and "dense_planted_ops_median" in c]
    e2, e3 = fit(m2), fit(m3)
    L4 = []
    for c in cells:
        if c["vkind"] == "random" and c["S"] >= 64 and "dense_planted_ops_median" in c:
            co = [d for d in cells if d["p"] == c["p"] and d["m"] == c["m"] and d["S"] == c["S"]
                  and d["vkind"] == "coset" and "dense_planted_ops_median" in d]
            if co:
                L4.append((c["cell"], c["dense_planted_ops_median"] / co[0]["dense_planted_ops_median"]))
    v = lambda bad, data: "falsified" if bad else ("held" if data else "no data")  # noqa: E731
    res = {
        "study": "EXP-PKM-DENSE-DEC-001 Amendment 1 (larger S)",
        "validity": {"V1_disagreements": v1, "V2_planted_without_relation": v2, "valid": not v1 and not v2},
        "L1": {"cells": L1, "verdict": v([x for x in L1 if x[1] >= 0.5], L1)},
        "L2": {"cells": L2, "verdict": v([x for x in L2 if x[1] <= 1], L2)},
        "L3": {"m2_points": m2, "m2_exponent": e2, "m3_points": m3, "m3_exponent": e3,
               "verdict_m2": v(e2 is not None and not (1.8 <= e2 <= 2.3), e2 is not None),
               "verdict_m3": v(e3 is not None and not (4.5 <= e3 <= 6.0), e3 is not None)},
        "L4": {"cells": L4, "verdict": v([x for x in L4 if not 0.5 < x[1] < 2], L4)},
        "cells": cells,
        "note": "Labelled extension (Amendment 1); frozen verdicts stand. No claim about cryptographic sizes or "
                "Pollard rho [pcc:unverified].",
    }
    json.dump(res, open(os.path.join(HERE, "results_large.json"), "w"), indent=1)
    print("valid", res["validity"]["valid"], "rows", len(rows))
    for k in ("L1", "L2", "L4"):
        print(k, res[k]["verdict"], res[k]["cells"])
    print("L3 m2", e2, res["L3"]["verdict_m2"], "m3", e3, res["L3"]["verdict_m3"])
    for c in cells:
        print(c["cell"], c["S"], c.get("dense_planted_ops_median"), c.get("tower_planted_ops_median"),
              c.get("maxrss_gb"), c["censored"])


if __name__ == "__main__":
    main()
