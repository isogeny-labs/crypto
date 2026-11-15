#!/usr/bin/env python3
"""Run-to-target summary of the large-prime sieve probe (not under protocol).

    python3 summarise_target.py target.txt > summary-target.md

Each arm runs from the same D1, D2 until fulls + independent cycles of its
large-prime graph reach #B + 10 (GTTD's 2LP relation count; the linear
algebra is not run).  Per (genus, p, κ): medians over 3 seeds of counted
field products to target, the walk/sieve ratio (ratio of medians, min-max of
the per-seed ratios), partials needed, and wall clock (shared host,
descriptive).  Sieve relations of every class were checked in the Jacobian
(the first 50 per class per run, excluded from the counts).
"""
import json
import statistics as st
import sys
from collections import defaultdict

G = defaultdict(list)
for line in open(sys.argv[1]):
    if line.startswith("JSON "):
        j = json.loads(line[5:])
        G[(j["genus"], j["p"], j["kappa"])].append(j)
med = st.median
print("| g | p | κ | #B | target | walk products | sieve products | **ratio** (min–max) | partials walk / sieve "
      "| walk trials | sieve lines | ms walk / sieve (ratio) | all reached | sieve verified |")
print("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|--:|")
for (g, p, k), v in sorted(G.items()):
    f = lambda key: med([x[key] for x in v])
    rs = [x["walk_products"] / x["sieve_products"] for x in v]
    print(f"| {g} | {p} | {k} | {v[0]['nb']} | {v[0]['target']} | {f('walk_products'):.3g} | {f('sieve_products'):.3g} "
          f"| **{f('walk_products') / f('sieve_products'):.1f}×** ({min(rs):.1f}–{max(rs):.1f}) "
          f"| {f('walk_partials'):.0f} / {f('sieve_partials'):.0f} | {f('walk_trials'):.3g} | {f('sieve_lines'):.0f} "
          f"| {f('walk_ms'):.0f} / {f('sieve_ms'):.1f} ({f('walk_ms') / f('sieve_ms'):.0f}×) "
          f"| {'yes' if all(x['walk_reached'] and x['sieve_reached'] for x in v) else 'NO'} | {sum(x['sieve_verified'] for x in v)} |")
