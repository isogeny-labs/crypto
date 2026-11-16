#!/usr/bin/env python3
"""Replay certificate and independent log check.

Usage: verify_certs.py DUMP_JSON FROZEN_REPLAY_JSON COMMITTED_JSON [COMMITTED_JSON ...] --out OUT

DUMP_JSON: rows from val_dump (the frozen bin source plus val_* fields).
FROZEN_REPLAY_JSON: rows from the rebuilt frozen ic_positive_control binary.
COMMITTED_JSON: the study's committed raw run file(s).

For every dump row:
  * replay certificate: the frozen binary's replay row equals the committed row
    in every deterministic field, and the dump row equals both (val_* aside);
  * independent check (jac3.py, validator-written Cantor arithmetic): f is the
    documented generator's output for some curve seed, squarefree; G and T are
    reduced divisors on y^2 = f; G != 0, [r]G = 0, r prime, r > h, order = h r,
    order in the Hasse-Weil interval, [order]D = 0 for 4 random divisors;
    [t]G = T for the recovered t; t = t_secret mod r.
"""
import json
import math
import os
import random
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import jac3 as J
import vlib as V

args = sys.argv[1:]
out_path = args[args.index("--out") + 1]
args = args[:args.index("--out")]
dump_path, replay_path, committed_paths = args[0], args[1], args[2:]
dump = json.load(open(dump_path))["rows"]
replay = json.load(open(replay_path))["rows"]
committed = []
for cp in committed_paths:
    committed += json.load(open(cp))["rows"]
key = lambda r: (r["arm"], r["p"], r["curve_idx"], r["seed"])
rep = {key(r): r for r in replay}
com = {}
for r in committed:
    com.setdefault(key(r), r)
assert len(rep) == len(replay), "duplicate replay rows"

rnd = random.Random(20261116)
curve_cache = {}
res = {"rows": [], "n": 0, "replay_identical": 0, "dump_identical": 0, "independent_ok": 0, "failures": []}
for d in dump:
    k = key(d)
    p, r, h, order = d["p"], d["r"], d["h"], d["order"]
    rec = {"key": k}
    fails = []
    # replay certificate
    c = com.get(k)
    rp = rep.get(k)
    if c is None:
        fails.append("no committed row")
    if rp is None:
        fails.append("no frozen-binary replay row")
    if c is not None and rp is not None:
        diff = V.compare_rows(rp, c)
        rec["replay_vs_committed_diffs"] = diff
        if diff:
            fails.append(f"replay != committed: {diff[:4]}")
        else:
            res["replay_identical"] += 1
    if rp is not None:
        dd = V.compare_rows({kk: vv for kk, vv in d.items() if not kk.startswith("val_")}, rp)
        if dd:
            fails.append(f"dump != replay: {dd[:4]}")
        else:
            res["dump_identical"] += 1
    # independent instance and log checks
    f = d["val_f"]
    ck = (p, tuple(f))
    if ck not in curve_cache:
        seed = next((s for s in range(1, 4001) if J.random_curve_f(p, s) == f), None)
        jac = J.Jac(p, f)
        info = {"seed": seed, "squarefree": J.squarefree(f, p)}
        curve_cache[ck] = (jac, info, {})
    jac, info, ocache = curve_cache[ck]
    rec["curve_seed"] = info["seed"]
    if info["seed"] is None:
        fails.append("f is not the generator's output for any seed <= 4000")
    if not info["squarefree"]:
        fails.append("f not squarefree")
    G, T = J.div_from_packed(d["val_G"]), J.div_from_packed(d["val_T"])
    divisors_ok = jac.valid(G) and jac.valid(T)
    if not divisors_ok:
        fails.append("G or T not a reduced divisor on the curve")
    if G == jac.ZERO:
        fails.append("G is the identity")
    if not J.is_prime(r):
        fails.append("r not prime")
    if not (r > h and h * r == order):
        fails.append("order != h r or r <= h")
    lo, hi = (math.sqrt(p) - 1) ** 6, (math.sqrt(p) + 1) ** 6
    if not (lo <= order <= hi):
        fails.append("order outside Hasse-Weil")
    t = d["val_recovered"]
    rec["recovered"] = t
    try:
        if order not in ocache:
            Ds = [jac.random_divisor(rnd) for _ in range(4)]
            ocache[order] = all(jac.mul(D, order) == jac.ZERO for D in Ds)
        if not ocache[order]:
            fails.append("[order]D != 0 for a random D")
        if divisors_ok:
            gk = ("G", repr(d["val_G"]))
            if gk not in ocache:
                ocache[gk] = jac.mul(G, r) == jac.ZERO
            if not ocache[gk]:
                fails.append("[r]G != 0")
            if t is None:
                fails.append("no recovered log")
            else:
                if jac.mul(G, t) != T:
                    fails.append("[t]G != T")
                if t % r != d["val_t_secret"] % r:
                    fails.append("t != t_secret mod r")
    except AssertionError:
        fails.append("Cantor arithmetic failed (inputs not on the curve)")
    if d["certified"] is not True:
        fails.append("row not certified")
    rec["fails"] = fails
    res["n"] += 1
    if not [x for x in fails if not x.startswith("replay") and not x.startswith("dump") and not x.startswith("no committed") and not x.startswith("no frozen")]:
        res["independent_ok"] += 1
    if fails:
        res["failures"].append(rec)
    res["rows"].append(rec)
# rows of the replay not in the dump, and replay rows compared on their own
res["replay_rows"] = len(replay)
res["replay_only_identical"] = sum(1 for kk, rr in rep.items() if kk in com and not V.compare_rows(rr, com[kk]))
res["replay_only_missing_committed"] = [kk for kk in rep if kk not in com]
res["curve_seeds"] = {f"{k[0]}:{k[1][:2]}...": v[1]["seed"] for k, v in curve_cache.items()}
json.dump(res, open(out_path, "w"), indent=1, default=str)
print(f"dump rows {res['n']}: replay==committed {res['replay_identical']}, dump==replay {res['dump_identical']}, independent checks ok {res['independent_ok']}")
print(f"frozen replay rows {res['replay_rows']}: identical to committed {res['replay_only_identical']}, missing in committed {len(res['replay_only_missing_committed'])}")
for f in res["failures"][:10]:
    print("FAIL", f["key"], f["fails"][:3])
