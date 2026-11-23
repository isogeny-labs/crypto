#!/usr/bin/env python3
"""Independent recompute of every point estimate in fit.json (jv_sieve_line_vs_rho_20261103).

Native operations enter the band at w GAE each over sqrt(l), with no 1/331 (PROTOCOL.md "Section 5a band").
Written by the validator from PROTOCOL.md's frozen definitions ("Units and per-run metrics",
"Estimand", "Section 5a band") without reading the study's Rust tool (fit/src). Reads only the
raw records in results/cover and results/rho. Primary reference: per-prime (fit.json controls.primary).

  python3 recompute.py STUDY_DIR > recompute.json
"""
import glob, json, math, os, sys

D = sys.argv[1]
C = 331.0
NAT6 = ("canon_cmp", "hash", "dp_check", "scalar_ops", "table_ops", "cycle_cmp")

def rows(path):
    return json.load(open(path))

rho = {}
for f in sorted(glob.glob(f"{D}/results/rho/rhostrong_p*_s*_run*.json")):
    for r in rows(f):
        assert r["correct"] and not r["capped"] and r["negation"] and r["batch"] == 16 and r["r"] == 256 and r["dp_bits"] == 16
        sl = math.sqrt(r["l"])
        o = r["ops"]
        rho.setdefault(r["p"], []).append(dict(
            U1=r["s_harness"], U2=r["s_tuned"], P1=r["s_plus_harness"], P2=r["s_plus_tuned"],
            natB=sum(o[k] for k in NAT6) / sl, natA=(sum(o[k] for k in NAT6) + o["fp_adds"]) / sl,
            l=r["l"], seed=r["seed"]))

def mean(xs): return sum(xs) / len(xs)
ref = {p: {k: mean([x[k] for x in v]) for k in ("U1", "U2", "P1", "P2", "natA", "natB")} for p, v in rho.items()}

cover = {}
for arm in "PHU":
    for p in (1009, 1511, 1823):
        for r in rows(f"{D}/results/cover/{arm}_{p}.json"):
            assert r["solved"] and r["correct"] and r["rels_failed_verify"] == 0 and not r["exhausted"]
            sl = math.sqrt(r["l"])
            fb = r.get("fb_line") or {}
            fa, fl = fb.get("adds", 0), fb.get("lookups", 0)
            natB = r["sieve_lookups"] + fl
            natA = (r["sieve_adds"] + r["sieve_lookups"] + r["enum_adds"] + fa + fl
                    + (r["total_muls"] - r["sieve_muls"] - r["enum_muls"]))
            cover.setdefault(arm, []).append(dict(p=p, seed=r["seed"], S=r["total_muls"] / (C * sl),
                Sp=r["total_plus"] / (C * sl), natB=natB / sl, natA=natA / sl))

def gm(xs): return math.exp(mean([math.log(x) for x in xs]))

out = {}
for arm, inst in cover.items():
    for scope in ("p=1009", "p=1511", "p=1823", "pooled (24 instances)"):
        I = inst if scope.startswith("pooled") else [i for i in inst if f"p={i['p']}" == scope]
        def put(name, f): out[f"per-prime|{arm}|{scope}|{name}"] = gm([f(i) for i in I])
        put("S/rho U1", lambda i: i["S"] / ref[i["p"]]["U1"])
        put("S/rho U2", lambda i: i["S"] / ref[i["p"]]["U2"])
        put("S+/rho U1 (rho unpriced)", lambda i: i["Sp"] / ref[i["p"]]["U1"])
        put("S+/rho U2 (rho unpriced)", lambda i: i["Sp"] / ref[i["p"]]["U2"])
        put("S+/rhoS+ U1", lambda i: i["Sp"] / ref[i["p"]]["P1"])
        put("S+/rhoS+ U2", lambda i: i["Sp"] / ref[i["p"]]["P2"])
        for layer in "AB":
            for u in ("U1", "U2"):
                for w in (0, 0.1, 1):
                    put(f"band {layer} {u} w={w}",
                        lambda i, layer=layer, u=u, w=w: (i["S"] + w * i["nat" + layer]) / (ref[i["p"]][u] + w * ref[i["p"]]["nat" + layer]))

fit = json.load(open(f"{D}/fit.json"))
cmp = []
for k, v in sorted(out.items()):
    claimed = fit[k]["point"]
    cmp.append(dict(name=k, claimed=claimed, recomputed=v, rel_err=abs(v - claimed) / abs(claimed)))
json.dump({"n_rho": {str(p): len(v) for p, v in rho.items()}, "n_cover": {a: len(v) for a, v in cover.items()},
           "reference_means": {str(p): v for p, v in ref.items()},
           "max_rel_err": max(c["rel_err"] for c in cmp), "cells": cmp}, sys.stdout, indent=1)
