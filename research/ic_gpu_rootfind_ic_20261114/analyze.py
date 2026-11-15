#!/usr/bin/env python3
"""Tabulate out_ric/ (job.sh) into RESULTS.md and results.json.
Part 1: ecbench session `main`: per curve and base size, the CPU and GPU rootfind-subtract arms paired by
(workload, round): identity (S, recovered, relations, targets equal), median solve and relation-phase wall,
their ratio, and S against the rho reference's mean S on the same curve. Part 2: the Eytzinger prefetch
arms' kernel times (bench throughput), medians over rounds, against the default build D (A/A: D2).
Values are the records' and the bench's own; medians and ratios are derived here.
    analyze.py DIR    (DIR holds out_ric/)
"""
import json, math, re, statistics, sys
from collections import defaultdict
from pathlib import Path

D = Path(sys.argv[1]); o = D / "out_ric"
med = statistics.median
md, res = [], {}
md.append("GPU: " + next((l.split(":", 1)[1].strip() for l in (o / "nvidia-smi-q.txt").read_text().splitlines() if "Product Name" in l), "?"))
md.append("Worker: " + (o / "serve-info.txt").read_text().strip())
md.append("REV: " + (o / "REV").read_text().strip())

# ---- Part 1 --------------------------------------------------------------------------------
recs = [json.loads(l) for l in (o / "sessions/main/records.jsonl").read_text().splitlines() if l.strip()]
def phase(r, name):
    return next((p for p in r["phases"] if p["name"] == name), {})
rows = defaultdict(dict)   # (curve, size, wid, round) -> kind -> rec
rho = defaultdict(list)    # curve -> [S]
rho_wall = defaultdict(list)
bits = {}
status = defaultdict(lambda: defaultdict(int))
for r in recs:
    if r.get("warmup"):
        continue
    c = r["workload"]["curve"]["slug"]; bits[c] = r["workload"]["curve"]["field_bits"]
    status[r["arm"]][r["outcome"]["status"]] += 1
    if r["arm"] == "rho-neg":
        rho[c].append(r["cost"]["s"]); rho_wall[c].append(r["time"]["solve_wall_ns"] / 1e9)
        continue
    m = re.match(r"rf2-(cpu|gpu)-F(\d+)", r["arm"])
    if not m:
        continue
    rows[(c, int(m.group(2)), r["workload"]["workload_id"], r["round"])][m.group(1)] = r
md.append("\nRecord status per arm: " + "; ".join(f"{a} {dict(v)}" for a, v in sorted(status.items())))
groups = defaultdict(list)
mismatch = []
for (c, sz, wid, rd), kv in rows.items():
    if set(kv) != {"cpu", "gpu"}:
        mismatch.append(f"{c} F{sz} {wid} r{rd}: missing arm"); continue
    a, b = kv["cpu"], kv["gpu"]
    keys = [("S", lambda x: x["cost"]["s"]), ("total_gae", lambda x: x["cost"]["total_gae"]),
            ("recovered", lambda x: x["outcome"]["recovered"]), ("status", lambda x: x["outcome"]["status"]),
            ("relations", lambda x: x["counters"].get("relations_found")),
            ("targets", lambda x: x["counters"].get("targets_tried")),
            ("relation_adds", lambda x: phase(x, "relations").get("adds"))]
    for k, f in keys:
        if f(a) != f(b):
            mismatch.append(f"{c} F{sz} {wid} r{rd}: {k} {f(a)} != {f(b)}")
    gn = phase(b, "relations").get("native", {})
    groups[(c, sz)].append({
        "S": a["cost"]["s"], "verified": a["outcome"]["status"] == "verified" and b["outcome"]["status"] == "verified",
        "cpu_solve_s": a["time"]["solve_wall_ns"] / 1e9, "gpu_solve_s": b["time"]["solve_wall_ns"] / 1e9,
        "cpu_rel_s": phase(a, "relations")["wall_ns"] / 1e9, "gpu_rel_s": phase(b, "relations")["wall_ns"] / 1e9,
        "cpu_la_s": (phase(a, "linear_algebra").get("wall_ns") or 0) / 1e9,
        "gpu_la_s": (phase(b, "linear_algebra").get("wall_ns") or 0) / 1e9,
        "gpu_setup_s": (phase(b, "oracle_setup").get("wall_ns") or 0) / 1e9,
        "targets": a["counters"].get("targets_tried"), "relations": a["counters"].get("relations_found"),
        "gpu_batches": gn.get("gpu_batches"), "gpu_targets_swept": gn.get("gpu_targets_swept"),
        "gpu_targets_unused": gn.get("gpu_targets_unused"),
    })
md.append(f"\nCPU/GPU pairs compared: {len(rows)}; mismatches: {len(mismatch)}")
md += [f"- {x}" for x in mismatch[:20]]
md.append("\n## Index calculus: rootfind-subtract (CPU) vs rootfind-subtract-gpu, paired runs\n")
md.append("| curve | log2 r | \\|F_x\\| | runs | verified | mean S | rho mean S | S / rho | CPU solve s | GPU solve s | solve ratio | CPU relations s | GPU relations s | relations ratio | GPU batches / targets swept |")
md.append("|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|---|")
p1 = []
for (c, sz) in sorted(groups, key=lambda k: (bits[k[0]], k[1])):
    g = groups[(c, sz)]
    S = statistics.fmean(x["S"] for x in g); rs = statistics.fmean(rho[c]) if rho[c] else float("nan")
    cs, gs = med(x["cpu_solve_s"] for x in g), med(x["gpu_solve_s"] for x in g)
    cr, gr = med(x["cpu_rel_s"] for x in g), med(x["gpu_rel_s"] for x in g)
    rlog = next(r["workload"]["curve"]["r"] for r in recs if r["workload"]["curve"]["slug"] == c)
    L = math.log2(rlog)
    nb = sum(x["gpu_batches"] or 0 for x in g); nt = sum(x["gpu_targets_swept"] or 0 for x in g)
    md.append(f"| {c} | {L:.2f} | {sz} | {len(g)} | {sum(x['verified'] for x in g)} | {S:.1f} | {rs:.3f} | {S / rs:.0f} | "
              f"{cs:.3f} | {gs:.3f} | {cs / gs:.1f}× | {cr:.3f} | {gr:.3f} | {cr / gr:.1f}× | {nb} / {nt} |")
    p1.append({"curve": c, "log2_r": L, "fb_abscissae": sz, "runs": len(g), "verified": sum(x["verified"] for x in g),
               "mean_S": S, "rho_mean_S": rs, "S_over_rho": S / rs, "cpu_solve_s_median": cs, "gpu_solve_s_median": gs,
               "solve_speedup": cs / gs, "cpu_relations_s_median": cr, "gpu_relations_s_median": gr,
               "relations_speedup": cr / gr, "runs_detail": g})
res["ic"] = {"pairs": len(rows), "mismatches": mismatch, "rows": p1,
             "rho": {c: {"S": rho[c], "solve_s": rho_wall[c]} for c in rho}}

# ---- Part 2 --------------------------------------------------------------------------------
ARMS = ["D", "D2", "P3", "P4", "P5", "P4L2"]
NAMES = {"D": "default (no prefetch)", "D2": "default (A/A)", "P3": "prefetch 3 levels", "P4": "prefetch 4 levels",
         "P5": "prefetch 5 levels", "P4L2": "prefetch 4 levels, L2"}
st = {a: (o / f"selftest-{a}.log").read_text().count("SELFTEST PASSED") for a in ARMS if (o / f"selftest-{a}.log").exists()}
md.append("\n## Eytzinger prefetch (bench throughput kernel ms, medians of rounds)\n")
md.append("Selftests passed (of 3): " + ", ".join(f"{a} {n}" for a, n in st.items()))
res["prefetch"] = {"selftests": st}
for pref, label in (("tp", "|F| = 65536, 65536 targets"), ("tpsmall", "|F| = 1024, 262144 targets")):
    data = {}
    for f in o.glob(f"{pref}-*-*.log"):
        _, a, r = f.stem.rsplit("-", 2)
        for m in re.finditer(r"cuda (\w+)\s+chunk=(\d+)\s*: ([\d.]+) ms", f.read_text()):
            data.setdefault(f"{m.group(1)}/C{m.group(2)}", {}).setdefault(a, []).append(float(m.group(3)))
    if not data:
        continue
    arms = [a for a in ARMS if any(a in v for v in data.values())]
    md.append(f"\n### {label}\n")
    md.append("| config | " + " | ".join(NAMES[a] for a in arms) + " |")
    md.append("|---|" + "---:|" * len(arms))
    out = {}
    for cfg in sorted(data):
        base = med(data[cfg]["D"])
        cells = []
        for a in arms:
            v = data[cfg].get(a)
            if not v:
                cells.append("—"); continue
            m_ = med(v)
            cells.append(f"{m_:.2f}" if a == "D" else f"{m_:.2f} ({m_ / base:.3f})")
        md.append(f"| {cfg} | " + " | ".join(cells) + " |")
        out[cfg] = {a: {"median": med(v), "min": min(v), "max": max(v), "n": len(v)} for a, v in data[cfg].items()}
    best = {}
    for a in arms:
        cfgs = {c: med(v[a]) for c, v in data.items() if a in v}
        c = min(cfgs, key=cfgs.get); best[a] = (c, cfgs[c])
    mb = best["D"][1]
    md.append("\nBest configuration per arm:\n")
    for a in arms:
        c, v = best[a]; md.append(f"- {NAMES[a]}: {c} {v:.2f} ms ({v / mb:.3f}× the default's best time)")
    aa = max(max(med(v["D2"]) / med(v["D"]), med(v["D"]) / med(v["D2"])) for v in data.values() if "D2" in v)
    spread = max(max(v[a]) / min(v[a]) for v in data.values() for a in v)
    md.append(f"\nA/A: largest |D2/D − 1| {aa - 1:.4f}; largest max/min of one arm's rounds {spread:.4f}.")
    res["prefetch"][pref] = {"configs": out, "best": {a: {"config": c, "ms": v} for a, (c, v) in best.items()},
                             "aa_max": aa, "round_spread_max": spread}
(D / "results.json").write_text(json.dumps(res, indent=1))
(D / "RESULTS.md").write_text("\n".join(md) + "\n")
print("\n".join(md))
