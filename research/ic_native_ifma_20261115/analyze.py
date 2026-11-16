#!/usr/bin/env python3
"""Tabulate research/ic_native_ifma_20261115 (PROTOCOL.md): per workload, the identity of every IC arm
against rootfind-subtract (trials, relations, recovered, status, S, total GAE, relation-phase adds and
the charged ledger of every phase); per curve and arm the median relation-phase and solve wall, and
the ratios.  S against the matched rho from ecbench band (band-*.json, default reading).
    analyze.py OUT_DIR(with sessions/) DEST_DIR
"""
import json, math, statistics, sys
from collections import defaultdict
from pathlib import Path

S = Path(sys.argv[1]); O = Path(sys.argv[2]); med = statistics.median
def phase(r, n): return next((p for p in r["phases"] if p["name"] == n), {})
def wall(r, n=None): return (phase(r, n).get("wall_ns") if n else r["outcome"].get("wall_ns") or sum(p.get("wall_ns", 0) for p in r["phases"])) / 1e9
keys = [("trials", lambda x: x["counters"].get("targets_tried")), ("relations", lambda x: x["counters"].get("relations_found")),
        ("recovered", lambda x: x["outcome"]["recovered"]), ("status", lambda x: x["outcome"]["status"]),
        ("S", lambda x: x["cost"]["s"]), ("total_gae", lambda x: x["cost"]["total_gae"]),
        ("charged", lambda x: [(p["name"], p["adds"], p["doubles"], p["scalar_mults"], p["gae"]) for p in x["phases"]])]
md, rows, mism, nchk = [], [], [], 0
for sess in ("ifma2", "ifma3"):
    recs = [json.loads(l) for l in (S / "sessions" / sess / "records.jsonl").read_text().splitlines() if l.strip()]
    band = {(b["curve"], b["arm"]): b for b in json.loads((S / f"band-{sess}.json").read_text())["rows"]}
    by = defaultdict(lambda: defaultdict(list)); wl = defaultdict(dict); bits = {}
    for r in recs:
        c = r["workload"]["curve"]["slug"]; bits[c] = math.log2(r["workload"]["curve"]["r"])
        by[c][r["arm"]].append(r)
        if r["arm"] != "rho-neg":
            wl[(c, r["workload"]["workload_id"], r.get("round", r.get("seq")))][r["arm"]] = r
    for k, kv in wl.items():
        base = next(v for a, v in kv.items() if "-cpu-" in a)
        for a, r in kv.items():
            if r is base: continue
            nchk += 1
            for n, f in keys:
                if f(base) != f(r): mism.append(f"{sess} {k} {a}: {n}")
    md.append(f"\n## {sess}\n\n| curve | log2 r | arm | runs | S / rho | relation phase s (median) | vs 1 thread | solve s (median) | vs 1 thread |")
    md.append("|---|--:|---|--:|--:|--:|--:|--:|--:|")
    for c in sorted(by, key=lambda c: bits[c]):
        cpu = next(a for a in by[c] if "-cpu-" in a)
        rel0 = med(wall(r, "relations") for r in by[c][cpu]); sol0 = med(r["time"]["solve_wall_ns"] / 1e9 for r in by[c][cpu])
        rho = band[(c, "rho-neg")]["mean_s"][0]
        for a in sorted(by[c]):
            if a == "rho-neg": continue
            rs = by[c][a]; rel = med(wall(r, "relations") for r in rs); sol = med(r["time"]["solve_wall_ns"] / 1e9 for r in rs)
            sr = band[(c, a)]["mean_s"][0] / rho
            rows.append({"session": sess, "curve": c, "log2_r": bits[c], "arm": a, "runs": len(rs), "s_over_rho": sr,
                         "relation_s": rel, "solve_s": sol, "relation_speedup": rel0 / rel, "solve_speedup": sol0 / sol,
                         "band_mean_s": band[(c, a)]["mean_s"], "rho_mean_s": band[(c, "rho-neg")]["mean_s"]})
            md.append(f"| {c} | {bits[c]:.1f} | {a} | {len(rs)} | {sr:.0f} | {rel:.3f} | {rel0 / rel:.2f}× | {sol:.3f} | {sol0 / sol:.2f}× |")
md.insert(0, f"Identity checks against the one-thread arm (per workload and round): {nchk}; mismatches: {len(mism)}")
md[1:1] = [f"- {m}" for m in mism]
(O / "results.json").write_text(json.dumps({"checks": nchk, "mismatches": mism, "rows": rows}, indent=1))
(O / "RESULTS.md").write_text("\n".join(md) + "\n"); print("\n".join(md))
