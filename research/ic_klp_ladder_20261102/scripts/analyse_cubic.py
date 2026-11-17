#!/usr/bin/env python3
"""Amendment 2 analysis: per-arm S, ratio to same-instance rho, exponent fit across p."""
import json, math, glob, collections, statistics as st
rows = [json.loads(l) for f in sorted(glob.glob("logs/cubic-p*.jsonl") + glob.glob("logs/quad-p*.jsonl")) for l in open(f)]
by = collections.defaultdict(list)
for r in rows: by[(r["p"], r["seed"])].append(r)
cells = collections.defaultdict(list); bad = 0
for (p, s), rs in by.items():
    rho = [r for r in rs if r["arm"] == "rho"]
    if not rho or not rho[0]["correct"]: bad += 1; continue
    for r in rs:
        if r["arm"] == "rho": continue
        if not (r["solved"] and r["correct"]): bad += 1; continue
        cells[(r["arm"], p)].append((r["s"], r["s"] / rho[0]["s"], r["total_ops"], r["n"], r["lp_full_relations"], r["lp_histogram"]))
    cells[("rho", p)].append((rho[0]["s"], 1.0, rho[0]["s"] * math.sqrt(rho[0]["n"]), rho[0]["n"], 0, None))
out = {}
arms = sorted({a for a, _ in cells}, key=lambda a: (a != "rho", a))
ps = sorted({p for _, p in cells})
print(f"unsolved or incorrect runs: {bad}")
for a in arms:
    line = []
    for p in ps:
        v = cells.get((a, p), [])
        if not v: continue
        S = st.mean(x[0] for x in v); R = st.mean(x[1] for x in v)
        Rse = st.stdev([x[1] for x in v]) / math.sqrt(len(v)) if len(v) > 1 else float("nan")
        out.setdefault(a, {})[p] = {"n_runs": len(v), "S": S, "ratio_to_rho": R, "ratio_se": Rse,
                                    "mean_total_ops": st.mean(x[2] for x in v), "n": v[0][3],
                                    "lp_full_relations": st.mean(x[4] for x in v)}
        line.append(f"p={p}: S={S:8.1f} ratio={R:7.1f}+-{Rse:5.1f} (n={len(v)})")
    xs = [math.log2(out[a][p]["n"]) for p in out.get(a, {})]; ys = [math.log2(out[a][p]["mean_total_ops"]) for p in out.get(a, {})]
    if len(xs) >= 2:
        mx, my = st.mean(xs), st.mean(ys); slope = sum((x-mx)*(y-my) for x, y in zip(xs, ys)) / sum((x-mx)**2 for x in xs)
        out[a]["exponent"] = slope
    print(f"{a:12s} " + " | ".join(line) + (f" | exponent {out[a]['exponent']:.3f}" if 'exponent' in out.get(a, {}) else ""))
json.dump({str(k): v for k, v in out.items()}, open("cubic-summary.json", "w"), indent=1)
