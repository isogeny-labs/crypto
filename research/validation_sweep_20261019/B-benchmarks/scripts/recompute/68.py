# Row 68 recompute: 20261015-ic-improve2-04-d4-2lp-fix. Reads committed files only. Run from repo root with python3 -I.
import json, statistics
R = "research/ic_improve_20261015/04-d4-2lp-fix"
def jl(p):
    return [json.loads(x) for x in open(p) if x.strip()]
test = jl(R + "/raw/test.jsonl")
print("test rows:", len(test), "top-level keys:", sorted(k for k in test[0].keys() if k != "arms"))
arms_all = {}
ok = 0; tot = 0
for row in test:
    for a in row["arms"]:
        arms_all.setdefault(a["arm"], []).append((row, a))
        tot += 1; ok += int(bool(a["correct"]))
print("arms:", sorted(arms_all.keys()))
print("logarithm correct: %d/%d" % (ok, tot))
def pooled(arm, field):
    num = sum(a["gae"][field] for _, a in arms_all[arm]) if field in ("whole", "relation", "la") else None
    return num
def tot_gae(arm, field):
    return sum(a["gae"][field] for _, a in arms_all[arm])
def per_curve(arm, field):
    base = {json.dumps({k: v for k, v in r.items() if k != "arms"}, sort_keys=True): a["gae"][field] for r, a in arms_all["d4"]}
    out = []
    for r, a in arms_all[arm]:
        key = json.dumps({k: v for k, v in r.items() if k != "arms"}, sort_keys=True)
        out.append(a["gae"][field] / base[key])
    return out
for arm in ["d4-ed", "d4-2lp", "d4-ed-all", "d4-late110", "d4-2lp-null"]:
    if arm not in arms_all: continue
    w = tot_gae(arm, "whole") / tot_gae("d4", "whole")
    r_ = tot_gae(arm, "relation") / tot_gae("d4", "relation")
    l_ = tot_gae(arm, "la") / tot_gae("d4", "la")
    pc = per_curve(arm, "whole")
    print("%-12s whole sum-ratio=%.4f relation=%.4f la=%.4f per-curve whole min=%.4f max=%.4f n=%d" % (arm, w, r_, l_, min(pc), max(pc), len(pc)))
# relations over (cols + 1): small_columns
for arm in ["d4", "d4-ed", "d4-2lp", "d4-ed-all"]:
    if arm not in arms_all: continue
    v = [a["full_relations"] / (a["small_columns"] + 1) for _, a in arms_all[arm]]
    print("%-10s mean full_relations/(cols+1) = %.4f" % (arm, statistics.fmean(v)))
# arms per curve count
print("curves:", len(test), "arms per curve:", sorted(set(len(r["arms"]) for r in test)))
# ecbench session
eb = jl(R + "/ecbench/session-20261015-d4-ed/records.jsonl")
from collections import defaultdict
by = defaultdict(dict)
arms_eb = set()
for r in eb:
    if r["warmup"]:
        continue
    arms_eb.add(r["arm"])
    by[(r["workload"]["workload_id"], r["round"])][r["arm"]] = r["cost"]["s"]
print("ecbench arms:", sorted(arms_eb), "records:", len(eb), "workloads:", len(set(k[0] for k in by)))
pairs = [(v) for k, v in by.items() if "ic-k4-d4" in v and "ic-k4-d4-ed" in v]
print("ecbench paired (workload,round) with d4 and d4-ed:", len(pairs))
if pairs:
    num = sum(v["ic-k4-d4-ed"] for v in pairs); den = sum(v["ic-k4-d4"] for v in pairs)
    print("ecbench sum ratio d4-ed/d4 = %.4f" % (num / den))
    print("ecbench mean of per-pair ratios = %.4f" % statistics.fmean(v["ic-k4-d4-ed"] / v["ic-k4-d4"] for v in pairs))
    wl = defaultdict(list)
    for k, v in by.items():
        if "ic-k4-d4" in v and "ic-k4-d4-ed" in v:
            wl[k[0]].append(v["ic-k4-d4-ed"] / v["ic-k4-d4"])
    print("ecbench workloads with pairs:", len(wl), "per-workload means min=%.4f max=%.4f" % (min(statistics.fmean(x) for x in wl.values()), max(statistics.fmean(x) for x in wl.values())))
