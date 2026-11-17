# Pre-registration: does k\* stop growing at ℓ = 9? (m = 3, cell (3, 25, 9))

Frozen by the commit that adds this file, before any ℓ = 9 draw is generated and before any
closure runs on the new engine path. Class: **stage diagnostic** (exact algebra on frozen
draws). No `S`, no rho ratio, no speed claim. Background and recorded k\* values:
[`SURVEY.md`](SURVEY.md).

## Question

The registered model `K_marginal`, `k = m(ℓ−7)+1`, predicts k\* = 7 at ℓ = 9. The
guess-and-determine route to the priced ECC2K-130 cell survives only if k\* saturates.
Do **five** guesses of the best measured kind hold the MutantXL closure at degree 4 at
ℓ = 9?

## Instrument

- **Tool.** `examples/dreg_linear_guess.rs` with the new `--engine pe --batch 64`: the
  same degree-D MutantXL closure, computed by `pe_closure_with` (partial enlargement), as
  used for C1.
- **Why pe.** The dense scan does not fit the cell in this 16 GB container.
- **Host.** This container: 4 vCPU, 16 GB, not isolated. Closure outcomes are exact
  algebra; times are indicative only.
- **Binary.** One `--release` build. Its SHA-256 is recorded in `runs/binary.sha256`.

## Gate A: engine agreement, before any ℓ = 9 run

The pe path must reproduce committed dense verdicts on ℓ = 8 draws, with the same
specialisation, at degree 4. Two committed outcomes are replayed:

| id | draw file | draw | guesses | committed dense verdict | source |
|---|---|--:|---|---|---|
| A1 | `research/dreg_closure_guess_20261022/draws/m3-22-8.jsonl` | 0 | `--fix 7=0,15=0,23=0,24=0` | refutes at 4 (124,314 columns) | `research/dreg_closure_guess_20261022/runs/m3-22-8.d0.G4b0.jsonl` |
| A2 | `research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl` | 0 | `--sum 15=7+0 --fix 23=0,24=1` (S3 b4) | saturates at 4, no refutation, closure rank 131,588 of 136,698 | `research/dreg_invariant_guess_20261029/runs/S3.d0.b4.jsonl` |

- **Pass:** both verdicts match. For A2 the pe basis rank at saturation (`closure_rank`) also equals 131,588: the closure is the same row space whichever engine computes it.
- **Fail:** the engine path is wrong and nothing at ℓ = 9 is read.

## Draws

- **Generator:** `examples/dreg_ladder --cells 25:9:4 --m 3 --ffd-max 3 --seed 20261106
  --max-draws 64`, written to `draws/m3-25-9.jsonl`.
- **Registered set:** the first 4 draws with zero solutions. The surplus is S = 25 − 27 =
  −2, the same as the (3, 22, 8) and (3, 19, 7) cells. `n_vars` = 27 + 25 = 52.

## Arms (degree 4, `--to 4`, `--max-cols 230000`, `--check`)

- **S5: the arm.** It extends the best ℓ = 8 set, S3 (one invariant coordinate plus the
  top of x₃ plus e₁[0]), by the next invariant and the next e₁ coordinate. Variable
  layout: x₁ = 0..8, x₂ = 9..17, x₃ = 18..26, e₁ = 27..51. The guesses:
  - `--sum 17=8+a,16=7+b`: the top two coordinates of s = x₁ + x₂;
  - `--fix 26=c,27=d,28=e`: x₃ top, e₁[0] and e₁[1].

  The 32 branches are `b = a + 2b + 4c + 8d + 16e`, run in order 0..31.
- **P: positive control.** One branch, `--fix 6=0,7=0,8=0,15=0,16=0,17=0,24=0,25=0,26=0`.
  This leaves summands of dimension (6, 6, 6) at n = 25, the analogue of the ℓ = 8 study's
  G6. It is expected to refute at degree 4.
- **Fail-fast.** For each draw, S5 stops at the first branch whose degree-4 entry is
  saturated and not refuted. That draw then does not suffice. A capped or killed branch is
  unmeasured: it is recorded as such, and the draw is undecided unless another branch
  fails.

## Decision rule

- **Controls valid** when Gate A passes, P refutes on ≥ 3 of 4 draws, and every
  `identity_check` is true.
- If valid:
  - **"k\* ≤ 5 at ℓ = 9 (saturation reading)"** if S5 suffices (all 32 branches refute)
    on ≥ 3 of the 4 draws. This contradicts `K_marginal`.
  - **"S5 does not suffice (consistent with K_marginal)"** if S5 fails on ≥ 3 of 4.
  - **Mixed** otherwise.
- **Scope of a "does not suffice" reading.** It is about this guess set only. ℓ = 8
  showed that the choice of guesses matters (invariant 7 of 12 against raw 1 of 12 at
  k = 3).

## Predictions (stated before data)

- Gate A passes: high confidence.
- P refutes: high confidence.
- S5: "does not suffice" on ≥ 3 of 4, confidence 0.7, following `K_marginal`.

## Budget and stop

- Each process runs under `prlimit --as=15000000000` and `timeout 7200`.
- About 12 hours of container time at most. Above that, the remaining branches are
  reported as not run.

## Amendment 1 (2026-11-06, before any S5 branch other than d1.b0 was scored; owner chose the cloud route)

- **Recorded before this amendment.** S5.d1.b0 refutes at degree 4 (`runs/S5.d1.b0.jsonl`:
  195,709 columns, 2,479 s, 4.7 GB). All four positive controls refute at degree 4, and
  Gate A passed. No other S5 branch has a record.
- **Host.** The remaining S5 branches run on one RunPod CPU pod (cpu3g-64-256: 64 vCPU,
  256 GB). It uses the launcher of `research/koblitz_ic_leads_build_20261024/pod/` and the
  same `dreg_linear_guess` binary, uploaded and checked by SHA-256.
  - **Why.** The 16 GB container's session-tracked tasks stop at 2 h, and up to 31 branches
    × 41 min per draw do not fit.
  - **Why not isolab-eks.** Its image carries only `ecbench` (see SURVEY §3).
  - **Cost cap.** $30. The pod's lifetime is capped at 16,200 s, which is about $11.50
    at $2.56/h.
- **All 32 branches of every draw run, in parallel** (≈30 at a time, `RAYON_NUM_THREADS=2`
  each). The registered fail-fast was a cost rule, not part of the verdict.
- **The decision rule is unchanged.** A draw suffices iff all 32 branches refute at degree
  4, and the verdict thresholds are as above. A capped or killed branch stays unmeasured.
- **Added reading (exploratory, outside the decision rule).** The per-draw count of
  refuting branches out of 32.
