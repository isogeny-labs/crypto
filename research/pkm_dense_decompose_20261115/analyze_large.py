#!/usr/bin/env python3
"""EXP-PKM-DENSE-DEC-001 Amendments 1 and 3 (larger S, RunPod): validity and L1-L4. Cells completed on the first pod
come from runs/large/; every cell rerun under Amendment 3 comes from runs/rerun/ (its partial copies in runs/large/
serve only the cross-pod A/A check). The m = 3 growth fit also uses the frozen runs/m3-t5-dense.jsonl. Writes
results_large.json. Counted field_ops only."""
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


def cell_rows():
    """Rerun cells replace their first-pod copies; returns (rows, aa) with aa the cross-pod A/A result."""
    rerun = {}
    for f in sorted(glob.glob(os.path.join(HERE, "runs", "rerun", "*-*.jsonl"))):
        if f.endswith(".rss.jsonl"):
            continue
        rerun[os.path.basename(f)[:-6]] = [json.loads(l) for l in open(f) if l.strip()]
    rows, aa = [], {}
    for f in sorted(glob.glob(os.path.join(HERE, "runs", "large", "*.jsonl"))):
        name = os.path.basename(f)[:-6]
        first = [json.loads(l) for l in open(f) if l.strip()]
        if name not in rerun:
            rows += first
            continue
        new = {r["index"]: r for r in rerun[name]}
        strip = lambda r: {k: ({kk: vv for kk, vv in v.items() if kk != "ms"} if isinstance(v, dict) else v)  # noqa: E731
                           for k, v in r.items() if k != "threads"}
        aa[name] = {"compared": len(first),
                    "differ": [r["index"] for r in first if strip(r) != strip(new.get(r["index"], {}))]}
    for rs in rerun.values():
        rows += rs
    return rows, aa


def stopping_point():
    """Amendment 2: m = 2 cells not finished when m3-t7-dense ended are *not run*. Returns (stop time, excluded)."""
    ev = [l.split() for l in open(os.path.join(HERE, "runs", "rerun", "progress.txt")) if l.strip()]
    stop = min((e[2] for e in ev if e[0] in ("done", "failed") and e[1] == "m3-t7-dense"), default=None)
    if stop is None:
        return None, []
    return stop, sorted({e[1] for e in ev if e[0] in ("done", "failed") and e[2] > stop and e[1] != "m3-t7-dense"})


def maxrss(name):
    out = []
    for sub in ("large", "rerun"):
        p = os.path.join(HERE, "runs", sub, f"{name}.rss.json")
        if os.path.exists(p):
            out.append(json.load(open(p)))
        p += "l"
        if os.path.exists(p):
            out += [json.loads(l) for l in open(p) if l.strip()]
    return out


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
        att = maxrss(name)
        if att:
            rec["maxrss_gb"] = max(a["maxrss_kb"] for a in att) / 2**20
            rec["attempts"] = [{"rc": a["rc"], "wall_s": a.get("wall_s"), "pod": a.get("pod")} for a in att]
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
    rows, aa = cell_rows()
    stop, excluded = stopping_point()
    rows = [r for r in rows if r["cell"] not in excluded]
    lower = {}  # fully censored cells: counted multiplications at the budget cutoff are a lower bound
    for name in sorted({r["cell"] for r in rows}):
        rs = [r for r in rows if r["cell"] == name]
        if all(r["dense"] and r["dense"]["undetermined"] for r in rs):
            lower[name] = {"S": rs[0]["S"], "budget_s": rs[0]["budget_s"],
                           "ops_at_cutoff_median": median(r["dense"]["field_ops"] for r in rs),
                           "ops_at_cutoff_min": min(r["dense"]["field_ops"] for r in rs),
                           "over_enum_at_least": median(r["dense"]["field_ops"] / r["enum_field_ops"] for r in rs)}
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
        "validity": {"V1_disagreements": v1, "V2_planted_without_relation": v2, "cross_pod_AA": aa,
                     "valid": not v1 and not v2 and not any(a["differ"] for a in aa.values())},
        "stopping_point": {"m3_t7_ended": stop, "not_run": excluded},
        "censored_lower_bounds": lower,
        "not_finished": {n: maxrss(n) for n in ("m3-t7-dense",)
                         if not os.path.exists(os.path.join(HERE, "runs", "rerun", f"{n}.done"))},
        "L1": {"cells": L1, "verdict": v([x for x in L1 if x[1] >= 0.5], L1)},
        "L2": {"cells": L2, "verdict": v([x for x in L2 if x[1] <= 1], L2)},
        "L3": {"m2_points": m2, "m2_exponent": e2, "m3_points": m3, "m3_exponent": e3,
               "verdict_m2": v(e2 is not None and not (1.8 <= e2 <= 2.3), e2 is not None),
               "verdict_m3": v(e3 is not None and not (4.5 <= e3 <= 6.0), e3 is not None)},
        "L4": {"cells": L4, "verdict": v([x for x in L4 if not 0.5 < x[1] < 2], L4)},
        "cells": cells,
        "note": "Labelled extension (Amendments 1-3); frozen verdicts stand. No claim about cryptographic sizes or "
                "Pollard rho [pcc:unverified].",
    }
    json.dump(res, open(os.path.join(HERE, "results_large.json"), "w"), indent=1)
    print("stop", stop, "not run", excluded)
    print("lower", lower)
    print("valid", res["validity"]["valid"], "rows", len(rows), "AA", aa, "not finished", res["not_finished"])
    for k in ("L1", "L2", "L4"):
        print(k, res[k]["verdict"], res[k]["cells"])
    print("L3 m2", e2, res["L3"]["verdict_m2"], "m3", e3, res["L3"]["verdict_m3"])
    for c in cells:
        print(c["cell"], c["S"], c.get("dense_planted_ops_median"), c.get("tower_planted_ops_median"),
              c.get("maxrss_gb"), c["censored"])


if __name__ == "__main__":
    main()
