#!/usr/bin/env python3
"""VALIDATOR certificate checks with the validator's own EC code (ecv.py):
 1. the 12 instances W{24,28,32}-{0..3}: p prime, n prime, G on the curve, [n]G = O, cofactor 1 by Hasse
    (n > (p + 1 + 2 sqrt p) / 2), bit lengths, sha256;
 2. every dumped run (dump build, nonce sample): planted list identical across runs of one configuration, every Q_t on
    the curve, 0 <= log < n, recovered log == planted log, and [log]G == Q_t ([log]G computed once per distinct
    (configuration, log) and compared for every run that claims it).
usage: ec_check.py <frozen root> <dump replay json> <out.json>"""
import json, sys, os, math, hashlib, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ecv import load, is_prime

ROOT, RD, OUT = sys.argv[1:4]
INST = os.path.join(ROOT, "research/mt_fieldops_v5_20261112/instances")
inst = {}
for pb in (24, 28, 32):
    for idx in range(4):
        path = os.path.join(INST, f"W{pb}-{idx}.json")
        d, C, G, n = load(path)
        p = C.p
        hasse_hi = p + 1 + 2 * math.isqrt(p) + 2
        inst[f"W{pb}-{idx}"] = dict(sha256=hashlib.sha256(open(path, "rb").read()).hexdigest(), p=p, n=n, p_bits=p.bit_length(),
                                     n_bits=n.bit_length(), p_prime=is_prime(p), n_prime=is_prime(n), G_on=C.on(G),
                                     nG_is_O=C.mul(n, G) is None, cofactor_1=2 * n > hasse_hi,
                                     disc_nonzero=(4 * C.a ** 3 + 27 * C.b ** 2) % p != 0)
inst_ok = all(v["p_prime"] and v["n_prime"] and v["G_on"] and v["nG_is_O"] and v["cofactor_1"] and v["disc_nonzero"] for v in inst.values())

rd = json.load(open(RD))["results"]
cache = {}; planted = {}; fails = collections.Counter(); per = []; nt = 0; mults = 0
curves = {}
for x in rd:
    if x.get("error") or not x.get("dump"):
        fails["no_dump"] += 1; continue
    name = f"W{x['pb']}-{x['idx']}"
    if name not in curves:
        curves[name] = load(os.path.join(INST, name + ".json"))
    _, C, G, n = curves[name]
    dd = json.load(open(x["dump"]))
    xs = [int(v) for v in dd["xs"]]; qx = [int(v) for v in dd["qx"]]; qy = [int(v) for v in dd["qy"]]; fd = [int(v) for v in dd["found"]]
    ck = (x["pb"], x["T"], x["idx"], x["seed"])
    if ck in planted:
        if planted[ck] != (xs, qx, qy): fails["planted_differs_across_runs"] += 1
    else:
        planted[ck] = (xs, qx, qy)
    f = collections.Counter()
    for t in range(len(fd)):
        nt += 1; Q = (qx[t], qy[t])
        if not C.on(Q): f["Q_off_curve"] += 1
        if not (0 <= fd[t] < n): f["log_out_of_range"] += 1
        if fd[t] != xs[t] % n: f["log_ne_planted"] += 1
        key = (name, fd[t], Q)
        if key not in cache:
            cache[key] = C.mul(fd[t], G) == Q; mults += 1
        if not cache[key]: f["logG_ne_Q"] += 1
    fails.update(f)
    per.append(dict(key=x["key"], arm=x["arm"], param=x["param"], doff=x.get("doff"), pb=x["pb"], T=x["T"], idx=x["idx"], seed=x["seed"],
                    targets=len(fd), targets_eq_row=len(fd) == x["targets"], failures=dict(f)))
summ = dict(instances_ok=inst_ok, runs=len(per), targets_checked=nt, distinct_ec_scalar_mults=mults, configurations=len(planted),
            arms=dict(collections.Counter(p["arm"] for p in per)), bits=dict(collections.Counter(p["pb"] for p in per)),
            failures=dict(fails), all_pass=inst_ok and not fails and all(p["targets_eq_row"] for p in per) and len(per) > 0)
json.dump(dict(summary=summ, instances=inst, runs=per), open(OUT, "w"), indent=1)
print(json.dumps(summ, indent=1))
print(json.dumps({k: [v["p_bits"], v["n_bits"], v["cofactor_1"]] for k, v in inst.items()}))
