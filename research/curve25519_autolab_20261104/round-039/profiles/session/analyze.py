#!/usr/bin/env python3
"""Paired analysis of the ABBA wall-time sessions (protocol: PROTOCOL.md).
usage: analyze.py SESSIONS_DIR  (contains S1-base S2-new S3-new S4-base)"""
import json, sys, random, statistics as st
D = sys.argv[1]
names = ["S1-base", "S2-new", "S3-new", "S4-base"]
random.seed(20261120)
data = {}
for n in names:
    d = {}
    lv = {}
    for line in open(f"{D}/{n}/records.jsonl"):
        r = json.loads(line)
        if r["warmup"]:
            continue
        key = (r["arm"], r["workload"]["workload_id"], r["workload"]["target_index"], r["round"])
        d[key] = (r["time"]["solve_wall_ns"], r["cost"]["total_gae"], r["outcome"]["recovered"], r["outcome"]["status"])
        lv[r["isolation"]["level"]] = lv.get(r["isolation"]["level"], 0) + 1
    data[n] = d
    s = json.load(open(f"{D}/{n}/session.json"))
    print(n, s["status"], "binary", s["binary_sha256"][:8], "records", len(d), "isolation", lv)

def keys(arm):
    return sorted(k for k in data[names[0]] if k[0] == arm)

def mean_ratio(num, den, arm_n, arm_d, idx):
    """ratio of means, bootstrap over (workload,target) units (rounds pooled)."""
    kn, kd = keys(arm_n), keys(arm_d)
    units = {}
    for a, b in zip(kn, kd):
        assert a[1:] == b[1:], (a, b)
    for k in kn:
        units.setdefault(k[1:3], []).append(k)
    ul = list(units)
    def agg(sample):
        sn = sd = 0.0; c = 0
        for u in sample:
            for k in units[u]:
                kd_ = (arm_d,) + k[1:]
                sn += sum(data[x][k][0] for x in num) / len(num)
                sd += sum(data[x][kd_][0] for x in den) / len(den)
                c += 1
        return sn / sd
    pt = agg(ul)
    bs = sorted(agg([random.choice(ul) for _ in ul]) for _ in range(1000))
    return pt, bs[25], bs[974]

out = {}
def rep(label, num, den, arm_n, arm_d=None):
    arm_d = arm_d or arm_n
    pt, lo, hi = mean_ratio(num, den, arm_n, arm_d, 0)
    out[label] = [pt, lo, hi]
    print(f"{label:46s} {pt:7.4f}  [{lo:.4f}, {hi:.4f}]")

base, new = ["S1-base", "S4-base"], ["S2-new", "S3-new"]
print("\n-- contrast: new / base, mean solve_wall_ns (ABBA pooled)")
for a in ("sel", "inc", "sel-aa", "inc-aa"):
    rep(f"{a} new/base", new, base, a)
print("\n-- noise floor: same binary, different sessions")
for a in ("sel", "inc"):
    rep(f"{a} S4/S1 (base vs base)", ["S4-base"], ["S1-base"], a)
    rep(f"{a} S3/S2 (new vs new)", ["S3-new"], ["S2-new"], a)
print("\n-- A/A within a session (copy arm / arm)")
for s in names:
    rep(f"{s} sel-aa/sel", [s], [s], "sel-aa", "sel")
    rep(f"{s} inc-aa/inc", [s], [s], "inc-aa", "inc")
print("\n-- sel / inc within session (the harness ratio)")
for s in names:
    rep(f"{s} sel/inc", [s], [s], "sel", "inc")
rep("base sessions sel/inc", base, base, "sel", "inc")
rep("new sessions sel/inc", new, new, "sel", "inc")

print("\n-- invariance (counted cost, scalar, status) across the four sessions")
bad = 0; total = 0
for k in data[names[0]]:
    vals = {(data[n][k][1], data[n][k][2], data[n][k][3]) for n in names}
    total += 1
    if len(vals) != 1:
        bad += 1
print(f"records compared {total}, mismatches {bad}")
out["invariance"] = {"records": total, "mismatches": bad}
out["means_ns"] = {n: {a: st.mean(data[n][k][0] for k in keys(a)) for a in ("inc", "sel", "inc-aa", "sel-aa")} for n in names}
json.dump(out, open(f"{sys.argv[2] if len(sys.argv) > 2 else 'analysis.json'}", "w"), indent=1)
