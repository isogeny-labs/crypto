#!/usr/bin/env python3
"""Generate ecbench specs: pruned k-sum incumbent vs free=1 candidates on three random-binary curves."""
import json, sys, copy
SRC = "../ic_ksum_prune_20261115/specs/B{b}-r1.json"
# incumbent sizes (frozen by research/ic_tableb_binary_ksum_20261115, reused by ic_ksum_prune_20261115)
SIZES = {21: (10, 9), 24: (12, 9), 27: (16, 12)}   # (ksum4 N, ksum5 N)
MAXT = "13528399621"
def arm(name, role, n, m, extra):
    return {"name": name, "role": role, "method": {"id": "ic.pipeline", "params": {
        "factor_base": f"binary-multiplier:size={n},mult=1", "oracle": f"ksum:m={m},prune=1{extra}", "max_trials": MAXT}}}
def make(b, rep, targets=48, rounds=3, warm=1, tag=""):
    d = json.load(open(SRC.format(b=b)))
    n4, n5 = SIZES[b]
    arms = [a for a in d["arms"] if a["name"] in ("rho-neg", "rho-neg-aa")]
    arms += [arm("ksum4-p", "baseline", n4, 4, ""), arm("ksum5-p", "baseline", n5, 5, ""),
             arm("ksum4-pf", "candidate", n4, 4, ",free=1"), arm("ksum5-pf", "candidate", n5, 5, ",free=1"),
             arm("ksum4-pf-N1", "candidate", n4 + 1, 4, ",free=1"), arm("ksum5-pf-N1", "candidate", n5 + 1, 5, ",free=1")]
    d["arms"] = arms
    d["label"] = f"ic_binary_generic_incumbent_20261119 B{b} replicate {rep}{tag}"
    d["description"] = "Generic binary one-target IC; free=1 on pruned k-sum vs pruned k-sum. Public synthetic curve; no security claim."
    d["workloads"]["targets_per_curve"] = targets
    d["workloads"]["target_seed"] = 20261119000 + b * 10 + rep
    d["measurement"].update({"rounds": rounds, "warmup": warm, "seed": 20261119500 + b * 10 + rep})
    return d
if __name__ == "__main__":
    mode = sys.argv[1]
    if mode == "study":
        for b in SIZES:
            for rep in (1, 2, 3):
                json.dump(make(b, rep), open(f"specs/B{b}-r{rep}.json", "w"), indent=1)
    else:
        json.dump(make(27, 0, targets=6, rounds=1, warm=0, tag=" SMOKE"), open("smoke/B27-smoke.json", "w"), indent=1)
