# Curve25519-family algebraic index calculus: no usable PKM tower

- **Status:** `open`
- **Kind:** proved obstruction (multiplicative Kummer tower) and implementation
  gap (isogeny-tower Semaev oracle)
- **Affected work:** [ratchet round 1, Part 4](../research/curve25519_ic_ratchet_20261102/README.md);
  the T₀-closed algebraic base proposed in
  [`curve25519_t0_symmetry_20261102`](../research/curve25519_t0_symmetry_20261102/README.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-11-02

## Problem

We cannot yet measure an algebraic-track (summation polynomial plus
factor-base constraint, solved by F4) index calculus on Curve25519-structured
curves at a useful base size. The lab's only prime-field algebraic oracle,
`pkm-semaev`, requires the Kummer base `x ∈ g·μ_{2^t}`, which needs
`2^t | p − 1`. Every Curve25519-family prime has `p ≡ 5 (mod 8)`, so
`v₂(p − 1) = 2` and `t ≤ 2`: at most 4 abscissae.

## Evidence and uncertainty

- **Observed:** `research/curve25519_ic_ratchet_20261102/sessions/pkm-smoke-k20`
  (k = 20 miniature, one target, `ecbench verify --replay 4` OK). With
  `prime-kummer:t=2` the base has 3 columns; `pkm-semaev:m=2,mode=full` costs
  S = 1027 / 49,513 / 485,893 and `m=3,mode=chain` costs S = 211 / 6.18·10⁶
  / 6.18·10⁷ at w = 0 / 0.1 / 1. The generic incumbent costs S = 9.05 / 9.46
  / 13.16 on the same target.
- **Derived or proved:** `p ≡ 5 (mod 8)` ⇒ `v₂(p − 1) = 2`. For Curve25519,
  `p − 1 = 2²·3·65147·q` with `q` a 236-bit prime (GNU `factor`). So the
  largest subgroup of `F_p^*` reachable as a composition of power maps of
  degree ≤ 3 has order 12. Multiplicative PKM bases (any smooth order) are
  limited to this, plus one prime-degree step of 65147.
- **Suspected explanation:** not a suspicion. The structure of `p − 1`
  decides it.
- **Missing capability:** an oracle that writes `S_{m+1}` against PKM's
  *isogeny-form* tower (`prime-isogeny:t=T` exists as a base, with zeros of a
  chain of Vélu 2-isogeny x-maps, and needs no condition on p). `f4_fp_tower`
  supports only the squaring rule `y_j² = y_{j+1}`.
- **What this evidence does not establish:** it says nothing about whether an
  isogeny-tower algebraic oracle would beat the generic incumbent. In the
  lab's prime-order studies the algebraic track trails the generic track, so
  the prior is that it would not.

## What would unblock it

- [ ] `pkm-semaev` (or a sibling) accepts `prime-isogeny:t=T`, with the tower
  ring relations `y_{j+1} = φ_j(y_j)` for the degree-2 Vélu x-maps, its
  refutations cross-checked on the grid (`crosscheck=1`), and unit tests on
  a curve with a known decomposition.
- [ ] The same, run on the k = 20 and k = 24 miniatures at `t` with
  `2^t ≈ N_a`, every solve verified, `ecbench verify --replay` OK.

Meeting both makes the comparison possible; it does not predict its outcome.
The T₀-closed variant needs a tower whose zero set is closed under
`x_M ↦ 1/x_M`. `μ_{2^t}` in Montgomery coordinates is closed under
inversion, but it is exactly the object `p ≡ 5 (mod 8)` forbids. An isogeny
tower would need to be chosen compatibly with the involution, which is open.

## Next discriminating step

- **Action:** implement the isogeny-form tower ring in `f4_fp_tower` (Vélu
  x-maps as rewrite rules) behind `pkm-semaev`, with the grid cross-check.
- **Inputs and controls:** the k = 20 miniature; a prime-order control curve
  where both the Kummer and isogeny forms exist (`2^t | p − 1`), so the two
  towers can be compared at equal `t`.
- **Execution limits and stop condition:** `p < 2^31` (engine limit);
  timeouts recorded as such.
- **Outcomes and next actions:** if it costs within 10× of the incumbent
  across the band, run a frozen holdout. If not, record the gap as the
  algebraic track's distance remaining and stop.
- **Recheck trigger:** any change to `f4_fp_tower` adding non-squaring tower
  rules.

## Resolution and history

- 2026-11-02: opened from ratchet round 1, Part 4. Unresolved.
