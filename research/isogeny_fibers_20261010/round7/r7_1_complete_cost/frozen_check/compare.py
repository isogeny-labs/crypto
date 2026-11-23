#!/usr/bin/env python3
"""Compares rows re-run by the UNMODIFIED round-6 example (rebuilt against the
library with the new addition tally) with the frozen round-6 records, field by
field, ignoring wall-clock keys.  Comparison only; no result is computed."""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
FROZEN = os.path.join(HERE, "..", "..", "..", "round6", "r6_4_vertical_margin_2")


def strip(o):
    if isinstance(o, dict):
        return {k: strip(v) for k, v in o.items() if "wall" not in k}
    if isinstance(o, list):
        return [strip(x) for x in o]
    return o


for new, old in [("r6_4_binary_ladder.json", "ladder_auto.json"),
                 ("r6_4_binary_psi.json", "psi.json"),
                 ("r6_4_binary_modpoly.json", "modpoly.json")]:
    n = json.load(open(os.path.join(HERE, new)))["rows"]
    o = {r["ell"]: r for r in json.load(open(os.path.join(FROZEN, old)))["rows"]}
    same = sum(strip(r) == strip(o[r["ell"]]) for r in n if r["ell"] in o)
    tot = sum(r["ell"] in o for r in n)
    print(f"{new}: {same}/{tot} rows identical to {old} (wall-clock keys excluded)")
