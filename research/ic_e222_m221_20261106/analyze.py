#!/usr/bin/env python3
"""Per (family, r-bit rung): median trials_total over instances (seed-averaged),
bootstrap 95% CI, ratio to the cofactor-1 control W, log2-slope vs r_bits."""
import json, sys, math, random, statistics, collections
rows=[json.loads(l) for l in open(sys.argv[1]+"/rows.jsonl")]
bad=[r for r in rows if not r["ok"] or r["status"]!="complete"]
by=collections.defaultdict(lambda: collections.defaultdict(list))  # (fam,rung)->name->[trials]
for r in rows:
    if r["ok"] and r["trials_total"]: by[(r["family"],r["r_bits"])][r["name"]].append(r["trials_total"])
def rung(rb): return rb
rng=random.Random(7)
def boot(xs,f=statistics.median,B=2000):
    xs=list(xs); return sorted(f([rng.choice(xs) for _ in xs]) for _ in range(B))
out={"unverified_rows":len(bad),"n_rows":len(rows),"cells":[]}
cells={}
for (fam,rb),inst in sorted(by.items()):
    m=[statistics.mean(v) for v in inst.values()]
    # A/A: relative seed spread within instance
    aa=[statistics.pstdev(v)/statistics.mean(v) for v in inst.values() if len(v)>1]
    b=boot(m)
    cells[(fam,rb)]=m
    out["cells"].append(dict(family=fam,r_bits=rb,n_inst=len(m),median=statistics.median(m),lo=b[50],hi=b[1949],
        seed_cv_median=statistics.median(aa) if aa else None))
# ratio to W at same rung (paired by rung only: independent curves, bootstrap ratio of medians)
for c in out["cells"]:
    w=cells.get(("W",c["r_bits"]))
    if w and c["family"]!="W":
        x=cells[(c["family"],c["r_bits"])]
        rs=sorted(statistics.median([rng.choice(x) for _ in x])/statistics.median([rng.choice(w) for _ in w]) for _ in range(2000))
        c["ratio_to_W"]=statistics.median(x)/statistics.median(w); c["ratio_lo"]=rs[50]; c["ratio_hi"]=rs[1949]
# slope of log2(median trials) vs r_bits per family
for fam in sorted({c["family"] for c in out["cells"]}):
    pts=[(c["r_bits"],math.log2(c["median"])) for c in out["cells"] if c["family"]==fam]
    n=len(pts);mx=sum(x for x,_ in pts)/n;my=sum(y for _,y in pts)/n
    sl=sum((x-mx)*(y-my) for x,y in pts)/sum((x-mx)**2 for x,_ in pts)
    out.setdefault("slopes",{})[fam]=sl
json.dump(out,open(sys.argv[1]+"/summary.json","w"),indent=1)
for c in out["cells"]:
    print(f'{c["family"]} r={c["r_bits"]:>2} n={c["n_inst"]:>2} median={c["median"]:9.0f} [{c["lo"]:.0f},{c["hi"]:.0f}] seedCV={c["seed_cv_median"]:.2f}'+
      (f' ratio/W={c["ratio_to_W"]:.2f} [{c["ratio_lo"]:.2f},{c["ratio_hi"]:.2f}]' if "ratio_to_W" in c else ""))
print("slopes log2(trials)/bit:",{k:round(v,3) for k,v in out["slopes"].items()},"unverified",len(bad),"of",len(rows))
