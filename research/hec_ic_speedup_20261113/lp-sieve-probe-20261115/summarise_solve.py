#!/usr/bin/env python3
"""End-to-end check of the large-prime probe (not under protocol).

    python3 summarise_solve.py solve.txt > summary-solve.md

Per (genus, p), κ = 1, 3 seeds, prime-order instance: each arm ran to
⌈μ(#B + 10)⌉ fulls + cycles for μ = 1, 1.1, ... until its captured rows, after
singleton large-prime pruning, solved (Solver::Fast) to a verified k.  Reports
the μ that worked, relation-phase products, the pruned matrix size, the
solve's mulmods (mod-ℓ multiplications, one field product each under
`count-ops`), and relation products + solve mulmods as an end-to-end count.
"""
import json
import statistics as st
import sys
from collections import defaultdict

G = defaultdict(lambda: defaultdict(list))
for line in open(sys.argv[1]):
    if line.startswith("JSON "):
        j = json.loads(line[5:])
        G[(j["genus"], j["p"])][j["arm"]].append(j)
med = st.median
print("| g | p | ℓ | #B | k recovered (seeds) | μ needed walk / sieve | relation products walk / sieve (ratio) "
      "| matrix rows × cols walk / sieve | solve mulmods walk / sieve | relation + solve walk / sieve (ratio) |")
print("|--:|--:|--:|--:|:-:|--:|--:|--:|--:|--:|")
for (g, p), d in sorted(G.items()):
    w, s = d["walk"], d["sieve"]
    ok = sum(x["k_ok"] for x in w + s)
    f = lambda xs, k: med([x[k] for x in xs])
    tot = lambda xs: med([x["products"] + x["solve_mulmods"] for x in xs])
    print(f"| {g} | {p} | {w[0]['ell']} | {w[0]['nb']} | {ok}/{len(w) + len(s)} "
          f"| {max(x['mu'] for x in w)} / {max(x['mu'] for x in s)} "
          f"| {f(w, 'products'):.3g} / {f(s, 'products'):.3g} ({f(w, 'products') / f(s, 'products'):.1f}×) "
          f"| {f(w, 'rows_kept'):.0f} × {w[0]['nb'] + f(w, 'lp_cols'):.0f} / {f(s, 'rows_kept'):.0f} × {s[0]['nb'] + f(s, 'lp_cols'):.0f} "
          f"| {f(w, 'solve_mulmods'):.3g} / {f(s, 'solve_mulmods'):.3g} "
          f"| {tot(w):.3g} / {tot(s):.3g} ({tot(w) / tot(s):.1f}×) |")
