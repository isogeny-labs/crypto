# Conductor prime-gap bounds (Lean 4)

Companion to the ordinary-vs-supersingular isogeny hardness split along the
horizontal/vertical axis: Bach-smooth rewriting on ordinary craters is
orthogonal to vertical conductor-prime steps at large `ℓ | f₀`.

Lake project: `lean/` (toolchain `leanprover/lean4:v4.35.0`, CryptoLean
`01750b4044989583e70e4886b4ab5a79fe83f128`, Mathlib
`cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`, path deps `deep_gap` →
`generic_invariance` / `TransferBound`).

Claim: PCC-20261120-70fc62 (`standing: unverified` until CI on `main`
signs it). Tag strong prose with `[pcc:unverified]`; do not cite this
claim id with a verified `[pcc:…]` tag while standing is unverified.

## Scope

Formalized:

1. Lattice model of `O_f = ℤ + f O_K` as `Submodule ℤ (ℤ × ℤ)`; mutual
   degree-`d` steps; `padicValNat` orthogonality to smooth / degree-`< ℓ`
   walks; gap ⇒ `ℓ`-power in the degree product and a step with `ℓ ∣ d`
   and `ℓ ≤ d`.
2. Hasse-style conductor bound `3 · f₀² ≤ 4 · q` from
   `t² − 4q = D₀ · f₀²` with `D₀ ≤ −3`; arbitrary-length `IsogenyChain`
   transfer with exact overhead `2 · costSum` on `TransferBound.Solver`;
   linear-Vélu / rho-scale comparison `3 (2 C f₀)² ≤ 16 C² q`.
3. Scalar Frobenius on `(ℤ/ℓ)²`: `ℓ + 1` projective lines, centralizer
   reachability from the line at infinity, adaptive membership decision-list
   query lower bound `ℓ ≤ Q`, cyclic units of order `ℓ − 1`, and the
   scalar-1 arithmetic `ℓ² ∣ 4(q + 1 − t)`.
4. Floor endomorphisms in `ℓ · O` have `quadraticNorm` divisible by `ℓ²`
   (no odd-`ℓ` Kani pure power-of-two norm); Hasse-interval thinness
   `|{t ∈ [-B, B] : t² ≡ a (mod ℓ)}| ≤ 2 (2B/ℓ + 1)`.
5. Kernel Lucas certificates for `263`, `146505763881528721`, `631409`;
   volcano-gap packaging; conductor-1 crater walks have no vertical
   valuation gap.

Explicit unformalized bridges (documented in module headers, **not**
axiomatized):

- Elliptic-curve identification `End(E) ≃ O_f` is not in Mathlib; lattice
  statements apply once that bridge is supplied.
- Damien Robert (2022) Remark 4.5 (`poly(log ℓ)` volcano navigation)
  remains open; this study does not claim a navigation algorithm.

## Proof map

Axioms for every reported theorem (proof-gate / AuditAxioms):
`[propext, Classical.choice, Quot.sound]`. Receipt:
`receipts/research_conductor_prime_gap_bounds_20261119_lean/` (`PASS`,
125 theorems audited, 0 failing, 0 trusted `native_decide`).

| Mathematical result | Lean declaration | File | Exact hypotheses (abbrev.) | Kind |
| --- | --- | --- | --- | --- |
| `O_{f₂} ≤ O_{f₁} ↔ f₁ ∣ f₂` | `quadraticOrderSubmodule_le_iff` | `ConductorOrthogonality.lean` | — | Law |
| `d • O_{f₂} ≤ O_{f₁} ↔ f₁ ∣ d f₂` | `smul_quadraticOrderSubmodule_le_iff` | same | — | Law |
| Coprime step preserves `v_ℓ` | `coprime_step_preserves_padicValNat` | same | `ValidConductorStep`, `¬ℓ ∣ d` | Law |
| Degree `< ℓ` walk preserves `v_ℓ` | `small_degree_walk_preserves_padicValNat` | same | `ConductorWalk`, `∀ d ∈ ds, d < ℓ` | Law |
| Gap forces large step | `conductor_gap_forces_large_step` | same | `v_ℓ(f) ≠ v_ℓ(endpoint)` | Law |
| `3 f₀² ≤ 4 q` | `three_mul_conductor_sq_le_four_q` | `HasseTransferCeiling.lean` | disc identity, `D₀ ≤ -3`, `0 < f₀` | Law |
| Chain transfer overhead `2 · Σ cost` | `chain_solver_transfer` | same | `IsogenyChain`, `Solver` | Law |
| Vertical gap DLP ceiling | `vertical_prime_gap_dlp_ceiling` | same | edge + walk costs | Law |
| Linear Vélu vs rho scale | `linear_velu_gap_is_rho_scale` | same | same disc hypotheses | Law |
| Centralizer reaches every line | `centralizer_transitive_on_projLines` | `ScalarFrobeniusOracle.lean` | `Fact ℓ.Prime` | Law |
| Membership oracle `ℓ ≤ Q` | `scalar_frob_oracle_worst_case_lower_bound` | same | decision list covering all lines | Law |
| Primitive unit order `ℓ − 1` | `exists_scalar_frob_attaining_max_degree` | same | cyclic `(ZMod ℓ)ˣ` | Law |
| Scalar-1 ⇒ `ℓ² ∣ 4(q+1−t)` | `rational_torsion_curve_order_sq_dvd` | same | disc, `ℓ ∣ t−2`, `ℓ ∣ f` | Law |
| Floor norm `ℓ² ∣ N` | `floor_endomorphism_sq_dvd_norm` | `FloorObstructionAndDensity.lean` | `FloorEndomorphism` | Law |
| No odd-`ℓ` Kani 2-power floor | `no_kani_two_power_floor_endomorphism` | same | `Odd ℓ`, norm `= 2^k` | Law |
| Hasse-interval thinness | `hasse_trace_conductor_prime_thinness` | same | filter on `Icc (-B) B` | Law |
| ECC2K-130 big prime | `prime_ecc2k130_big` | `ConductorGapEndpoints.lean` | Lucas/Pratt cert | Instance |
| Gap cert ⇒ large step | `checkVolcanoGapCert_sound` | same | `VolcanoGapCert` | Law |
| ECC2K-130 big gap bounds | `ecc2k130_big_gap_bounds` | same | — | Endpoint |
| Deep-volcano prime `631409` | `deep_volcano_gap_bounds` | same | ties `DeepGap.N` | Endpoint |
| Conductor-1 crater: no `v_ℓ` gap | `p256_crater_no_vertical_gaps` | same | walk `1 → … → 1` | Endpoint |

## lean-instance-gate

1. **Structure above.** `Submodule ℤ (ℤ × ℤ)`; `Submodule (ZMod ℓ) (ZMod ℓ × ZMod ℓ)`;
   `LinearMap` / `LinearEquiv`; `(ZMod ℓ)ˣ`; `CryptoLean.quadraticNorm`;
   `TransferBound.Isogeny` / `Solver`; `DeepGap.Isogeny.comp`;
   `Mathlib.NumberTheory.LucasPrimality.lucas_primality`.
2. **Theorems used as stated.** `padicValNat.mul` / `pow_padicValNat_dvd`;
   `transfer_cost`; `DeepGap.comp_cost` / `twist_order_sq_dvd` pattern;
   `lucas_primality`; `IsCyclic` generator for units; `Finset.card_le_card`.
3. **Objects as instances.** `quadraticOrderSubmodule f`; `ValidConductorStep` /
   `ConductorWalk`; `IsogenyChain`; `projLine`; `LineSearchTree` decision lists;
   `FloorEndomorphism`; `VolcanoGapCert`; Lucas certificates for study primes.
4. **Assembly probe.** Lattice containment ↔ conductor divisibility; mutual
   step ⇒ valuation bound by `v_ℓ(d)`; chain transfer = `2 · Σ cost`; oracle
   covering bound from leaf-cardinality ≤ `Q + 1`.
5. **Missing components only.** Generic walk induction and fuelled `powMod`
   (study-local); Hasse interval residue counting (study-local). Upstream
   candidates: `quadraticOrderSubmodule` laws → `CryptoLean.Curve.Conductor`;
   fuelled Lucas checker → `CryptoLeanCore`.
6. **Inheritance table.**

| Operation / law | Inherited by |
| --- | --- |
| Submodule add/smul | `quadraticOrderSubmodule`, `smulQuadraticOrder` |
| `padicValNat` multiplicativity | Mathlib, used as stated |
| `transfer` / `Solver.correct` | `TransferBound`, used as stated |
| `Isogeny.comp` | `DeepGap`, folded into `IsogenyChain.compose` |
| `quadraticNorm` | `CryptoLean.Curve.Conductor` |
| `(ZMod ℓ)ˣ` cyclic / `orderOf` | Mathlib |
| `lucas_primality` | Mathlib, discharged by `pratt` |

Statement classification: laws live in the five modules; Lucas primes and
`ecc2k130_*` / `deep_volcano_gap_bounds` are instance obligations / endpoints.

## Literature & Knowledge Base

`scripts/research-kb status`: `ready: true`, revision
`d47bbc4827ae4884b5df841f6110b2f2`, last refresh `2026-11-20T01:04:31Z`
(UTC; `2026-11-19 17:04:31 PST`).

| Source | Role |
| --- | --- |
| `source:61978a6dbd7332419587ebcddfa6e562` (`crypto:docs/ic/RESEARCH_ISOGENY_CONDUCTOR_GAP_20261007.md`) | ECC2K-130 volcano profile `f = 263 · 146505763881528721` |
| `source:0a8d21e5814d34be9ffe9c1ff14fabdb` (`crypto-lean:CryptoLean/Curve/Conductor.lean`) | `quadraticNorm`, `conductor_gain` |
| `source:4056891b622a5aeba5003c9cd86b3bf6` (autoresearcher volcano citations) | Galbraith / Kohel / Sutherland volcano references |

Primary sources read at source: Damien Robert (2022) Thm 4.2 & Remark 4.5;
Galbraith–Gilchrist–Robert (2025) ascending volcanoes; repo baselines
`TransferBound.lean`, `DeepGap.lean`.

## Verification

```bash
export PATH="$HOME/.elan/bin:/opt/homebrew/bin:$PATH"   # Homebrew bash before /bin/bash
tools/lean/proof-gate.sh --out research/conductor_prime_gap_bounds_20261119/receipts \
  research/conductor_prime_gap_bounds_20261119/lean
python3 tools/pcc/pcc.py ledger verify --base origin/main
python3 tools/pcc/pcc.py standing PCC-20261120-70fc62
```

Proof-gate line: `PASS  125 thm   0 trusted  research/conductor_prime_gap_bounds_20261119/lean`.
