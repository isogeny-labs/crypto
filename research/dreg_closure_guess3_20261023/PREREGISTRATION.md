# Pre-registration: do three guessed variables suffice at ℓ = 8?

Registered 2026-10-23, before any G3 degree-4 reading. Successor of
`research/dreg_closure_guess_20261022/` (RESULTS.md there).

## Why

- **Previous study.** m = 3 chained S3 decomposition systems at cell
  (n, ℓ) = (22, 8): N = 46 unknowns, 44 equations. On the four frozen
  unsatisfiable draws (0, 3, 5, 6) the four-variable guess G4 (variables 7,
  15, 23, 24, all 16 branches) made the degree-4 MutantXL closure refute on 64
  of 64 branches, and the dimension-6 control G6 (6, 7, 14, 15, 22, 23 fixed
  to 0) refuted on 4 of 4. Four was the fewest guesses measurable under the
  then cap of 130,000 columns.
- **One cell cannot decide whether the guess count is bounded.** The guess
  count was about 1 at ℓ = 7 and is at most 4 at ℓ = 8. Whether it grows with
  ℓ (killing the guess-then-degree-4 route) or stays flat needs a tighter upper
  bound at ℓ = 8. Three guesses give N = 43 and `C(43, ≤4)` = 136,698 columns,
  which the cap of 140,000 admits. Memory is the open risk (below).

## Hypotheses and predictions

- **H_flat:** three guesses suffice at ℓ = 8 (k ≤ 3) for this guess set.
  Prediction: all 8 branches refute at degree ≤ 4 on at least 3 of 4 draws.
- **H_grow:** three do not suffice for this set. Prediction: on at least 3 of 4
  draws some branch saturates degree 4 without refuting.
- No strong prior (about 50/50). G4 refuted everywhere, but dropping one
  intermediate coordinate is exactly the step that ℓ = 7 could not afford on
  every draw.

## Arms (per draw, each branch a separate process)

Summand i has coordinates 8i … 8i+7; the intermediate point's coordinates
start at 24.

| arm | variables fixed | branches | N after | columns `C(N, ≤4)` |
|---|---|--:|--:|--:|
| G3 | 7, 15, 23 (top coordinate of each summand; no intermediate) | 8, `G3b<B>`, bit j of B is the value of the j-th variable in the order 7, 15, 23 | 43 | 136,698 |
| G6 (control) | 6, 7, 14, 15, 22, 23, all 0 | 1 | 40 | 102,091 |
| N7 (control) | none: unspecialised `research/dreg_closure_scale_20261016/draws/m3-19-7.jsonl`, draw 0, no `--fix`, `--max-cols 130000` | 1 | 40 | 102,091 |

G3 and G6 run as `prlimit --as=15000000000 timeout 7200 dreg_closure --to 4
--max-cols 140000 --draw K --fix … research/dreg_closure_guess_20261022/draws/m3-22-8.jsonl`.
Every specialisation of an unsatisfiable system is unsatisfiable, so every
branch is a well-posed refutation problem.

## Controls

- **N7 (instrument identity and negative control).** Must reproduce the
  committed record `research/dreg_closure_scale_20261016/runs/fresh-m3-19-7.d0.jsonl`
  exactly in its closure outcome and counts: closure not refuting at degrees 3
  and 4, closure ranks 2027 and 94230, columns 7224 and 102091 (XL ranks 797
  and 16136), AND saturate degree 4 without refuting. Timings may differ.
  `dreg_closure` was rebuilt after a rustfmt-only change relative to the binary
  of the previous study (hashes in `runs/binary.sha256`); N7 is exactly the
  check that this changed nothing.
- **G6 (positive control).** Refutes on at least 3 of the 4 draws.
- Controls are valid only if both hold; otherwise the study is uninformative.

## Verdict rules

Per run: refutes at 4 (closure refutes at degree 3 or 4), does not (degree 4
saturated without `1`), or unmeasured (see memory rule).

Per draw: three suffice if all 8 branches refute; three do not suffice if any
branch saturates degree 4 without refuting; otherwise incomplete.

Study (controls valid):
- at least 3 of 4 draws suffice: "three guesses suffice at ℓ = 8 for this guess
  set (k ≤ 3)" (H_flat);
- at least 3 of 4 draws fail: "three do not suffice for this guess set"
  (H_grow);
- otherwise mixed.

Controls not valid: uninformative, every run shown. Scored natively by
`dreg_score guess --k 3 runs/m3-22-8.*.jsonl` (the control N7 file is not
named in that pattern and is checked separately).

## Memory rule

The previous 124,314-column runs peaked at 10.6 GB RSS under
`prlimit --as=15000000000` on a 16 GB container; 136,698 columns may hit the
ceiling. A run that is killed, exits non-zero or leaves an empty output is
UNMEASURED and never negative evidence. Order: the pilot G3b0 on draw 0 runs
first; if it dies, the study stops and reports the memory ceiling. The cap is
not changed without a written amendment.

## Instrument, order, early stop

- `run.sh CLOSURE_BIN control`: N7 first, then G6 on draws 0, 3, 5, 6.
- `run.sh CLOSURE_BIN measure`: G3b0 on draw 0 (pilot), then branch-major over
  draws 0, 3, 5, 6 for branches 0..7.
- One process per run; non-empty output is skipped on restart; a draw's
  remaining branches are skipped once one saturates degree 4 without refuting;
  `MAX_RUNS=N` caps runs per invocation (prints `BATCH_LIMIT`); `ALL_DONE` is
  printed when a step has nothing left.
- `runs/binary.sha256` records the sha256 of `dreg_closure` and `dreg_score`,
  taken before the first run.

## Scope and class

- Class: stage diagnostic. No S, no rho ratio, no speed claim.
- One guess set, the same four draws, one cell, a 4-core 16 GB container that
  is not isolated. A failure says this set does not suffice, not that no
  three-variable set does. k ≤ 3 would still be an upper bound on the guess
  count. The unspecialised ℓ = 8 system is not measured.
- Conductor is not installed in this container, so no task was taken.

## Amendments

Allowed only before any G3 degree-4 reading; additive afterwards.
