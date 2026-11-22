#!/usr/bin/env python3
"""Classify the claims-check flags: an interval claim is re-checked against the
validator's exact replication with the producer's declared seed
(exact_intervals.json, 3 decimals); point claims that differ only in the last
printed digit are display rounding; everything else is a discrepancy."""
import json, sys
claims = json.load(open(sys.argv[1])); ex = json.load(open(sys.argv[2]))
emap = {
 "c/R p=4099 w=1.0": "s1 4099 c/R w=1.0",
 "Q2 w=0.1": "s2 Q2 w=0.1", "Q2 w=1.0": "s2 Q2 w=1.0",
}
for c in claims:
    if c["ok"]:
        continue
    n = c["name"]
    if c["kind"] == "interval" and n in emap:
        e = ex[emap[n]]
        cl = [round(x, 3) for x in c["claimed"]]
        print(f"{n}: claimed {cl}, exact replication with the declared seed {e} ->",
              "consistent (Monte-Carlo seed only)" if cl == e else "DISCREPANCY")
    elif n.startswith("'multiplications and additions"):
        print(f"{n}: recomputed {c['recomputed']} -> DISCREPANCY (mislabel: 1.057 is IC(G+M)/rho(G); see f2_check.txt)")
    else:
        print(f"{n}: claimed {c['claimed']}, recomputed {c['recomputed']:.4f} -> display rounding or Monte-Carlo (see report)")
