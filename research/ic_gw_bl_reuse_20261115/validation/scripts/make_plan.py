#!/usr/bin/env python3
"""plan.json (from select.py) -> plan.tsv: idx role arm q seed block. usage: make_plan.py plan.json > plan.tsv
role cnt: REF and BLR (counted comparison); role aud: oracle-audit build (identity check, counted fields must
equal BLR); role neg: mutant builds mut_c2 / mut_pm (negative controls) on the first two block units' pairs."""
import json, sys
p = json.load(open(sys.argv[1]))
rows = []
units = [(u["q"], u["seed"], 4) for u in p["sample"]] + [(u["q"], u["seed"], u["block"]) for u in p["block_units"]]
for q, s, b in units:
    rows += [("cnt", "REF", q, s, b), ("cnt", "BLR", q, s, b), ("aud", "AUD", q, s, b)]
neg = [(p["sample"][0]["q"], p["sample"][0]["seed"], 4), (p["block_units"][1]["q"], p["block_units"][1]["seed"], p["block_units"][1]["block"])]
for q, s, b in neg:
    rows += [("neg", "MUTC2", q, s, b), ("neg", "MUTPM", q, s, b)]
for i, r in enumerate(rows, 1):
    print(i, *r, sep="\t")
