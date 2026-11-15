#!/usr/bin/env python3
"""Summary of the line-sieve feasibility probe (not under protocol).

    python3 summarise.py round8-timing-v2.txt larger-timing-v2.txt > summary.md

Per (genus, p): medians over seeds.  Relation phase = total − solve − factor
base, both arms.  Counts: the sieve's field multiplications per relation
(Horner, batched inversion, c values) and line additions per relation
(one Cantor addition per line, not in field_mults); solve multiplications
mod ℓ for both arms (walk: Dense; sieve: Fast).  Wall clock comes from a
shared host and is descriptive only.
"""
import json
import statistics as st
import sys
from collections import defaultdict


def main():
    G = defaultdict(lambda: defaultdict(list))
    for path in sys.argv[1:]:
        for line in open(path):
            if line.startswith("JSON "):
                j = json.loads(line[5:])
                G[(j["genus"], j["p"])][j["method"]].append(j)
    med = lambda xs, f: st.median(f(x) for x in xs)
    print("| g | p | m | walk trials/rel | sieve rel/line (exp) | sieve mults/rel | sieve line adds/rel "
          "| relation phase ms walk / sieve (ratio) | solve mulmods walk (Dense) / sieve (Fast) "
          "| solve ms walk / sieve | total ms walk / sieve (ratio) | k ok |")
    print("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|")
    for (g, p), d in sorted(G.items()):
        w, s = d["walk"], d["sieve"]
        rel = lambda x: x["total_ms"] - x["solve_ms"] - x["fb_ms"]
        wr, sr = med(w, rel), med(s, rel)
        wt, stt = med(w, lambda x: x["total_ms"]), med(s, lambda x: x["total_ms"])
        ok = all(x["correct"] for x in w + s)
        print(
            f"| {g} | {p} | {s[0]['m']} | {med(w, lambda x: x['trials_per_rel']):.2f} "
            f"| {med(s, lambda x: x['rel_per_line']):.2f} ({s[0]['expected_rel_per_line']:.2f}) "
            f"| {med(s, lambda x: x['mults_per_rel']):.1f} | {med(s, lambda x: x['line_adds_per_rel']):.3f} "
            f"| {wr:.3f} / {sr:.3f} ({wr / sr:.1f}×) "
            f"| {med(w, lambda x: x['solve_mulmods']):.0f} / {med(s, lambda x: x['solve_mulmods']):.0f} "
            f"| {med(w, lambda x: x['solve_ms']):.3f} / {med(s, lambda x: x['solve_ms']):.3f} "
            f"| {wt:.3f} / {stt:.3f} ({wt / stt:.2f}×) | {'yes' if ok else 'NO'} |"
        )


main()
