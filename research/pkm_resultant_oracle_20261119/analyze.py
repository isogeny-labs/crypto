#!/usr/bin/env python3
"""EXP-PKM-RESULTANT-001: validity and R1-R5 from runs/ (frozen PROTOCOL.md). Writes results.json.
Dense comparison rows come from EXP-PKM-DENSE-DEC-001 (runs/, runs/large/, runs/rerun/), joined on the same target."""
import glob
import json
import math
import os
from statistics import median

HERE = os.path.dirname(os.path.abspath(__file__))
DD = os.path.join(HERE, "..", "pkm_dense_decompose_20261115", "runs")
P1, P3 = 1399680001, 3221225473
NOT_RUN = {"P3-m2-t14-random", "P3-m2-t15-dense", "P3-m2-t15-random"}  # DENSE-DEC Amendment 2


def rows(pattern):
    out = []
    for f in sorted(glob.glob(pattern)):
        if f.endswith(".rss.jsonl") or "controller" in f:
            continue
        out += [json.loads(l) for l in open(f) if l.strip()]
    return out


def key(r):
    return (r["p"], r["m"], r["S"], r["vkind"], r["seed"], r["kind"], r["index"])


def fit(points):
    xs = [math.log2(s) for s, _ in points]
    ys = [math.log2(v) for _, v in points]
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)


def main():
    res = rows(os.path.join(HERE, "runs", "*.jsonl"))
    dense = {}
    for r in rows(os.path.join(DD, "*.jsonl")) + rows(os.path.join(DD, "large", "*.jsonl")) + \
            rows(os.path.join(DD, "rerun", "*.jsonl")):
        if r["m"] != 2 or r["cell"] in NOT_RUN:
            continue
        dense[key(r)] = r  # rerun files come last and replace first-pod partial copies
    v1_disagree = [(r["cell"], r["index"]) for r in res if not r["resultant"]["agree"]]
    v1_planted_empty = [(r["cell"], r["index"]) for r in res if r["kind"] == "planted" and r["enum_relations"] == 0]
    v1_target_mismatch, shared = [], 0
    for r in res:
        d = dense.get(key(r))
        if d is None:
            continue
        shared += 1
        if (d["target"], d["enum_relations"], d["enum_field_ops"]) != (r["target"], r["enum_relations"], r["enum_field_ops"]):
            v1_target_mismatch.append((r["cell"], r["index"]))
    cells = {}
    for r in res:
        cells.setdefault(r["cell"], []).append(r)
    summ = []
    for name, rs in cells.items():
        pl = [r for r in rs if r["kind"] == "planted"]
        c = {"cell": name, "p": rs[0]["p"], "S": rs[0]["S"], "vkind": rs[0]["vkind"], "rows": len(rs),
             "res_ops": median(r["resultant"]["field_ops"] for r in pl),
             "enum_ops": median(r["enum_field_ops"] for r in pl),
             "res_over_enum": median(r["resultant"]["ratio_over_enum"] for r in pl),
             "stage_share": {k: median(r["resultant"][k] / r["resultant"]["field_ops"] for r in pl)
                             for k in ("tree_ops", "product_ops", "eval_ops", "finish_ops")},
             "res_ms": median(r["resultant"]["ms"] for r in pl)}
        dr = [(r["resultant"]["field_ops"], dense[key(r)]["dense"]) for r in pl
              if key(r) in dense and dense[key(r)].get("dense") and not dense[key(r)]["dense"]["undetermined"]]
        if dr:
            c["dense_ops"] = median(d["field_ops"] for _, d in dr)
            c["res_over_dense"] = median(a / d["field_ops"] for a, d in dr)
        tw = [(r["resultant"]["field_ops"], dense[key(r)]["tower"]) for r in pl
              if key(r) in dense and dense[key(r)].get("tower") and not dense[key(r)]["tower"]["undetermined"]]
        if tw:
            c["res_over_tower"] = median(a / t["field_ops"] for a, t in tw)
        summ.append(c)
    summ.sort(key=lambda c: (c["p"], c["vkind"], c["S"]))
    v = lambda bad, data: "falsified" if bad else ("held" if data else "no data")  # noqa: E731
    R1 = [(c["cell"], c["S"], c["res_over_dense"]) for c in summ if "res_over_dense" in c and c["S"] >= 256]
    R2 = [(c["cell"], c["res_over_enum"]) for c in summ]
    p3c = sorted((c["S"], c) for c in summ if c["p"] == P3 and c["vkind"] == "coset")
    e3 = fit([(s, c["res_ops"]) for s, c in p3c]) if len(p3c) > 1 else None
    by = {c["S"]: c for s, c in p3c}
    r4 = by[65536]["res_over_enum"] / by[256]["res_over_enum"] if 65536 in by and 256 in by else None
    R5 = []
    for s, c in p3c:
        rnd = [d for d in summ if d["p"] == P3 and d["vkind"] == "random" and d["S"] == s]
        if rnd:
            R5.append((s, rnd[0]["res_ops"] / c["res_ops"]))
    out = {
        "study": "EXP-PKM-RESULTANT-001",
        "validity": {"disagreements": v1_disagree, "planted_without_relation": v1_planted_empty,
                     "shared_targets_checked": shared, "shared_target_mismatches": v1_target_mismatch,
                     "valid": not (v1_disagree or v1_planted_empty or v1_target_mismatch)},
        "R1": {"cells": R1, "verdict": v([x for x in R1 if x[2] >= 0.01], R1)},
        "R2": {"cells": R2, "verdict": v([x for x in R2 if x[1] <= 1], R2)},
        "R3": {"exponent": e3, "points": [(s, c["res_ops"]) for s, c in p3c],
               "verdict": v(e3 is not None and not (1.05 <= e3 <= 1.45), e3 is not None)},
        "R4": {"ratio_growth_2^8_to_2^16": r4, "verdict": v(r4 is not None and not r4 < 16, r4 is not None)},
        "R5": {"cells": R5, "verdict": v([x for x in R5 if not 0.8 < x[1] < 1.25], R5)},
        "cells": summ,
        "note": "Counted F_p multiplications (NTT words at 1 M, inversions at 2 log2 p). "
                "No claim about cryptographic sizes or Pollard rho [pcc:unverified].",
    }
    json.dump(out, open(os.path.join(HERE, "results.json"), "w"), indent=1)
    print("valid", out["validity"]["valid"], "rows", len(res), "shared", shared)
    for k in ("R1", "R2", "R3", "R4", "R5"):
        print(k, out[k]["verdict"])
    for c in summ:
        print(f'{c["cell"]:22} S={c["S"]:6} res={c["res_ops"]:.3g} enum={c["enum_ops"]:.3g} '
              f'res/enum={c["res_over_enum"]:.2f} res/dense={c.get("res_over_dense", float("nan")):.4g} '
              f'res/tower={c.get("res_over_tower", float("nan")):.3g} eval%={c["stage_share"]["eval_ops"]:.2f}')
    print("R3 exponent", e3, "R4", r4, "R5", R5)


if __name__ == "__main__":
    main()
