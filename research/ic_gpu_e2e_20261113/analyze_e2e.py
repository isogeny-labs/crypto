#!/usr/bin/env python3
"""Tabulate `ic bench --sweep` outputs (out_e2e/*.json under DIR) into RESULTS.md and results.json.
Counted cost (S, s_over_rho) is the metric; wall time is a practicality note (AGENTS.md section 6).
Every number is read from the bench JSON; ratios and means are derived here and labelled.
    analyze_e2e.py DIR
"""
import json, statistics, sys
from collections import OrderedDict
from pathlib import Path

D = Path(sys.argv[1]); o = D / "out_e2e"
md, res = [], {}
md.append("GPU: " + next((l.split(":", 1)[1].strip() for l in (o / "nvidia-smi-q.txt").read_text().splitlines() if "Product Name" in l), "?"))
md.append("Binary and worker sha256:\n\n```\n" + (o / "ic.sha256").read_text().strip() + "\n```\n")
for s in ("koblitz-n23", "char2-n25", "char2-n29"):
    f = o / f"{s}.json"
    if not f.exists():
        md.append(f"\n## {s}: no output (see {s}.log)\n"); continue
    d = json.loads(f.read_text())
    rows = d.get("rows", [])
    rho = d.get("rho_reference", {})
    groups = OrderedDict()
    for r in rows:
        sol = (r.get("decomposition") or {}).get("solver") or {}
        key = (r.get("label") or "?", sol.get("name") or "—")
        groups.setdefault(key, []).append(r)
    md.append(f"\n## {s} (r = {rows[0]['r'] if rows else '?'})\n")
    md.append("| configuration | solver | repeats | verified | hit rate | S mean | S / rho | solver calls | solver wall per call (ms) | solver wall total (s) | row wall (s) | GPU calls / fallback / capped |")
    md.append("|:--|:--|--:|:--|--:|--:|--:|--:|--:|--:|--:|:--|")
    out = []
    for (label, solver), rs in groups.items():
        dec = [r.get("decomposition") or {} for r in rs]
        sols = [x.get("solver") or {} for x in dec]
        calls = [x.get("calls", 0) for x in sols]
        swall = [x.get("wall_ns", 0) / 1e9 for x in sols]
        ex = [x.get("extra") or {} for x in sols]
        row = {
            "label": label, "solver": solver, "repeats": len(rs),
            "verified": all(r.get("verified") for r in rs),
            "hit_rate": [x.get("hit_rate") for x in dec],
            "s": [r.get("s") for r in rs], "s_over_rho": [r.get("s_over_rho") for r in rs],
            "solver_calls": calls, "solver_wall_s": swall,
            "row_wall_s": [r.get("wall_seconds") for r in rs],
            "gpu_worker": [e.get("gpu_worker") for e in ex], "gpu_fallback": [e.get("gpu_fallback") for e in ex],
            "gpu_capped": [e.get("gpu_capped") for e in ex],
        }
        out.append(row)
        mean = lambda v: statistics.mean([x for x in v if x is not None]) if any(x is not None for x in v) else None
        per_call = sum(swall) / max(1, sum(calls)) * 1e3
        gpu = f"{sum(x or 0 for x in row['gpu_worker'])} / {sum(x or 0 for x in row['gpu_fallback'])} / {sum(x or 0 for x in row['gpu_capped'])}" if solver == "fes-f2-gpu" else ""
        sm, so = mean(row["s"]), mean(row["s_over_rho"])
        md.append(f"| `{label}` | `{solver}` | {len(rs)} | {'y' if row['verified'] else 'NO'} | {mean(row['hit_rate']) or 0:.4f} | "
                  f"{sm:.2f} | {so:.2f} | {sum(calls)} | {per_call:.3f} | {sum(swall):.2f} | {sum(x or 0 for x in row['row_wall_s']):.2f} | {gpu} |")
    res[s] = {"rows": out, "rho_reference": {k: rho.get(k) for k in ("mean_s", "s_mean", "label") if k in rho}}
    by = {r["solver"]: r for r in out}
    for a, b in (("fes-f2", "fes-f2-gpu"), ("inherited-f4", "fes-f2-gpu")):
        if a in by and b in by:
            wa, wb = sum(by[a]["solver_wall_s"]), sum(by[b]["solver_wall_s"])
            ra, rb = sum(x or 0 for x in by[a]["row_wall_s"]), sum(x or 0 for x in by[b]["row_wall_s"])
            same_s = by[a]["s"] == by[b]["s"]
            md.append(f"\n- derived: `{a}` → `{b}`: solver wall {wa:.2f} s → {wb:.2f} s ({wa / wb:.1f}×); "
                      f"row wall {ra:.2f} s → {rb:.2f} s ({ra / rb:.1f}×); counted S {'identical' if same_s else 'differs'} "
                      f"({statistics.mean(by[a]['s']):.2f} vs {statistics.mean(by[b]['s']):.2f}).")
(D / "results.json").write_text(json.dumps(res, indent=1))
(D / "RESULTS.md").write_text("\n".join(md) + "\n")
print("\n".join(md))
