# Decomposition cost on conjugate-pair lines: the same per solve, but the standard solver fails on one curve's decomposable targets

2026-11-06. This follows
[`lift_dense_conjugate_pair_cost_20261106`](../lift_dense_conjugate_pair_cost_20261106/README.md),
whose cost model **assumed** that one decomposition trial costs the same on
a conjugate-pair line as on F_p. Here that assumption is measured with the
repository's own counted k = 4 Gaudry solver (`src/cryptanalysis/gaudry_quartic.rs`,
§11.17 of `research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md`), called
unchanged.

- **Protocol.** Frozen before any data at `5387a0bd9c`. Amendment A1 was
  registered before its own data at `e42f258bcb` ([`PROTOCOL.md`](PROTOCOL.md)).
- **Instrument.** [`examples/conjpair_decomp_cost.rs`](../../examples/conjpair_decomp_cost.rs),
  at `f6b12d33c6` for the main run.
- **Host.** This container, 4 cores and 15 GB. Iso Lab EKS needs an AWS
  identity this container does not have. The metric is a deterministic
  count of F_p multiplications, and A/A confirms that.

Every statement below is an unverified assertion of this session
[pcc:unverified]. The claim record is at the end.

## Method

- **Setup.** p = 29, K = F_{29⁴}, and 2 curves with full 2-torsion and
  j ∉ F_{p²} [pcc:unverified].
- **Arms.** On each curve:
  - `conj`: the first all-lift conjugate-pair line, x = x₀ + u·t;
  - `std`: the line F_p.
- **Building H.** For each line, H(e(t), X) = S₅(x₀ + u t₁, …, X) is built
  by the same interpolation `SymmetrisedS5::precompute` uses, and checked
  against fresh resultant evaluations. On both curves the std arm's H
  equals `precompute`'s exactly, which is the positive control.
- **Solve.** Each target is solved with `weil_restrict` and then
  `solve_system4_with` (chains and sparsest pivots), Macaulay degree 29.
  Every solve is checked against an exact meet-in-the-middle decomposition
  over the line's factor base.
- **Targets.** Per arm: one planted target (a sum of 4 points of that arm's
  base) and 2 random points of E(K) shared by both arms.

## Results (main run, Macaulay degree 29)

| curve | target | conj: dim, F_p muls, solver/oracle | std: dim, F_p muls, solver/oracle |
|--:|:--|:--|:--|
| 1 | planted | **4,090**, 1.040·10¹¹, **0/3 FAILED** | 4,096, 1.152·10¹¹, 1/1 |
| 1 | common 0 | **4,090**, 1.042·10¹¹, **0/2 FAILED** | 4,096, 1.125·10¹¹, 0/0 |
| 1 | common 1 | 4,096, 1.159·10¹¹, 0/0 | 4,096, 1.131·10¹¹, 0/0 |
| 2 | planted | 4,096, 1.193·10¹¹, 3/3 | 4,096, 1.132·10¹¹, 1/1 |
| 2 | common 0 | 4,096, 1.136·10¹¹, 0/0 | 4,096, 1.157·10¹¹, 0/0 |
| 2 | common 1 | 4,096, 1.163·10¹¹, 0/0 | 4,096, 1.135·10¹¹, 0/0 |

- **When the solve works, it costs the same** [pcc:unverified].
  - The Macaulay matrix is the same, 41,780 × 40,920 with rank 36,824, on
    every solve.
  - conj/std on the shared targets where both solves are correct: 1.025,
    0.982 and 1.025.
  - The median over all solves is 1.012.
  - The assumption of the cost study holds wherever the solver returns
    correctly: one conjugate-pair decomposition trial costs what an F_p
    trial costs, about 1.1·10¹¹ F_p multiplications at this size.
- **On curve 1 the solver fails exactly where it matters.**
  - On both of curve 1's conj targets that decompose, the quotient has
    dimension **4,090, not the Bézout 4,096**, and the solver returns
    nothing. The oracle finds 3 and 2 decompositions there.
  - The conj target on curve 1 with no decomposition, and every target on
    curve 2, have dimension 4,096 and solve correctly. That includes curve
    2's planted target, where all 3 decompositions are found.
  - The std arm solves every target on both curves.
- **Raising the Macaulay degree does not help** (Amendment A1). The deficit
  is identical at degrees 30, 31 and 32 on both failing targets, while the
  cost grows to 2.57·10¹¹:

  | curve | target | degree | matrix | dim | F_p muls | solver/oracle |
  |--:|:--|--:|:--|--:|--:|--:|
  | 1 | planted | 30 | 48,580 × 46,376 | 4,090 | 1.439·10¹¹ | 0/3 |
  | 1 | planted | 31 | 56,124 × 52,360 | 4,090 | 1.944·10¹¹ | 0/3 |
  | 1 | planted | 32 | 64,458 × 58,905 | 4,090 | 2.570·10¹¹ | 0/3 |
  | 1 | common 0 | 30 | 48,580 × 46,376 | 4,090 | 1.441·10¹¹ | 0/2 |
  | 1 | common 0 | 31 | 56,124 × 52,360 | 4,090 | 1.945·10¹¹ | 0/2 |
  | 1 | common 0 | 32 | 64,458 × 58,905 | 4,090 | 2.572·10¹¹ | 0/2 |
  | 2 | planted | 30 | 48,580 × 46,376 | 4,096 | 1.596·10¹¹ | 3/3 |
  | 2 | planted | 31 | 56,124 × 52,360 | 4,096 | 2.106·10¹¹ | 3/3 |
  | 2 | planted | 32 | 64,458 × 58,905 | 4,096 | 2.739·10¹¹ | 3/3 |

  On curve 2 [pcc:unverified], the same planted-type target solves at every degree from 29
  to 32, with all 3 decompositions found. A deficit that is stable across
  degrees means curve 1's system itself is not in the generic position the
  solver assumes, not that the Macaulay matrix is too small. Six of the 4,096 solutions over the algebraic closure are
  missing, or sit at infinity or in a degenerate component, on those
  targets.

## What this means for the cost of the family

- **Per trial.** Equal cost, measured, about 1.1·10¹¹ F_p multiplications
  either way, wherever the generic solver applies.
- **Relation phase.** The cost study's 11–17× fewer trials then carries
  over as 11–17× less relation-phase work, but **only on curves where the
  solver handles the conjugate-pair system**.
- **Curve 1.** The degree-29 to degree-32 solver loses every decomposition
  there, so the factor base yields no relations through this solver.
  Recovering them needs a solver that does not assume a zero-dimensional
  system of degree 4,096. That is not built here.
- **Scope.** Two curves at p = 29 only. Whether the degeneracy is rare, as
  1 of 2 might suggest, or tied to a property of the curve (for instance its
  2-descent structure) is open.

No ECDLP cost against rho is claimed, and there is no leaderboard or
benchmark row.

## Registered verdicts

| rule | verdict | detail |
|:--|:--|:--|
| Control: std H = `precompute` | PASS | both curves |
| K1 shape (dim 4,096 everywhere) | **FAIL** | curve 1 conj planted and common 0: dim 4,090 |
| K2 cost ratio on shared targets in [0.9, 1.1] with both correct | **FAIL** | curve 1 common 0: conj solve failed. The other three ratios are 1.025, 0.982 and 1.025. The median over all solves is 1.012 (the median band passes) |
| K3 solver = oracle | **FAIL** | curve 1 conj planted (0/3) and common 0 (0/2) |
| A/A | PASS | repeated solves give identical counts |
| A1 / M1 (first working degree among 30–32) | none on curve 1 | curve 1: no degree ≤ 32 works on either decomposable target. Curve 2 planted: 30 is the first A1 degree that works; it already worked at 29 in the main run |

## Reproduce

```sh
cargo build --release --example conjpair_decomp_cost
./target/release/examples/conjpair_decomp_cost --p 29 --curves 2 --out main.jsonl --aa      # ~53 min
./target/release/examples/conjpair_decomp_cost --p 29 --curves 2 --out a1.jsonl \
    --only-arm conj --only-decomposable --macdeg 30,31,32                                   # ~90 min
./target/release/examples/conjpair_decomp_cost --summarize main.jsonl a1.jsonl out/ claimed.json
```

## Claim record

| claim | standing | content |
|:--|:--|:--|
| CLAIM_ID | not covered (unsigned) | per-solve costs, shapes, solver/oracle agreement, A1 degrees ([`validator-report.json`](reports/run-20261106T0643Z/validator-report.json)) |
