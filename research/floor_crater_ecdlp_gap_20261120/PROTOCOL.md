# Protocol: floor vs crater ECDLP over a large prime conductor gap

Frozen 2026-11-20, before new Lean edits and before new experiment runs.
Builds on `research/conductor_gap_cm_pairs_20261010/` (pairs + ecbench) and
`research/conductor_prime_gap_bounds_20261119/` (Lean gap orthogonality).

## Question (two quantities)

For crater (`f_E = 1`) and floor (`f_E = ℓ`) curves over the same `GF(q)`
with the same prime order `N`, same trace `t`, and
`t² − 4q = −19 · ℓ²` (`ℓ` prime):

1. **Solver ratio.** For each implemented method `M`,
   `S(M) = cost_M(floor) / cost_M(crater)`.
2. **Bridge ceiling.** For always-correct solvers, the Lean transfer bound
   says best costs differ by at most `2 · C_eval(φ)` where `φ` is a
   degree-`ℓ` isogeny between the levels. Separately, conductor orthogonality
   forces any conductor-valuation bridge to include a step with `ℓ ∣ d` and
   `ℓ ≤ d`. Let `C_bridge(ℓ)` be a counted lower/upper accounting of that
   step (Vélu evaluation cost in field operations, or a kernel-search proxy
   when Φ_ℓ is unavailable).

The ECDLP *difference that this study claims* is the bridge ceiling, not a
claim that Pollard rho is harder on the floor. Prior sessions
(`conductor_gap_cm_pairs_20261010`, 2026-10-10) found no reproducible
solver conductor effect for implemented methods; that remains the null for
`S(M)`.

## Pre-registered predictions

- **Lean (always-correct solvers).** With mutual degree-`ℓ` isogenies both
  ways, `|bestCost(floor) − bestCost(crater)| ≤ 2 · max(C_eval(φ), C_eval(ψ))`
  in the abstract `TransferBound` cost model. A conductor valuation gap
  forces some step with `ℓ ≤ d`. These are theorems, not measurements.
- **`bsgs.*`:** `S = 1` within bootstrap (theorem: generic invariance).
- **`rho.*` / `kangaroo.*`:** expected `S = 1`, not a theorem
  (representation-hashing). Re-test `rho` at `ℓ = 8209` under a fresh
  measurement seed (open follow-up from 2026-10).
- **`ic.pipeline`:** expected `S = 1` (prior 16-target sessions).
- **Bridge accounting.** `C_bridge(ℓ)` grows at least linearly in `ℓ` under
  classical Vélu evaluation of a known kernel (and worse if the kernel must
  be found). Compare `C_bridge(ℓ)` to a single rho cost on the same `N`
  (`Θ(√N)` group ops). Prediction: for the study's ladder
  (`ℓ ∈ {263, 8209, 131101}` and any larger constructible `ℓ`),
  `C_bridge(ℓ) / cost_rho(crater)` increases with `ℓ` while `S(rho)` stays
  consistent with 1.

## Decision rule

- A method "sees the conductor in ECDLP cost" only if two independent `ℓ`
  values have bootstrap intervals for `log₂ S` excluding 0 with the same
  sign, and the A/A control includes 0 (same rule as
  `conductor_gap_cm_pairs_20261010/PROTOCOL.md`).
- The study *establishes a floor/crater difference* when:
  1. Lean transfer + gap-necessity theorems pass `proof-gate.sh`, and
  2. Bridge accounting shows `C_bridge(ℓ)` is the only cost component that
     scales with the conductor gap while solver `S` does not, on at least
     two `ℓ` values with audit `OK`.

## What is not claimed

- Unconditional ECDLP hardness lower bounds.
- That floor ECDLP is asymptotically harder than crater ECDLP for generic
  walks.
- Robert Remark 4.5 (`poly(log ℓ)` volcano navigation): still open; if
  true, it would reduce `C_bridge`, not create a solver-side conductor
  effect.

## Specs and arms

Reuse certified pairs in
`research/conductor_gap_cm_pairs_20261010/results/ell_*.txt` and
`ecbench` `CurveSpec::ConductorGap`. New work: rho re-test seed, bridge
cost harness (native Rust; no Python crypto), Lean module under `lean/`.
