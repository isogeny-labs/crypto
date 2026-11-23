#!/usr/bin/env python3
"""Descriptive check on recorded cell means (paired.json structure): predicted Wiedemann ratio
n_out*(generic_out+n_out) / (n_in*(generic_in+n_in)) against the recorded Wiedemann ratio.
Not a fitted model; no parameter. Usage: python3 model_check.py paired.json"""
import json, sys
P = json.load(open(sys.argv[1]))
print("| g | p | setting | recorded Wiedemann mul ratio | n(generic+n) ratio |\n|--:|--:|---|--:|--:|")
for x in P["structure"]:
    n0, n1 = x["mean_sge_cols_in"], x["mean_sge_cols_out"]
    g0, g1 = x["mean_sge_generic_in"], x["mean_sge_generic_out"]
    print(f"| {x['g']} | {x['p']} | {x['setting']} | {x['mean_la_wied_mul'] / x['mean_base_la_wied_mul']:.3f} | {n1 * (g1 + n1) / (n0 * (g0 + n0)):.3f} |")
