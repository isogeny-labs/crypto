#!/usr/bin/env python3
"""Validator recompute (own code; does not import the producer's analyze.py).
Cost per target in GAE = [(cI*I + M + S)/(cI + 3) + h*(H + L)]/T  (PROTOCOL.md), cI in {3,10,30}, h in {0,0.1,1}.
Per configuration (pb, T, idx, seed): rhobest = min over the 12 rho-family runs at that (cI, h); bsgs2 = min over
its 3 mf runs; ic5/ic4 = the single run.  Ratio cand/ref per configuration; median over configurations.
CIs: (a) re-implementation of the frozen procedure (Python random.Random(1).choices, B = 4000, sorted medians,
indices int(.025B) and int(.975B)-1) to compare bit-for-bit with summary.json; (b) independent numpy bootstrap
(B = 20000, seeds 7 and 2026, 2.5/97.5 percentiles) for the decisions.
Decision per (cell, cI): cheaper if hi < 1 at all three h; dearer if lo > 1 at all three; else unranked.
usage: recompute.py v3|v2 <runs.json> <rows.jsonl> <summary.json> <out.json>"""
import json, sys, hashlib, random, statistics, collections
import numpy as np
mode, RUNS, ROWS, SUMM, OUT = sys.argv[1:6]
CI = [3, 10, 30]; HP = [0, 0.1, 1]
RHO = ["rho", "rho2", "rho2x", "rhoseq"]
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()
runs = json.load(open(RUNS)); F = runs["fields"]
R = [dict(zip(F, r)) for r in runs["rows"]]
rows = [json.loads(l) for l in open(ROWS)]
# runs.json must be an exact projection of rows.jsonl
proj_ok = len(R) == len(rows) and all(all((rw.get(f) if f in rw else None) == r[f] for f in F) for r, rw in zip(R, rows))
src_sha_ok = runs.get("source_sha256") == sha(ROWS)
bad = [r for r in R if r["ok"] != r["targets"]]
def cost(r, ci, h): return ((ci * r["I"] + r["M"] + r["S"]) / (ci + 3) + h * (r["H"] + r["L"])) / r["targets"]
cfg = collections.defaultdict(lambda: collections.defaultdict(list))
for r in R: cfg[(r["pb"], r["T"], r["idx"], r["seed"])][r["arm"]].append(r)
grid = {}
for k, v in cfg.items(): grid[k] = {a: len(x) for a, x in v.items()}
def val(k, a, ci, h, costf=cost):
    rs = [r for x in RHO for r in cfg[k][x]] if a == "rhobest" else cfg[k][a]
    return min(costf(r, ci, h) for r in rs)
def argbest(k, ci, h):
    rs = [r for x in RHO for r in cfg[k][x]]
    b = min(rs, key=lambda r: cost(r, ci, h)); return f'{b["arm"]}@{b["param"]}'
def frozen_boot(xs, B=4000, seed=1):
    rnd = random.Random(seed); n = len(xs)
    meds = sorted(statistics.median(rnd.choices(xs, k=n)) for _ in range(B))
    return meds[int(.025 * B)], meds[int(.975 * B) - 1]
def np_boot(xs, B=20000, seed=7):
    x = np.asarray(xs); g = np.random.default_rng(seed)
    m = np.median(x[g.integers(0, len(x), size=(B, len(x)))], axis=1)
    return float(np.percentile(m, 2.5)), float(np.percentile(m, 97.5))
if mode == "v3":
    pairs = [("ic5", "rhobest"), ("ic5", "bsgs2"), ("ic5", "ic4"), ("ic4", "rhobest"), ("ic4", "bsgs2")]
else:
    pairs = [("ic4", a) for a in ["rhobest"] + RHO + ["bsgs2"]]
cells = sorted({k[:2] for k in cfg})
E = []
winners = collections.Counter()
for ci in CI:
    for pb, T in cells:
        ks = sorted(k for k in cfg if k[:2] == (pb, T))
        for h in HP:
            for k in ks: winners[argbest(k, ci, h).split("@")[0]] += 1
            for cand, ref in pairs:
                rat = [val(k, cand, ci, h) / val(k, ref, ci, h) for k in ks]
                lo, hi = frozen_boot(rat); lo2, hi2 = np_boot(rat, seed=7); lo3, hi3 = np_boot(rat, seed=2026)
                E.append(dict(cand=cand, vs=ref, cI=ci, bits=pb, T=T, h=h, n=len(rat), median=statistics.median(rat),
                              lo=lo, hi=hi, lo_np=min(lo2, lo3), hi_np=max(hi2, hi3), wins=sum(x < 1 for x in rat),
                              ic=statistics.median([val(k, cand, ci, h) for k in ks]),
                              ref=statistics.median([val(k, ref, ci, h) for k in ks])))
def decide(sel, lo="lo", hi="hi"):
    return "cheaper" if all(s[hi] < 1 for s in sel) else ("dearer" if all(s[lo] > 1 for s in sel) else "unranked")
D = {}
for cand, ref in pairs:
    for ci in CI:
        for pb, T in cells:
            sel = [e for e in E if e["cand"] == cand and e["vs"] == ref and e["cI"] == ci and e["bits"] == pb and e["T"] == T]
            D[f"{cand}|{ref}|{ci}|{pb}|{T}"] = dict(frozen=decide(sel), numpy=decide(sel, "lo_np", "hi_np"),
                                                     max_hi=max(s["hi"] for s in sel), min_lo=min(s["lo"] for s in sel),
                                                     max_hi_np=max(s["hi_np"] for s in sel), min_lo_np=min(s["lo_np"] for s in sel))
# compare with summary.json
S = json.load(open(SUMM))
def skey(s): return (s.get("cand", "ic4"), s["vs"], s["cI"], s["bits"], s["T"], s["h"])
mine = {(e["cand"], e["vs"], e["cI"], e["bits"], e["T"], e["h"]): e for e in E}
cmp = dict(entries=len(S), matched=0, max_rel_median=0.0, max_abs_ci=0.0, missing=[], ic_ref_max_rel=0.0, wins_mismatch=0)
for s in S:
    k = skey(s)
    if k not in mine: cmp["missing"].append(k); continue
    e = mine[k]; cmp["matched"] += 1
    cmp["max_rel_median"] = max(cmp["max_rel_median"], abs(e["median"] - s["median"]) / abs(s["median"]))
    cmp["max_abs_ci"] = max(cmp["max_abs_ci"], abs(e["lo"] - s["lo"]), abs(e["hi"] - s["hi"]))
    cmp["ic_ref_max_rel"] = max(cmp["ic_ref_max_rel"], abs(e["ic"] - s["ic"]) / s["ic"], abs(e["ref"] - s["ref"]) / s["ref"])
    cmp["wins_mismatch"] += e["wins"] != s["wins"]
counts = collections.defaultdict(collections.Counter)
for key, d in D.items():
    cand, ref = key.split("|")[:2]; counts[f"{cand} vs {ref}"][d["frozen"]] += 1
disagree = {k: d for k, d in D.items() if d["frozen"] != d["numpy"]}
out = dict(mode=mode, runs_json=RUNS, runs_sha256=sha(RUNS), rows_sha256=sha(ROWS), summary_sha256=sha(SUMM),
           runs_json_exact_projection=proj_ok, runs_source_sha256_matches_rows=src_sha_ok, n_runs=len(R),
           unverified_runs=len(bad), configs=len(cfg),
           grid_arm_counts=collections.Counter(tuple(sorted(g.items())) for g in grid.values()).most_common(),
           summary_compare=cmp, decision_counts={k: dict(v) for k, v in counts.items()},
           frozen_vs_numpy_decision_disagreements=disagree, rhobest_winner_counts=dict(winners), decisions=D, entries=E)
out["grid_arm_counts"] = [[list(map(list, k)), v] for k, v in out["grid_arm_counts"]]
json.dump(out, open(OUT, "w"), indent=1)
print(json.dumps({k: out[k] for k in ["runs_json_exact_projection", "runs_source_sha256_matches_rows", "n_runs", "unverified_runs", "configs", "summary_compare", "decision_counts", "rhobest_winner_counts"]}, indent=1))
print("frozen vs numpy decision disagreements:", len(disagree))
