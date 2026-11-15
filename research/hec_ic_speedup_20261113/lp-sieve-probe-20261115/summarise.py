#!/usr/bin/env python3
"""Summary of the reduced-base large-prime sieve probe (not under protocol).

    python3 summarise.py probe.txt > summary.md

Per (genus, p, κ): medians over seeds of counted field products per full,
per one-partial (1P) and per two-partial (2P) relation, walk and sieve, and
the walk/sieve ratios.  Each arm's products are all of its relation-phase
work (one run yields all three classes), so "products per 2P" charges the
whole run to the 2Ps; the ratio is the same whichever class is charged only
if both arms produce the classes in the same proportions — the class mix is
reported.  Wall clock (ms) is from a shared host: descriptive only.
"""
import json
import statistics as st
import sys
from collections import defaultdict

G = defaultdict(lambda: defaultdict(list))
for line in open(sys.argv[1]):
    if line.startswith("JSON "):
        j = json.loads(line[5:])
        G[(j["genus"], j["p"], j["kappa"])][j["method"]].append(j)
# The "graph" lines: large-prime graph cycles (GTTD 2LP: each independent
# cycle gives one full relation) at an equal partial count and at equal
# counted products (the arm that spent more is truncated pro rata).
med = st.median
print("| g | p | κ | #B | walk prod/trial | sieve prod/line | class mix F:1P:2P per 1k, walk / sieve "
      "| prod/2P walk / sieve (ratio) | prod/1P walk / sieve (ratio) | prod/F walk / sieve (ratio) "
      "| µs/2P walk / sieve | sieve verified |")
print("|--:|--:|--:|--:|--:|--:|--|--:|--:|--:|--:|--:|")
for (g, p, k), d in sorted(G.items()):
    w, s = d["walk"], d["sieve"]
    per = lambda xs, f: med([f(x) for x in xs])
    def cls(xs, c):
        return per(xs, lambda x: x["ops_products"] / max(x[c], 1))
    def mix(xs):
        tot = sum(x["full"] + x["p1"] + x["p2"] for x in xs)
        return ":".join(f"{1000 * sum(x[c] for x in xs) / tot:.0f}" for c in ("full", "p1", "p2"))
    r2, r1, rf = cls(w, "p2") / cls(s, "p2"), cls(w, "p1") / cls(s, "p1"), cls(w, "full") / cls(s, "full")
    print(f"| {g} | {p} | {k} | {s[0]['nb']} | {per(w, lambda x: x['ops_products'] / x['trials']):.0f} "
          f"| {per(s, lambda x: x['ops_products'] / x['lines']):.0f} | {mix(w)} / {mix(s)} "
          f"| {cls(w, 'p2'):.0f} / {cls(s, 'p2'):.0f} (**{r2:.1f}×**) "
          f"| {cls(w, 'p1'):.0f} / {cls(s, 'p1'):.0f} ({r1:.1f}×) "
          f"| {cls(w, 'full'):.3g} / {cls(s, 'full'):.3g} ({rf:.1f}×) "
          f"| {per(w, lambda x: 1e3 * x['ms'] / max(x['p2'], 1)):.1f} / {per(s, lambda x: 1e3 * x['ms'] / max(x['p2'], 1)):.2f} "
          f"| {sum(x['verified'] for x in s)} |")
print()
print("Totals per arm (all seeds): " + "; ".join(
    f"g{g} p{p} κ{k}: walk F/1P/2P {sum(x['full'] for x in d['walk'])}/{sum(x['p1'] for x in d['walk'])}/{sum(x['p2'] for x in d['walk'])}, "
    f"sieve {sum(x['full'] for x in d['sieve'])}/{sum(x['p1'] for x in d['sieve'])}/{sum(x['p2'] for x in d['sieve'])}"
    for (g, p, k), d in sorted(G.items())))

print()
print("Large-prime graph at an equal partial count (the first n partials of each arm; cycles are GTTD's")
print("2LP full-relation count). The time-to-target comparison is `summarise_target.py`; an")
print("equal-products cycle count is not reported, because cycles grow faster than linearly in partials.")
print()
print("| g | p | κ | partials n | cycles walk / sieve |")
print("|--:|--:|--:|--:|--:|")
for (g, p, k), d in sorted(G.items()):
    gr = d["graph"]
    per = lambda xs, f: med([f(x) for x in xs])
    print(f"| {g} | {p} | {k} | {per(gr, lambda x: x['partials']):.0f} "
          f"| {per(gr, lambda x: x['walk_cycles']):.0f} / {per(gr, lambda x: x['sieve_cycles']):.0f} |")
