#!/usr/bin/env python3
"""L10 seed selection (frozen with PROTOCOL.md; reads the existing census only).

For every k in {16,18,20,22,24} and t in {4,5,6,7}: the J/columns-selected seed
S*(k,t) = argmax J/columns over the 2,000-seed census (ties: larger J, then
smaller seed), the T9/T10 selector. Seed 0 is recorded as the control.

Usage: python3 -I select_seeds.py <census dir> <out.json>
"""
import hashlib, json, os, sys

KS = (16, 18, 20, 22, 24)
TS = (4, 5, 6, 7)
R = {16: 8209, 18: 32719, 20: 131203, 22: 524731, 24: 2097349}


def main(cdir, out):
    res = {"rule": "argmax J/columns; ties: larger J, then smaller seed",
           "census": {}, "cells": {}}
    for k in KS:
        path = os.path.join(cdir, f"k{k}.jsonl")
        raw = open(path, "rb").read()
        res["census"][str(k)] = {"path": path, "sha256": hashlib.sha256(raw).hexdigest()}
        rows = [json.loads(l) for l in raw.decode().splitlines() if l.strip()]
        for t in TS:
            cell = [r for r in rows if r["k"] == k and r["t"] == t]
            assert len(cell) == 2000, (k, t, len(cell))
            s0 = next(r for r in cell if r["seed"] == 0)
            sel = max(cell, key=lambda r: (r["J"] / r["columns"], r["J"], -r["seed"]))
            pick = lambda r: {x: r[x] for x in ("seed", "columns", "J", "covered_points", "coverage_exact")}
            res["cells"][f"k{k}_t{t}"] = {"k": k, "t": t, "r": R[k], "seed0": pick(s0),
                                         "selected": pick(sel)}
    json.dump(res, open(out, "w"), indent=1, sort_keys=True)
    for key, c in res["cells"].items():
        s = c["selected"]; z = c["seed0"]
        print(f"{key} S*={s['seed']} cols={s['columns']} cov={s['covered_points']} | seed0 cols={z['columns']} cov={z['covered_points']}")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
