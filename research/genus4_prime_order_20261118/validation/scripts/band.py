#!/usr/bin/env python3
"""C8: the AGENTS.md section 5a band for every (prime, arm), from out/recompute.json (lesson L-546d37ca7750: the
band is recomputed with the canonical rule, every listed unpriced operation counter priced at w on every arm).

These are genus_ladder rows, not ecbench sessions, so `ecbench band` does not apply; the rule is applied directly:
  IC arm:   U = rel_mul + rel_add + la_mul + la_add (the counters the rows list), cost = group_ops + w U.
            Not emitted by the walk (listed, and priced in a second reading): one mix64 partition hash and two
            coefficient additions mod r per walk step (genus_ladder.rs 1590-1594), counted as 3 walk_ops; the
            large-prime table lookups per relation (HashMap/DSU) are not emitted either (about 1.5e4 relations per run).
  Reference: the ideal negation bound sqrt(pi r / 4), nothing charged (strongest available: identity and the
            hyperelliptic involution are the only F_p-automorphisms, certs.json; Frobenius is trivial on J(F_p)).
  Calibrated weight (owner rule 2026-11-12): no w_cal was measured; the break-even w at which the IC mean equals
            the reference is reported, so any w_cal with 10 w_cal below it keeps the order.
Writes out/band.json."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *

rc = json.load(open(os.path.join(OUT, "recompute.json")))
out = {"rule": __doc__, "cells": {}}
for p in ("16411", "32771"):
    for arm in ("x", "x*"):
        a = rc["primes"][p]["arms"][arm]
        Rn, Rp = a["mean_R_neg"], a["mean_R_plain"]
        cell = {"mean_group_ops": a["mean_cost_w0"], "mean_native_U": a["mean_native"], "mean_walk_ops": a["mean_walk_ops"],
                "mean_R_neg": Rn, "mean_R_plain": Rp, "band": {}, "band_with_walk_hash_coef": {},
                "break_even_w_vs_neg": a["break_even_w_vs_neg"],
                "break_even_w_la_with_rel_w1": a["break_even_w_la_with_rel_w1"], "break_even_w_la_with_rel_w0": a["break_even_w_la_with_rel_w0"],
                "cantor_inv_per_group_op": a["mean_cantor_inv_per_group_op"],
                "cantor_mul_per_group_op": a["mean_cantor_mul_per_group_op"]}
        for wn, w in (("0", 0.0), ("0.1", 0.1), ("1", 1.0)):
            ic = a["mean_cost_w0"] + w * a["mean_native"]
            ic2 = ic + w * 3 * a["mean_walk_ops"]
            cell["band"][wn] = {"ic": ic, "rho_neg": Rn, "ic_over_rho_neg": ic / Rn, "ic_over_rho_plain_expect": ic / Rp,
                                "ci_hi_over_rho_neg": a[f"neg_w{wn}"]["hi"]}
            cell["band_with_walk_hash_coef"][wn] = {"ic": ic2, "ic_over_rho_neg": ic2 / Rn}
        cell["order_below_at_all_w_point"] = all(cell["band"][w]["ic"] < Rn for w in cell["band"])
        cell["order_below_at_all_w_ci_hi"] = all(cell["band"][w]["ci_hi_over_rho_neg"] < 1 for w in cell["band"])
        cell["order_below_with_walk_hash_coef"] = all(cell["band_with_walk_hash_coef"][w]["ic"] < Rn for w in cell["band"])
        out["cells"][f"{p} {arm}"] = cell
dump("band.json", out)
for k, c in out["cells"].items():
    print(k, {w: round(v["ic_over_rho_neg"], 5) for w, v in c["band"].items()}, "point", c["order_below_at_all_w_point"],
          "ci", c["order_below_at_all_w_ci_hi"], "with hash/coef w=1", round(c["band_with_walk_hash_coef"]["1"]["ic_over_rho_neg"], 5),
          "break-even w", round(c["break_even_w_vs_neg"], 4), "LA-only break-even (rel at 1 / 0)", round(c["break_even_w_la_with_rel_w1"], 3), round(c["break_even_w_la_with_rel_w0"], 3), "inv/op", round(c["cantor_inv_per_group_op"], 2), "cantor_mul/op", round(c["cantor_mul_per_group_op"], 1))
