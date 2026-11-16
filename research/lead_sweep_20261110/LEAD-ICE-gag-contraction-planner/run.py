import json, sys
from math import comb
SEED = 20261110  # no randomness used; recorded for the protocol
def s_deg(k): return (k - 1) * 2 ** (k - 2)   # total degree of Semaev S_k
def presentation(kind, m, n):
    """(neqs, nvars, deg) of the Weil-descended relation system, n coords per point, m summands."""
    if kind == "mono": return (n, n * m, s_deg(m + 1))
    if kind in ("chain", "tree"): return ((m - 1) * n, n * (m + (m - 2)), s_deg(3))
    if kind == "dup": return (2 * (m - 1) * n, n * (m + (m - 2)), s_deg(3))  # positive control: redundant eqs
    raise ValueError(kind)
def first_D(neqs, nv, d, cap=400):
    for D in range(d, cap):
        rows = neqs * comb(nv + D - d, nv); cols = comb(nv + D, nv)
        if rows >= cols: return D, rows, cols
    return None
def cells(kind, m, n):
    r = first_D(*presentation(kind, m, n))
    return None if r is None else {"D": r[0], "rows": r[1], "cols": r[2], "cells": r[1] * r[2]}
def ratio(a, b, m, n):
    x, y = cells(a, m, n), cells(b, m, n)
    return None if not x or not y else x["cells"] / y["cells"]
def main():
    smoke = "--smoke" in sys.argv
    cfg = [(3, 3)] if smoke else [(3, 3), (3, 2), (4, 3), (3, 4), (4, 2)]
    out = {"lead": "LEAD-ICE-gag-contraction-planner", "seed": SEED, "smoke": smoke, "rows": []}
    for m, n in cfg:
        out["rows"].append({"m": m, "n": n,
            "mono_over_chain": ratio("mono", "chain", m, n),
            "tree_over_chain": ratio("tree", "chain", m, n),
            "chain_over_chain_AA": ratio("chain", "chain", m, n),
            "dup_over_chain_poscontrol": ratio("dup", "chain", m, n),
            "mono": cells("mono", m, n), "chain": cells("chain", m, n)})
    rs = out["rows"]
    ok_aa = all(r["chain_over_chain_AA"] == 1.0 for r in rs)
    ok_tree = all(r["tree_over_chain"] == 1.0 for r in rs)
    ok_pos = all(r["dup_over_chain_poscontrol"] is not None and r["dup_over_chain_poscontrol"] != 1.0 for r in rs)
    mono = [r["mono_over_chain"] for r in rs if r["mono_over_chain"] is not None]
    out["controls"] = {"AA_exact_1": ok_aa, "tree_equals_chain_topology_blind": ok_tree, "positive_control_detects_change": ok_pos}
    out["statistic"] = "mono_over_chain counted Macaulay cells at first D with rows>=cols"
    if not (ok_aa and ok_pos) or not mono: out["verdict"] = "inconclusive"
    elif ok_tree and all(x > 1 for x in mono): out["verdict"] = "consistent_with_prediction"
    elif any(x < 1 for x in mono) or not ok_tree: out["verdict"] = "falsified"
    else: out["verdict"] = "inconclusive"
    print(json.dumps(out)); return 0
sys.exit(main())
