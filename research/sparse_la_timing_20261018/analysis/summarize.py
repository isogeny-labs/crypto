#!/usr/bin/env python3
"""Lay out recorded values of one session as a summary table (no new
measurement). Inputs: the session's records.jsonl, its `ecbench band --json`
output and its `ecbench compare --save` files. Every ratio with an interval
is ecbench's; medians and per-workload means are plain aggregations of
recorded counts, labelled as such.

    python3 analysis/summarize.py SESSION_DIR BAND_JSON BASE_PREFIX OUT_JSON OUT_MD
"""
import json
import statistics
import sys
from pathlib import Path

sess, band_p, base, out_json, out_md = sys.argv[1:6]
sess = Path(sess)
recs = [json.loads(l) for l in open(sess / "records.jsonl")]
band = json.load(open(band_p))
weights = band["weights"]
band_s = {r["arm"]: r["mean_s"] for r in band["rows"]}

ARMS = ["sg", "sg-aa", "ig", "igfr", "sparse-cado", "filter-wiedemann",
        "filter-lanczos", "wiedemann", "lanczos"]


def split(arm):
    head, _, tail = arm.rpartition("-")
    return (head, int(tail)) if tail.isdigit() else (arm, None)


def phase(r, name):
    for p in r["phases"]:
        if p["name"] == name:
            return p
    return {}


by_arm = {}
for r in recs:
    by_arm.setdefault(r["arm"], []).append(r)

rho_arm = "rho-neg"
rho_s = band_s[rho_arm]
rows = []
for arm, rs in sorted(by_arm.items()):
    if arm == rho_arm:
        continue
    kind, size = split(arm)
    la = [phase(r, "linear_algebra") for r in rs]
    row = {
        "arm": arm,
        "linalg_arm": kind,
        "columns_param": size,
        "runs": len(rs),
        "verified": sum(r["outcome"]["status"] == "verified" for r in rs),
        "s_band": dict(zip(map(str, weights), band_s[arm])),
        "gap_to_rho_band": {str(w): s / rho_s[i] for i, (w, s) in enumerate(zip(weights, band_s[arm]))},
        "median_relations_found": statistics.median(r["counters"].get("relations_found", 0) for r in rs),
        "median_matrix_rank": statistics.median(r["counters"].get("matrix_rank", 0) for r in rs),
        "median_targets_tried": statistics.median(r["counters"].get("targets_tried", 0) for r in rs),
        "median_la_row_ops": statistics.median((p.get("native") or {}).get("row_ops", 0) for p in la),
        "median_la_wall_ms": statistics.median((p.get("wall_ns") or 0) / 1e6 for p in la),
        "median_relations_wall_ms": statistics.median((phase(r, "relations").get("wall_ns") or 0) / 1e6 for r in rs),
    }
    base_arm = f"{base}-{size:04d}" if size is not None else base
    cmp_p = sess / "comparisons" / f"{base_arm}__{arm}.json"
    if cmp_p.exists():
        c = json.load(open(cmp_p))
        row["vs_base"] = {"base": base_arm, "ratio": c["ops"]["ratio_b_over_a"],
                          "ci95": c["ops"]["ci95"], "status": c["ops"]["status"],
                          "wall_median_ratio_L0": c["wall"].get("median_ratio")}
    rows.append(row)

order = {a: i for i, a in enumerate(ARMS)}
rows.sort(key=lambda r: (r["columns_param"] or 0, order.get(r["linalg_arm"], 99)))
json.dump({"session": str(sess), "weights": weights, "rho_s_band": rho_s, "rows": rows},
          open(out_json, "w"), indent=1)

w = [str(x) for x in weights]
lines = ["| size | arm | S w=0 | S w=0.1 | S w=1 | ÷ " + base + " (w=0) [95%] | relations | LA row_ops | LA wall ms (L0) | gap to rho w=0 / w=1 |",
         "|--:|:--|--:|--:|--:|:--|--:|--:|--:|:--|"]
for r in rows:
    v = r.get("vs_base")
    vb = f"{v['ratio']:.3f} [{v['ci95'][0]:.3f}, {v['ci95'][1]:.3f}]" if v else "—"
    lines.append(
        f"| {r['columns_param'] or '—'} | {r['linalg_arm']} | {r['s_band'][w[0]]:.4g} | {r['s_band'][w[1]]:.4g} | "
        f"{r['s_band'][w[2]]:.4g} | {vb} | {r['median_relations_found']:g} | {r['median_la_row_ops']:.4g} | "
        f"{r['median_la_wall_ms']:.3g} | {r['gap_to_rho_band'][w[0]]:.3g}× / {r['gap_to_rho_band'][w[2]]:.3g}× |")
lines.append("")
lines.append(f"rho-neg S at w = {', '.join(w)}: " + ", ".join(f"{s:.4g}" for s in rho_s))
open(out_md, "w").write("\n".join(lines) + "\n")
