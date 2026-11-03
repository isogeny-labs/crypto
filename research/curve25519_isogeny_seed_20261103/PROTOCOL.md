# Protocol: tower-seed selection for the PKM isogeny base (T4), frozen 2026-11-03

Frozen in git before any census or engine count. It comes from the lead list in
`research/curve25519_torsion_leads_20261103/LEADS.md`, whose lesson was that
selection among parameterisations is free while algebraic restriction is not.

## Lead

- `prime-isogeny:t=T,seed=S` builds V = x(R' + ⟨Q⟩) on an auxiliary curve and
  a Vélu 2-isogeny chain. The base is every point of the target curve whose
  x is in V.
- The seed picks the auxiliary curve, Q and R'. The F4 system has the same
  shape for every seed: t tower variables per summand and the same rule
  structure.
- So the seed is a free parameter, like a frame. The incumbent algebraic arm
  `pkm-iso-t6-g4` (`curve25519_algebraic_ecbench_20261102`, k = 20) uses
  seed 0.

## Design

- **Census.** `curve25519_family iso-census <k> <t> 0 2000` (code at the
  freeze commit). Curves: family-rule miniatures k ∈ {16, 18, 20, 22, 24};
  t ∈ {4, 5, 6, 7}; seeds 0 … 1999. Seed 0 is the incumbent. For each seed:
  columns, exact m = 2 subgroup coverage (`covered_points`, the engine's
  `coverage_exact` numerator), and the log-free selector J (signed pairs whose
  [r]-images cancel). Seeds whose tower fails to build are skipped and counted.
- **Selection.** argmax J, ties by columns, then the lowest seed. The oracle
  is argmax covered.
- **Engine (k = 20, the incumbent's configuration t = 6, G = 4).**
  `f4cost 20 isogeny@S* 6 4 100 2026111506 900 2 full 0 30000000` against the
  same command with `isogeny` (seed 0).
- **Search cost**, priced and reported:
  - per seed, the tower build's group operations (`build_group_ops`) plus the
    selector's [r]P and pair additions (`selector_group_ops`);
  - total = the sum over the K seeds searched;
  - reported both as charged to one target and as a per-curve precomputation
    (amortised). Only the charged figure is a single-target cost.

## Predictions

- **R1.** Selected ÷ mean coverage (over the seeds that built) ≥ 1.5 in all 15
  cells with t ∈ {5, 6, 7}.
- **R2.** Selected coverage ≥ 0.9 × the oracle-best in all 20 cells.
- **R3.** At k = 20, t = 6 (the incumbent cell), selected ÷ seed-0 coverage
  ≥ 1.5.
- **R4 (engine).** For S* against seed 0:
  - `coverage_exact` equals the census;
  - `solving_degree_max` and `poly_terms_per_specialisation` are equal;
  - mul-adds/target are within ±10%.

  Hence the F4 mul-adds per relation (derived) fall by about the coverage
  ratio.

## Scope

- This is a component result. The end-to-end ecbench re-pricing is T5. It
  must charge the search, and at k = 20 the search is expected to dominate a
  single solve.
- Rho remains ahead on every track. The distance remaining is reported, not
  gated.

## Addendum T5 (frozen before its session): end-to-end ecbench, k = 20

Census and R1–R4 are done (R1, R2, R3 and R4 held). At the incumbent cell,
seed 1093 (log-free selected) covers 528 points against seed 0's 172. In the
engine its F4 mul-adds/target are 0.991×, with the same degree (5) and terms
(16).

- **Session.** `ecbench/spec.json`:
  - same curve and arms as `curve25519_algebraic_ecbench_20261102`
    (`rho-neg`, `proj-a`, `pkm-iso-t6-g4` at seed 0), plus
    `pkm-iso-t6-g4-s1093` (`prime-isogeny:t=6,seed=1093`, same oracle);
  - fresh `target_seed` 2026110340, 6 public targets, 1 round;
  - full solves with the DLP verified by ecbench;
  - S at w = 0, 0.1 and 1 from `ecbench band`.
- **V1.** The s1093 arm verifies 6/6.
- **V2.** Mean S of s1093 ÷ the seed-0 arm is ≤ 0.6 at w = 0.1 and at w = 1
  (≈ 1/3 predicted from the coverage ratio).
- **V3.** Mean S of s1093 < the seed-0 arm at w = 0.
- **V4 (scope).** Both algebraic arms stay behind `proj-a` and `rho-neg` at
  w = 0.1 and 1. A change of band order against `proj-a` at w = 0 is
  reported, but no ranking is claimed from w = 0 alone.
- **Search cost, charged separately.** ecbench does not see the seed search.
  The K = 2000 search at k = 20, t = 6 cost 1.35·10⁷ group operations
  (`analysis/summary.json`). That is about 3.7·10⁴ in S (÷ √r = 362.2),
  which dominates one solve. The s1093 arm's S is therefore reported twice:
  without the search, as an amortised per-curve precomputation, and with it
  charged to the single target.
