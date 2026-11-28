#!/usr/bin/env python3
"""Sizing rule for the k-sum index-calculus arm (prune=1, split=1, targets=stride-renew).

A cold attacker applies it from r (the prime subgroup order) and the curve's field family alone.

Counted group additions of one run, as a function of the base size N (abscissae) and summands k:

    build(N, k)  = 2^(k-1) C(N, k)                   one add per signed k-sum (split build)
                 + 2 C(N-k+2, 2)                     the pair-sum table
                 + sum_{j=2}^{k-2} 2^(j-1) C(N-k+j, j)   pruned prefixes of j summands
    trials(N, k) = (N + RHO) / (ALPHA T / r),  T = 2^(k-1) C(N, k)
    C0(N, k; r)  = build(N, k) + trials(N, k)        (one add per trial; set-up of the walk is independent of N, k)
    Cw(N, k; r)  = C0 + w (T + 2 trials)             (section 5a band: one insert per entry, one lookup and one
                                                      guard probe per trial, priced at w GAE)

Revision R1 (after the pilot, PROTOCOL.md): the rule picks the (k, N) minimising Cw at the band middle W = 0.1
(R0, frozen in c3a7472845, minimised C0) over k in 3..7, N > k, T <= 2^31 (ties: smaller T).
ALPHA (hit rate x r / T) and RHO (relations - N) are fitted once, on the committed split study
(research/ic_ksum_split_20261118, 16 arm-rows of -psr, measured rounds): ALPHA = 1.93-2.04, RHO = 0.6
W = 0.1  # R1; R0 used 0
(B24/B27/P24 k = 5: 9.88/12.91/11.93 relations for N = 9/12/11). Neither depends on a target.
"""
import json, math, sys
from math import comb

ALPHA = 2.0
RHO = 0.6
W = 0.1  # R1; R0 used 0
K_RANGE = range(3, 8)
MAX_T = 1 << 31


def table(N, k):
    return 2 ** (k - 1) * comb(N, k)


def build(N, k):
    pre = sum(2 ** (j - 1) * comb(N - k + j, j) for j in range(2, k - 1))
    return table(N, k) + 2 * comb(N - k + 2, 2) + pre


def c0(N, k, r, w=0.0):
    trials = (N + RHO) * r / (ALPHA * table(N, k))
    return build(N, k) + trials + w * (table(N, k) + 2 * trials)


def pick(r, ks=K_RANGE, w=W):
    best = None
    for k in ks:
        N = k + 1
        while table(N, k) <= MAX_T:
            c = (c0(N, k, r, w), table(N, k))
            if best is None or c < best[0]:
                best = (c, k, N)
            if table(N, k) > 4 * math.sqrt(r) * (N + 1):  # past the minimum for this k
                break
            N += 1
    return best[1], best[2]


def factor_base(field, N):
    return f"prime-abscissa:size={N}" if field == "prime" else f"binary-multiplier:size={N},mult=1"


if __name__ == "__main__":
    # usage: rule.py r [field]  -> prints JSON {k, N, factor_base, oracle, predicted_s0}
    r = int(sys.argv[1]); field = sys.argv[2] if len(sys.argv) > 2 else "prime"
    k, N = pick(r)
    print(json.dumps({"r": r, "log2_r": round(math.log2(r), 3), "k": k, "N": N,
                      "factor_base": factor_base(field, N), "oracle": f"ksum:m={k},prune=1,split=1",
                      "targets": "stride-renew", "rule": "R1 (w = 0.1)",
                      "predicted_c0_over_sqrt_r": round(c0(N, k, r) / math.sqrt(r), 4),
                      "predicted_c0p1_over_sqrt_r": round(c0(N, k, r, W) / math.sqrt(r), 4)}))
