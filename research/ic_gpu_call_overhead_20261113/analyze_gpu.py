#!/usr/bin/env python3
"""A100 A/B (job_ab.sh): per-call phases on both sides of the pipe, whole-call solver wall, and the
answer-identity check between the before and after binaries.
    analyze_gpu.py RESULTS_DIR results_a100.json timings_a100.tsv identity_a100.txt
RESULTS_DIR/ab holds <instance>_<rep>_<arm>.json/.log.  Rust-side phases come from the last
`fes-timing:` line of a log, worker-side phases from its last `serve-timing:` line (the worker is
killed at exit, so that line is the running mean at the last multiple of 200 calls).  Whole-call
wall is the sum of the fes-f2-gpu rows' solver wall_ns over their solver calls."""
import glob, json, re, statistics as st, sys

res, out_json, out_tsv, out_id = sys.argv[1:5]
RUST = re.compile(r"fes-timing: calls=(\d+) points/call=([\d.]+) per call \(ms\): forms=([\d.]+) pack=([\d.]+) round_trip=([\d.]+) sort=([\d.]+) search_total=([\d.]+) verify=([\d.]+)")
WORK = re.compile(r"serve-timing: calls=(\d+) per call \(us\): read=([\d.]+) qsym=([\d.]+) h2d=([\d.]+) kernel_wall=([\d.]+) kernel_gpu=([\d.]+) d2h=([\d.]+) reply=([\d.]+)")
SKIP = re.compile(r"(wall|_ns$|seconds|elapsed|time|timestamp|duration|per_second|^calibration$|ns_per_|^software$)", re.I)
# Priced fields: counted ops times a per-run machine calibration.  They move between runs of the SAME
# binary, so they cannot show a difference between arms; the counted fields (ops, calls, verdicts,
# hit rate, relations) are compared strictly without them.
PRICED = re.compile(r"(^gae$|^s$|^s_over_rho$|^total_gae$|^markdown$)")

def strip(o, priced_too=True):
    if isinstance(o, dict):
        return {k: strip(v, priced_too) for k, v in o.items() if not SKIP.search(k) and not (priced_too and PRICED.search(k))}
    if isinstance(o, list):
        return [strip(v, priced_too) for v in o]
    return o

def diff(a, b, path=""):
    num = lambda v: isinstance(v, (int, float)) and not isinstance(v, bool)
    if type(a) is not type(b) and not (num(a) and num(b)):
        return [path + " type"]
    bad = []
    if isinstance(a, dict):
        for k in set(a) | set(b):
            bad += [path + "/" + k + " missing"] if (k not in a or k not in b) else diff(a[k], b[k], path + "/" + k)
    elif isinstance(a, list):
        if len(a) != len(b):
            return [path + " length"]
        for i, (x, y) in enumerate(zip(a, b)):
            bad += diff(x, y, path + f"[{i}]")
    elif isinstance(a, float) or isinstance(b, float):
        if abs(a - b) > 1e-9 * max(abs(a), abs(b), 1e-300):
            bad.append(f"{path}: {a} vs {b}")
    elif a != b:
        bad.append(f"{path}: {a} vs {b}")
    return bad

def last(rx, text):
    hits = [m for m in rx.finditer(text)]
    return hits[-1].groups() if hits else None

rows, runs, priced = [], {}, {}
for f in sorted(glob.glob(res + "/ab/*_*_*.json")):
    name = f.rsplit("/", 1)[1][:-5]
    inst, rep, arm = name.split("_")
    text = open(f[:-5] + ".log").read()
    r = last(RUST, text); w = last(WORK, text)
    j = json.load(open(f))
    solver = [((x.get("decomposition") or {}).get("solver")) for x in j["rows"]]
    solver = [s for s in solver if s and s.get("name") == "fes-f2-gpu"]
    calls = sum(s["calls"] for s in solver)
    wall = sum(s["wall_ns"] for s in solver)
    calls_r, ppc, forms, pack, rt, sort, search, verify = (float(x) for x in r)
    rec = dict(instance=inst, rep=int(rep), arm=arm, calls=calls, points_per_call=ppc, solver_wall_ms_per_call=wall / calls / 1e6,
               forms_ms=forms, pack_ms=pack, round_trip_ms=rt, sort_ms=sort, verify_ms=verify, prep_ms=forms + pack)
    if w:
        names = ("worker_calls", "read_us", "qsym_us", "h2d_us", "kernel_wall_us", "kernel_gpu_us", "d2h_us", "reply_us")
        rec.update({n: float(v) for n, v in zip(names, w)})
    rows.append(rec)
    runs.setdefault(inst, []).append((arm, rep, strip(j)))
    priced.setdefault(inst, []).append((arm, rep, strip(j, False)))

cols = ["instance", "rep", "arm", "calls", "points_per_call", "solver_wall_ms_per_call", "forms_ms", "pack_ms", "round_trip_ms", "sort_ms", "verify_ms", "prep_ms",
        "worker_calls", "read_us", "qsym_us", "h2d_us", "kernel_wall_us", "kernel_gpu_us", "d2h_us", "reply_us"]
with open(out_tsv, "w") as t:
    t.write("\t".join(cols) + "\n")
    for r in rows:
        t.write("\t".join(str(r.get(c, "")) for c in cols) + "\n")

summary = {}
for inst in sorted({r["instance"] for r in rows}):
    s = {}
    for arm in ("before", "after"):
        xs = [r for r in rows if r["instance"] == inst and r["arm"] == arm]
        agg = {"n": len(xs), "calls": xs[0]["calls"], "points_per_call": xs[0]["points_per_call"]}
        for k in ("solver_wall_ms_per_call", "prep_ms", "round_trip_ms", "verify_ms", "h2d_us", "kernel_wall_us", "kernel_gpu_us", "d2h_us", "read_us", "qsym_us", "reply_us"):
            v = [r[k] for r in xs if k in r]
            if v:
                agg[k] = dict(mean=st.mean(v), min=min(v), max=max(v))
        s[arm] = agg
    b, a = s["before"]["solver_wall_ms_per_call"]["mean"], s["after"]["solver_wall_ms_per_call"]["mean"]
    s["whole_call_saved_ms"] = b - a
    s["whole_call_saved_fraction"] = (b - a) / b
    summary[inst] = s

lines = []
for inst, rs in sorted(runs.items()):
    ref = next(x for a, rep, x in rs if a == "before")
    bad = [(a, rep, diff(ref, x)) for a, rep, x in rs]
    n_bad = sum(1 for _, _, d in bad if d)
    lines.append(f"{inst}: {len(rs)} runs; runs differing from the first before-run (floats to 1e-9 relative; wall-clock fields, calibration and binary hash ignored): {n_bad}")
    lines += [f"   {a} rep{rep}: {d[:3]}" for a, rep, d in bad if d]
for inst, rs in sorted(priced.items()):
    ref = next(x for a, rep, x in rs if a == "before")
    for arm in ("before", "after"):
        n = sum(1 for a, rep, x in rs if a == arm and diff(ref, x))
        lines.append(f"{inst}: priced fields (gae, s, s_over_rho, total_gae, markdown) differ from the first before-run in {n} of {sum(1 for a, _, _ in rs if a == arm)} {arm}-arm runs (per-run calibration; includes before-vs-before)")
open(out_id, "w").write("\n".join(lines) + "\n")
json.dump(dict(rows=rows, summary=summary, identity=lines), open(out_json, "w"), indent=1)
print("\n".join(lines))
for inst, s in summary.items():
    print(inst, "whole call ms:", {a: round(s[a]["solver_wall_ms_per_call"]["mean"], 4) for a in ("before", "after")},
          "saved %.4f ms (%.1f%%)" % (s["whole_call_saved_ms"], 100 * s["whole_call_saved_fraction"]))
    print("   prep ms:", {a: round(s[a]["prep_ms"]["mean"], 4) for a in ("before", "after")}, "| round trip ms:", {a: round(s[a]["round_trip_ms"]["mean"], 4) for a in ("before", "after")})
    if "kernel_gpu_us" in s["after"]:
        w = s["after"]
        print("   worker us/call (after arm): " + ", ".join(f"{k.replace('_us','')}={w[k]['mean']:.1f}" for k in ("read_us", "qsym_us", "h2d_us", "kernel_wall_us", "kernel_gpu_us", "d2h_us", "reply_us") if k in w))
