import json, os, sys, glob
D = "research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm"
R = D + "/replicate"
def strip_wall_line(line):
    s = line.rstrip("\n")
    if any(s.startswith('["%s"' % t) for t in ("RHO", "IC", "PCA", "PCC", "NCA", "NCB", "NCC")):
        # wall time is the last element of RHO/IC rows
        try:
            v = json.loads(s)
            return json.dumps(v[:-1])
        except Exception:
            return s
    return s
def strip_wall_json(o):
    if isinstance(o, dict):
        return {k: strip_wall_json(v) for k, v in o.items() if k not in ("wall", "wall_s", "wall_time")}
    if isinstance(o, list):
        return [strip_wall_json(x) for x in o]
    return o
def cmp_raw(a, b):
    la = [strip_wall_line(x) for x in open(a)] if os.path.exists(a) else None
    lb = [strip_wall_line(x) for x in open(b)] if os.path.exists(b) else None
    if la is None or lb is None:
        return "MISSING"
    return "OK" if la == lb else "DIFF(%d vs %d lines)" % (len(la), len(lb))
results = {"raw_full": [0, []], "raw_reduced": [0, []]}
n_full = 0; bad = []
for f in sorted(glob.glob(D + "/out/*.txt")):
    base = os.path.basename(f)
    if base.startswith("main_") or base.startswith("ctrl_"):
        st = cmp_raw(f, R + "/full/" + base)
        n_full += 1
        if st != "OK": bad.append(("full", base, st))
for f in sorted(glob.glob(D + "/out_reduced/*.txt")):
    base = os.path.basename(f)
    if base.startswith("main_") or base.startswith("ctrl_"):
        st = cmp_raw(f, R + "/reduced/" + base)
        results["raw_reduced"][0] += 1
        if st != "OK": bad.append(("reduced", base, st))
print("raw full files compared:", n_full)
print("raw reduced files compared:", results["raw_reduced"][0])
print("raw mismatches:", bad)
# JSON comparisons
def cj(a, b):
    A = strip_wall_json(json.load(open(a))); B = strip_wall_json(json.load(open(b)))
    return A == B
print("analyze results.json == original (wall-stripped):", cj(D + "/results.json", R + "/analyze/results.json"))
print("full results.json == original (wall-stripped):", cj(D + "/results.json", R + "/full/results.json"))
print("reduced results json == original (wall-stripped):", cj(D + "/results_reduced.json", R + "/reduced/results_reduced.json"))
def ct(a, b):
    return open(a).read() == open(b).read()
print("analyze summary.txt byte-identical:", ct(D + "/out/summary.txt", R + "/analyze/summary.txt"))
print("full summary.txt byte-identical:", ct(D + "/out/summary.txt", R + "/full/summary.txt"))
print("full posthoc.txt byte-identical:", ct(D + "/out/posthoc.txt", R + "/full/posthoc.txt"))
print("assembled RESULTS.md byte-identical:", ct(D + "/RESULTS.md", R + "/assembly/RESULTS.md"))
