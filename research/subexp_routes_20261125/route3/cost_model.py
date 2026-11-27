#!/usr/bin/env python3
"""m-general cost model for the PKM relation route, three decision engines.

Notation: b = log2 p (field bits), m = Semaev summands, T = log2|V| abscissae,
q = relation probability per target, R = |V| relations needed (columns).

Base size for a constant yield: |V| = p^(1/m), i.e. T = b/m.

  q_m        = min(1, c_m * |V|^m / p)          c_2 = 1.94, c_3 = 0.90 measured
  targets    = R / q = |V| / q
  engine A (F4)      : ops = targets * C_F4(T),  C_F4(T) = c0 * lam^(T-5)   (measured)
  engine B (tree, no table): ops = targets * |V|^(m-1) * 125   (route 1: 125 mults/leaf)
  engine C (tree + reusable k-sum table): ops = 125 * (|V|^k + targets * |V|^(m-k))

Exponent in p is the numerical d log2(ops) / d b at large b.
"""
import json
import math

C2 = 1.94          # measured m=2 yield constant (ic prime toy:13,1 size 18)
C3 = 0.90          # measured m=3 yield constant (sumset_sweep, T=5, unsaturated)
LAM = 15.72        # measured F4 multiplier per +1 aux variable (docs, T=5 -> 6)
C0 = 127.4e6       # measured F4 multiply-adds per call at T=5 (interval arm)
LEAF = 125         # measured mults per tree leaf (route 1, flat T=5..8)


def q(m, b, c=None):
    cm = c if c is not None else (C2 if m == 2 else C3)
    return min(1.0, cm * 2.0 ** (m * (b / m) - b))


def cost_f4(m, b):
    T = b / m
    return (2.0 ** (b / m) / q(m, b)) * C0 * LAM ** max(0.0, T - 5)


def cost_tree_noreuse(m, b):
    T = b / m
    return (2.0 ** (b / m) / q(m, b)) * 2.0 ** ((m - 1) * T) * LEAF


def cost_tree_table(m, b, k=None, mem_bits=None):
    """k-sum table, optionally capped by a memory budget of 2**mem_bits entries."""
    T = b / m
    V = 2.0 ** T
    ks = list(range(1, m)) if k is None else [k]
    if mem_bits is not None:
        ks = [kk for kk in ks if kk * T <= mem_bits] or [1]
    return min(LEAF * (V ** kk + (V / q(m, b)) * V ** (m - kk)) for kk in ks)


def exponent(fn, m, b0=128.0, b1=384.0):
    return (math.log2(fn(m, b1)) - math.log2(fn(m, b0))) / (b1 - b0)


def main():
    BITS = (13, 32, 64, 128, 192, 256)
    out = {"constants": {"c2": C2, "c3": C3, "lambda_f4": LAM,
                         "f4_ops_at_T5": C0, "tree_mults_per_leaf": LEAF},
           "engines": {}}
    engines = [("A_f4", cost_f4), ("B_tree_noreuse", cost_tree_noreuse),
               ("C_tree_table", cost_tree_table)]
    for w in (40, 60, 80):
        engines.append((f"C_table_mem2^{w}", lambda m, b, w=w: cost_tree_table(m, b, mem_bits=w)))
    for name, fn in engines:
        rows = []
        for m in range(2, 9):
            rows.append({"m": m, "T_over_b": 1.0 / m,
                         "exponent_asymptotic": round(exponent(fn, m), 3),
                         "log2_ops": {b: round(math.log2(fn(m, b)), 1) for b in BITS}})
        out["engines"][name] = rows

    # closed forms for the two tree engines (independent of the numeric fit)
    out["closed_forms"] = {
        "A_f4": "(1 + log2 lam)/m = 4.97/m  [valid only while the F4 shape is fixed;"
                " at m=3 the shape changes (16 vars, two quartics) and the measured"
                " per-call work is 1.3e4x the m=2 call at the same T, unresolved]",
        "B_tree_noreuse": "1.0 for every m  (|V|^(m-1) per target x |V|/q targets = |V|^m = p)",
        "C_tree_table": "min_k max(k, m - k + 1)/m  ->  1.0, 0.667, 0.75, 0.6, 0.667,... -> 0.5",
    }
    out["table_exponent_by_m"] = {
        str(m): round(min(max(k, m - k + 1) for k in range(1, m + 1)) / m, 3)
        for m in range(2, 11)}

    # 13-bit anchors, measured
    out["anchors_13bit"] = {
        "m2_f4_candidate_ops_per_call_mean": 12808615498 / 3220,
        "m2_f4_interval_ops_per_call_mean": 419082970972 / 3290,
        "m2_relation_prob_per_trial_candidate": 0.1279,
        "m2_relation_prob_per_trial_interval": 0.1315,
        "m2_total_gae_candidate": 632065518.4,
        "m2_total_gae_interval": 2246119779.6,
        "m2_s_over_rho_median_candidate": 115377.2,
        "m2_s_over_rho_median_interval": 412301.6,
        "m3_f4_candidate_ops_per_call_mean_censored": 419469920059 / 8,
        "m3_f4_interval_ops_per_call_mean_censored": 7961130404 / 8,
        "m3_f4_verified_rows": 0,
        "m3_f4_censored_rows": 8,
        "m3_exact_decision_adds_per_target": 18 ** 2 + 18,
        "m3_exact_decision_mults_priced_tree": 18 ** 2 * LEAF,
        "m3_exact_decision_adds_with_hash_table": 18 ** 2 + 18,
        "measured_ns_per_add": 47.8,
    }
    out["m3_over_m2_measured_per_call"] = (
        (419469920059 / 8) / (12808615498 / 3220))
    print(json.dumps(out, indent=1, sort_keys=True))
    with open("cost_model.json", "w") as f:
        json.dump(out, f, indent=1, sort_keys=True)


if __name__ == "__main__":
    main()
