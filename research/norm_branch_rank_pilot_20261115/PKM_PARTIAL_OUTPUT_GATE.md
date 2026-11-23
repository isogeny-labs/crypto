# Sparse PKM m=8: partial useful-output exponent gate (2026-11-15)

**Standing:** mathematical model derivation and producer-local exact-rational checks [pcc:unverified]. Not a measured solver, proof of asymptotic complexity for an actual sparse oracle, or a PCC-signed statement.

## Why this test

[Prime-field continuation protocol](../prime_leads_chase_20261113/CONTINUATION_PROTOCOL.md) gives two endpoints for optimistic full-call accounting: a B^gamma oracle returning every needed independent relation (threshold gamma<3.5) or one useful relation per call (threshold gamma<3). A real oracle may lie strictly between them. To avoid mistaking *raw decompositions* for useful rank, model the number of **independent, nonduplicate, verified rows actually added** per invocation as at most B^(delta+o(1)), with 0<=delta<=1.

The m=8 factor-base relation model in the frozen protocol assumes probability scale B^7/p. In the optimistic batching envelope, the invocation count is at least max(B^(1-delta), p/B^7), with the same assumptions as the frozen all-output cost floor (in particular it does not additionally multiply the two counts). If each **complete** call including failures and witness extraction costs B^(gamma+o(1)), plus a B^2 linear-algebra term, then:

    W(p,B) >= max(B^(1-delta), p/B^7)*B^gamma + B^2.

This is an optimistic necessary-condition model: independence correlations, additional random failures, setup, matrix density, memory and target recovery can only worsen costs relative to this envelope. It is not an unconditional complexity lower bound for every PKM implementation or unrelated index-calculus attack.

## Exact optimization

Put B=p^beta, 0<=beta<=1/7, and define h=max(gamma+1-delta,2). For gamma<7, the decreasing relation-density exponent and increasing output/linear-algebra exponent intersect at

    beta_opt = 1/(7-gamma+h),
    E_opt = h/(7-gamma+h).

For gamma>=7 the infimum exponent is at least 1 (beta=0). Since for delta in [0,1] the E=1/2 crossing occurs in the h=gamma+1-delta branch, the precise exponent gate for an optimistic sub-sqrt result is

    gamma < 3 + delta/2.

Equivalently, a measured gamma in [3,3.5) requires delta > 2*(gamma-3). At gamma=3.3, independent useful output must grow strictly faster than B^0.6. At gamma>=3.5, even delta=1 cannot pass the exponent gate. Passing the gate is *not sufficient*: full constant factors, setup, failed calls and rho must still be measured.

| delta | maximum gamma (strict) |
|---:|---:|
| 0 | 3 |
| 1/4 | 25/8 = 3.125 |
| 1/2 | 13/4 = 3.25 |
| 3/4 | 27/8 = 3.375 |
| 1 | 7/2 = 3.5 |

The exact-rational reference script pkm_partial_output_gate.py evaluates the optimal beta and exponent and checks the analytic result against a 4,097-point rational beta grid for multiple gamma/delta values. It intentionally does **not** fit gamma or delta from producer-only data.

## Required experiment amendment

For each *natural*, hidden-answer target in the previously frozen sparse PKM panel, store (1) full invocation cost including zero-output and timeouts, (2) exact decompositions whose EC sums verify, (3) duplicate-filtered rows, (4) the rank before and after adding all rows, and (5) factor-base, linear-algebra and individual-target costs. Fit gamma to the full invocation-cost statistic and delta to **incremental independent rank per invocation** on two uncensored held-out sizes. Never substitute candidate count, monomial density, a planted relation, or F4 solving degree for delta. Report a joint uncertainty region; ADVANCE only if its worst-cost/least-yield boundary has gamma < 3+delta/2 and a complete matched-rho comparison also passes. Censored/zero-positive cells remain inconclusive, not silently discarded.

Keep the previously frozen A1/A2 panel's ratios log_B(p) near 7,8,9 and its positive/negative controls; this is a **post-hoc diagnostic refinement**, not permission to modify the frozen data-selection rules.

## Reproduce

    python3 research/norm_branch_rank_pilot_20261115/pkm_partial_output_gate.py

### Sources reviewed

Repository [continuation protocol](../prime_leads_chase_20261113/CONTINUATION_PROTOCOL.md), [PKM tower implementation PR #4091](https://github.com/isogeny-labs/crypto/pull/4091), and [prior cost gate](STRIDE_COST_GATE.md). Primary: Petit, Kosters, Messeng, *Algebraic approaches to the ECDLP over prime fields* (PKC 2016, DOI 10.1007/978-3-662-49387-8_1); Shoup, *Lower Bounds for Discrete Logarithms and Related Problems* (EUROCRYPT 1997, DOI 10.1007/3-540-69053-0_18). The shared research-KB, Conductor, full Rust checkout and authorized EKS benchmark route were not exposed in this GitHub connector session. No novelty claim. [pcc:unverified]
