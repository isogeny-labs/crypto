# Results: two guesses suffice on 1 of 12 draws at (22, 8), against 6–7 for three

Standing: claim [pcc:PCC-20261107-462bb0], an **unverified assertion** until a pcc
verdict exists (AGENTS.md, "Sabotage is assumed").

- **Scoring.** Natively by `dreg_two_guess_score` (`score-output.txt`) against
  `PREREGISTRATION.md`, frozen before any run. No amendments.
- **Run, 2026-11-06 22:16 UTC to 2026-11-07 01:17 UTC.** 31 processes, all
  exited 0; `runs/attempts/` is empty. Every system has 44 occurring variables
  (149,986 degree-4 columns); each degree-4 scan took 244–484 s.
- **Binary.** `runs/binary.sha256` (current main with the pe engine; the dense
  path is used). The scorer's hash is in `score-binary.sha256`.
- **Host.** The 4-core, 16 GB container, not isolated, alongside the guess-bit
  study. Closure outcomes are exact; timings are indicative only.
- **Report** with graph and PDF: `reports/run-20261106T2216Z/`.

## Validity

- **Identity control N: valid.** The new build's `S3.d5.b0` equals the committed
  record on every scan field except `secs`.
- **Identity and exactness: valid.** Every record matches its draw and guesses
  (K2L's second-link sources re-derived from the basis); `--check` is true on
  every one.
- **Every draw is decided** for both arms.

## The measurement

| draw | K2L: `a₇+b₇`, `(e₁+x₃)₂₀` | K2S: `a₇+b₇`, `c₇` | L3 (3 guesses) | S3 (3 guesses) |
|--:|---|---|---|---|
| 0 | fails at b0 | fails at b0 | fails at b6 | fails at b4 |
| 3 | fails at b0 | fails at b0 | fails at b0 | fails at b0 |
| 5 | fails at b0 | fails at b0 | suffices | suffices |
| **6** | **suffices** | **suffices** | suffices | suffices |
| 8 | fails at b0 | fails at b0 | suffices | suffices |
| 11 | fails at b0 | fails at b0 | suffices | suffices |
| 14 | fails at b0 | fails at b0 | fails at b6 | fails at b2 |
| 15 | fails at b0 | fails at b0 | suffices | suffices |
| 17 | fails at b0 | fails at b0 | fails at b4 | fails at b6 |
| 24 | fails at b0 | fails at b0 | suffices | suffices |
| 25 | fails at b0 | fails at b0 | fails at b2 | fails at b2 |
| 27 | fails at b0 | fails at b0 | fails at b0 | suffices |

- **Primary verdict: no detected difference.** The two arms agree on all 12
  draws (no discordant draw, p = 1.0).
- **Secondary: two guesses suffice on some draws.** Both K2L and K2S suffice on
  draw 6, so k\* ≤ 2 there; on the other 11 draws both fail at branch 0.
- **Predictions.** I gave "two guesses never suffice" 0.6 and each arm
  sufficing on at least one draw 0.3; no detected difference (0.85) held.

## What it does and does not say

- **Dropping `e₁[0]` from the three-guess sets costs 5 of the 6–7 draws they
  covered.** At ℓ = 8, three guesses are the floor for these guess kinds on all
  but one draw.
- **Draw 6 is the exception:** 4 branches suffice there, against 8 for S3. It
  is also the draw whose basis has no vector with bit 21 (#3504's
  pre-registration), a structural outlier.
- **Not shown.** Whether a different two-guess pair (for example one using
  `e₁[0]`) does better; this tested only the two sets obtained by dropping
  `e₁[0]`.

## What this means for ECC2K-130 (derived)

- **No general halving of the branch count at ℓ = 8.** k\* = 3 stays the
  measured floor on 11 of 12 draws for these guess kinds.
- Stage diagnostic: no `S`, no rho ratio, no leaderboard row; rho is not
  reached.

## Successors (not run)

- The two-guess pairs that keep `e₁[0]`: {`a₇+b₇`, `e₁[0]`}, {`(e₁+x₃)₂₀`,
  `e₁[0]`}.
- The guess-bit reading of #3504/#2883's three-guess records: which of the
  three decides refutation on each draw.
