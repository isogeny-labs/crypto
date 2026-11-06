#!/usr/bin/env python3
"""Where does S go on the ladder rows?  Recompute, from the committed LEAD-19 records,
the mean share of each phase in total group-addition equivalents (GAE) for the
index-calculus rows, and the S ratio of the three-summand to the two-summand walk.

Analysis only (repository rule: Python for analysis and rendering).  Reads
research/lead19_projected_base/char2_repeats20.json (20 repeats, schema v1) and
writes phase_shares.json and phase_shares.md next to this script.

    python3 phase_shares.py [path/to/char2_repeats20.json]

Phase fields in the record: factor_base (table build plus the scan or projection
charge), relations (the walk: candidates, lookups, restarts), linear_algebra
(the matrix), verify.  S = total_gae / sqrt(r).
"""
import json
import math
import statistics as st
import sys
from pathlib import Path

SRC = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(
    "research/lead19_projected_base/char2_repeats20.json")
HERE = Path(__file__).resolve().parent

# Published rungs of arm B (research/lead19_projected_base/README.md section 3):
# std base at f2m15 and f2m21, balanced base at f2m27.
B_ROWS = {
    "f2m15": ("mitm_m2_negfold_walk_projected", 13.99),
    "f2m21": ("mitm_m2_negfold_walk_projected", 20.00),
    "f2m27": ("mitm_m2_negfold_walk_projected_balanced", 24.41),
}
PHASES = (("factor_base", "factor base"), ("relations", "walk"),
          ("linear_algebra", "matrix"), ("verify", "verify"))


def mean_share(vs, key):
    return st.mean(v[key]["gae"] / v["total_gae"] for v in vs)


def main():
    d = json.loads(SRC.read_text())
    out = {"source": str(SRC), "repeats": d["config"]["repeats"], "rows": [],
           "m3_over_m2_raw": []}
    for inst in d["ledger"]["instances"]:
        by = {}
        for v in inst["variants"]:
            by.setdefault(v["name"], []).append(v)
        sc = math.sqrt(inst["r"])
        m = lambda n: st.mean(v["s"] for v in by[n])
        out["m3_over_m2_raw"].append({
            "log2_r": round(inst["log2_r"], 2), "cofactor": inst["cofactor"],
            "ratio": round(m("mitm_m3_negfold_walk") / m("mitm_m2_negfold_walk"), 3)})
        for label, (name, l2) in B_ROWS.items():
            if abs(inst["log2_r"] - l2) > 0.05:
                continue
            vs = by[name]
            out["rows"].append({
                "row": label, "arm": "B (projected)", "variant": name,
                "log2_r": round(inst["log2_r"], 2), "cofactor": inst["cofactor"],
                "n": len(vs),
                "S_mean": round(st.mean(v["total_gae"] for v in vs) / sc, 3),
                "share_percent": {k: round(100 * mean_share(vs, k), 3) for k, _ in PHASES},
            })
    (HERE / "phase_shares.json").write_text(json.dumps(out, indent=2) + "\n")

    lines = ["| row | variant | S (mean of 20) | factor base | walk | matrix | verify |",
             "|:--|:--|--:|--:|--:|--:|--:|"]
    for r in out["rows"]:
        s = r["share_percent"]
        lines.append(f"| {r['row']} (h={r['cofactor']}) | `{r['variant']}` | {r['S_mean']:.2f} | "
                     f"{s['factor_base']:.1f}% | {s['relations']:.1f}% | "
                     f"{s['linear_algebra']:.3f}% | {s['verify']:.2f}% |")
    lines += ["", "S(three-summand negfold walk) / S(two-summand negfold walk), raw base, ratio of means:", "",
              "| log2 r | h | ratio |", "|--:|--:|--:|"]
    for r in out["m3_over_m2_raw"]:
        lines.append(f"| {r['log2_r']} | {r['cofactor']} | {r['ratio']} |")
    (HERE / "phase_shares.md").write_text("\n".join(lines) + "\n")
    print("\n".join(lines))


if __name__ == "__main__":
    main()
