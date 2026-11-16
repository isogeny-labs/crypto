# Results: at m = 4, the four link-1 guesses never decide degree-3 refutation; one to three link-3 guesses carry it on 7 of 13 arm-draws

Standing: claim [pcc:PCC-20261107-c12257], an **unverified assertion** until a pcc
verdict exists (AGENTS.md, "Sabotage is assumed").

- **Scoring.** Natively by `dreg_m4_guess_bits` (`score-output.txt`) against
  `PREREGISTRATION.md`, frozen before any run. Validity of phase 1 by
  `dreg_m4_link_rescore --study` (`phase1-score.txt`; the run used the same code as a `--study` option in `dreg_m4_link_score.rs`, moved to its own file before merge so that #3571's claim, which pins that source, stays valid; the two differ only in doc comments).
- **Disclosed change after the run:** the analyser gained a report-only Q2 line
  (essential guesses by link). It changes no reading.
- **Run, 2026-11-06 22:18 UTC to 22:55 UTC.** 16,384 phase-1 runs and 424
  phase-2 runs; every run left a record, and `runs/attempts/` is empty.
- **Binary.** `runs/binary.sha256` (the same build as
  `research/dreg_two_guess_20261106/`, whose identity control N it uses).
- **Host.** The 4-core, 16 GB container, not isolated, alongside the two-guess
  study. Closure outcomes are exact; timings are not used.
- **Report** with graph and PDF: `reports/run-20261106T2218Z/`.

## Validity

- **V1: valid.** Every phase-1 record matches its draw, arm and branch, with
  arm I's link guesses re-derived from the basis; the identity control N
  matches; all branches are present.
- **V2: valid.** The per-draw refuting counts reproduce #3571's committed
  `score-output.txt` line for line.
- **V3: valid.** Every phase-2 record matches its draw and its reduced guesses,
  with `identity_check` true.

## The measurement

E is the set of guesses whose value changes refutation on some branch. Guess
slots: link 1 = 0–3 (pair sums in I, raw x₂ in R), link 2 = 4–6
(`e₁ + x₃` in I, raw x₃ in R), link 3 = 7–9 (`e₂ + x₄` in I, raw x₄ in R).

| arm | draw | refuting (of 1024) | E | E only: refuting / 2^E | lost |
|---|--:|--:|---|--:|--:|
| I | 24 | 512 | {8} | 1 / 2 | 0 |
| R | 24 | 128 | {7, 8, 9} | 1 / 8 | 0 |
| I | 26 | 0 | ∅ | — | — |
| R | 26 | 128 | {7, 8, 9} | 1 / 8 | 0 |
| I | 47 | 192 | {4–9} | 8 / 64 | 4 |
| R | 47 | 224 | {4–9} | 10 / 64 | 4 |
| I | 52 | 512 | {8} | 1 / 2 | 0 |
| R | 52 | 240 | {4–9} | 10 / 64 | 5 |
| I | 8 | 0 | ∅ | — | — |
| R | 8 | 0 | ∅ | — | — |
| I | 67 | 320 | {4–9} | 6 / 64 | 14 |
| R | 67 | 208 | {4–9} | 10 / 64 | 3 |
| I | 107 | 128 | {7, 8, 9} | 1 / 8 | 0 |
| R | 107 | 256 | {7, 9} | 1 / 4 | 0 |
| I | 143 | 224 | {4–9} | 8 / 64 | 6 |
| R | 143 | 128 | {7, 8, 9} | 1 / 8 | 0 |

No E-only run refuted where the full ten did not (gained = 0 everywhere).

- **Q1.** |E| is 0 to 6 on every arm-draw, never more. Prediction (|E| ≤ 6 on
  most) held.
- **Q2.** Essential guesses by link: arm I 0, 9, 14; arm R 0, 9, 20. **No link-1
  guess is essential on any arm-draw**, in either arm. Prediction
  (concentration on link 1) failed: it is the opposite.
- **Q3.** No pair-sum guess is essential, so there is nothing to read.
- **Q4.** Guessing only E keeps every refutation on **7 of 13** tested
  arm-draws: all those with |E| ≤ 3. It loses some on all 6 with |E| = 6.
  Prediction (at most a quarter) failed.

## What it does and does not say

- **With all ten guesses in place, the four link-1 guesses never change the
  outcome.** On these draws they multiply the branch count by 16 and refute
  nothing that the other six would not.
- **Where |E| ≤ 3, the refuting branches survive with one to three guesses.**
  Arm I on draws 24 and 52: one link-3 guess (`e₂ + x₄`, slot 8) and 2
  branches, against 1024, keep the one refuting assignment.
- **Not shown.**
  - That link-1 guesses are useless without the link-2 and link-3 guesses: E
    is read with the other nine guesses fixed.
  - Any complete guess-and-determine. No draw refutes on every branch with ten
    guesses, so the non-refuting assignments still need more work.

## What this means for ECC2K-130 (derived)

- **At m = 4, guesses spent on the last link's invariant or raw coordinates do
  the work at degree 3; the first link's do not.** This is the opposite of the
  m = 3 degree-4 picture, where the pair invariant `a₇+b₇` was the useful
  guess (#2883).
- **A guess budget at m = 4 should go to links 2 and 3 first.** The next test
  is a complete guess set built that way.
- Stage diagnostic: no `S`, no rho ratio, no leaderboard row; rho is not
  reached.

## Successors (not run)

- A guess set of only link-2 and link-3 coordinates (six guesses, 64
  branches), at degree 3 and, with the pe engine, at degree 4.
- The same reading at m = 3: whether the guesses that decide refutation at
  degree 4 are also on the last link.
