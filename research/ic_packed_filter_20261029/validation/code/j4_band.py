"""J4 (TASK-20261029-654f73): S' of PROTOCOL section 5 and the section 5a order,
own code. Counted sessions are read through vcommon.load_counted.

S'_w(record) = (priced_gae_w + w * table_filter_bit_sets) / sqrt(r), with
priced_gae_w = G - C + w (U + n_C) as band.rs (3271a25d02) defines it (no
record carries pinned calibration units, so C = n_C = 0; vcommon stops if one
does). Ratios are sum S'_w(B) / sum S'_w(A) over matched measured pairs keyed
by (curve, target, round), overall and per curve. Without the bit-set term
the same ratio is band.rs's priced S ratio.

usage: j4_band.py OUT.json
"""
import math
import sys

import vcommon as v

W = (0.0, 0.1, 1.0)


def ratio(arms, a, b, w, curve=None, with_bits=True):
    A, B = arms[a], arms[b]
    if set(A) != set(B):
        raise SystemExit(f"STOP: unmatched keys {a} {b}")
    ks = [k for k in A if curve is None or k[0] == curve]
    f = v.sprime if with_bits else (lambda r, w: v.priced_gae(r, w) / math.sqrt(r["workload"]["curve"]["r"]))
    sa = sum(f(A[k], w) for k in ks)
    sb = sum(f(B[k], w) for k in ks)
    return sb / sa, len(ks), sa / len(ks), sb / len(ks)


def table(arms, pairs):
    curves = sorted({k[0] for k in next(iter(arms.values()))})
    out = {}
    for a, b in pairs:
        row = {}
        for w in W:
            R, n, ma, mb = ratio(arms, a, b, w)
            R0, _, _, _ = ratio(arms, a, b, w, with_bits=False)
            row[str(w)] = {"ratio_b_over_a": R, "pairs": n, "mean_Sprime_a": ma, "mean_Sprime_b": mb,
                           "ratio_without_bitset_term": R0,
                           "per_curve": {c: ratio(arms, a, b, w, c)[0] for c in curves},
                           "per_curve_mean_Sprime": {c: {"a": ratio(arms, a, b, w, c)[2], "b": ratio(arms, a, b, w, c)[3]} for c in curves}}
        # 5a: order per curve at each weight (B costlier means ratio > 1)
        orders = {c: [("A<B" if row[str(w)]["per_curve"][c] > 1 else "A>B" if row[str(w)]["per_curve"][c] < 1 else "tie") for w in W] for c in curves}
        row["order_per_curve_at_w0_0.1_1"] = orders
        row["strict_same_order_across_band_every_curve"] = all(len(set(o)) == 1 and o[0] != "tie" for o in orders.values())
        out[f"{b}/{a}"] = row
    return out


def main():
    res = {}
    for b in (32, 24):
        wall = v.by_arm(v.load_counted(b, "wall"))  # counts only; no wall field needed here
        res[f"P{b}-wall"] = table(wall, [("dgb", "dgbf"), ("dgb", "dgb-aa"), ("dgbf-edge", "dgbf")])
    for b in (32, 24, 28):
        c = v.by_arm(v.load_counted(b))
        res[f"P{b}-counted"] = table(c, [("rho-neg", "dgb"), ("rho-neg", "dgbf"), ("dgb", "dgbf"), ("rho-neg", "rho-neg-aa")])
    v.dump(res, sys.argv[1])
    for s, t in res.items():
        for pair, row in t.items():
            print(s, pair, " ".join("%.6f" % row[str(w)]["ratio_b_over_a"] for w in W),
                  "| no-bits", " ".join("%.9f" % row[str(w)]["ratio_without_bitset_term"] for w in W),
                  "| w=0.1 per curve", " ".join("%.4f" % x for x in row["0.1"]["per_curve"].values()),
                  "| w=1 per curve", " ".join("%.4f" % x for x in row["1.0"]["per_curve"].values()),
                  "| strict order", row["strict_same_order_across_band_every_curve"], row["order_per_curve_at_w0_0.1_1"])


if __name__ == "__main__":
    main()
