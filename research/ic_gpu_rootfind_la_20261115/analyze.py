#!/usr/bin/env python3
"""Tabulate research/ic_gpu_rootfind_la_20261115 (PROTOCOL.md) from out_rla/ into RESULTS.md and results.json.
Arms rf3-{cpu,gpup}-{gauss,echelon}-F<size>, rf3-gpup-echelon-aa-F4096, rho-neg, rho-neg-aa.
Identity per (workload, round) against rf3-cpu-gauss: trials, relations, recovered, status, S, total GAE and
every phase's charged adds, doubles, scalar mults and GAE (keys indexed, so a missing key fails loudly).
Per curve and size: median solve, relation, set-up and LA wall and LA row_ops per arm; echelon/gauss ratios;
the A/A copy's per-pair ratio range.  S against rho from `ecbench band` (band-rf3la.json, default reading).
    analyze.py DIR(holding out_rla/) DEST
"""
import json, math, re, statistics, sys
from collections import defaultdict
from pathlib import Path

o = Path(sys.argv[1]) / "out_rla"; O = Path(sys.argv[2]); med = statistics.median
def phase(r, n): return next(p for p in r["phases"] if p["name"] == n)
def charged(r): return [(p["name"], p["adds"], p["doubles"], p["scalar_mults"], p["gae"]) for p in r["phases"]]
keys = [("trials", lambda x: x["counters"]["targets_tried"]), ("relations", lambda x: x["counters"]["relations_found"]),
        ("recovered", lambda x: x["outcome"]["recovered"]), ("status", lambda x: x["outcome"]["status"]),
        ("S", lambda x: x["cost"]["s"]), ("total_gae", lambda x: x["cost"]["total_gae"]), ("charged", charged)]
md, res = [], {}
md.append("REV: " + (o / "REV").read_text().strip())
md.append("Daemon: " + ((o / "daemon-start.txt").read_text().strip() if (o / "daemon-start.txt").exists() else "?"))
md.append("cgroup: " + (o / "cgroup-cpu.txt").read_text().strip().replace("\n", " | "))
recs = [json.loads(l) for l in (o / "sessions" / "rf3la" / "records.jsonl").read_text().splitlines() if l.strip()]
band = {(b["curve"], b["arm"]): b for b in json.loads((o / "band-rf3la.json").read_text())["rows"]} if (o / "band-rf3la.json").exists() else {}
status = defaultdict(lambda: defaultdict(int)); bits = {}; wl = defaultdict(dict); per = defaultdict(lambda: defaultdict(list))
for r in recs:
    c = r["workload"]["curve"]["slug"]; bits[c] = math.log2(r["workload"]["curve"]["r"])
    status[r["arm"]][r["outcome"]["status"]] += 1
    m = re.match(r"rf3-(cpu|gpup)-(gauss|echelon)(-aa)?-F(\d+)$", r["arm"])
    if not m: continue
    arm = f"{m.group(1)}-{m.group(2)}{m.group(3) or ''}"; sz = int(m.group(4))
    wl[(c, sz, r["workload"]["workload_id"], r["round"])][arm] = r; per[(c, sz)][arm].append(r)
md.append("Record status: " + "; ".join(f"{a} {dict(v)}" for a, v in sorted(status.items())))
mism, nchk = [], 0
for k, kv in wl.items():
    base = kv["cpu-gauss"]
    for a, r in kv.items():
        if a == "cpu-gauss": continue
        nchk += 1
        for n, f in keys:
            if f(base) != f(r): mism.append(f"{k} {a}: {n}")
md.append(f"Identity checks against rf3-cpu-gauss: {nchk}; mismatches: {len(mism)}"); md += [f"- {x}" for x in mism[:20]]
aa = []
for k, kv in wl.items():
    if "gpup-echelon-aa" in kv:
        aa.append(kv["gpup-echelon-aa"]["time"]["solve_wall_ns"] / kv["gpup-echelon"]["time"]["solve_wall_ns"])
md.append(f"A/A (gpup-echelon-aa / gpup-echelon solve, per pair): {min(aa):.2f}–{max(aa):.2f} (n={len(aa)})" if aa else "A/A: none")
md.append("\n| curve | log2 r | \\|F_x\\| | arm | S / rho | solve s | relations s | set-up s | LA s | LA row_ops | solve echelon/gauss | LA row_ops echelon/gauss |")
md.append("|---|--:|--:|---|--:|--:|--:|--:|--:|--:|--:|--:|")
rows = []
for (c, sz) in sorted(per, key=lambda k: (bits[k[0]], k[1])):
    g = per[(c, sz)]
    st = {a: {"solve_s": med(r["time"]["solve_wall_ns"] / 1e9 for r in rs), "rel_s": med(phase(r, "relations")["wall_ns"] / 1e9 for r in rs),
              "setup_s": med(phase(r, "oracle_setup")["wall_ns"] / 1e9 for r in rs), "la_s": med(phase(r, "linear_algebra")["wall_ns"] / 1e9 for r in rs),
              "row_ops": med(phase(r, "linear_algebra")["native"]["row_ops"] for r in rs), "runs": len(rs)} for a, rs in g.items()}
    rho = band.get((c, "rho-neg"), {}).get("mean_s")
    for a in sorted(st):
        s = st[a]; full = f"rf3-{a.replace('-aa', '')}{'-aa' if a.endswith('-aa') else ''}-F{sz}"
        b = band.get((c, full), {}).get("mean_s")
        s["band_over_rho"] = [b[i] / rho[i] for i in range(3)] if b and rho else None
        dev = a.split("-")[0]
        if a.endswith("echelon") and f"{dev}-gauss" in st:
            s["solve_vs_gauss"] = s["solve_s"] / st[f"{dev}-gauss"]["solve_s"]; s["rowops_vs_gauss"] = s["row_ops"] / st[f"{dev}-gauss"]["row_ops"]
        rows.append({"curve": c, "log2_r": bits[c], "fb": sz, "arm": a, **s})
        sr = f"{s['band_over_rho'][0]:.0f}" if s["band_over_rho"] else "?"
        md.append(f"| {c} | {bits[c]:.1f} | {sz} | {a} | {sr} | {s['solve_s']:.3f} | {s['rel_s']:.3f} | {s['setup_s']:.3f} | {s['la_s']:.3f} | "
                  f"{s['row_ops']:,.0f} | {s.get('solve_vs_gauss', float('nan')):.2f} | {s.get('rowops_vs_gauss', float('nan')):.3f} |")
(O / "results.json").write_text(json.dumps({"checks": nchk, "mismatches": mism, "aa_solve_ratio": aa, "rows": rows,
    "status": {a: dict(v) for a, v in status.items()}}, indent=1))
(O / "RESULTS.md").write_text("\n".join(md) + "\n"); print("\n".join(md))
