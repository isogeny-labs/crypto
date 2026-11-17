# Pre-registration: eight more draws for the invariant-guess comparison at ℓ = 8

Frozen 2026-10-29 before any new draw was generated or measured.

## Question

`research/dreg_invariant_guess_20261029/` (#2851) compared two three-guess
sets on the 4 committed unsatisfiable (22, 8) draws:

- **S3** guesses the swap-invariant coordinate `a₇ + b₇` of `s = x₁ + x₂`;
- **X3** guesses the raw `a₇`;
- both also guess `c₇` and one intermediate coordinate.

S3 sufficed on 2 of 4 draws and X3 on 1 of 4, so the registered verdict was
mixed. Paired by draw, S3 was never worse. **Question: on fresh draws from
the same cell, does S3 suffice on draws where X3 does not, more often than
the reverse?**

## Draws

- Continue the committed seeded sequence:
  `dreg_ladder --cells 22:8:3 --m 3 --ffd-max 3 --seed 20261022 --shard 0/1
  --first-draw 7 --max-draws 40` → `draws/m3-22-8-more.jsonl`.
  - These are the same cell, seed and generator as the committed draws 0–6.
  - The draw index continues at 7.
- **Replay check, before use:** the same command with `--first-draw 0
  --max-draws 7` must reproduce the committed draws 0–6 (`v_basis`, `x_r`,
  `solutions` and `outcome`) of
  `research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl`.
- **The registered set** is the first 8 unsatisfiable draws (exact solution
  count 0) with index ≥ 7, in draw order. Satisfiable draws are recorded but
  not measured.
  - Their solution-holding branches cannot refute, and at this size they ran
    past 16 GB in #2851.
- If fewer than 8 unsatisfiable draws are found in the 40, use all of them
  and report the shortfall.

## Arms, branches, early stop (identical to #2851)

- **Binary.** The same `dreg_linear_guess` build (SHA-256 `8376d13a…8dff`),
  run as `--to 4 --max-cols 140000 --check`.
- **S3:** `--sum 15=7+c --fix 23=f,24=g`.
- **X3:** `--fix 7=c,23=f,24=g`.
- **Branch b:** `c = b & 1`, `f = (b >> 1) & 1`, `g = (b >> 2) & 1`.
- **Early stop:** an (arm, draw) stops at its first branch that saturates
  degree 4 without refuting.
- **Positive control P:** `--fix 7=0,15=0,23=0,24=0` on each registered draw.
- **Exactness:** `--check` on every run.

## Validity

- **V0:** the replay check above passes.
- **V1:** `identity_check` is true on every scored run, and each record's
  draw and guesses match its name.
- **V2:** P refutes at degree 4 on at least 6 of the 8 registered draws.
- **Soundness.** No new satisfiable-draw control is run.
  - A closure refutation is a membership certificate for `1` in the ideal:
    every row is a polynomial combination of the equations.
  - #2851 checked the instrument's refutations against an exact solution
    list.
  - So no refutation here can be wrong unless the exactness check fails, and
    V1 covers that.

## Verdict rules

A *draw* is decided for an arm when every one of its 8 branches refutes at
degree 4 or below (it *suffices*), or when one branch saturates degree 4
without refuting (it *fails*).

- **Primary (the 8 new draws only).** A draw is *discordant* when exactly one
  arm suffices.
  - Let k be the number of discordant draws where S3 is the arm that
    suffices, out of d discordant draws.
  - One-sided exact sign test: `p = P(Bin(d, ½) ≥ k)`.
  - **S3_better:** p ≤ 0.05.
  - **X3_better:** the mirror case, `P(Bin(d, ½) ≥ d − k) ≤ 0.05`.
  - **No detected difference:** anything else, including d = 0.
- **Secondary (descriptive):**
  - the combined 12 draws, with the same test;
  - each arm's suffice count;
  - the #2851 labels on the new draws (an arm suffices if it does on ≥ 6 of
    8, and fails if it does on ≤ 2).
- **Predictions.** From the first 4 draws, where 1 was discordant and favoured
  S3: P(S3_better) ≈ 0.3, P(no detected difference) ≈ 0.65,
  P(X3_better) ≈ 0.05.

## What a result would mean for ECC2K-130

- **S3_better:** guessing invariant coordinates of the summand pair is a
  measured, cheaper guess choice for the guess-then-degree-4 oracle. That is
  a stage-level improvement under the success criterion.
  - It moves no row: the guess count at ℓ = 33 stays far above the
    break-even of 6.
- **No detected difference:** the #2851 direction does not survive fresh
  draws at this power, and invariant guessing at m = 3, ℓ = 8 is not
  established.
- In every case: no `S`, no rho ratio, no leaderboard row.

## Scope

m = 3, one cell, the 4-core 16 GB container, not isolated.

- Iso Lab EKS and the S3 resume checkpoint are not reachable from it.
- Closure outcomes are exact algebra; timings are indicative only.
- Conductor is not installed.
