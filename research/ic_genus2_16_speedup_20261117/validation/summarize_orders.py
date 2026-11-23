#!/usr/bin/env python3
"""Aggregate check_orders.py outputs (orders_chunk*.out, one JSON object per
line) and own_jorder.out into one table for VALIDATION_P1P2.md."""
import glob
import json
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
res = {}
for fn in sorted(glob.glob(os.path.join(HERE, "orders_chunk*.out"))):
    with open(fn) as fh:
        for line in fh:
            if line.startswith("{"):
                d = json.loads(line)
                res[(d["p"], d["g"], d["curve_seed"])] = d
own2 = {}
fn = os.path.join(HERE, "own_jorder.out")
if os.path.exists(fn):
    with open(fn) as fh:
        for line in fh:
            m = re.match(r"p=(\d+) g=(\d+) curve=(\d+):.*#J_own=(\d+); #J_pari=(\d+); own in candidates=(\w+); equal=(\w+)", line)
            if m:
                own2[(int(m[1]), int(m[2]), int(m[3]))] = (int(m[4]), m[6] == "True", m[7] == "True")
print("| g | p | curve | order (record) | r (record) | #J own | #J PARI | own method | #J/order | r prime | r \\| #J | [order]D=0 (8/8) | verdict |")
print("|---|---|---|---|---|---|---|---|---|---|---|---|---|")
n_eq = n_mult = n_fail = n_own = 0
for k in sorted(res, key=lambda k: (k[1], k[0], k[2])):
    d = res[k]
    if d["own_unique_equals_pari"]:
        jown, how = d["J_pari"], f"unique candidate (K={d['K_own']} counts + HW mod p + e')"
        n_own += 1
    elif k in own2 and own2[k][2] and own2[k][1]:
        jown, how = own2[k][0], f"Sylow closure ({d['own_candidates']} candidates)"
        n_own += 1
    else:
        jown, how = None, "not pinned"
    ok_cross = d["pari_funceq"] and d["own_a_match_pari"] and d["hw_match_pari"] and d["J_kills_D"] == d["random_D"]
    if d["order_eq_J"]:
        v = "PASS (order = #J)"
        n_eq += 1
    elif d["r_prime"] and d["r_divides_J"] and d["order_kills_D"] == d["random_D"] and d["order"] % d["e_lower"] == 0:
        v = "PASS (exponent multiple)"
        n_mult += 1
    else:
        v = "FAIL"
        n_fail += 1
    if not ok_cross or jown != d["J_pari"]:
        v += " / #J cross-check FAIL"
    print(f"| {d['g']} | {d['p']} | {d['curve_seed']} | {d['order']} | {d['r']} | {jown} | {d['J_pari']} | {how} | "
          f"{d['J_pari'] // d['order'] if d['J_pari'] % d['order'] == 0 else round(d['J_over_order'], 4)} | "
          f"{d['r_prime']} | {d['r_divides_J']} | {d['order_kills_D']}/{d['random_D']} | {v} |")
print(f"\ninstances: {len(res)}; order == #J: {n_eq}; order = exponent multiple != #J: {n_mult}; FAIL: {n_fail}; "
      f"#J pinned by own code: {n_own}")
