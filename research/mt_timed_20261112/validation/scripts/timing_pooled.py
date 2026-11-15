#!/usr/bin/env python3
"""VALIDATOR: per-cell icbest/rhobest from the validator's own timings with the producer's per-configuration keys,
for round 1 (timing.py, 3 repeats), round 2 (timing_round2.py, 5 repeats, added after round 1) and both pooled
(8 repeats); protocol bootstrap and decision; per-configuration and per-repeat sign counts; interference.
usage: timing_pooled.py <timing.jsonl> <timing_r2.jsonl> <recompute.json> <out.json>"""
import json, sys, random, collections, statistics as st, math

T1, T2, RC, OUT = sys.argv[1:5]
rc = json.load(open(RC))


def med(xs):
    xs = sorted(xs); n = len(xs)
    return xs[n // 2] if n % 2 else 0.5 * (xs[n // 2 - 1] + xs[n // 2])


def boot(xs, B=4000, seed=1):
    rr = random.Random(seed); n = len(xs)
    m = sorted(st.median(rr.choices(xs, k=n)) for _ in range(B))
    return m[int(.025 * B)], m[int(.975 * B) - 1]


def verdict(lo, hi): return "faster" if hi < 1 else ("slower" if lo > 1 else "unranked")


def load(path):
    L = [json.loads(l) for l in open(path)]
    d = collections.defaultdict(list); runs = [x for x in L if x.get("kind") == "run" and x.get("binary", "producer") == "producer"]
    for r in runs:
        d[(r["pb"], r["T"], r["idx"], r["seed"], r["key"][0], r["key"][1], r["key"][2])].append((r["rep"], r["ms_arm"]))
    return d, runs, L


d1, r1, L1 = load(T1); d2, r2, L2 = load(T2)
out = dict(rounds={})
for name, srcs in (("round1", [d1]), ("round2", [d2]), ("pooled", [d1, d2])):
    cells = []; cfgs = []
    for e in rc["per_configuration"]:
        c = (e["bits"], e["T"], e["idx"], e["seed"]); ki = c + tuple(e["ms_arm"]["ic_key"]); kr = c + tuple(e["ms_arm"]["rho_key"])
        ti = [t for s in srcs for _, t in s.get(ki, [])]; tr = [t for s in srcs for _, t in s.get(kr, [])]
        cfgs.append(dict(bits=c[0], T=c[1], idx=c[2], n_ic=len(ti), n_rho=len(tr), ratio=med(ti) / med(tr), prod=e["ms_arm"]["ic_rho"]))
    for pb in (24, 28, 32):
        for T in (1024, 4096, 16384):
            xs = [x["ratio"] for x in cfgs if (x["bits"], x["T"]) == (pb, T)]; lo, hi = boot(xs)
            pc = [c for c in rc["cells"] if (c["bits"], c["T"]) == (pb, T)][0]
            cells.append(dict(bits=pb, T=T, median=med(xs), lo=lo, hi=hi, verdict=verdict(lo, hi), below1=sum(x < 1 for x in xs), values=xs,
                              prod_median=pc["ic_rho_solve"]["median"], prod_verdict=pc["verdict_solve"],
                              prod_fixed_key_verdict=verdict(pc["fixed_ic_rho_solve"]["lo"], pc["fixed_ic_rho_solve"]["hi"])))
    size = {pb: dict(below1=sum(x["ratio"] < 1 for x in cfgs if x["bits"] == pb), n=sum(x["bits"] == pb for x in cfgs),
                     median=med([x["ratio"] for x in cfgs if x["bits"] == pb])) for pb in (24, 28, 32)}
    out["rounds"][name] = dict(cells=cells, size_level=size, per_configuration=cfgs)
# round-2 per-repeat sign counts (paired within a repeat round) and interference
rep_ratio = collections.defaultdict(list)
for e in rc["per_configuration"]:
    c = (e["bits"], e["T"], e["idx"], e["seed"]); ki = c + tuple(e["ms_arm"]["ic_key"]); kr = c + tuple(e["ms_arm"]["rho_key"])
    a = dict(d2[ki]); b = dict(d2[kr])
    for rep in a:
        rep_ratio[(e["bits"], e["T"])].append(a[rep] / b[rep])
out["round2_per_repeat_below1"] = {f"{k[0]}/{k[1]}": [sum(x < 1 for x in v), len(v)] for k, v in sorted(rep_ratio.items())}
st_ = [r["steal_jiffies_pin"] for r in r2]; ob = [r["other_cpus_busy_jiffies"] for r in r2]; wall = sum(r["wall_s"] for r in r2)
out["round2_interference"] = dict(executions=len(r2), all_verified=all(r.get("ok") == r.get("targets") for r in r2), total_wall_s=wall,
                                  steal_share_of_wall=sum(st_) / 100 / wall, other_cpus_busy_share=sum(ob) / 100 / (3 * wall),
                                  loadavg_min_median_max=[min(r["loadavg"] for r in r2), med([r["loadavg"] for r in r2]), max(r["loadavg"] for r in r2)],
                                  header=[x for x in L2 if x.get("header")][0], footer=[x for x in L2 if x.get("footer")][0])
json.dump(out, open(OUT, "w"), indent=1)
for name, v in out["rounds"].items():
    print(name, json.dumps(v["size_level"]))
    for c in v["cells"]:
        print("  %d %5d %.3f [%.3f, %.3f] %-8s %d/4<1 | producer %.3f %s, producer fixed-key %s" % (
            c["bits"], c["T"], c["median"], c["lo"], c["hi"], c["verdict"], c["below1"], c["prod_median"], c["prod_verdict"], c["prod_fixed_key_verdict"]))
print("round2 per-repeat <1", out["round2_per_repeat_below1"])
print("round2 interference", json.dumps({k: v for k, v in out["round2_interference"].items() if k not in ("header", "footer")}))
