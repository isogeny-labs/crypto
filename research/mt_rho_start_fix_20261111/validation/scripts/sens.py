#!/usr/bin/env python3
"""Validator sensitivity of the ic5 (or ic4) vs rhobest decisions to the rho reference's avoidable overheads and
to the standard Kuhn-Struik retention rule.  Inputs: rows.jsonl (producer, replayed), the instrumented replay
(restart cost vectors, phase-2 lookups and repeated lookups), and optionally the validator's KS rows.
Readings (each re-runs the frozen decision rule: per (cell, cI), cheaper iff the bootstrap-95% upper bound of the
median per-configuration ratio is < 1 at h = 0, 0.1, 1; dearer iff the lower bound > 1 at all three):
  as_measured          rhobest = min of the 12 measured rho runs
  no_restart           every rho run minus its measured chained-restart additions (phase 1 and phase 2)
  no_restart_no_repeat no_restart, and phase-2 walk cost scaled by (1 - repeated/lookups)
  ks                   rhobest' = min(rhobest, rho2xks at kappa 0.05/0.1/0.25/0.5)  [validator arm, measured]
  ks_no_restart        ks with rho2xks also minus its restart cost estimated at the rho2x rate (not measured)
Each with and without the shared verification phase.
usage: sens.py <cand> <rows.jsonl> <dbg replay.jsonl> <out.json> [ks_rows.jsonl]"""
import json, sys, random, statistics, collections
CAND, ROWS, DBG, OUT = sys.argv[1:5]; KS = sys.argv[5] if len(sys.argv) > 5 else None
RHO = ["rho", "rho2", "rho2x", "rhoseq"]
rows = [json.loads(l) for l in open(ROWS)]
dbg = {r["i"]: r for r in map(json.loads, open(DBG))}
assert len(dbg) == len(rows) and all(not d["mismatch"] for d in dbg.values())
def vec(r): return [r["I"], r["M"], r["S"], r["H"], r["L"]]
def sub(a, b): return [x - y for x, y in zip(a, b)]
def ph(r, k): return r["phases"][k]
def verify_vec(r):
    ks = list(r["phases"]); return sub(r["phases"][ks[-1]], r["phases"][ks[-2]])
def g(v, ci, h, T): return ((ci * v[0] + v[1] + v[2]) / (ci + 3) + h * (v[3] + v[4])) / T
variants = {}
for i, r in enumerate(rows):
    d = dbg[i]["instr"]; base = vec(r)
    v = {"as_measured": base}
    if r["arm"] in RHO:
        nr = sub(sub(base, d["restart_p1"]), d["restart_p2"])
        v["no_restart"] = nr
        p2 = sub(sub(ph(r, "targets"), ph(r, "db")), d["restart_p2"])
        rep = d["repeat_lookups_p2"] / max(d["lookups_p2"], 1)
        v["no_restart_no_repeat"] = [x - rep * y for x, y in zip(nr, p2)]
    else:
        v["no_restart"] = base; v["no_restart_no_repeat"] = base
    variants[i] = (v, verify_vec(r))
ksrows = [json.loads(l) for l in open(KS)] if KS else []
cfg = collections.defaultdict(lambda: collections.defaultdict(list))
for i, r in enumerate(rows): cfg[(r["pb"], r["T"], r["idx"], r["seed"])][r["arm"]].append(i)
kscfg = collections.defaultdict(list)
for r in ksrows:
    if r["arm"] == "rho2xks" and "error" not in r and r["ok"] == r["targets"]: kscfg[(r["pb"], r["T"], r["idx"], r["seed"])].append(r)
# rho2x restart cost per step (measured, used only for the ks_no_restart estimate)
rr = [dbg[i]["instr"]["restart_p2"] for i, r in enumerate(rows) if r["arm"] == "rho2x"]
def boot(xs, B=4000, seed=1):
    rnd = random.Random(seed); n = len(xs)
    meds = sorted(statistics.median(rnd.choices(xs, k=n)) for _ in range(B)); return meds[int(.025 * B)], meds[int(.975 * B) - 1]
def cost_i(i, var, ci, h, nover):
    v, ver = variants[i]; x = v[var]
    if nover: x = sub(x, ver)
    return g(x, ci, h, rows[i]["targets"])
def cost_ks(r, ci, h, nover, norestart):
    x = [r["I"], r["M"], r["S"], r["H"], r["L"]]
    if norestart:  # one padd per 32 target steps (I + 2M + S) and per 32 phase-1 steps
        nres = (r["db_steps"] + r["target_steps"]) / 32.0; x = [x[0] - nres, x[1] - 2 * nres, x[2] - nres, x[3], x[4]]
    if nover:
        ks_ = list(r["phases"]); x = sub(x, sub(r["phases"][ks_[-1]], r["phases"][ks_[-2]]))
    return g(x, ci, h, r["targets"])
readings = ["as_measured", "no_restart", "no_restart_no_repeat"] + (["ks", "ks_no_restart"] if KS else [])
cells = sorted({k[:2] for k in cfg})
res = {}
for rd in readings:
    for nover in (False, True):
        key = rd + ("_noverify" if nover else ""); dec = {}; ent = []
        for ci in [3, 10, 30]:
            for pb, T in cells:
                ks_ = sorted(k for k in cfg if k[:2] == (pb, T)); stats = []
                for h in [0, 0.1, 1]:
                    rat = []; who = collections.Counter()
                    for k in ks_:
                        var = rd if rd in ("as_measured", "no_restart", "no_restart_no_repeat") else "as_measured"
                        c = cost_i(cfg[k][CAND][0], "as_measured", ci, h, nover)
                        best = min((cost_i(i, var, ci, h, nover), rows[i]["arm"]) for a in RHO for i in cfg[k][a])
                        if rd.startswith("ks"):
                            for r in kscfg[k]:
                                best = min(best, (cost_ks(r, ci, h, nover, rd == "ks_no_restart"), "rho2xks"))
                        who[best[1]] += 1; rat.append(c / best[0])
                    lo, hi = boot(rat); stats.append((lo, hi))
                    ent.append(dict(cI=ci, bits=pb, T=T, h=h, median=statistics.median(rat), lo=lo, hi=hi, ref_winner=dict(who)))
                dec[f"{pb}/{T}/cI{ci}"] = "cheaper" if all(s[1] < 1 for s in stats) else ("dearer" if all(s[0] > 1 for s in stats) else "unranked")
        res[key] = dict(decision_counts=dict(collections.Counter(dec.values())), decisions=dec,
                        median_range=[min(e["median"] for e in ent), max(e["median"] for e in ent)],
                        max_hi=max(e["hi"] for e in ent), entries=ent)
# overhead shares of the rho runs (cI = 10, h = 0): restart cost and repeated lookups
sh = collections.defaultdict(list)
for i, r in enumerate(rows):
    if r["arm"] not in RHO: continue
    d = dbg[i]["instr"]; tot = g(vec(r), 10, 0, 1)
    sh[(r["arm"], r["pb"], r["T"])].append(dict(restart=(g(d["restart_p1"], 10, 0, 1) + g(d["restart_p2"], 10, 0, 1)) / tot,
                                                repeat=d["repeat_lookups_p2"] / max(d["lookups_p2"], 1),
                                                p1_steps_per_db=r["db_steps"] / r["db"]))
share = {f"{a}/{pb}/{T}": {f: [round(min(x[f] for x in v), 4), round(statistics.median(x[f] for x in v), 4), round(max(x[f] for x in v), 4)] for f in v[0]}
         for (a, pb, T), v in sorted(sh.items())}
json.dump(dict(cand=CAND, readings=res, rho_overhead_min_median_max=share), open(OUT, "w"), indent=1)
for k, v in res.items(): print(k, v["decision_counts"], "median range %.3f-%.3f max_hi %.3f" % (*v["median_range"], v["max_hi"]))
