#!/usr/bin/env python3
"""Validator: recompute the frozen tune objective (PROTOCOL.md: mean over the 16 tune configurations of
cost(cI = 10, h = 0.1) relative to the configuration's best grid point) from tune/rows.jsonl; check the frozen
choice, grid completeness, tune/holdout disjointness (idx, seeds) and verification.
usage: tune_check.py <tune rows.jsonl> <holdout rows.jsonl> <frozen_ic5.json> <out.json>"""
import json, sys, collections, statistics
T_, H_, FZ, OUT = sys.argv[1:5]
tr = [json.loads(l) for l in open(T_)]; hr = [json.loads(l) for l in open(H_)]; fz = json.load(open(FZ))
def c(r): return ((10 * r["I"] + r["M"] + r["S"]) / 13 + 0.1 * (r["H"] + r["L"])) / r["targets"]
cfg = collections.defaultdict(dict)
for r in tr: cfg[(r["pb"], r["T"], r["seed"])][(r["param"], r["k"])] = c(r)
grid = sorted({g for v in cfg.values() for g in v})
score = {g: statistics.mean(v[g] / min(v.values()) for v in cfg.values()) for g in grid}
best = min(score, key=score.get)
out = dict(tune_rows=len(tr), tune_unverified=sum(r["ok"] != r["targets"] for r in tr), configs=len(cfg),
           grid_complete=all(len(v) == 10 for v in cfg.values()) and len(grid) == 10,
           tune_idx=sorted({r["idx"] for r in tr}), tune_seeds=sorted({r["seed"] for r in tr}),
           holdout_idx=sorted({r["idx"] for r in hr}), holdout_seeds=sorted({r["seed"] for r in hr}),
           holdout_ic5_params=sorted({(r["param"], r["k"]) for r in hr if r["arm"] == "ic5"}),
           score={f"mu={g[0]},k={g[1]}": round(s, 6) for g, s in sorted(score.items(), key=lambda kv: kv[1])},
           argmin=dict(mu=best[0], k=best[1]), frozen=dict(mu=fz["mu"], k=fz["k"]),
           frozen_matches_argmin=(best[0], best[1]) == (fz["mu"], fz["k"]),
           tune_rows_commits=sorted({r["commit"] for r in tr}), tune_bin=sorted({r["bin_sha256"] for r in tr}))
out["disjoint"] = not (set(out["tune_idx"]) & set(out["holdout_idx"])) and not (set(out["tune_seeds"]) & set(out["holdout_seeds"]))
json.dump(out, open(OUT, "w"), indent=1); print(json.dumps(out, indent=1))
