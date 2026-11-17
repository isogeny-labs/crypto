# Row 70 recompute: 20261014-ic-improve-02-equivariant-oracle. Reads committed files only. Run from repo root with python3 -I.
import json, math, statistics
from collections import defaultdict
R = "research/ic_improve_20261014/02-equivariant-oracle/runs"
def jl(p): return [json.loads(x) for x in open(p) if x.strip()]
def gm(x): return math.exp(statistics.fmean(math.log(v) for v in x))
orc = jl(R + "/oracle.jsonl")
print("oracle rows:", len(orc), "gate true:", sum(1 for r in orc if r["gate"]), "gate false:", sum(1 for r in orc if not r["gate"]), "decomposes true:", sum(1 for r in orc if r["decomposes"]))
by = defaultdict(list)
for r in orc: by[r["n"]].append(r)
for n in sorted(by):
    rows = by[n]
    print("n=%d targets=%d I/C gm=%.4f I/F gm=%.4f F/C gm=%.4f decompose=%d" % (n, len(rows), gm([r["i_cpu"] / r["c_cpu"] for r in rows]), gm([r["i_cpu"] / r["f_cpu"] for r in rows]), gm([r["f_cpu"] / r["c_cpu"] for r in rows]), sum(1 for r in rows if r["decomposes"])))
# step level
st = jl(R + "/step_small.jsonl") + jl(R + "/step_large.jsonl")
print("step rows:", len(st), "gate false:", sum(1 for r in st if r["gate"] is False), "gate true:", sum(1 for r in st if r["gate"] is True))
fewer = sum(1 for r in st if r.get("spec_linear") is not None and r.get("gauge_linear") is not None and r["spec_linear"] == r["gauge_linear"] - 1)
print("step rows with spec_linear = gauge_linear - 1:", fewer)
print("step n values:", sorted(set(r["n"] for r in st)))
ccheck = [r for r in st if r.get("i_cpu") and r.get("c_cpu")]
for n in sorted(set(r["n"] for r in st)):
    rr = [r for r in st if r["n"] == n and r.get("i_cpu") and r.get("c_cpu")]
    if rr: print("step n=%d I/C gm=%.4f count=%d" % (n, gm([r["i_cpu"] / r["c_cpu"] for r in rr]), len(rr)))
# pipeline probes
pl = jl(R + "/lead_fall_rows_pipeline.jsonl")
print("pipeline rows:", len(pl), "keys probe:", sorted(set(r["probe"] for r in pl)))
cells = defaultdict(list)
for r in pl: cells[(r["n"], r["m"], r["ell"])].append(r["i_cpu"] / r["f_cpu"])
for k in sorted(cells):
    print("pipeline cell n=%d m=%d l=%d I/F gm=%.4f count=%d" % (k[0], k[1], k[2], gm(cells[k]), len(cells[k])))
allr = [r["i_cpu"] / r["f_cpu"] for r in pl]
print("pipeline I/F per-record min %.4f max %.4f" % (min(allr), max(allr)))
