#!/usr/bin/env python3
"""Rho-reference ecbench specs (matched rho-neg + A/A) on the three random-binary curves, and the example's configs.

  make_specs.py rho      -> specs/B<n>-r<rep>.json   (ecbench: rho-neg, rho-neg-aa; fixes the 48 public targets per replicate)
  make_specs.py configs  -> configs/B<n>-r<rep>.json (example config: targets read from the rho session records)
"""
import json, sys, os, gzip
SRC = "../ic_ksum_prune_20261115/specs/B{b}-r1.json"
SIZES = {21: (10, 9), 24: (12, 9), 27: (16, 12)}   # (m=4 N0, m=5 N0), frozen by research/ic_tableb_binary_ksum_20261115
MAXT = 13528399621
DELTAS = (-1, 0, 1, 2)
def arms(b):
    n4, n5 = SIZES[b]
    out = []
    for m, n0 in ((4, n4), (5, n5)):
        for free in (0, 1):
            for d in DELTAS:
                out.append({"name": f"k{m}-{'pf' if free else 'p'}-N{d:+d}".replace("+", "u").replace("-", "d") if False else f"k{m}-{'pf' if free else 'p'}-{('m' if d<0 else 'p')}{abs(d)}",
                            "m": m, "free": free, "delta": d, "N": n0 + d,
                            "factor_base": f"binary-multiplier:size={n0+d},mult=1",
                            "oracle": f"ksum:m={m},prune=1" + (",free=1" if free else "")})
    out.append({"name": "k5-p-p0-aa", "m": 5, "free": 0, "delta": 0, "N": n5, "factor_base": f"binary-multiplier:size={n5},mult=1", "oracle": "ksum:m=5,prune=1"})
    return out
def rho_spec(b, rep):
    d = json.load(open(SRC.format(b=b)))
    d["arms"] = [a for a in d["arms"] if a["name"] in ("rho-neg", "rho-neg-aa")]
    d["label"] = f"ic_binary_generic_incumbent_20261119 B{b} rho reference replicate {rep}"
    d["description"] = "Matched rho reference and target source for lane B. Public synthetic curve; no security claim."
    d["workloads"]["targets_per_curve"] = 48
    d["workloads"]["target_seed"] = 20261119000 + b * 10 + rep
    d["measurement"].update({"rounds": 3, "warmup": 0, "order": "alternate", "seed": 20261119500 + b * 10 + rep})
    return d
if __name__ == "__main__":
    mode = sys.argv[1]
    if mode == "rho":
        os.makedirs("specs", exist_ok=True)
        for b in SIZES:
            for rep in (1, 2, 3):
                json.dump(rho_spec(b, rep), open(f"specs/B{b}-r{rep}.json", "w"), indent=1)
    else:
        os.makedirs("configs", exist_ok=True)
        for b in SIZES:
            curve = json.load(open(SRC.format(b=b)))["workloads"]["curves"][0]
            for rep in (1, 2, 3):
                p = f"sessions/B{b}-r{rep}/records.jsonl"
                tg, slug = {}, None
                for l in open(p):
                    r = json.loads(l)
                    if r["arm"] == "rho-neg":
                        tg[r["workload"]["target_index"]] = r["workload"]["target"]; slug = r["workload"]["curve"]["slug"]
                cfg = {"curve": {k: curve[k] for k in ("n", "modulus", "a", "b", "group_order", "r", "gx", "gy")}, "slug": slug,
                       "targets": [tg[i] for i in sorted(tg)], "rounds": 3, "seed_base": 20261119700 + b * 10 + rep,
                       "max_trials": MAXT, "arms": arms(b)}
                json.dump(cfg, open(f"configs/B{b}-r{rep}.json", "w"))
