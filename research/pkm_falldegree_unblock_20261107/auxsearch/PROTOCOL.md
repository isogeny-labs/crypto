# Auxiliary-curve search at P1: protocol (frozen before data, 2026-11-15)

Owner request (2026-11-15): "What about the auxiliary curve with smooth order for factor base and map did we measure
that and replicate the results", then "Yes" to: replicate the 14 single searches by their seeds; fix a prediction from
the 2-Sylow structure before running; run 200 seeds per t for t = 8..14 at P1; report tries and cost.
Local run on the session machine (no RunPod, no EKS). Every number is `[pcc:unverified]`.

## What is measured

`isogeny_with(p, t, rng, AuxOrder::Mestre)` in `examples/pkm_tower_pilot.rs`, the code that built every isogeny tower
in iso-a / iso-14, called alone by `pkm_tower_pilot --aux-search FILE` (lines `p t seed`). It draws (a, b) uniformly,
computes #E' by Mestre/BSGS (private stream), and accepts when some [#E'/2^t]P (16 tries) has exact order 2^t, then
builds R', V and the t-step Vélu chain. Per search: curves tried, (a, b, #E'), group operations of the search
(`aux_ops_search`: everything before the chain) and of the chain (`aux_ops_chain`), wall ms.

## Part 1: replication

The 14 iso-a/iso-14 searches at P1 = 2013265921, m = 2, g = 1, t = 1..14, cell seed
`0x504B4D54 ^ 2<<40 ^ 2<<32 ^ t<<16 ^ 1` (kind Isogeny = 2). Pass: (aux_a, aux_b, aux_order, curves_tried) equal to the
recorded rows for all 14. Any mismatch stops the study.

## Part 2: prediction (`predict.py`, exact count, brute force for t <= 4)

Model H: Frobenius uniform in SL2(Z/2^t) (det = p = 1 mod 2^27). Accept <=> tr F = 2 mod 2^t and F != I mod 2
(cyclic 2-Sylow of order >= 2^t). Result: P_accept = 2^-t exactly, so curves tried ~ Geometric(2^-t), mean 2^t.
Contrast model D (divisibility 2^t | #E' only, the acceptance one would assume from "smooth order"):
P = (2 - O(2^-t)) 2^-t, mean ~ 2^(t-1).

Model H ignores finite p: at t = 12, 13, 14 only about 44, 22, 11 traces s = 2 mod 2^t lie in the Hasse interval,
so class-number fluctuation can move the true P at those t. (An exact class-number value was not computed: installing
PARI was refused in this session.)

## Part 3: sweep

t = 8..14, 200 searches each, seed_i = 0xA0C5_0000_0000 ^ t<<16 ^ i (i = 0..199), independent of all cell seeds.

## Reading (per t, fixed now)

- r_t = (mean curves tried) / 2^t. Under H the sum of 200 geometrics gives a 99% interval for r_t of about
  [0.83, 1.19] (gamma approximation, computed in `analyze.py`).
- **H holds at t** if r_t is in its 99% interval. **D is rejected at t** if r_t is above D's 99% interval
  (around 0.5).
- t <= 11: a miss of H is a finding against the model. t = 12..14: a miss is reported with the finite-trace caveat
  and is not a claim against H.
- Also reported: Kolmogorov–Smirnov of tries/2^t against Exp(1) per t (descriptive), the 14 replicated values'
  position in their distributions, and the search cost in group operations per search and per curve tried, against
  the chain cost.

## Not claimed

No change to any fall-degree reading. The search cost is a per-curve precomputation; it is reported, not charged
against rho here.
