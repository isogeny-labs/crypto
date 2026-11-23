#!/usr/bin/env python3
"""Part C table (PROTOCOL.md A3). Run: python3 -I summarise_partc2.py > runs_c2_summary.txt"""
import json, math, os
here = os.path.join(os.path.dirname(os.path.abspath(__file__)), "runs_c2")
def pois_cdf(k, mu): return sum(math.exp(-mu + i * math.log(mu) - math.lgamma(i + 1)) for i in range(k + 1)) if mu > 0 else 1.0
def garwood(k, a=0.05):
    lo = 0.0
    if k > 0:
        a_, b_ = 0.0, 10.0 * k + 10
        for _ in range(100):
            m = (a_ + b_) / 2
            if 1 - pois_cdf(k - 1, m) < a / 2: a_ = m
            else: b_ = m
        lo = (a_ + b_) / 2
    a_, b_ = 0.0, 10.0 * k + 40
    for _ in range(100):
        m = (a_ + b_) / 2
        if pois_cdf(k, m) > a / 2: a_ = m
        else: b_ = m
    return lo, (a_ + b_) / 2
print("base            lifts  targets  hits  verified  hits95%CI        F4_calls    F4_muladds(1e9)  lifts/target  hits_per_1e9_muladds [95% CI]   setup_ops")
for tag, name in (("bits", "bits@0 t=5"), ("sq0", "square-bits@0 t=5"), ("sq414", "square-bits@414271 t=5")):
    for L in (0, 1):
        d = json.load(open(os.path.join(here, f"{tag}_L{L}.json"))); n = d.get("native", {})
        h = d["hits_verified"]; ma = d["f4_muladds"] / 1e9; lo, hi = garwood(h)
        tried = n.get("pkm_lift_targets_tried")
        print("%-22s %d %8d %5d %8d  [%5.1f,%6.1f] %10d %14.2f %12s %12.3f [%.3f, %.3f] %10s" % (
            name, L, d["targets_done"], d["hits"], h, lo, hi, d["f4_calls"], ma,
            "-" if tried is None else "%.2f" % (tried / d["targets_done"]), h / ma, lo / ma, hi / ma, d.get("setup_group_ops")))
