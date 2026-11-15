#!/usr/bin/env python3
"""Target 4: recompute the quantitative statements of RESULTS.md 'Answer' (relation-phase
ratios A2/A1 and A3/A0 at w = 0.1, the layered-vs-control difference, the m = 2 amortised
growth 16 -> 28 bits, the 28-bit amortised range) from replayed raw data (validator code).
Usage: check_small_de_prose.py small_de.json"""
import json, sys
raw = json.load(open(sys.argv[1]))
def nat(n): return n["muls"] + n["invs"] + n["tests"] + n["la_ops"]
rel = {}; amort = {}
for c in raw["cells"]:
    key = (c["bits"], c["m"], c["d"])
    rb = min(sum(x["gae"] + 0.1 * x["native"] for x in r["runs"]) / len(r["runs"]) for r in c["rho"])
    for a in c["arms"]:
        rl = sum(r["gae"] + a["build_gae"] + 0.1 * (nat(r["native"]) + nat(a["build_native"])) for r in a["reps"]) / len(a["reps"])
        tg = sum(t["gae"] + 0.1 * nat(t["native"]) for t in a["targets"]) / len(a["targets"])
        rel[key + (a["arm"],)] = rl; amort[key + (a["arm"],)] = (rl / len(a["targets"]) + tg) / rb
def rr(d):
    v = [rel[(b, m, d, "A2")] / rel[(b, m, d, "A1")] for b in (20, 24, 28) for m in (2, 3)]
    return round(min(v), 3), round(max(v), 3)
for d in (-3, -4, -7, -8, -11): print(f"A2/A1 relation phase w=0.1, 20-28 bits, D={d}: {rr(d)}")
v = [rel[(b, m, 0, "A3")] / rel[(b, m, 0, "A0")] for b in (20, 24, 28) for m in (2, 3)]
print("A3/A0 relation phase w=0.1, 20-28 bits:", round(min(v), 3), round(max(v), 3))
dd = lambda bs: [abs(rel[(b, m, d, "A2")] / rel[(b, m, d, "A1")] - rel[(b, m, 0, "A3")] / rel[(b, m, 0, "A0")]) for b in bs for m in (2, 3) for d in (-7, -8, -11)]
print("max |A2/A1 - A3/A0|, layered D, 24 and 28 bits:", round(max(dd((24, 28))), 4))
print("cells within 0.1 over all sizes:", sum(1 for x in dd((12, 16, 20, 24, 28)) if x <= 0.1), "of 30")
for arm, d in (("A0", 0), ("A2", -3), ("A1", -3)):
    print(f"m=2 amortised ratio growth 16->28 bits {arm} D={d}:", round(amort[(28, 2, d, arm)] / amort[(16, 2, d, arm)], 1))
am28 = [v for k, v in amort.items() if k[0] == 28]
print("28-bit amortised ratio range (w=0.1):", round(min(am28), 1), round(max(am28), 1))
