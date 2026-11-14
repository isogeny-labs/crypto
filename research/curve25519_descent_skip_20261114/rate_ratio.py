#!/usr/bin/env python3
"""Post hoc (not registered): hits per counted mul-add, lifts=1 vs lifts=0, by exact conditional binomial test.
Under 'equal yield per mul-add', hits_L1 ~ Binomial(h0 + h1, e1 / (e0 + e1)). Run: python3 -I rate_ratio.py > rate_ratio.out.txt"""
import json, math, os
here = os.path.join(os.path.dirname(os.path.abspath(__file__)), "runs_c2")
def load(t, L): d = json.load(open(os.path.join(here, f"{t}_L{L}.json"))); return d["hits_verified"], d["f4_muladds"] / 1e9
def binom_two_sided(k, n, q):
    pm = lambda i: math.exp(math.lgamma(n + 1) - math.lgamma(i + 1) - math.lgamma(n - i + 1) + i * math.log(q) + (n - i) * math.log(1 - q))
    pk = pm(k)
    return sum(pm(i) for i in range(n + 1) if pm(i) <= pk * (1 + 1e-9))
def test(name, tags):
    h0 = sum(load(t, 0)[0] for t in tags); e0 = sum(load(t, 0)[1] for t in tags)
    h1 = sum(load(t, 1)[0] for t in tags); e1 = sum(load(t, 1)[1] for t in tags)
    n = h0 + h1; q = e1 / (e0 + e1)
    print("%-34s L0 %4d hits / %8.1f e9 = %.4f   L1 %4d hits / %8.1f e9 = %.4f   rate ratio L1/L0 = %.2f   exact two-sided p = %.4f"
          % (name, h0, e0, h0 / e0, h1, e1, h1 / e1, (h1 / e1) / (h0 / e0), binom_two_sided(h1, n, q)))
for t, n in (("bits", "bits@0 t=5"), ("sq0", "square-bits@0 t=5"), ("sq414", "square-bits@414271 t=5")):
    test(n, [t])
test("square-bits, both cells pooled", ["sq0", "sq414"])
