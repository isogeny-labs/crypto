# Results: at m = 4, degree 3, one invariant guess per link is not detectably better than raw guesses (4 against 3 draws)

Standing: claim [pcc:PCC-20261106-6052e8], an **unverified assertion** until a pcc
verdict exists (AGENTS.md, "Sabotage is assumed").

- **Scoring.** Natively scored by `dreg_m4_link_score`
  (`score-output.txt`) against `PREREGISTRATION.md`, frozen before any study
  run.
- **Amendment 1.** The first attempt's runner omitted the F4 caps, so the
  identity control failed. It was stopped and its outputs kept unread in
  `runs/attempts/run1-no-f4-env-20261106/`. The study reran from the start
  with the caps; arms, draws and rules are unchanged.
- **Run, 2026-11-06 16:33 UTC to 17:13 UTC.** 16,384 branch runs plus the
  identity control; every run left a record. 32–34 variables occur after the
  guesses, and each degree-3 closure took 0.002–0.51 s.
- **Binary.** `runs/binary.sha256`. The scorer's hash is in
  `score-binary.sha256`.
- **Host.** The 4-core, 16 GB container, not isolated. Closure outcomes are
  exact algebra; timings are indicative only.
- **Report** with graph and PDF: `reports/run-20261106T1633Z/`.

## Validity

- **Identity control N: valid.** The new build's m = 3 `S3.d5.b0` run equals
  the committed record on every scan field except `secs`.
- **Identity and exactness: valid.** Every record matches its draw, arm and
  branch. Arm I's link coordinates and sources are re-derived from the draw's
  basis by the scorer. `--check` is true on every record.
- **All 1024 branches** of both arms are present on every draw.

## The measurement

Branches whose degree-3 closure refutes, of 1024:

| draw | n | I: one invariant per link | R: raw, same places |
|--:|--:|--:|--:|
| 24 | 12 | **512** | 128 |
| 26 | 12 | 0 | **128** |
| 47 | 12 | 192 | **224** |
| 52 | 12 | **512** | 240 |
| 8 | 11 | 0 | 0 |
| 67 | 11 | **320** | 208 |
| 107 | 11 | 128 | **256** |
| 143 | 11 | **224** | 128 |

- **Pooled:** I refutes on 1,888 of 8,192 branches (23 %), R on 1,312
  (16 %).
- **By draw:** I is higher on 4, R on 3, and both are 0 on draw 8.
- **Registered verdict: no detected difference.** The two-sided exact sign
  test on 7 discordant draws gives p = 1.0.
- **Prediction.** I gave I_better about 0.45 and no detected difference
  about 0.4. The outcome is the second.
- **Descriptive.** Every count is a multiple of 16. Refutation depends on
  only a few of the ten guessed bits on each draw; which bits was not
  registered and is not analysed here.

## What it does and does not say

- **At m = 4 in the degree-3 regime, placing the ten guesses on the links'
  invariants does not reliably help.** I is far ahead on draws 24 and 52 and
  behind on 26 and 107, and the draw-level split is even.
- **The pooled share favours I (23 % against 16 %).** It is not the
  registered test, and two draws carry most of it.
- **Not shown.** That the m = 3 degree-4 three-guess advantage (#2883) fails
  at m = 4. This is a different regime: ten guesses, degree 3.

## What this means for ECC2K-130 (derived)

- **The per-link invariant rule is not established at m = 4** in the regime
  this host can reach. The m = 3 result stays a statement about m = 3.
- **No row moves, and rho is not reached.** Stage diagnostic: there is no
  `S`, no rho ratio, and no benchmark or leaderboard row.

## Successors (not run)

- **The degree-4, three-guess regime at m = 4.** It needs a cell where
  unguessed m = 4 refutes only at degree 5, beyond this host's 16 GB.
- **Which guessed bits decide refutation.** The multiple-of-16 structure
  suggests a few bits per draw do. A pre-registered bit-importance analysis
  could say which, and whether they are invariant or raw.
