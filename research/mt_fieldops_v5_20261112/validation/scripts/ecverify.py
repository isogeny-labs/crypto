#!/usr/bin/env python3
"""VALIDATOR certificate check of recovered logs from instrumented dumps, with the validator's own EC code (ec.py):
for every dumped target t of every dumped run: Q_t on the curve, 0 <= log < n, log == planted x_t, [log]G == Q_t,
and the planted list identical across all runs of one configuration.  [log]G is computed once per distinct
(curve, log, Q) and compared for every run that claims it.
usage: ecverify.py <frozen study dir> <replay_dump.json> <out.json>"""
import json, sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ec import load
FZ, RD, OUT = sys.argv[1:4]
rd = json.load(open(RD))["results"]
rows = [json.loads(l) for l in open(os.path.join(FZ, "results/holdout/rows.jsonl"))]
cache = {}; per = []; planted = {}; fails = collections.Counter(); nt = 0; ec_mults = 0
for x in rd:
    r = rows[x["row"]]; d = json.load(open(x["dump"]))
    inst = os.path.join(FZ, "instances", f"W{r['pb']}-{r['idx']}.json")
    _, C, G, n = load(inst)
    xs = [int(v) for v in d["xs"]]; qx = [int(v) for v in d["qx"]]; qy = [int(v) for v in d["qy"]]; fd = [int(v) for v in d["found"]]
    key = (r["pb"], r["T"], r["idx"], r["seed"])
    if key in planted:
        if planted[key] != (xs, qx, qy): fails["planted_differs_across_runs"] += 1
    else:
        planted[key] = (xs, qx, qy)
    f = collections.Counter()
    for t in range(len(fd)):
        nt += 1; Q = (qx[t], qy[t])
        if not C.on(Q): f["Q_off_curve"] += 1
        if not (0 <= fd[t] < n): f["log_out_of_range"] += 1
        if fd[t] != xs[t] % n: f["log_ne_planted"] += 1
        ck = (inst, fd[t], Q)
        if ck not in cache:
            cache[ck] = C.mul(fd[t], G) == Q; ec_mults += 1
        if not cache[ck]: f["logG_ne_Q"] += 1
    fails.update(f)
    per.append(dict(row=x["row"], arm=r["arm"], param=r["param"], doff=r.get("doff"), pb=r["pb"], T=r["T"], idx=r["idx"], seed=r["seed"],
                    targets=len(fd), failures=dict(f), targets_eq_row=len(fd) == r["targets"]))
summ = dict(runs=len(per), targets_checked=nt, distinct_ec_scalar_mults=ec_mults, configurations=len(planted),
            arms=dict(collections.Counter(p["arm"] for p in per)), bits=dict(collections.Counter(p["pb"] for p in per)),
            failures=dict(fails), all_pass=not fails and all(p["targets_eq_row"] for p in per))
json.dump(dict(summary=summ, runs=per), open(OUT, "w"), indent=1)
print(json.dumps(summ, indent=1))
