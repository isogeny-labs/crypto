#!/usr/bin/env python3
"""Per-call time split of fes-f2-gpu on a CUDA worker, from run_arms.sh output.

    analyze.py RESULTS_AB_DIR [OUT_JSON]

Arms: A = ic-before (text --serve), B = ic-main (binary --serve-bin), C = ic-main + null worker (pipe floor).
Reads <inst>_<rep>_<arm>.log (the `fes-timing:` and worker timing lines) and .json (solver wall and counted
fields).  Prints and writes means with the per-run range; the identity check compares only counted fields
of A against B (C answers wrongly on purpose).  Nothing here is a measurement: it parses what the runs printed.
"""
import glob, json, os, re, statistics, sys

d = sys.argv[1]
out = sys.argv[2] if len(sys.argv) > 2 else None
FT = re.compile(r"fes-timing: calls=(\d+) points/call=([\d.]+) per call \(ms\): forms=([\d.]+) pack=([\d.]+) round_trip=([\d.]+) sort=([\d.]+) search_total=([\d.]+) verify=([\d.]+)")
ST = re.compile(r"serve-timing: calls=(\d+) per call \(us\): read=([\d.]+) qsym=([\d.]+) h2d=([\d.]+) kernel_wall=([\d.]+) kernel_gpu=([\d.]+) d2h=([\d.]+) reply=([\d.]+)")
BT = re.compile(r"serve-bin-timing: calls=(\d+) busy_us_per_call=([\d.]+) kernel_gpu_us_per_call=([\d.]+)")
COUNTED_SKIP = {"wall_ns", "gae", "ns_per_op", "peak_bytes", "priced_by"}

def solver(j):
    for r in j.get("rows", []):
        s = (r.get("decomposition") or {}).get("solver")
        if s:
            yield s

def counted(j):
    rows = []
    for s in solver(j):
        e = {k: v for k, v in s.get("extra", {}).items() if not k.startswith("gpu_")}
        rows.append({"calls": s["calls"], "ops": s["ops"], "extra": e})
    return rows

runs = {}
for log in sorted(glob.glob(os.path.join(d, "*.log"))):
    base = os.path.basename(log)[:-4]
    inst, rep, arm = base.rsplit("_", 2)
    txt = open(log, errors="replace").read()
    ft = FT.findall(txt)
    if not ft:
        continue
    last = ft[-1]
    rec = {"calls": int(last[0]), "points_per_call": float(last[1]), "forms_ms": float(last[2]), "pack_ms": float(last[3]),
           "round_trip_ms": float(last[4]), "search_total_ms": float(last[6]), "verify_ms": float(last[7])}
    st = ST.findall(txt)
    if st:
        s = st[-1]
        rec.update(worker_calls=int(s[0]), read_us=float(s[1]), qsym_us=float(s[2]), h2d_us=float(s[3]), kernel_wall_us=float(s[4]),
                   kernel_gpu_us=float(s[5]), d2h_us=float(s[6]), reply_us=float(s[7]))
    bt = BT.findall(txt)
    if bt:
        b = bt[-1]
        rec.update(worker_calls=int(b[0]), busy_us=float(b[1]), kernel_gpu_us=float(b[2]))
    jp = log[:-4] + ".json"
    if os.path.exists(jp):
        try:
            j = json.load(open(jp))
            rec["_counted"] = counted(j)
            walls = [(s["wall_ns"], s["calls"]) for s in solver(j) if s.get("calls")]
            if walls:
                rec["solver_wall_ms_per_call"] = sum(w for w, _ in walls) / sum(c for _, c in walls) / 1e6
        except Exception as exc:  # noqa: BLE001
            rec["_json_error"] = repr(exc)
    runs.setdefault(inst, {}).setdefault(arm, []).append(rec)

def agg(recs, key):
    v = [r[key] for r in recs if key in r]
    return None if not v else {"mean": statistics.fmean(v), "min": min(v), "max": max(v), "n": len(v)}

summary = {}
for inst, arms in sorted(runs.items()):
    summary[inst] = {}
    for arm, recs in sorted(arms.items()):
        keys = ["calls", "forms_ms", "pack_ms", "round_trip_ms", "search_total_ms", "verify_ms", "solver_wall_ms_per_call",
                "read_us", "qsym_us", "h2d_us", "kernel_wall_us", "kernel_gpu_us", "d2h_us", "reply_us", "busy_us"]
        summary[inst][arm] = {k: agg(recs, k) for k in keys if agg(recs, k)}
        summary[inst][arm]["runs"] = len(recs)
    ident = None
    a, b = arms.get("A"), arms.get("B")
    if a and b:
        ref = a[0].get("_counted")
        ident = {"A_vs_B_counted_differ": sum(1 for r in a[1:] + b if r.get("_counted") != ref), "of": len(a) - 1 + len(b)}
    summary[inst]["identity"] = ident
    B, C = summary[inst].get("B", {}), summary[inst].get("C", {})
    if B.get("round_trip_ms") and B.get("busy_us"):
        summary[inst]["derived"] = {
            "B_round_trip_minus_busy_us": B["round_trip_ms"]["mean"] * 1e3 - B["busy_us"]["mean"],
            "C_floor_round_trip_us": C["round_trip_ms"]["mean"] * 1e3 if C.get("round_trip_ms") else None,
        }

def show(v, f="%.4f"):
    return "-" if not v else (f % v["mean"]) + " [" + (f % v["min"]) + "," + (f % v["max"]) + "]"

for inst, s in summary.items():
    print("==", inst)
    for arm in "ABC":
        if arm in s:
            a = s[arm]
            print(" ", arm, "runs", a["runs"], "calls", show(a.get("calls"), "%.0f"), "| round_trip ms", show(a.get("round_trip_ms")),
                  "| pack", show(a.get("pack_ms")), "forms", show(a.get("forms_ms")), "verify", show(a.get("verify_ms")),
                  "| solver wall/call ms", show(a.get("solver_wall_ms_per_call")))
            w = [(k, show(a.get(k), "%.1f")) for k in ("read_us", "qsym_us", "h2d_us", "kernel_wall_us", "kernel_gpu_us", "d2h_us", "reply_us", "busy_us") if a.get(k)]
            if w:
                print("     worker us/call:", ", ".join("%s=%s" % x for x in w))
    print("  identity:", s.get("identity"), " derived:", s.get("derived"))
if out:
    json.dump(summary, open(out, "w"), indent=1, sort_keys=True)
