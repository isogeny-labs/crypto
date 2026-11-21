#!/usr/bin/env python3
"""Independent 95% percentile bootstrap of the pooled and per-prime band cells (validator's own RNG:
Python random.Random(20261111), 5000 resamples). Resampling as PROTOCOL.md: cover seeds within each
prime (one draw shared by the arms), rho runs within each prime's matched set (per-prime reference).
Reports, per cell, the interval and whether hi < 1. Tolerance against fit.json is not expected to be
tight (different RNG and resample count); the check is the verdict.

  python3 bootstrap.py STUDY_DIR > bootstrap.json
"""
import importlib.util, json, math, random, sys
D = sys.argv[1]
spec = importlib.util.spec_from_file_location("rc", f"{D}/review/validator/recompute.py")
sys.argv = [sys.argv[0], D]
import io, contextlib
buf = io.StringIO()
with contextlib.redirect_stdout(buf):
    rc = importlib.util.module_from_spec(spec); spec.loader.exec_module(rc)
rng = random.Random(20261111)
mean = lambda xs: sum(xs) / len(xs)
gm = lambda xs: math.exp(mean([math.log(x) for x in xs]))
primes = (1009, 1511, 1823)
seeds = {p: sorted({i["seed"] for i in rc.cover["P"] if i["p"] == p}) for p in primes}
idx = {(a, i["p"], i["seed"]): i for a in rc.cover for i in rc.cover[a]}
def metrics(arm, I, ref):
    m = {"S/rho U2": [i["S"] / ref[i["p"]]["U2"] for i in I], "S/rho U1": [i["S"] / ref[i["p"]]["U1"] for i in I],
         "S+/rhoS+ U2": [i["Sp"] / ref[i["p"]]["P2"] for i in I]}
    for L in "AB":
        for u in ("U1", "U2"):
            for w in (0, 0.1, 1):
                m[f"band {L} {u} w={w}"] = [(i["S"] + w * i["nat" + L]) / (ref[i["p"]][u] + w * ref[i["p"]]["nat" + L]) for i in I]
    return {k: gm(v) for k, v in m.items()}
B = 5000; draws = {}
for b in range(B):
    ss = {p: [rng.choice(seeds[p]) for _ in seeds[p]] for p in primes}
    ref = {}
    for p in primes:
        rr = [rng.choice(rc.rho[p]) for _ in rc.rho[p]]
        ref[p] = {k: mean([x[k] for x in rr]) for k in ("U1", "U2", "P1", "P2", "natA", "natB")}
    for arm in "PHU":
        for scope in ("pooled", *primes):
            ps = primes if scope == "pooled" else (scope,)
            I = [idx[(arm, p, s)] for p in ps for s in ss[p]]
            for k, v in metrics(arm, I, ref).items():
                draws.setdefault(f"{arm}|{scope}|{k}", []).append(v)
out = {}
for k, v in sorted(draws.items()):
    v.sort(); lo, hi = v[int(0.025 * B)], v[int(0.975 * B) - 1]
    out[k] = {"lo": lo, "hi": hi, "below_rho": hi < 1}
json.dump({"resamples": B, "rng": "random.Random(20261111)", "all_below": all(x["below_rho"] for x in out.values()),
           "max_hi": max(x["hi"] for x in out.values()), "cells": out}, sys.stdout, indent=1)
