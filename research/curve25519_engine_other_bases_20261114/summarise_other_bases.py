#!/usr/bin/env python3
"""Tables and tests for PROTOCOL.md. Run: python3 -I summarise_other_bases.py > summary.txt"""
import json, math, os
here = os.path.dirname(os.path.abspath(__file__))
ref = os.path.join(here, "..", "curve25519_descent_skip_20261114", "runs_c2")
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
def binom_two_sided(k, n, q):
    pm = lambda i: math.exp(math.lgamma(n + 1) - math.lgamma(i + 1) - math.lgamma(n - i + 1) + i * math.log(q) + (n - i) * math.log(1 - q))
    pk = pm(k)
    return sum(pm(i) for i in range(n + 1) if pm(i) <= pk * (1 + 1e-9))
bases = [("isogeny t=5 seed0", here, "iso"), ("edwards-bits t=5", here, "edw"), ("t0-bits t=5", here, "t0"),
         ("digits set 1 t=5", here, "dgA"), ("digits set 2 t=6", here, "dgB"),
         ("REF bits@0 t=5", ref, "bits"), ("REF square-bits@0 t=5", ref, "sq0"), ("REF square-bits@414271 t=5", ref, "sq414")]
def load(d, tag, L):
    p = os.path.join(d, "runs" if d == here else "", f"{tag}_L{L}.json")
    return json.load(open(p))
print("base                         lifts targets  hits   F4 mul-adds(1e9)  mul-adds/target   lifts/target  hits per 1e9 mul-adds [95% Poisson]   setup_ops")
res = {}
for name, d, tag in bases:
    for L in (0, 1):
        x = load(d, tag, L); n = x.get("native", {}); h = x["hits_verified"]; e = x["f4_muladds"] / 1e9; lo, hi = garwood(h)
        tr = n.get("pkm_lift_targets_tried"); res[(tag, L)] = (h, e, x["targets_done"], x["muladds_per_target"])
        print("%-28s %d %8d %5d %14.1f %16.3e %12s %12.3f [%.3f, %.3f] %12s" % (
            name, L, x["targets_done"], h, e, x["muladds_per_target"], "-" if tr is None else "%.3f" % (tr / x["targets_done"]),
            h / e, lo / e, hi / e, x.get("setup_group_ops")))
print("\nH4: cost per target, lifts=1 over blind (bases tried here and references)")
for name, d, tag in bases:
    print("%-28s x%.3f" % (name, res[(tag, 1)][3] / res[(tag, 0)][3]))
print("\nH5: rate ratio lifts=1 / blind in hits per counted mul-add, exact conditional binomial, two-sided")
for name, d, tag in bases:
    h0, e0, _, _ = res[(tag, 0)]; h1, e1, _, _ = res[(tag, 1)]; n = h0 + h1
    if n == 0 or h0 == 0 or h1 == 0:
        print("%-28s L0 %d hits, L1 %d hits: rate ratio undefined (a zero count)" % (name, h0, h1)); continue
    q = e1 / (e0 + e1)
    print("%-28s L0 %4d/%.1f=%.4f  L1 %4d/%.1f=%.4f  ratio %.2f  p = %.4f" % (name, h0, e0, h0 / e0, h1, e1, h1 / e1, (h1 / e1) / (h0 / e0), binom_two_sided(h1, n, q)))
