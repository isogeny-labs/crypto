#!/usr/bin/env python3
"""Parse the A/B output of run_ab.sh into results.json and check that the two arms give the same
answers.   analyze.py OUT_DIR results.json timings.tsv identity.txt
Per-call milliseconds come from the `fes-timing:` line of each run (FES_GPU_TIMING=1).  The
preparation cost is forms + pack (before: QuadraticForm conversion then PackedSystem packing;
after: the direct pack alone).  The identity check compares every run's result JSON with the
first before-run, ignoring wall-clock fields, the per-run machine calibration and the binary
hash, with floats compared to 1e-9 relative and everything else exactly."""
import glob, json, re, statistics as st, sys

out, res_path, tsv_path, id_path = sys.argv[1:5]
LINE = re.compile(r"fes-timing: calls=(\d+) points/call=([\d.]+) per call \(ms\): forms=([\d.]+) pack=([\d.]+) round_trip=([\d.]+) sort=([\d.]+) search_total=([\d.]+) verify=([\d.]+)")
SKIP = re.compile(r"(wall|_ns$|seconds|elapsed|time|timestamp|duration|per_second|^calibration$|ns_per_|^software$)", re.I)

def strip(o):
    if isinstance(o, dict):
        return {k: strip(v) for k, v in o.items() if not SKIP.search(k)}
    if isinstance(o, list):
        return [strip(v) for v in o]
    return o

def diff(a, b, path=""):
    bad = []
    num = lambda v: isinstance(v, (int, float)) and not isinstance(v, bool)
    if type(a) is not type(b) and not (num(a) and num(b)):
        return [path + " type"]
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

rows, runs = [], {}
for f in sorted(glob.glob(out + "/*_*_*.json")):
    name = f.rsplit("/", 1)[1][:-5]
    inst, rep, arm = name.split("_")
    line = [l for l in open(f[:-5] + ".log") if l.startswith("fes-timing:")][-1]
    m = LINE.match(line)
    calls, ppc, forms, pack, rt, sort, search, verify = (float(x) for x in m.groups())
    rows.append(dict(instance=inst, rep=int(rep), arm=arm, calls=int(calls), points_per_call=ppc, forms_ms=forms,
                     pack_ms=pack, round_trip_ms=rt, sort_ms=sort, verify_ms=verify, prep_ms=forms + pack))
    runs.setdefault(inst, []).append((arm, rep, strip(json.load(open(f)))))

with open(tsv_path, "w") as t:
    t.write("instance\trep\tarm\tcalls\tpoints_per_call\tforms_ms\tpack_ms\tround_trip_ms\tsort_ms\tverify_ms\tprep_ms\n")
    for r in rows:
        t.write("\t".join(str(r[k]) for k in ("instance", "rep", "arm", "calls", "points_per_call", "forms_ms", "pack_ms", "round_trip_ms", "sort_ms", "verify_ms", "prep_ms")) + "\n")

summary = {}
for inst in sorted({r["instance"] for r in rows}):
    s = {}
    for arm in ("before", "after"):
        xs = [r for r in rows if r["instance"] == inst and r["arm"] == arm]
        prep = [r["prep_ms"] for r in xs]
        s[arm] = dict(n=len(xs), calls=xs[0]["calls"], points_per_call=xs[0]["points_per_call"],
                      prep_ms_mean=st.mean(prep), prep_ms_min=min(prep), prep_ms_max=max(prep),
                      verify_ms_mean=st.mean(r["verify_ms"] for r in xs),
                      round_trip_ms_mean=st.mean(r["round_trip_ms"] for r in xs))
    s["saved_ms_per_call"] = s["before"]["prep_ms_mean"] - s["after"]["prep_ms_mean"]
    s["saved_fraction_of_prep"] = s["saved_ms_per_call"] / s["before"]["prep_ms_mean"]
    summary[inst] = s

lines = []
for inst, rs in sorted(runs.items()):
    ref = next(x for a, rep, x in rs if a == "before")
    bad = [(a, rep, diff(ref, x)) for a, rep, x in rs]
    n_bad = sum(1 for _, _, d in bad if d)
    lines.append(f"{inst}: {len(rs)} runs; runs differing from the first before-run (floats to 1e-9 relative, wall-clock fields, calibration and binary hash ignored): {n_bad}")
    for a, rep, d in bad:
        if d:
            lines.append(f"   {a} rep{rep}: {d[:3]}")
open(id_path, "w").write("\n".join(lines) + "\n")
json.dump(dict(rows=rows, summary=summary, identity=lines), open(res_path, "w"), indent=1)
print("\n".join(lines))
for inst, s in summary.items():
    print(inst, {a: round(s[a]["prep_ms_mean"], 4) for a in ("before", "after")}, "saved ms/call", round(s["saved_ms_per_call"], 4),
          f"({100 * s['saved_fraction_of_prep']:.1f}% of the preparation cost)")
