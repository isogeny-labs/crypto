#!/usr/bin/env python3
"""A/B of the early worker spawn: the fes-f2-gpu row's wall time and the whole run's wall time per arm, and whether the
counted fields differ between runs.   analyze.py OUT_DIR [OUT_JSON]   (reads <arm>_<rep>.json and run_ab.sh's output)"""
import glob, json, os, re, statistics as st, sys
d = sys.argv[1]
rows, tot, counted = {}, {}, {}
for line in open(os.path.join(d, "ab.out")):
    m = re.match(r"rep(\d+) (\w+) rc=(\d+) total_s=([\d.]+)", line.strip())
    if m:
        tot.setdefault(m.group(2), []).append(float(m.group(4)))
SKIP = {"wall_ns", "gae", "ns_per_op", "peak_bytes", "priced_by"}
for f in sorted(glob.glob(os.path.join(d, "*_*.json"))):
    arm = os.path.basename(f).split("_")[0]
    if arm not in ("on", "off", "off2"):
        continue
    j = json.load(open(f))
    for r in j["rows"]:
        s = (r.get("decomposition") or {}).get("solver")
        if s and s.get("name") == "fes-f2-gpu":
            rows.setdefault(arm, []).append(r["wall_seconds"])
            counted.setdefault(arm, []).append(json.dumps({"calls": s["calls"], "ops": s["ops"], "extra": {k: v for k, v in s["extra"].items() if not k.startswith("gpu_")}}, sort_keys=True))
ref = None
for arm in counted:
    for c in counted[arm]:
        ref = ref or c
differ = sum(1 for arm in counted for c in counted[arm] if c != ref)
out = {"row_wall_s": {a: {"mean": st.fmean(v), "min": min(v), "max": max(v), "n": len(v)} for a, v in rows.items()},
       "total_wall_s": {a: {"mean": st.fmean(v), "min": min(v), "max": max(v), "n": len(v)} for a, v in tot.items()},
       "counted_runs_differing_from_first": differ, "counted_runs": sum(len(v) for v in counted.values())}
for k in ("row_wall_s", "total_wall_s"):
    print(k)
    for a, v in sorted(out[k].items()):
        print("  %-5s mean %.4f  [%.4f, %.4f]  n=%d" % (a, v["mean"], v["min"], v["max"], v["n"]))
if "on" in rows and "off" in rows:
    print("row wall, off - on: %.4f s   (off2 - off, the noise floor: %.4f s)" % (st.fmean(rows["off"]) - st.fmean(rows["on"]), st.fmean(rows.get("off2", rows["off"])) - st.fmean(rows["off"])))
print("counted fields differing from the first run: %d of %d" % (differ, out["counted_runs"]))
if len(sys.argv) > 2:
    json.dump(out, open(sys.argv[2], "w"), indent=1, sort_keys=True)
