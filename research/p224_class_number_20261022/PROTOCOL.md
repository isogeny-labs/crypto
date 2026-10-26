---
title: "Protocol: class number of the CM field of NIST P-224 (and P-256)"
date: "2026-10-22"
---

# Question

`research/p224_hypothesis_ranking_20261008/` left "class number of `D_K`" as
**OPEN** with only the heuristic `h = sqrt|D_K| L(1,chi)/pi ~ 2^106..2^111`.
Close that item as far as is feasible and honest. Frozen before any
computation on `D_K`.

# Inputs (frozen)

* P-224: `p = 2^224 - 2^96 + 1`, trace `t = 4733100108545601916421827343930821`
  (`research/p224_hypothesis_ranking_20261008/run/cm_discriminant.json`),
  `t^2 - 4p = -9 |D_K|`, conductor of `Z[pi]` is 3, so `D_K` is the product of
  the signed factors `-3, 29, 79, 7523, 40927, 11549194661` and the 45-digit
  prime `388425074903852603481408727235725774844022299`.
* P-256: `p = 2^256 - 2^224 + 2^192 + 2^96 - 1`, `t = p + 1 - n` with the
  standard order `n`; `D_K = t^2 - 4p` (conductor 1), factors `3, 5,
  456597257999, 1428624589419343516204097,
  46523541035814968339936406074986559003387`
  (`run/published_factorizations.json`, SafeCurves).
* The program recomputes `t^2 - 4p` from `(p, t)` and refuses to run unless it
  equals `-c^2 * prod(factors)` exactly, `D` is `1 mod 4` and every factor
  passes a strong-probable-prime test. A mismatch is a stop, not a warning.

# What is computed

1. **Euler product.** `L_B = prod_{p<=B} (1 - chi(p)/p)^{-1}`, `chi(p) =
   (D/p)` by Jacobi symbol of `D mod p`, a segmented sieve over `p <= B`,
   `B = 10^10` (stop: if the run needs more than 60 minutes of wall time, report the
   largest `B` completed). Running values are recorded at `B = 10^3 ... 10^10`
   to show convergence. `h = sqrt|D| L(1,chi)/pi` (`w = 2`).
2. **Error statements, each labelled.**
   * HEURISTIC (random-sign model): tail of `sum chi(p)/p` over `p > B` has
     standard deviation `~ (sum_{p>B} p^-2)^{1/2}`.
   * GRH-CONDITIONAL (explicit, with an *assumed* constant): if
     `|sum_{p<=x} chi(p) ln p| <= c sqrt(x) ln^2(|D| x)` for `x >= B` then the
     tail of `ln L` is bounded by an explicit partial-summation integral,
     evaluated numerically. `c = 1/pi` is the headline (a conservative
     multiple of the Schoenfeld-type `1/(8 pi)` shape quoted from memory, not
     re-derived here); `c = 1/(8 pi)` is also printed. The measured maximum of
     `|sum chi(p) ln p| / (sqrt(x) ln^2(|D| x))` over the checkpoints is
     reported next to the assumption.
   * PROVED: nothing about the size of `h(D_K)` beyond the exact items below.
3. **Cohen / theta series** `L(1,chi) = sum chi(n) e^{-pi n^2/|D|}/n + (pi /
   sqrt|D|) sum chi(n) erfc(n sqrt(pi/|D|))` (derived from the functional
   equation; converges absolutely, error bounded by the first neglected term).
   Used as an independent check on small `D`, and to state its cost at
   `|D_K|`: it needs about `sqrt(|D|)` terms, so it is not runnable here.
4. **Genus theory, exact.** `t` = number of distinct primes dividing `D`;
   2-rank of `Cl(D) = t - 1`; so `2^(t-1) | h`. Then the **Redei 4-rank**
   (exact from the prime discriminants' Kronecker symbols), which fixes
   whether the 2-part is elementary abelian.
5. **Feasibility** of an exact `h`: assessment with stated operation counts and
   what would be required; no exact value is claimed.

# Controls (before the data)

* Positive control: known `h` for `D = -23, -47, -71, -163, -3, -4, -8, ...`
  and exact counts of reduced forms for every fundamental `D` with
  `|D| < 3000`; the Euler product (B = 10^6), the theta series (to 1e-9) and
  the genus-theory 2-rank must match.
* Coverage control: for 400 fundamental `D` near `|D| ~ 10^6..10^7` (exact `h`
  by form counting) the exact `h` must lie in the GRH-conditional interval for
  every `D` and the heuristic 3-sigma interval for at least 99% of `D`. If the
  coverage fails, the interval for `D_K` is withdrawn and the failure
  reported.
* Redei control: for every fundamental `D` with `|D| < 3000`, 4-rank 0 implies
  `v2(h) = t - 1`, and 4-rank `> 0` implies `v2(h) >= t`.

# Acceptance

All unit tests pass; the controls pass; the report states `h(D_K)` as an
interval with each assumption named, the exact 2-rank and Redei 4-rank, and an
explicit verdict on exact computability. Nothing here is an ECDLP cost; no
scoreboard row, rho comparison or speedup is claimed (AGENTS.md section 7a).
