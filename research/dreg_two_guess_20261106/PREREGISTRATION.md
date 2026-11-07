# Pre-registration: two guesses at (22, 8), with one invariant guess per link

Frozen 2026-11-06 before any run of this study.

## Question

At m = 3, (22, 8), degree 4, the measured guess sets with three guesses are:

- S3 = {`a₇+b₇`, `c₇`, `e₁[0]`}, which suffices on 7 of 12 draws (#2883);
- L3 = {`a₇+b₇`, `(e₁+x₃)₂₀`, `e₁[0]`}, which suffices on 6 of 12 (#3504).

Raw guesses need four (`research/dreg_guess_scaling_20261106/kstar_recorded.json`).
No two-guess set has been measured at ℓ = 8. The cost of guess-then-degree-4 is
2^k branches, so a two-guess set that suffices halves the branch count.

**Question: with the raw `e₁[0]` guess dropped, does either two-guess set
suffice (all 4 branches refute at degree 4) on any draw, and does keeping one
invariant guess per link (K2L) beat the pair invariant plus raw `c₇` (K2S)?**

## Arms

Branch `b` ∈ 0..3: `c = b & 1`, `h = (b >> 1) & 1`.

- **K2L:** `--sum 15=7+c --link2 20=h`, which is `a₇+b₇` and `(e₁+x₃)₂₀`
  (L3 without `e₁[0]`).
- **K2S:** `--sum 15=7+c --fix 23=h`, which is `a₇+b₇` and `c₇` (S3 without
  `e₁[0]`).
- Both arms leave 44 occurring variables, so degree 4 has 149,986 columns.
  `--max-cols` is raised to 150,000 to build it; nothing else changes from #3504.

## Draws and comparators (all committed)

- **Draws:** the same 12 unsatisfiable (22, 8) draws as #2885 and #3504:
  0, 3, 5, 6 from `research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl`;
  8, 11, 14, 15, 17, 24, 25, 27 from
  `research/dreg_invariant_guess_more_20261029/draws/m3-22-8-more.jsonl`.
- **Comparators (descriptive):** S3's and L3's committed records.
- **Identity control N:** the new build (`runs/binary.sha256`, current main
  with the pe engine; this study uses the dense path) reruns
  `S3.d5.b0` and must equal the committed
  `research/dreg_invariant_guess_20261029/runs/S3.d5.b0.jsonl` on every scan
  field except `secs`.

## Measurement (`run.sh`)

- `dreg_linear_guess --to 4 --max-cols 150000 --check`, dense engine, under
  `prlimit --as=15e9 timeout 7200`, one process at a time, with the F4 caps
  exported as in #3504.
- **Early stop:** an arm stops on a draw at its first branch that saturates
  degree 4 without refuting.
- **Order:** N first, then branch by branch; within a branch, draw by draw;
  within a draw, K2L then K2S.
- Every attempt is kept; an empty earlier attempt moves to `runs/attempts/`.

## Validity

- **V1:** every record's draw and guesses match its name (K2L's sources
  re-derived from the draw's basis), and `identity_check` is true.
- **V2:** N matches.
- **V3:** every draw is decided for both arms.

## Verdict rules

- **Primary (K2L against K2S):** a draw is discordant when exactly one arm
  suffices. Two-sided exact sign test,
  `p = min(1, 2·P(Bin(d, ½) ≥ max(k_L, k_S)))`.
  - **K2L_better** or **K2S_better:** p ≤ 0.05 in that direction.
  - **No detected difference:** anything else, including d = 0.
- **Secondary (the guess count):** for each arm, the number of draws where two
  guesses suffice.
  - Any draw where an arm suffices bounds k\* ≤ 2 on that draw at ℓ = 8 for
    that guess set: its branch count is 4 against S3's 8.
  - **"Two guesses never suffice"** if neither arm suffices on any draw.
- **Descriptive:** suffice counts against S3 and L3 on the same draws, and
  where both fail, which arm fails first.

## Predictions (before any run)

- P(two guesses never suffice) ≈ 0.6. Raw sets need 4 and the best
  three-guess set reaches 7 of 12, so dropping a guess should usually break it.
- P(K2L suffices on ≥ 1 draw) ≈ 0.3, P(K2S suffices on ≥ 1 draw) ≈ 0.3.
- Primary: no detected difference, about 0.85.

## What it would mean for ECC2K-130

- **Two guesses suffice on some draws:** the guess-then-degree-4 branch count
  at ℓ = 8 halves on those draws, a component speed-up of the stage. It is not
  an `S` or a rho ratio.
- **Never:** k\* = 3 is the floor for these guess kinds at ℓ = 8.
- Unverified until a pcc verdict exists.

## Scope

m = 3, one cell, 12 draws, the 4-core 16 GB container, not isolated. Iso Lab
EKS is not reachable from it, and Conductor is not installed.
