#!/usr/bin/env python3
"""T10 frozen analysis: applies PROTOCOL.md's rules to data/cells/*.json.

E_w(t, G, s) = columns * (mpt + w * nat) / coverage_exact, where
mpt = muladds_per_target and nat = (sum of native.*_uncharged) / targets_done.
Writes analysis/summary.json. Usage: python3 -I analyze.py <lead dir>
"""
import glob
import json
import os
import sys

SEL = {5: 1307, 6: 1308, 7: 1594}
WEIGHTS = (0.0, 0.1, 1.0)
P0_REF = 710094.0            # t6 G4 seed 0, target seed 2026111506, prior binary
PRED_WINNER = (5, 4, 1307)


def load(lead):
    cells, failed = {}, []
    census = json.load(open(os.path.join(lead, "selection", "seeds.json")))["cells"]
    for t in (5, 6, 7):
        for g in range(t):
            for s in (0, SEL[t]):
                f = os.path.join(lead, "data", "cells", f"t{t}_g{g}_s{s}.json")
                try:
                    d = json.load(open(f))
                except Exception:
                    failed.append((t, g, s, "missing/failed"))
                    continue
                n = d.get("targets_done", 0)
                if d.get("capped") and n < 10:
                    failed.append((t, g, s, f"capped with {n} targets"))
                    continue
                nat = sum(v for k, v in d.get("native", {}).items() if k.endswith("_uncharged")) / max(n, 1)
                cen = census[str(t)]["seed0" if s == 0 else "selected_J_per_col"]
                cells[(t, g, s)] = {
                    "mpt": d["muladds_per_target"], "nat": nat, "cols": d["columns"],
                    "cov": d["coverage_exact"], "covered": d["covered_points"],
                    "targets": n, "capped": bool(d.get("capped")), "undecided": d.get("undecided", 0),
                    "degree": d.get("solving_degree_max"), "hits": d.get("hits"),
                    "census_match": d["columns"] == cen["columns"] and d["covered_points"] == cen["covered_points"],
                }
                for w in WEIGHTS:
                    c = cells[(t, g, s)]
                    c[f"E_{w}"] = c["cols"] * (c["mpt"] + w * c["nat"]) / c["cov"]
    return cells, failed


def main(lead):
    cells, failed = load(lead)
    key = lambda c: f"t{c[0]}_g{c[1]}_s{c[2]}"
    out = {"cells": {key(c): v for c, v in cells.items()}, "failed": failed, "verdicts": {}}
    V = out["verdicts"]
    flagged = [key(c) for c, v in cells.items() if v["capped"] or v["undecided"]]
    out["flagged_capped_or_undecided"] = flagged

    # P0: instrument reproduction.
    c = cells.get((6, 4, 0))
    V["P0"] = None if c is None else {"value": c["mpt"], "rel": abs(c["mpt"] / P0_REF - 1),
                                      "held": abs(c["mpt"] / P0_REF - 1) <= 1e-3}
    # P1: census consistency.
    V["P1"] = {"mismatch": [key(c) for c, v in cells.items() if not v["census_match"]]}
    V["P1"]["held"] = not V["P1"]["mismatch"]
    # P2: seed invariance of mpt.
    pairs = {}
    for t in (5, 6, 7):
        for g in range(t):
            a, b = cells.get((t, g, 0)), cells.get((t, g, SEL[t]))
            if a and b:
                pairs[f"t{t}_g{g}"] = b["mpt"] / a["mpt"]
    V["P2"] = {"ratios": pairs, "n": len(pairs), "outside": [k for k, r in pairs.items() if not 0.90 <= r <= 1.10]}
    V["P2"]["held"] = len(pairs) == 18 and not V["P2"]["outside"]
    # P3: mpt falls with every G step, argmin at G = t-1, every step ratio >= 1.25.
    series = {}
    for t in (5, 6, 7):
        for s in (0, SEL[t]):
            m = [cells.get((t, g, s), {}).get("mpt") for g in range(t)]
            ok = all(x is not None for x in m)
            steps = [m[g] / m[g + 1] for g in range(t - 1)] if ok else None
            series[f"t{t}_s{s}"] = {"complete": ok, "steps": steps,
                                    "argmin_G": (min(range(t), key=lambda g: m[g]) if ok else None)}
    V["P3"] = {"series": series}
    V["P3"]["held"] = all(v["complete"] and v["argmin_G"] == int(k[1]) - 1 and min(v["steps"]) >= 1.25
                          for k, v in series.items())
    # P4-P7: winner and magnitudes (w = 0 is the ranking weight; P7 checks the band).
    if cells:
        win = min(cells, key=lambda c: cells[c]["E_0.0"])
        ranked = sorted(cells, key=lambda c: cells[c]["E_0.0"])
        out["ranking_w0"] = [(key(c), cells[c]["E_0.0"]) for c in ranked]
        inc = cells.get((6, 4, 0))
        inc_sel = cells.get((6, 4, SEL[6]))
        relevant = [(t, t - 1, s) for t in (5, 6, 7) for s in (0, SEL[t])] + [(6, 4, 0), (6, 4, SEL[6])]
        incomplete = [key(c) for c in relevant if c not in cells or key(c) in flagged]
        V["P4"] = {"winner": key(win), "E_w0": cells[win]["E_0.0"], "held": win == PRED_WINNER,
                   "inconclusive": incomplete}
        if inc:
            r = inc["E_0.0"] / cells[win]["E_0.0"]
            V["P5"] = {"incumbent_over_winner": r, "held": r >= 10.0,
                       "selseed_incumbent_over_winner": (inc_sel["E_0.0"] / cells[win]["E_0.0"]) if inc_sel else None}
        best_tg = {}
        for (t, g, s), v in cells.items():
            if (t, g) not in best_tg or v["E_0.0"] < best_tg[(t, g)][1]:
                best_tg[(t, g)] = (s, v["E_0.0"])
        if (6, 4) in best_tg:
            other = {tg: x for tg, x in best_tg.items() if tg != (6, 4)}
            btg = min(other, key=lambda tg: other[tg][1])
            r6 = best_tg[(6, 4)][1] / other[btg][1]
            V["P6"] = {"best_other_tg": f"t{btg[0]}_g{btg[1]}_s{other[btg][0]}", "ratio_vs_best_t6g4": r6,
                       "held": r6 > 1.5, "recommend": r6 > 1.5}
        if inc:
            band = {str(w): inc[f"E_{w}"] / cells[win][f"E_{w}"] for w in WEIGHTS}
            V["P7"] = {"incumbent_over_winner_by_w": band, "held": all(x > 1.0 for x in band.values())}
    c = cells.get((5, 4, 0))
    V["P8"] = None if c is None else {"value": c["mpt"], "held": 3.0e4 <= c["mpt"] <= 8.0e4}
    os.makedirs(os.path.join(lead, "analysis"), exist_ok=True)
    json.dump(out, open(os.path.join(lead, "analysis", "summary.json"), "w"), indent=1, sort_keys=True)
    print(json.dumps(V, indent=1, sort_keys=True))


if __name__ == "__main__":
    main(sys.argv[1])
