#!/usr/bin/env python3
"""T10 seed selection (frozen with PROTOCOL.md; reads existing census data only).

For k = 20 and each t in {5, 6, 7}: the seed maximising J / columns over the
2,000-seed census, ties broken by larger J, then smaller seed. Also records
seed 0 and the J-only selection (for reference; not measured here).

Usage: python3 -I select_seeds.py <census k20.jsonl> <out.json>
"""
import hashlib
import json
import sys


def main(src, out):
    raw = open(src, "rb").read()
    rows = [json.loads(l) for l in raw.decode().splitlines() if l.strip()]
    res = {"source": src, "source_sha256": hashlib.sha256(raw).hexdigest(),
           "k": 20, "rule": "argmax J/columns; ties: larger J, then smaller seed",
           "cells": {}}
    for t in (5, 6, 7):
        cell = [r for r in rows if r["k"] == 20 and r["t"] == t]
        assert len(cell) == 2000, (t, len(cell))
        s0 = next(r for r in cell if r["seed"] == 0)
        sel = max(cell, key=lambda r: (r["J"] / r["columns"], r["J"], -r["seed"]))
        jsel = max(cell, key=lambda r: (r["J"], -r["seed"]))
        pick = lambda r: {k: r[k] for k in ("seed", "columns", "J", "covered_points", "coverage_exact")}
        res["cells"][str(t)] = {
            "seed0": pick(s0),
            "selected_J_per_col": pick(sel),
            "selected_J_only_reference": pick(jsel),
            "census_cols_per_cov_ratio_sel_over_seed0":
                (sel["columns"] / sel["covered_points"]) / (s0["columns"] / s0["covered_points"]),
        }
    json.dump(res, open(out, "w"), indent=1, sort_keys=True)
    print(json.dumps(res, indent=1, sort_keys=True))


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
