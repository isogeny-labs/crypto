#!/usr/bin/env python3
"""Independent check of the logs the retention arms claim for the points they insert (samples written by
the instrumented build, x % 211 == 0, first 300 per run, from every run of the full replay):
  ks    (rho2xks retention): [log]G has abscissa x; if the full y was stored, [log]G == (x, y); if only x
        was stored (sibling P - A), [log]G == prev - A_h, recomputed here from the dumped prev and A_h
  dppre (rhodp precomputation) and dpret (rhodp retention): [log]G == the canonical point (x, y <= (p-1)/2)
  icret (ic5ks retention): [log]G has abscissa x (the class x(Q_t +- W_k); the harness resolves the sign
        by a counted recomputation)
usage: verify_samples.py <frozen study dir> <replay_all.json> <out.json>"""
import json, sys, os, collections
from multiprocessing import Pool
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ecv import load
study, rp, out = sys.argv[1:4]
RP = json.load(open(rp))["runs"]
rows = [json.loads(l) for l in open(os.path.join(study, "results/holdout/rows.jsonl"))]
def check(run):
    r = rows[run["i"]]
    _, C, G, n = load(os.path.join(study, "instances", os.path.basename(r["argv"][1])))
    p = C.p; bad = []; cnt = collections.Counter()
    for s in run.get("samp", []):
        kind = s[0]; cnt[kind] += 1; x, lg = s[1], s[2]
        P = C.mul(lg, G)
        if kind == "ks":
            hasy, y, px, py, ax, ay = s[3], s[4], s[5], s[6], s[7], s[8]
            ok = P is not None and P[0] == x and (P == (x, y) if hasy else P == C.add((px, py), C.neg((ax, ay))))
        elif kind in ("dppre", "dpret"):
            ok = P is not None and P[0] == x and P[1] <= (p - 1) // 2
        elif kind == "icret":
            ok = P is not None and P[0] == x
        else:
            ok = False
        if not ok: bad.append(s)
    return dict(i=run["i"], arm=run["arm"], n=dict(cnt), bad=bad)
with Pool(4) as pool:
    res = pool.map(check, [r for r in RP if r.get("samp")])
tot = collections.Counter(); badn = 0
for r in res:
    tot.update(r["n"]); badn += len(r["bad"])
summ = dict(runs_with_samples=len(res), samples_checked=dict(tot), total=sum(tot.values()), failures=badn,
            per_arm_runs=dict(collections.Counter(r["arm"] for r in res)),
            failure_examples=[b for r in res for b in r["bad"]][:10])
json.dump(summ, open(out, "w"), indent=1)
print(json.dumps(summ, indent=1))
