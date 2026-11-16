#!/usr/bin/env python3
"""Validator sensitivity analyses (not the pre-registered rule): phases, verification constant,
rho walk start/restart overhead.  usage: sens.py <rows.jsonl> <replay_dbg.jsonl> <out json>"""
import json, sys, collections
import numpy as np

rows = [json.loads(l) for l in open(sys.argv[1])]
dbg = {r["i"]: r for r in (json.loads(l) for l in open(sys.argv[2]))}
CIS = [3, 10, 30]; HS = [0, 0.1, 1]; RHO = ["rho", "rho2", "rho2x", "rhoseq"]

def vec(r): return np.array([r["I"], r["M"], r["S"], r["H"], r["L"]], dtype=float)
def phase_incr(r):
    ph = list(r["phases"].items()); out = {}; prev = np.zeros(5)
    for k, v in ph:
        v = np.array(v, dtype=float); out[k] = v - prev; prev = v
    return out
def gae(v, ci, h, T):
    I, M, S, Hh, L = v
    return ((ci * I + M + S) / (ci + 3) + h * (Hh + L)) / T

for i, r in enumerate(rows):
    r["_i"] = i; r["_v"] = vec(r); r["_ph"] = phase_incr(r)
    assert np.allclose(sum(r["_ph"].values()), r["_v"])
    d = dbg[i]
    r["_rs2"] = np.array(d["restart_p2"], dtype=float); r["_nrs2"] = d["n_restart_p2"]
    r["_rs1"] = np.array(d["restart_p1"], dtype=float); r["_nrs1"] = d["n_restart_p1"]

conf = collections.defaultdict(list)
for r in rows: conf[(r["pb"], r["T"], r["idx"], r["seed"])].append(r)
cells = sorted({k[:2] for k in conf})

# verification constant identical across the 16 runs of a configuration?
ver_same = sum(len({tuple(r["_ph"]["verify"]) for r in rs}) == 1 for rs in conf.values())

def boot(x, B=20000, seed=0):
    g = np.random.default_rng(seed); x = np.asarray(x)
    med = np.median(x[g.integers(0, len(x), size=(B, len(x)))], axis=1)
    return float(np.quantile(med, .025)), float(np.quantile(med, .975))

def scenario(costfn):
    """costfn(r) -> 5-vector to price; returns per (cI, cell, h, ref) median/lo/hi and verdicts"""
    out = {}; verd = {}
    for ci in CIS:
        for c in cells:
            ks = sorted(k for k in conf if k[:2] == c)
            per_h = {}
            for h in HS:
                ic = []; rb = []; bs = []
                for k in ks:
                    rs = conf[k]
                    ic.append(gae(costfn(next(r for r in rs if r["arm"] == "ic4")), ci, h, c[1]))
                    rb.append(min(gae(costfn(r), ci, h, c[1]) for r in rs if r["arm"] in RHO))
                    bs.append(min(gae(costfn(r), ci, h, c[1]) for r in rs if r["arm"] == "bsgs2"))
                for ref, den in (("rhobest", rb), ("bsgs2", bs)):
                    rat = [a / b for a, b in zip(ic, den)]
                    lo, hi = boot(rat)
                    out[f"{ref}|cI{ci}|{c[0]}b|T{c[1]}|h{h}"] = [float(np.median(rat)), lo, hi]
                    per_h.setdefault(ref, []).append((lo, hi))
            for ref, lh in per_h.items():
                verd[f"{ref}|cI{ci}|{c[0]}b|T{c[1]}"] = ("cheaper" if all(hi < 1 for lo, hi in lh) else
                                                        "dearer" if all(lo > 1 for lo, hi in lh) else "unranked")
    return out, verd

def as_measured(r): return r["_v"]
def no_verify(r): return r["_v"] - r["_ph"]["verify"]
def rho_no_start(r): return r["_v"] - (r["_rs2"] if r["arm"] in RHO else 0)
def rho_no_start_no_verify(r): return no_verify(r) - (r["_rs2"] if r["arm"] in RHO else 0)

res = {"verify_identical_in_configs": [ver_same, len(conf)]}
for name, fn in (("as_measured", as_measured), ("no_verify", no_verify), ("rho_no_start", rho_no_start),
                 ("rho_no_start_no_verify", rho_no_start_no_verify)):
    o, v = scenario(fn)
    res[name] = {"ratios": o, "verdicts": v,
                 "verdict_counts": {ref: dict(collections.Counter(x for k, x in v.items() if k.startswith(ref + "|"))) for ref in ("rhobest", "bsgs2")},
                 "range_rhobest_T16384": [min(x[0] for k, x in o.items() if k.startswith("rhobest") and "T16384" in k), max(x[0] for k, x in o.items() if k.startswith("rhobest") and "T16384" in k)],
                 "range_rhobest_T4096": [min(x[0] for k, x in o.items() if k.startswith("rhobest") and "T4096" in k), max(x[0] for k, x in o.items() if k.startswith("rhobest") and "T4096" in k)],
                 "range_rhobest_T1024": [min(x[0] for k, x in o.items() if k.startswith("rhobest") and "T1024" in k), max(x[0] for k, x in o.items() if k.startswith("rhobest") and "T1024" in k)],
                 "range_bsgs2": [min(x[0] for k, x in o.items() if k.startswith("bsgs2")), max(x[0] for k, x in o.items() if k.startswith("bsgs2"))],
                 "max_hi_rhobest_T16384": max(x[2] for k, x in o.items() if k.startswith("rhobest") and "T16384" in k)}

# phase table: per cell, medians of phase GAE/target at cI = 10, h = 0 (and h = 1 for H+L share)
ptab = {}
for c in cells:
    ks = sorted(k for k in conf if k[:2] == c)
    def med(arm, param, ph, ci=10, h=0):
        return float(np.median([gae(next(r for r in conf[k] if r["arm"] == arm and (param is None or r["param"] == param))["_ph"][ph], ci, h, c[1]) for k in ks]))
    def medtot(arm, param, ci=10, h=0):
        return float(np.median([gae(next(r for r in conf[k] if r["arm"] == arm and (param is None or r["param"] == param))["_v"], ci, h, c[1]) for k in ks]))
    ptab[f"{c[0]}b_T{c[1]}"] = {
        "ic4": {ph: med("ic4", None, ph) for ph in ("fb", "table", "relations", "solve", "targets", "verify")} | {"total": medtot("ic4", None)},
        "rho2x_k0.7": {ph: med("rho2x", 0.7, ph) for ph in ("db", "targets", "verify")} | {"total": medtot("rho2x", 0.7)},
        "rho2x_k0.7_start_restart_p2": float(np.median([gae(next(r for r in conf[k] if r["arm"] == "rho2x" and r["param"] == 0.7)["_rs2"], 10, 0, c[1]) for k in ks])),
        "rho2x_k0.7_restart_p1": float(np.median([gae(next(r for r in conf[k] if r["arm"] == "rho2x" and r["param"] == 0.7)["_rs1"], 10, 0, c[1]) for k in ks])),
        "rho2x_k0.7_starts_per_target": float(np.median([next(r for r in conf[k] if r["arm"] == "rho2x" and r["param"] == 0.7)["_nrs2"] / c[1] for k in ks])),
        "rhoseq_k0.7": {ph: med("rhoseq", 0.7, ph) for ph in ("db", "targets", "verify")} | {"total": medtot("rhoseq", 0.7)},
        "bsgs2_mf1.0": {ph: med("bsgs2", 1.0, ph) for ph in ("baby", "targets", "verify")} | {"total": medtot("bsgs2", 1.0)},
    }
res["phase_table_cI10_h0"] = ptab
# restart-overhead share of each rho arm's cost (cI = 10, h = 0), median over runs
share = {}
for arm in RHO:
    for c in cells:
        v = [gae(r["_rs2"], 10, 0, c[1]) / gae(r["_v"], 10, 0, c[1]) for r in rows if r["arm"] == arm and (r["pb"], r["T"]) == c]
        v1 = [gae(r["_rs1"], 10, 0, c[1]) / gae(r["_v"], 10, 0, c[1]) for r in rows if r["arm"] == arm and (r["pb"], r["T"]) == c]
        share[f"{arm}|{c[0]}b|T{c[1]}"] = {"p2_start_restart_share": float(np.median(v)), "p1_restart_share": float(np.median(v1))}
res["rho_overhead_share_cI10_h0"] = share
json.dump(res, open(sys.argv[3], "w"), indent=1)
for name in ("as_measured", "no_verify", "rho_no_start", "rho_no_start_no_verify"):
    s = res[name]
    print(name, s["verdict_counts"], "T16384", [round(x, 4) for x in s["range_rhobest_T16384"]], "max hi", round(s["max_hi_rhobest_T16384"], 4),
          "T4096", [round(x, 4) for x in s["range_rhobest_T4096"]], "T1024", [round(x, 4) for x in s["range_rhobest_T1024"]], "bsgs2", [round(x, 3) for x in s["range_bsgs2"]])
print("verify identical in configs", res["verify_identical_in_configs"])
