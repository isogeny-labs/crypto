#!/usr/bin/env python3
"""Tabulate out_rbig/ (job.sh) into RESULTS.md and results.json.  For each ecbench session (rf3, rf2d):
arms rfM-{cpu,gpu,gpud}-F<size> grouped by (workload, round); identity of every GPU arm to the CPU arm
(S, total GAE, recovered, status, relations, targets, relation-phase additions); per curve and size the
median solve, relation-phase and oracle set-up wall of each arm, and S against the rho reference's
mean S.  The daemon's start time comes from daemon-start.txt / daemon.log.  Values are the records';
medians and ratios are derived here.
    analyze.py DIR    (DIR holds out_rbig/)
"""
import json, math, re, statistics, sys
from collections import defaultdict
from pathlib import Path

D = Path(sys.argv[1]); o = D / "out_rbig"
med = statistics.median
md, res = [], {}
md.append("GPU: " + next((l.split(":", 1)[1].strip() for l in (o / "nvidia-smi-q.txt").read_text().splitlines() if "Product Name" in l), "?"))
md.append("Worker: " + (o / "serve-info.txt").read_text().strip())
md.append("REV: " + (o / "REV").read_text().strip())
ds = (o / "daemon-start.txt").read_text().strip() if (o / "daemon-start.txt").exists() else "?"
dl = [l for l in (o / "daemon.log").read_text().splitlines() if "daemon" in l][:2] if (o / "daemon.log").exists() else []
md.append(f"Daemon: {ds}; " + " | ".join(dl))
tl = [l for l in (o / "daemon.log").read_text().splitlines() if "timing init" in l] if (o / "daemon.log").exists() else []
tim = defaultdict(list)
for l in tl:
    m_ = re.search(r"n_probe=(\d+) lookup=(\d+) receive ([\d.]+) s create ([\d.]+) s", l)
    if m_:
        tim[(int(m_.group(1)) > 0, int(m_.group(2)))].append((int(m_.group(1)), float(m_.group(3)), float(m_.group(4))))
md.append("Daemon INIT timings (RF_SERVE_TIMING; probe set?, lookup -> n, median receive s, median create s): " +
          "; ".join(f"probe={k[0]} lookup={k[1]}: n={len(v)} receive {statistics.median(x[1] for x in v):.4f} create {statistics.median(x[2] for x in v):.4f}"
                    for k, v in sorted(tim.items())))
res["daemon"] = {"start": ds, "log": dl, "init_timings": {f"probe={k[0]},lookup={k[1]}": v for k, v in tim.items()}}

def phase(r, name):
    return next((p for p in r["phases"] if p["name"] == name), {})

for sess in ("rf3big", "rf2mt", "smoke"):
    f = o / "sessions" / sess / "records.jsonl"
    if not f.exists():
        continue
    recs = [json.loads(l) for l in f.read_text().splitlines() if l.strip()]
    status = defaultdict(lambda: defaultdict(int))
    rho = defaultdict(list); rho_w = defaultdict(list); bits = {}; rlog = {}
    rows = defaultdict(dict)
    for r in recs:
        if r.get("warmup"):
            continue
        c = r["workload"]["curve"]["slug"]; rlog[c] = math.log2(r["workload"]["curve"]["r"])
        status[r["arm"]][r["outcome"]["status"]] += 1
        if r["arm"] == "rho-neg":
            rho[c].append(r["cost"]["s"]); rho_w[c].append(r["time"]["solve_wall_ns"] / 1e9); continue
        m = re.match(r"rf(\d)-(cpu|gpu|gpud|gpuh|mt|cpuaa|gpuaa)(?:-F(\d+))?$", r["arm"])
        if not m:
            continue
        rows[(c, int(m.group(1)), int(m.group(3) or 0), r["workload"]["workload_id"], r["round"])][m.group(2)] = r
    md.append(f"\n## Session {sess}\n")
    md.append("Record status: " + "; ".join(f"{a} {dict(v)}" for a, v in sorted(status.items())))
    keys = [("S", lambda x: x["cost"]["s"]), ("total_gae", lambda x: x["cost"]["total_gae"]),
            ("recovered", lambda x: x["outcome"]["recovered"]), ("status", lambda x: x["outcome"]["status"]),
            ("relations", lambda x: x["counters"].get("relations_found")),
            ("targets", lambda x: x["counters"].get("targets_tried")),
            ("relation_adds", lambda x: phase(x, "relations").get("adds"))]
    mism, groups = [], defaultdict(list)
    for (c, m_, sz, wid, rd), kv in rows.items():
        if "cpu" not in kv:
            mism.append(f"{c} m{m_} F{sz} r{rd}: no cpu arm"); continue
        for g in ("gpu", "gpud", "gpuh", "mt", "cpuaa", "gpuaa"):
            if g not in kv:
                continue
            for k, fn in keys:
                if fn(kv["cpu"]) != fn(kv[g]):
                    mism.append(f"{c} m{m_} F{sz} {wid} r{rd} {g}: {k} {fn(kv['cpu'])} != {fn(kv[g])}")
        e = {"S": kv["cpu"]["cost"]["s"]}
        for g, r in kv.items():
            e[f"{g}_solve_s"] = r["time"]["solve_wall_ns"] / 1e9
            e[f"{g}_rel_s"] = (phase(r, "relations").get("wall_ns") or 0) / 1e9
            e[f"{g}_setup_s"] = (phase(r, "oracle_setup").get("wall_ns") or 0) / 1e9
            e[f"{g}_la_s"] = (phase(r, "linear_algebra").get("wall_ns") or 0) / 1e9
            e[f"{g}_verified"] = r["outcome"]["status"] == "verified"
        e["targets"] = kv["cpu"]["counters"].get("targets_tried")
        groups[(c, m_, sz)].append(e)
    md.append(f"GPU-vs-CPU pair checks: {sum(len([g for g in kv if g != 'cpu']) for kv in rows.values())}; mismatches: {len(mism)}")
    md += [f"- {x}" for x in mism[:20]]
    md.append("\n| curve | log2 r | m | \\|F_x\\| | runs | mean S | S / rho | arm | solve s | relations s | set-up s | LA s | solve vs CPU | relations vs CPU |")
    md.append("|---|--:|--:|--:|--:|--:|--:|---|--:|--:|--:|--:|--:|--:|")
    out = []
    for (c, m_, sz) in sorted(groups, key=lambda k: (rlog[k[0]], k[1], k[2])):
        g = groups[(c, m_, sz)]
        S = statistics.fmean(x["S"] for x in g); rs = statistics.fmean(rho[c]) if rho[c] else float("nan")
        row = {"curve": c, "log2_r": rlog[c], "m": m_, "fb_abscissae": sz, "runs": len(g), "mean_S": S,
               "rho_mean_S": rs, "S_over_rho": S / rs,
               "rho_solve_s_median": med(rho_w[c]) if rho_w[c] else None, "arms": {}, "runs_detail": g}
        cs, cr = med(x["cpu_solve_s"] for x in g), med(x["cpu_rel_s"] for x in g)
        for arm in ("cpu", "cpuaa", "mt", "gpu", "gpuaa", "gpuh", "gpud"):
            if any(f"{arm}_solve_s" not in x for x in g):
                continue
            a = {k: med(x[f"{arm}_{k}"] for x in g) for k in ("solve_s", "rel_s", "setup_s", "la_s")}
            a["verified"] = sum(x[f"{arm}_verified"] for x in g)
            a["solve_speedup"] = cs / a["solve_s"]; a["relations_speedup"] = cr / a["rel_s"] if a["rel_s"] else float("nan")
            row["arms"][arm] = a
            md.append(f"| {c} | {rlog[c]:.2f} | {m_} | {sz} | {len(g)} | {S:.1f} | {S / rs:.0f} | {arm} | {a['solve_s']:.3f} | "
                      f"{a['rel_s']:.3f} | {a['setup_s']:.3f} | {a['la_s']:.3f} | {a['solve_speedup']:.2f}× | {a['relations_speedup']:.1f}× |")
        out.append(row)
    res[sess] = {"mismatches": mism, "rows": out, "status": {a: dict(v) for a, v in status.items()}}
(D / "results.json").write_text(json.dumps(res, indent=1))
(D / "RESULTS.md").write_text("\n".join(md) + "\n")
print("\n".join(md))
