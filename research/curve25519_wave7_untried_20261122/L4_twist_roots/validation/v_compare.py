#!/usr/bin/env python3
"""Compare the validator's tables with the producer's run JSONs (hist_rows), cell by cell.
Usage: v_compare.py VALIDATOR.json PRODUCER.json [flagkey]"""
import json, sys

v = json.load(open(sys.argv[1]))
d = json.load(open(sys.argv[2]))
flag = sys.argv[3] if len(sys.argv) > 3 else "x1_eq_x2"
prod = {}
for r in d["hist_rows"]:
    key = (r["pair_class"], bool(r["degenerate_a0"]))
    t = prod.setdefault(key, dict(pairs=0, roots0=0, roots1=0, roots2=0, roots_on_E=0, roots_on_twist=0,
                                  roots_on_Z=0, pairs_with_root_on_Z=0))
    c = r["count"]
    t["pairs"] += c
    t[f"roots{r['roots']}"] += c
    t["roots_on_E"] += c * r["roots_on_E"]
    t["roots_on_twist"] += c * r["roots_on_twist"]
    t["roots_on_Z"] += c * r["roots_on_Z"]
    t["pairs_with_root_on_Z"] += c * (r["roots_on_Z"] > 0)
mine = {(t["pair_class"], bool(t[flag])): t for t in v["table"]}
allok = set(prod) == set(mine)
for key in sorted(set(prod) | set(mine)):
    a, b = mine.get(key), prod.get(key)
    if a is None or b is None:
        print(key, "MISSING", a is None, b is None)
        allok = False
        continue
    diffs = {f: (a[f], b[f]) for f in b if a[f] != b[f]}
    allok &= not diffs
    print(key, "pairs", a["pairs"], "OK" if not diffs else f"DIFF {diffs}")
print("npairs validator", v["npairs"], "producer", d["npairs"], "equal", v["npairs"] == d["npairs"])
print("ALL CELLS EQUAL" if allok and v["npairs"] == d["npairs"] else "MISMATCH")
