# Row 72 recompute: 20261014-ic-improve-01-d4-2lp. Reads committed files only. Run from repo root with python3 -I.
import json, math, statistics, os
from collections import defaultdict
R = "research/ic_improve_20261014/01-d4-2lp"
test = [json.loads(x) for x in open(R + "/raw/test.jsonl") if x.strip()]
def key(r): return json.dumps({k: v for k, v in r.items() if k != "arms"}, sort_keys=True)
arm_map = defaultdict(list)
for r in test:
    for a in r["arms"]:
        arm_map[a["arm"]].append((r, a))
base = {key(r): a for r, a in arm_map["d4"]}
def gm(x): return math.exp(statistics.fmean(math.log(v) for v in x))
def ols(x, y):
    mx = statistics.fmean(x); my = statistics.fmean(y)
    return sum((a - mx) * (b - my) for a, b in zip(x, y)) / sum((a - mx) ** 2 for a in x)
def ratio(arm, field):
    rr = []; lE = []
    for r, a in arm_map[arm]:
        rr.append(a["gae"][field] / base[key(r)]["gae"][field]); lE.append(r["log2_E"])
    return rr, lE
print("test curves:", len(test), "arms:", sorted(arm_map.keys()))
for arm, field in [("d4-2lp", "whole"), ("d4-2lp", "relation"), ("d4-2lp", "la"), ("d4-2lp-gaudry", "whole"), ("d4-2lp-null", "whole"), ("d4-half", "whole")]:
    rr, lE = ratio(arm, field)
    print("%-15s %-9s pooled geomean=%.4f min=%.4f max=%.4f n=%d" % (arm, field, gm(rr), min(rr), max(rr), len(rr)))
rr, lE = ratio("d4-2lp", "whole")
print("d4-2lp whole exponent OLS slope of log2 ratio on log2_E = %.5f" % ols(lE, [math.log2(x) for x in rr]))
# ecbench session
eb = [json.loads(x) for x in open(R + "/ecbench/session-20261014-d4-2lp/records.jsonl") if x.strip()]
by = defaultdict(dict)
for r in eb:
    if r["warmup"]: continue
    by[r["workload"]["workload_id"]][r["arm"]] = r["cost"]["s"]
pr = [(v["ic-k4-d4-2lp"], v["ic-k4-d4"]) for v in by.values() if "ic-k4-d4-2lp" in v and "ic-k4-d4" in v]
print("ecbench pairs d4-2lp/d4:", len(pr), "sum ratio %.4f" % (sum(a for a, b in pr) / sum(b for a, b in pr)), "mean of pair ratios %.4f" % statistics.fmean(a / b for a, b in pr))
print("ecbench verified:", sum(1 for r in eb if r["outcome"]["status"] == "verified"), "of", len(eb))
# fits / handover in summary
s = json.load(open(R + "/raw/summary.json"))
def walk(d, path=""):
    if isinstance(d, dict):
        for k, v in d.items():
            if "handover" in k.lower() or "fit" in k.lower(): print("summary key:", path + k, str(v)[:200])
            walk(v, path + k + ".")
walk(s)
print("replay audit files in session dir:", sorted(os.listdir(R + "/ecbench/session-20261014-d4-2lp")))
