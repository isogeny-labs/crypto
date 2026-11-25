#!/usr/bin/env python3
"""Table for the m-ladder at p = 3907 (12-bit roster curve, bench-12bit).

Reads `results/m*_p12t*.json` from `ic bench`; one row per m:
|V| = 2^t, p^(1/m), |F|, columns, the paper's density with deg L = 2^t and
with the measured |F|, the observed rate, targets, rows, rank, solving
degree, recovered scalar and charged cost per target.

`D = 0` means the F4 exceeded its degree bound on every system and the
oracle decided on the exact |V|^m grid; those rows are enumeration, not
Groebner work, and are labelled as such in the README.
"""
import glob
import json
import math
import os

P = 3907  # bench-12bit, src/cryptanalysis/research_bench.rs
HERE = os.path.dirname(os.path.abspath(__file__))


def main():
    print(f"{'m':>2s} {'t':>2s} {'|V|':>4s} {'p^1/m':>6s} {'|F|':>4s} {'cols':>4s} "
          f"{'predL':>7s} {'predF':>7s} {'obs':>6s} {'tgt':>5s} {'rows':>4s} "
          f"{'rank':>4s} {'D':>2s} {'k':>6s} {'GAE/tgt':>9s} {'note':>9s}")
    for f in sorted(glob.glob(os.path.join(HERE, "results", "m*_p12t*.json"))):
        d = json.load(open(f))
        r = d["rows"][0]
        sp = r["spec"]
        m = int(sp["oracle_params"]["m"])
        t = int(sp["factor_base_params"]["t"])
        F = r["factor_base"]["signed_points"]
        cols = r["factor_base"]["columns"]
        dec = r["decomposition"]
        obs = dec["hit_rate"]
        tgt = dec["targets_tried"]
        D = dec["solver"]["solving_degree_max"]
        gae = r["total_gae"] / max(tgt, 1)
        predL = (2 ** t) ** m / math.factorial(m) / P
        predF = F ** m / math.factorial(m) / P
        note = "grid" if D == 0 else "f4"
        print(f"{m:2d} {t:2d} {2**t:4d} {P ** (1 / m):6.1f} {F:4d} {cols:4d} "
              f"{predL:7.4f} {predF:7.4f} {obs:6.3f} {tgt:5d} "
              f"{r['linear_algebra']['rows']:4d} {r['linear_algebra']['rank']:4d} "
              f"{D:2d} {r.get('recovered') or 0:6d} {gae:9.0f} {note:>9s}")


if __name__ == "__main__":
    main()
