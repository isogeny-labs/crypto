#!/usr/bin/env python3
"""Cost landscape of the Gaudry/Diem/Joux-Vitse index-calculus family on EcGFp5, as a function of the
per-system solving cost c (analysis only; every input is a cited constant, no measurement).

Unit: Mp = one multiplication in GF(p), p = 2^64 - 2^32 + 1 (10.18 cycles on the paper's i5-8259U).

Sources (frozen copies in crypto-autoresearcher/inputs/):
  JV  = Joux-Vitse, ePrint 2010/157, paper_fulltext.md lines 322-370 (Gaudry/Diem: relation search
        n! c(n,q) q/2, sparse linear algebra O~(n q^2); large primes: n! c(n,q) q^{2-2/n}) and 372-440
        ((n-1)-point variant: (n-1)! c~(n,q) q^2 / 2).
  P22 = Pornin, ePrint 2022/274, paper_fulltext.md lines 113-152 (counts p^{2-2/5} systems x D^2 >= 2^142)
        and the benchmark table after line 928 (GF(p) mul 10.18 cycles, point addition 1328.50 cycles).
Model choices (stated, not measured):
  * factor base: any subset of the ~q/2 abscissae in F_p having a point; decomposition probability of a
    random target into n points of a base of size B is (2B/q)^n / n!  (= 1/n! for the full base);
    relations needed = B; each trial solves one system at cost c.
  * linear algebra: Wiedemann/Lanczos on B columns with n non-zeros per row, cost u * n * B^2 with u the
    cost of one operation mod the 319-bit group order in Mp (u = 1 for additions of small multiples,
    u = 10 as a pessimistic multiplication cost).
  * rho: sqrt(pi * N / 4) steps with N ~ 2^319 (negation map), 37 to 130 Mp per step (affine addition
    with batched inversion, to the paper's 1328.5-cycle addition).
    python3 cost_landscape.py   ->  cost_landscape.json, cost_landscape.md
"""
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
n = 5
LOG_Q = 64.0                                  # p = 2^64 - 2^32 + 1
LOG_HALF_Q = LOG_Q - 1                        # abscissae with a point: about q/2
LOG_NFACT = math.log2(math.factorial(n))      # 6.91
LOG_N_GROUP = 319.0
LOG_RHO_STEPS = 0.5 * (math.log2(math.pi / 4) + LOG_N_GROUP)        # 159.33
RHO_STEP_MP = (37.0, 1328.50 / 10.18)                                # 37 .. 130.5 Mp
LOG_RHO = tuple(LOG_RHO_STEPS + math.log2(s) for s in RHO_STEP_MP)


def lg_add(a, b):
    return max(a, b) + math.log2(1 + 2 ** (-abs(a - b)))


def full_base(lc, lu):
    """Gaudry/Diem basic algorithm with the full factor base (JV lines 352-361)."""
    rel = lc + LOG_NFACT + LOG_HALF_Q
    la = lu + math.log2(n) + 2 * LOG_HALF_Q
    return lg_add(rel, la), LOG_HALF_Q


def tuned_base(lc, lu):
    """Same algorithm with the factor-base size B chosen to minimise relation search + linear algebra."""
    best = None
    for i in range(0, 6301):
        lb = 40 + i * 0.01
        if lb > LOG_HALF_Q:
            break
        # relation search: B relations x n! (q/2B)^n trials x c
        rel = lc + LOG_NFACT + lb + n * (LOG_HALF_Q - lb)
        la = lu + math.log2(n) + 2 * lb
        tot = lg_add(rel, la)
        if best is None or tot < best[0]:
            best = (tot, lb)
    return best


def double_large_prime(lc, lu):
    """Gaudry's rebalanced version as JV quote it (lines 362-367): n! c q^{2-2/n}, LA n q^{2-2/n}."""
    lq = (2 - 2 / n) * LOG_Q
    rel = lc + LOG_NFACT + lq
    la = lu + math.log2(n) + lq
    return lg_add(rel, la), (1 - 1 / n) * LOG_Q


def jv_n_minus_1(lc_tilde, lu):
    """JV (n-1)-point variant (lines 418-436): (n-1)! c~ q^2/2, LA O~(n q^2)."""
    rel = lc_tilde + math.log2(math.factorial(n - 1)) + 2 * LOG_Q - 1
    la = lu + math.log2(n) + 2 * LOG_HALF_Q
    return lg_add(rel, la), LOG_HALF_Q


def memory_bytes(lb):
    """Relations (n indices x 8 B) plus two Wiedemann vectors of 40-byte residues."""
    return math.log2(n * 8 * 2 ** lb + 2 * 40 * 2 ** lb)


def main():
    rows = []
    for lc in range(28, 73, 2):
        r = {"log2_c_Mp": lc}
        for lu, tag in ((0.0, "u1"), (math.log2(10), "u10")):
            f, lbf = full_base(lc, lu)
            t, lbt = tuned_base(lc, lu)
            d, lbd = double_large_prime(lc, lu)
            r[f"full_base_{tag}"] = round(f, 2)
            r[f"tuned_base_{tag}"] = round(t, 2)
            r[f"tuned_log2_B_{tag}"] = round(lbt, 2)
            r[f"tuned_log2_mem_bytes_{tag}"] = round(memory_bytes(lbt), 1)
            r[f"double_large_prime_{tag}"] = round(d, 2)
            r[f"jv_n_minus_1_ctilde_eq_c_{tag}"] = round(jv_n_minus_1(lc, lu)[0], 2)
        rows.append(r)
    D = {"quotient 2^16 (conjectured, KN-OPEN-9b4a2b)": 16.0, "S5-symmetrized 2^20": 20.0, "raw 2^26.9": 26.9}
    bands = {k: {"c_low_D2": 2 * v, "c_high_5D3": math.log2(5) + 3 * v} for k, v in D.items()}

    def crossing(fn, lu, target):
        lo, hi = 0.0, 140.0
        if fn(lo, lu)[0] > target:
            return None
        if fn(hi, lu)[0] <= target:
            return hi
        for _ in range(100):
            mid = (lo + hi) / 2
            if fn(mid, lu)[0] <= target:
                lo = mid
            else:
                hi = mid
        return round(lo, 2)

    out = {
        "unit": "Mp = one GF(p) multiplication (10.18 cycles, Pornin 2022 benchmark table)",
        "n": n, "log2_q": LOG_Q,
        "rho_log2_steps": round(LOG_RHO_STEPS, 2),
        "rho_log2_Mp_range": [round(x, 2) for x in LOG_RHO],
        "target_log2": 128.0,
        "designer_claim": "p^{2-2/5} x D^2 >= 2^142 (no n! factor; large-prime count)",
        "per_system_cost_bands_log2_Mp": bands,
        "rows": rows,
        "max_log2_c_with_tuned_base_below_128_u1": crossing(tuned_base, 0.0, 128.0),
        "max_log2_c_with_tuned_base_below_128_u10": crossing(tuned_base, math.log2(10), 128.0),
        "max_log2_c_with_tuned_base_below_rho_u10": crossing(tuned_base, math.log2(10), LOG_RHO[0]),
        "full_base_floor_log2_Mp": {"u1": round(full_base(0, 0)[0], 2), "u10": round(full_base(0, math.log2(10))[0], 2)},
    }
    (HERE / "cost_landscape.json").write_text(json.dumps(out, indent=2) + "\n")
    lines = [f"Unit Mp (GF(p) multiplication). Rho: 2^{LOG_RHO_STEPS:.2f} steps = 2^{LOG_RHO[0]:.1f}..2^{LOG_RHO[1]:.1f} Mp.",
             "", "| log2 c | full base | tuned base (log2 B, log2 memory bytes) | double large prime | JV n-1 (c~ = c) |",
             "|--:|--:|:--|--:|--:|"]
    for r in rows:
        if r["log2_c_Mp"] % 4:
            continue
        lines.append(f"| {r['log2_c_Mp']} | {r['full_base_u1']} – {r['full_base_u10']} | "
                     f"{r['tuned_base_u1']} – {r['tuned_base_u10']} (B 2^{r['tuned_log2_B_u10']}, mem 2^{r['tuned_log2_mem_bytes_u10']}) | "
                     f"{r['double_large_prime_u1']} – {r['double_large_prime_u10']} | {r['jv_n_minus_1_ctilde_eq_c_u1']} |")
    lines += ["", "Ranges are u = 1 – u = 10 Mp per linear-algebra operation.", "",
              "Per-system cost bands (log2 Mp, D^2 .. 5 D^3): " + "; ".join(f"{k}: {v['c_low_D2']:.1f}..{v['c_high_5D3']:.1f}" for k, v in bands.items()),
              f"Largest log2 c keeping the tuned base below 2^128: {out['max_log2_c_with_tuned_base_below_128_u1']} (u=1), {out['max_log2_c_with_tuned_base_below_128_u10']} (u=10).",
              f"Largest log2 c keeping the tuned base below rho (2^{LOG_RHO[0]:.1f} Mp, u=10): {out['max_log2_c_with_tuned_base_below_rho_u10']}.",
              f"Full-base linear-algebra floor: 2^{out['full_base_floor_log2_Mp']['u1']} (u=1) .. 2^{out['full_base_floor_log2_Mp']['u10']} (u=10) Mp."]
    (HERE / "cost_landscape.md").write_text("\n".join(lines) + "\n")
    print("\n".join(lines))


if __name__ == "__main__":
    main()
