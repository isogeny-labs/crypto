"""Collects tables/compare-*.txt (ecbench compare, legacy / v2 and legacy / A/A)
and the per-arm mean S and counters from sessions/*/records.jsonl into
summary.json.  python3 summarize.py"""
import collections, json, re
CELLS = ["a0-n13", "a1-n17", "a0-n19", "a0-n23", "a1-n29", "a0-n31", "a0-n37", "a0-n41"]
pat = re.compile(r"= ([0-9.]+) \[([0-9.]+), ([0-9.]+)\]")
out = []
for c in CELLS:
    v = pat.search(open(f"tables/compare-{c}-rho-frob-vs-rho-frob-v2.txt").readline())
    a = pat.search(open(f"tables/compare-{c}-rho-frob-vs-rho-frob-aa.txt").readline())
    recs = [json.loads(l) for l in open(f"sessions/{c}/records.jsonl") if l.strip()]
    by = collections.defaultdict(list)
    for r in recs:
        if not r.get("warmup"):
            by[r["arm"]].append(r)
    arms = {}
    for arm, rs in by.items():
        agg = collections.defaultdict(float)
        for r in rs:
            for k, x in (r.get("counters") or {}).items():
                if isinstance(x, (int, float)):
                    agg[k] += x / len(rs)
        arms[arm] = {"runs": len(rs), "mean_s": sum(r["cost"]["s"] for r in rs) / len(rs),
                     "walk_operations": agg["walk_operations"], "walks_capped": agg["walks_capped"],
                     "cycle_escape_doublings": agg["cycle_escape_doublings"]}
    out.append({"cell": c, "log2r": recs[0]["workload"]["curve"].get("log2_r") or None,
                "legacy_over_v2": float(v.group(1)), "ci": [float(v.group(2)), float(v.group(3))],
                "legacy_over_aa": float(a.group(1)), "arms": arms})
json.dump(out, open("summary.json", "w"), indent=1)
for o in out:
    print(o["cell"], o["legacy_over_v2"], o["ci"], {a: round(x["mean_s"], 3) for a, x in o["arms"].items()})
