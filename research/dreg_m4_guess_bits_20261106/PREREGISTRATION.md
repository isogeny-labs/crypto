# Pre-registration: which of the ten guesses decide degree-3 refutation at m = 4

Frozen 2026-11-06 before any run of this study.

## Question

In the m = 4 per-link study (`research/dreg_m4_guess_bits_20261106` follows
`research/dreg_m4_link_guess_d3_20261106/`, #3571), every per-draw refuting
count is a multiple of 16. That suggests refutation on each draw depends on
only a few of the ten guesses. Guess-then-degree-3 costs 2^k branches, so a
guess that does not affect refutation is a branch doubling with no return.

**Questions:**

- **Q1.** For each arm and draw, which guesses are *essential*: the set E of
  guesses `i` with `f(b) ≠ f(b ⊕ 2^i)` for some branch `b`, where `f` is
  refutation at degree 3?
- **Q2.** Where do the essential guesses sit: link 1 (pair sums or raw x₂),
  link 2 (`e₁ + x₃` or raw x₃) or link 3 (`e₂ + x₄` or raw x₄)?
- **Q3.** For arm I's essential pair-sum guesses, is refutation more frequent
  with the guessed coordinate of `s = x₁ + x₂` at 1 or at 0? #2885's failures
  clustered where the top coordinates of `s` were 0.
- **Q4 (the cost lever).** If only E is guessed and the other guesses are
  dropped, do the branches that refuted with all ten still refute?

## Data seen before freezing

- The per-draw refuting counts of #3571 (published; for example I 512/1024 on
  draw 24). No per-branch pattern, essential set or per-guess count has been
  computed or looked at.
- The #3571 branch records are in the artifact store, which this container
  cannot read. Phase 1 therefore regenerates them.

## Phase 1: regenerate the 16,384 branch records

- The same commands as #3571's `run.sh` (arms I and R, k = 10, degree 3, all
  1024 branches, 8 draws), with the current build (`runs/binary.sha256`).
- **V1:** `dreg_m4_link_score --study dreg_m4_guess_bits_20261106
  --control-from dreg_two_guess_20261106 research` is valid. That study's
  identity control N uses the same binary.
- **V2 (reproduction):** the per-draw refuting counts equal #3571's committed
  `score-output.txt`, line for line.

## Phase 2: guess only the essential set

- For every arm and draw with 1 ≤ |E| ≤ 9, guess only the members of E (same
  variables and values as phase 1); the dropped guesses' variables stay free.
- Run all 2^|E| assignments at degree 3 (`--to 3 --max-cols 140000 --check`).
- Line `e` of `runs/P2.<arm>.d<draw>.jsonl` is the assignment where bit `j` of
  `e` is the value of the `j`-th member of E (ascending).
- Each assignment's refutation is compared with `f` at the same values
  (`f` does not depend on the dropped bits, by the definition of E).
  - *kept:* `f` = 1 and E-only refutes;
  - *lost:* `f` = 1 and E-only does not;
  - *gained:* `f` = 0 and E-only refutes.
- **V3:** every phase-2 record matches its draw and guesses (link sources
  re-derived from the basis), with `identity_check` true.

## Readings (all from `dreg_m4_guess_bits`, natively)

- Q1: |E| and E for each arm and draw.
- Q2: counts of essential guesses by link, per arm.
- Q3: among arm I's essential pair-sum guesses, the number with more refuting
  branches at value 1 than at 0, and the reverse. Descriptive; with this few
  draws no test is registered.
- Q4: the number of arm-draws where guessing only E keeps every refutation
  (lost = 0), out of those tested, and the branch counts 2^|E| against 1024.

## Predictions (before any run)

- Q1: |E| ≤ 6 on most arm-draws, since the counts are multiples of 16
  (confidence 0.7).
- Q2: essential guesses concentrate on link 1 in both arms (0.5).
- Q3: value 1 refutes more often than value 0 for most essential pair-sum
  guesses (0.6).
- Q4: dropping the inessential guesses usually loses refutations, because it
  frees variables that degree 3 cannot absorb. Lost = 0 on at most a quarter of
  the tested arm-draws (0.6).

## What it would mean for ECC2K-130

- If Q4 keeps refutations with few guesses, the useful guesses at m = 4 are a
  small, identifiable subset. That cuts the branch count of the
  guess-then-degree-3 stage on those draws by 2^(10−|E|).
- None of the 10-guess sets suffices on any draw (every branch refuting), so
  this is about which guesses carry the refutations, not about a complete
  guess-and-determine.
- Stage diagnostic: no `S`, no rho ratio, no leaderboard row. Unverified until a
  pcc verdict exists.

## Scope

m = 4, ℓ = 5, n = 11 and 12, 8 draws, the 4-core 16 GB container, not
isolated, run alongside `research/dreg_two_guess_20261106/`. Closure outcomes
are exact; timings are not used.
