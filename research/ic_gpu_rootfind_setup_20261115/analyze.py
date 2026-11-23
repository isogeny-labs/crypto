#!/usr/bin/env python3
"""Tabulate a round-6/7 pod run (research/ic_gpu_rootfind_setup_20261115, _devmap_20261115) into RESULTS.md
and results.json.  Arms rf3-{cpu,gpup}-echelon(-aa)-F<size>, rho-neg(-aa).  Identity per (workload, round)
against rf3-cpu-echelon: trials, relations, recovered, status, S, total GAE and every phase's charged adds,
doubles, scalar mults and GAE (keys indexed: a missing key fails).  Per curve and size: median solve,
relation, set-up and LA wall; the GPU A/A per-pair solve ratio.  Daemon timing lines: pairs-mode session
creation (`timing init ... lookup=4 ... create`) and the build's stages (`timing pairs`) by base size;
bench-pairs.txt.  S against rho from `ecbench band` (band-<session>.json, default reading).
    analyze.py OUTDIR SESSION DEST     (OUTDIR holds sessions/, daemon.log, bench-pairs.txt)
"""
import json, math, re, statistics, sys
from collections import defaultdict
from pathlib import Path

o = Path(sys.argv[1]); sess = sys.argv[2]; O = Path(sys.argv[3]); med = statistics.median
def phase(r, n): return next(p for p in r["phases"] if p["name"] == n)
def charged(r): return [(p["name"], p["adds"], p["doubles"], p["scalar_mults"], p["gae"]) for p in r["phases"]]
keys = [("trials", lambda x: x["counters"]["targets_tried"]), ("relations", lambda x: x["counters"]["relations_found"]),
        ("recovered", lambda x: x["outcome"]["recovered"]), ("status", lambda x: x["outcome"]["status"]),
        ("S", lambda x: x["cost"]["s"]), ("total_gae", lambda x: x["cost"]["total_gae"]), ("charged", charged)]
md, res = [], {}
md.append("REV: " + (o / "REV").read_text().strip())
md.append("bench-pairs: " + " | ".join((o / "bench-pairs.txt").read_text().strip().splitlines()))
recs = [json.loads(l) for l in (o / "sessions" / sess / "records.jsonl").read_text().splitlines() if l.strip()]
band = {(b["curve"], b["arm"]): b for b in json.loads((o / f"band-{sess}.json").read_text())["rows"]}
status = defaultdict(lambda: defaultdict(int)); bits = {}; wl = defaultdict(dict); per = defaultdict(lambda: defaultdict(list)); rho_w = defaultdict(list)
for r in recs:
    c = r["workload"]["curve"]["slug"]; bits[c] = math.log2(r["workload"]["curve"]["r"])
    status[r["arm"]][r["outcome"]["status"]] += 1
    if r["arm"] == "rho-neg": rho_w[c].append(r["time"]["solve_wall_ns"] / 1e9)
    m = re.match(r"rf3-(cpu|gpup)-echelon(-aa)?-F(\d+)$", r["arm"])
    if not m: continue
    arm = m.group(1) + (m.group(2) or ""); sz = int(m.group(3))
    wl[(c, sz, r["workload"]["workload_id"], r["round"])][arm] = r; per[(c, sz)][arm].append(r)
md.append("Record status: " + "; ".join(f"{a} {dict(v)}" for a, v in sorted(status.items())))
mism, nchk = [], 0
for k, kv in wl.items():
    for a, r in kv.items():
        if a == "cpu": continue
        nchk += 1
        for n, f in keys:
            if f(kv["cpu"]) != f(r): mism.append(f"{k} {a}: {n}")
md.append(f"Identity checks against rf3-cpu-echelon: {nchk}; mismatches: {len(mism)}"); md += [f"- {x}" for x in mism[:20]]
aa = [kv["gpup-aa"]["time"]["solve_wall_ns"] / kv["gpup"]["time"]["solve_wall_ns"] for kv in wl.values() if "gpup-aa" in kv]
md.append(f"GPU A/A per-pair solve ratio: {min(aa):.3f}–{max(aa):.3f} (n={len(aa)})")
# daemon timing
dl = (o / "daemon.log").read_text().splitlines()
stages = defaultdict(list); creates = defaultdict(list)
for l in dl:
    m = re.search(r"timing pairs n_fb=(\d+) keys ([\d.]+) dedupe ([\d.]+) compact ([\d.]+) probe ([\d.]+) copy ([\d.]+)", l)
    if m: stages[int(m.group(1))].append([float(x) for x in m.groups()[1:]])
    m = re.search(r"timing init n_probe=(\d+) lookup=4 receive ([\d.]+) s create ([\d.]+) s", l)
    if m: creates[int(m.group(1))].append(float(m.group(3)))
st_rows = {}
md.append("\nPair build stages (daemon RF_SERVE_TIMING, median s): ")
for n in sorted(stages):
    v = stages[n]; st_rows[n] = {k: med(x[i] for x in v) for i, k in enumerate(("keys", "dedupe", "compact", "probe", "copy"))}
    md.append(f"- n_fb={n} (n={len(v)}): " + ", ".join(f"{k} {st_rows[n][k]:.4f}" for k in st_rows[n]))
cr_rows = {}
for n in sorted(creates):
    if n < 100000: continue
    cr_rows[n] = {"median": med(creates[n]), "min": min(creates[n]), "max": max(creates[n]), "n": len(creates[n])}
    md.append(f"- session create, n_probe={n}: median {cr_rows[n]['median']:.4f} s ({cr_rows[n]['min']:.4f}–{cr_rows[n]['max']:.4f}, n={len(creates[n])})")
md.append("\n| curve | log2 r | \\|F_x\\| | arm | S / rho (w=0 / 0.1 / 1) | solve s | relations s | set-up s | LA s | rho solve s |")
md.append("|---|--:|--:|---|--:|--:|--:|--:|--:|--:|")
rows = []
for (c, sz) in sorted(per, key=lambda k: (bits[k[0]], k[1])):
    rho = band[(c, "rho-neg")]["mean_s"]
    for a, rs in sorted(per[(c, sz)].items()):
        full = {"cpu": f"rf3-cpu-echelon-F{sz}", "gpup": f"rf3-gpup-echelon-F{sz}", "gpup-aa": f"rf3-gpup-echelon-aa-F{sz}"}[a]
        b = band[(c, full)]["mean_s"]; br = [b[i] / rho[i] for i in range(3)]
        s = {"solve_s": med(r["time"]["solve_wall_ns"] / 1e9 for r in rs), "rel_s": med(phase(r, "relations")["wall_ns"] / 1e9 for r in rs),
             "setup_s": med(phase(r, "oracle_setup")["wall_ns"] / 1e9 for r in rs), "la_s": med(phase(r, "linear_algebra")["wall_ns"] / 1e9 for r in rs),
             "runs": len(rs), "band_over_rho": br, "rho_solve_s": med(rho_w[c])}
        rows.append({"curve": c, "log2_r": bits[c], "fb": sz, "arm": a, **s})
        md.append(f"| {c} | {bits[c]:.1f} | {sz} | {a} | {br[0]:.0f} / {br[1]:.0f} / {br[2]:.0f} | {s['solve_s']:.3f} | {s['rel_s']:.3f} | {s['setup_s']:.3f} | {s['la_s']:.3f} | {s['rho_solve_s']:.4f} |")
(O / "results.json").write_text(json.dumps({"checks": nchk, "mismatches": mism, "aa_solve_ratio": aa, "stages": st_rows,
    "session_create": cr_rows, "rows": rows, "status": {a: dict(v) for a, v in status.items()}}, indent=1))
(O / "RESULTS.md").write_text("\n".join(md) + "\n"); print("\n".join(md))
