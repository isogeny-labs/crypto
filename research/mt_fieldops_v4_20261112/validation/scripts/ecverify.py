#!/usr/bin/env python3
"""Independent scalar verification of the dumped runs (validation/scripts/replay.py mode dump):
for every target of every dumped run: Q_t on the curve, 0 <= log < n, log == planted x_t mod n,
[log] G == Q_t with validation/scripts/ecv.py (Jacobian double-and-add, Fermat inversion; no harness code);
and the planted lists (x_t, Q_t) identical across all 20 runs of a configuration.
usage: ecverify.py <frozen study dir> <replay_dump.json> <out.json>"""
import json, sys, os, hashlib, collections
from multiprocessing import Pool
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ecv import load
study, dumpjson, out = sys.argv[1:4]
D = json.load(open(dumpjson)); ddir = dumpjson + ".dumps"
def check(run):
    d = json.load(open(os.path.join(ddir, run["dump"])))
    _, C, G, n = load(os.path.join(study, "instances", os.path.basename(d["argv"][1])))
    bad = 0
    for x, lg, qx, qy in d["q"]:
        Q = (qx, qy)
        if not (C.on(Q) and 0 <= lg < n and lg == x % n and C.mul(lg, G) == Q):
            bad += 1
    dig = hashlib.sha256(json.dumps([[x, qx, qy] for x, _, qx, qy in d["q"]]).encode()).hexdigest()
    return dict(i=run["i"], arm=run["arm"], cfg=[run["pb"], run["T"], run["idx"], run["seed"]], targets=len(d["q"]), bad=bad, planted_digest=dig)
with Pool(4) as p:
    res = p.map(check, [r for r in D["runs"] if "dump" in r])
by = collections.defaultdict(set)
for r in res: by[tuple(r["cfg"])].add(r["planted_digest"])
summ = dict(runs=len(res), targets=sum(r["targets"] for r in res), failures=sum(r["bad"] for r in res),
            configs=len(by), planted_lists_identical_within_config=all(len(v) == 1 for v in by.values()),
            per_arm_runs=dict(collections.Counter(r["arm"] for r in res)),
            per_bits_targets={b: sum(r["targets"] for r in res if r["cfg"][0] == b) for b in (24, 28, 32)})
json.dump(dict(summary=summ, runs=res), open(out, "w"), indent=1)
print(json.dumps(summ, indent=1))
