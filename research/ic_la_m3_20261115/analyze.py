#!/usr/bin/env python3
"""Tabulate the ecbench session of specs/la.json (PROTOCOL.md) into RESULTS.md and results.json.
Per workload (curve, target) and base size: the identity check across the three solvers (trials,
relations, recovered, status, S, total GAE, relation-phase additions); linear-algebra row_ops and wall
per solver; per curve the medians and the ratios to incremental-gauss; and the §5a band: per arm,
cost_w = total_gae + w * (sum of the record's unpriced native counts), w = 0, 0.1, 1, against the
matched rho's cost_w (means over the curve's records).  Values are the records'; ratios are derived.
    analyze.py SESSION_DIR OUT_DIR
"""
import json, math, re, statistics, sys
from collections import defaultdict
from pathlib import Path

S = Path(sys.argv[1]); O = Path(sys.argv[2])
recs = [json.loads(l) for l in (S / "records.jsonl").read_text().splitlines() if l.strip()]
med = statistics.median

def phase(r, n):
    return next((p for p in r["phases"] if p["name"] == n), {})

NON_OPERATION = {"residual_table_bytes_uncharged", "factor_base_vec_bytes_floor_uncharged",
                 "native_table_bytes_uncharged"}
TABLE_WORK = ["table_insert_attempts"]

def unpriced_total(r):
    """The band's U for one record, as `ecbench band` (src/cryptanalysis/ecbench/band.rs) reads it:
    each counter named in cost.unpriced from the top-level counters, else summed over the phases'
    `native` maps as `X_uncharged` or `X`; memory-size counters excluded; plus the table work
    (`table_insert_attempts` in `work_native`) where a phase carries its `counted:` marker.
    (Corrected after the validator review: the first version read the phases' native maps only, so
    rho's `canonicalisations_uncharged`, a top-level counter, counted as zero.)"""
    tot = 0
    for name in r["cost"].get("unpriced", []):
        if name in NON_OPERATION:
            continue
        if name in r.get("counters", {}):
            tot += r["counters"][name]; continue
        base = name[: -len("_uncharged")] if name.endswith("_uncharged") else name
        for p in r["phases"]:
            nat = p.get("native", {})
            v = nat.get(name, nat.get(base))
            if isinstance(v, (int, float)):
                tot += v
    for name in TABLE_WORK:
        if any(f"counted:{name}" in p.get("work_native", {}) for p in r["phases"]):
            tot += sum(p.get("work_native", {}).get(name, 0) for p in r["phases"])
    return tot

def la_row_ops(r):
    la = phase(r, "linear_algebra")
    return la.get("native", {}).get("row_ops", 0)

status = defaultdict(lambda: defaultdict(int))
groups = defaultdict(dict)   # (curve, wid, size) -> solver -> rec
rho = defaultdict(list)
bits = {}
for r in recs:
    if r.get("warmup"):
        continue
    c = r["workload"]["curve"]["slug"]; bits[c] = math.log2(r["workload"]["curve"]["r"])
    status[r["arm"]][r["outcome"]["status"]] += 1
    if r["arm"] == "rho-neg":
        rho[c].append(r); continue
    m = re.match(r"rf3-(gauss|echelon|structured)-F(\d+)$", r["arm"])
    groups[(c, r["workload"]["workload_id"], int(m.group(2)))][m.group(1)] = r

md = ["Record status: " + "; ".join(f"{a} {dict(v)}" for a, v in sorted(status.items()))]
keys = [("trials", lambda x: x["counters"].get("targets_tried")), ("relations", lambda x: x["counters"].get("relations_found")),
        ("recovered", lambda x: x["outcome"]["recovered"]), ("status", lambda x: x["outcome"]["status"]),
        ("S", lambda x: x["cost"]["s"]), ("total_gae", lambda x: x["cost"]["total_gae"]),
        ("relation_adds", lambda x: phase(x, "relations").get("adds"))]
mism = []
for (c, wid, sz), kv in groups.items():
    base = kv.get("gauss")
    for sol, r in kv.items():
        if sol == "gauss" or base is None:
            continue
        for k, f in keys:
            if f(base) != f(r):
                mism.append(f"{c} {wid} F{sz} {sol}: {k} {f(base)} != {f(r)}")
md.append(f"Identity checks (echelon and structured against gauss, per workload): {sum(len(v) - 1 for v in groups.values())}; mismatches: {len(mism)}")
md += [f"- {m}" for m in mism]

def rho_cost(c, w):
    return statistics.fmean(r["cost"]["total_gae"] + w * unpriced_total(r) for r in rho[c])

rows = []
md.append("\n| curve | log2 r | \\|F_x\\| | solver | LA row_ops (median) | vs gauss | LA wall s | vs gauss | S / rho | cost / rho at w = 0.1 | at w = 1 |")
md.append("|---|--:|--:|---|--:|--:|--:|--:|--:|--:|--:|")
per = defaultdict(lambda: defaultdict(list))
for (c, wid, sz), kv in groups.items():
    for sol, r in kv.items():
        per[(c, sz)][sol].append(r)
for (c, sz) in sorted(per, key=lambda k: (bits[k[0]], k[1])):
    g = per[(c, sz)]
    base_ops = med(la_row_ops(r) for r in g["gauss"]); base_wall = med((phase(r, "linear_algebra").get("wall_ns") or 0) / 1e9 for r in g["gauss"])
    for sol in ("gauss", "echelon", "structured"):
        rs = g.get(sol, [])
        if not rs:
            continue
        ops = med(la_row_ops(r) for r in rs); wall = med((phase(r, "linear_algebra").get("wall_ns") or 0) / 1e9 for r in rs)
        band = {w: statistics.fmean(r["cost"]["total_gae"] + w * unpriced_total(r) for r in rs) / rho_cost(c, w) for w in (0.0, 0.1, 1.0)}
        rows.append({"curve": c, "log2_r": bits[c], "fb": sz, "solver": sol, "la_row_ops_median": ops, "la_wall_s_median": wall,
                     "ops_vs_gauss": ops / base_ops if base_ops else None, "wall_vs_gauss": wall / base_wall if base_wall else None,
                     "band": band, "runs": len(rs)})
        md.append(f"| {c} | {bits[c]:.2f} | {sz} | {sol} | {ops:,.0f} | {ops / base_ops:.3f} | {wall:.3f} | {wall / base_wall:.2f} | "
                  f"{band[0.0]:.0f} | {band[0.1]:.0f} | {band[1.0]:.0f} |")
(O / "results.json").write_text(json.dumps({"mismatches": mism, "rows": rows, "status": {a: dict(v) for a, v in status.items()}}, indent=1))
(O / "RESULTS.md").write_text("\n".join(md) + "\n")
print("\n".join(md))
