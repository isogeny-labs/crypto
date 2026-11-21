#!/usr/bin/env python3
"""A/B of the early worker spawn: the fes-f2-gpu row's wall time and the whole run's wall time per arm, and whether the
counted fields differ between runs.   analyze.py OUT_DIR [OUT_JSON]   (reads <arm>_<rep>.json and run_ab.sh's output)"""
import glob, json, os, re, statistics as st, sys
d = sys.argv[1]
rows, tot, counted = {}, {}, {}   # rows: (arm, row index) -> walls; counted: (arm, row index) -> record
for line in open(os.path.join(d, "ab.out")):
    m = re.match(r"rep(\d+) (\w+) rc=(\d+) total_s=([\d.]+)", line.strip())
    if m:
        tot.setdefault(m.group(2), []).append(float(m.group(4)))
for f in sorted(glob.glob(os.path.join(d, "*_*.json"))):
    arm = os.path.basename(f).split("_")[0]
    if arm not in ("on", "off", "off2"):
        continue
    j = json.load(open(f))
    for idx, r in enumerate(j["rows"]):
        s = (r.get("decomposition") or {}).get("solver")
        if s and s.get("name") == "fes-f2-gpu":
            rows.setdefault((arm, idx), []).append(r["wall_seconds"])
            counted.setdefault((arm, idx), []).append(json.dumps({"calls": s["calls"], "ops": s["ops"], "extra": {k: v for k, v in s["extra"].items() if not k.startswith("gpu_")}}, sort_keys=True))
# every run's counted fields against the first run's of the same row index, whatever the arm
first, differ, total = {}, 0, 0
for (arm, idx), recs in sorted(counted.items()):
    for c in recs:
        first.setdefault(idx, c)
        total += 1
        differ += c != first[idx]
idxs = sorted({i for _, i in rows})
out = {"row_wall_s": {"%s|row%d" % (a, i): {"mean": st.fmean(v), "min": min(v), "max": max(v), "n": len(v)} for (a, i), v in sorted(rows.items())},
       "total_wall_s": {a: {"mean": st.fmean(v), "min": min(v), "max": max(v), "n": len(v)} for a, v in tot.items()},
       "counted_runs_differing_from_the_first_of_the_same_row": differ, "counted_runs": total}
for k in ("row_wall_s", "total_wall_s"):
    print(k)
    for a, v in sorted(out[k].items()):
        print("  %-5s mean %.4f  [%.4f, %.4f]  n=%d" % (a, v["mean"], v["min"], v["max"], v["n"]))
for i in idxs:
    on, off, off2 = (st.fmean(rows[(a, i)]) for a in ("on", "off", "off2"))
    print("row %d wall: off - on = %.4f s (on %.4f, off %.4f); off2 - off, the noise floor = %.4f s" % (i, off - on, on, off, off2 - off))
print("counted fields differing from the first run of the same row: %d of %d" % (differ, total))
if len(sys.argv) > 2:
    json.dump(out, open(sys.argv[2], "w"), indent=1, sort_keys=True)
