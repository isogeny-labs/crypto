# Item 01: Gaudry `k = 2` index calculus on `E(F_{p²})` against matched rho

Assessed 2026-10-18 by the assess stage, from `raw/summary.json`
(`blindspot01.v1`, 945 rows from 32 files, card 11) read against the frozen
rule in [`PROTOCOL.md`](PROTOCOL.md). Graph: [`gap_vs_n.svg`](gap_vs_n.svg) /
[`gap_vs_n.png`](gap_vs_n.png), source [`plot_gap.py`](plot_gap.py). PDF:
[`REPORT.pdf`](REPORT.pdf) (source `REPORT.typ`).

## Verdict

**REGRESSES at the constant level; INCONCLUSIVE at the exponent level.** No index-calculus arm
is cheaper than its matched rho at any size from `log₂ #E = 24` to `40`, at any of
the three weights on unpriced native work (0, 0.1, 1 GAE per operation). The
order holds across the whole band in every cell (`band_order`), so the ranking is
admitted by AGENTS.md §5a. Nothing here is an improvement over a baseline, and
no claim is made beyond the toy range.

* **Constant (distance remaining).** At the largest size (`log₂ #E ≈ 40`) the best
  arms, `ic-b` and `ic-c2` on the 2-torsion family, cost **50.0 (95 % CI
  42.0 to 60.3)** and **51.6 (43.3 to 62.0)** times matched `rho-neg` at `w = 0`,
  and **22.2 (19.7 to 25.5)** and **14.1 (12.0 to 16.9)** times at `w = 1`. Every
  arm's interval at 40 bits lies above 1 at every weight (smallest lower bound
  12.0), so each is `REGRESSES (constant)`.
* **Exponent.** Fitted `α` in `cost ∝ r^α` over `log₂ r ≈ 21 to 40` is above
  rho's for every arm in the all-size fit, but the three-largest-size fit
  reproduces that only for plain `ic-a` on the torsion family. The frozen rule
  therefore gives `REGRESSES (exponent)` for torsion `ic-a` and
  `INCONCLUSIVE (exponent)` for the rest (table below). The exponent is not
  separated from 1/2 by this data; five sizes and three to six curves a size
  cannot do that.
* **Extrapolated FourQ gap** (`log₂ r = 246`, labelled extrapolation from a
  `log₂ r ≤ 40` fit, a six-fold range): best arms `log₂` gap 16.7 to 17.4 at
  `w = 0` and 6.9 to 11.6 at `w = 1` from the all-size fit; 4.7 to 7.0 from the
  three-largest-size fit, whose intervals (about −4 to +15) include parity. With
  rho at its analytic asymptote the `log₂` gap is 5.8 to 7.8 for `ic-c2` and 1.0 to 7.2
  for `ic-b` (`w = 1` and `w = 0`). Reported as distance remaining,
  not as a forecast: the data do not exclude a crossover and do not show one.
* **Level.** Constant for the verdict; exponent not decided.

## Method

Full design in [`PROTOCOL.md`](PROTOCOL.md). In brief: a `std`-only instrument
(`examples/blindspots_01_fp2_gaudry.rs`, sha256 `0629a73c…`; the library does not
compile at the branch base) solves `Q = [d]G` on random ordinary curves over
`F_{p²}` with five arms: `ic-a` (Weil-descended line oracle, `⟨−1⟩` fold,
filter plus Wiedemann), `ic-b` (adds the 2-torsion fold `⟨−1, τ_T⟩` and the FGHR
symmetrised system), `ic-c1`, `ic-c2` (`ic-b` with one and two large primes),
`ic-gls-neg`, `ic-gls-psi` (GLS curves, folds by `⟨−1⟩`, `⟨−1, ψ⟩`), against
`rho-neg` (and `rho-psi` on GLS). Cost is in group-addition equivalents (GAE)
with inversions at 4 multiplications; unpriced native work is listed per row and
priced at 0, 0.1, 1 GAE per operation for every arm, rho included. Instances:
`log₂ #E` nominal 24, 28, 32, 36, 40; 6 torsion-family curves (cofactor 4), 3
prime-order, 3 GLS a size . Index-calculus arms: 2 targets a curve; rho: 8.

## Numbers (measured; `S = cost / √r`, mean of curves, `w = 0`; gap = ratio of means)

| Torsion family, `S` at `log₂ #E` | 24 | 28 | 32 | 36 | 40 | gap at 40, `w=0` (95 % CI) | gap at 40, `w=1` |
|---|---|---|---|---|---|---|---|
| `ic-a` | 233 | 239 | 275 | 306 | 337 | 384 (350 to 430) | 116 (106 to 130) |
| `ic-b` | 40.5 | 39.7 | 50.8 | 42.1 | 44.0 | 50.0 (42.0 to 60.3) | 22.2 (19.7 to 25.5) |
| `ic-c1` | 65.4 | 75.0 | 104 | 102 | 124 | 141 (120 to 168) | 38.1 (32.8 to 45.1) |
| `ic-c2` | 40.3 | 40.1 | 51.8 | 43.3 | 45.4 | 51.6 (43.3 to 62.0) | 14.1 (12.0 to 16.9) |
| `rho-neg` | 1.35 | 1.14 | 1.06 | 0.89 | 0.88 | 1 | 1 |

| Other families, gap at 40 (95 % CI) | `w=0` | `w=1` |
|---|---|---|
| prime order, `ic-a` / `rho-neg` | 187 (155 to 232) | 57.1 (46.8 to 70.5) |
| GLS, `ic-gls-neg` / `rho-neg` | 196 (145 to 406) | 58.3 (43.2 to 121) |
| GLS, `ic-gls-psi` / `rho-neg` | 104 (77.8 to 213) | 32.9 (23.9 to 68.3) |
| GLS, `ic-gls-psi` / `rho-psi` (matched) | 167 (119 to 486) | 42.2 (29.3 to 125) |

Gaps at `w = 0.1` lie between the two columns (`summary.json`, `gaps`). Per-size gaps
at 24 to 36 bits are in `summary.json`; the smallest at any size is 9.9 (`ic-c2`,
24 bits, `w = 1`). Within the instrument, the 2-torsion fold and symmetrised
system lower `ic-a`'s `S` by 7.7× at 40 bits (`w = 0`; 5.2× at `w = 1`); this is a
lever comparison inside one port, not a comparison with the repository's
committed baseline (the library does not build), and is not claimed as an improvement. The
second large prime matches `ic-b` at `w = 0` and is 1.6× better at `w = 1`
because it cuts the filter's native touches from 15.9 M to 2.0 M an instance at
40 bits; one large prime (`ic-c1`) is worse than none (about 4 times the
trials of `ic-b` for the same hit rate). GLS cells have 3 curves and mixed cofactors, so
their per-size series are not monotone (319 at 36 bits, 222 at 40 for `ic-gls-neg`);
read them through the intervals.

### Exponent (all-size fit `α`, 95 % CI; three-largest-size fit in brackets), `w = 0` and `w = 1`

| Arm | `α` `w=0` | `α` `w=1` | matched rho `α` `w=0` / `w=1` | Frozen-rule label |
|---|---|---|---|---|
| torsion `ic-a` | 0.536 (0.531 to 0.540) [0.536] | 0.526 [0.530] | 0.456 (0.444 to 0.466) [0.469] / 0.473 [0.473] | REGRESSES (exponent), reproduced top-3 |
| torsion `ic-b` | 0.507 (0.494 to 0.521) [0.473] | 0.484 [0.476] | same | INCONCLUSIVE: REGRESSES at `w` 0, 0.1, MATCHES at 1; top-3 MATCHES |
| torsion `ic-c1` | 0.556 (0.543 to 0.569) [0.529] | 0.546 [0.527] | same | INCONCLUSIVE (boundary): top-3 above rho at `w=0` only on marginal intervals, at every `w` on the paired difference |
| torsion `ic-c2` | 0.510 (0.497 to 0.524) [0.474] | 0.509 [0.476] | same | INCONCLUSIVE: top-3 MATCHES |
| prime `ic-a` | 0.528 (0.522 to 0.534) [0.537] | 0.506 [0.529] | 0.468 [0.477] / 0.479 [0.482] | INCONCLUSIVE: all-size MATCHES at `w=1`; top-3 REGRESSES at `w=0` only |
| GLS `ic-gls-neg`, `ic-gls-psi` (vs `rho-neg`, `rho-psi`) | 0.572, 0.567 [0.495, 0.488] | 0.556, 0.552 | `rho-neg` 0.488 [0.506]; `rho-psi` 0.457 [0.445] | INCONCLUSIVE: all-size REGRESSES, top-3 MATCHES |

Rho's own fitted `α` is 0.46 to 0.51, below the analytic 1/2 because of fixed
set-up at small `r`; the all-size fit therefore biases every difference upward,
which is why the rule requires the three-largest-size fit to reproduce it.
Marginal intervals decide the label (the frozen wording); the paired
cluster-bootstrap difference intervals in `summary.json` agree everywhere except the two
boundary cases flagged above, where they say REGRESSES.

## Controls

* **Positive control.** 330 of 330 index-calculus rows and 936 of 936 solve rows
  recovered the planted `d` (`verified`); no arm has an unverified row. The 24-bit
  smoke run (card 1) passed on every arm and family.
* **Oracle cross-check.** 101,415 oracle decisions compared with exact enumeration,
  0 disagreements (`summary.json`, `positive_control`); the symmetric, resultant
  and exhaustive oracles agreed on 300 of 300 targets at 24 bits.
* **Negative control (shuffled base, no line structure).** Full solves at 24 and 28
  bits (3 curves each): decomposition rate within 0.03 of `ic-a`'s on the same
  curve in all 6 cells, and `S_control / S_a` between 11.0 and 52.7 (≥ 10 required).
  Rate-only at 32, 36, 40 bits (9 curves): rate within 0.08 of `ic-a`'s in 8 of 9
  cells; the ninth (40 bits, curve 2) differs by 0.084 (control 0.310 of 200
  targets against 0.394), about 2.5 binomial standard errors, so it does not meet the
  frozen 0.08 criterion strictly. The control's null (rate is a property of base
  size, the line base only makes the cost per target far lower, `S_control / S_a` 11 to 53 at the full sizes) holds
  in the other 8 and in the mean (difference −0.027); this is recorded as a control
  miss at one cell, not explained away.
* **Reproducibility.** The re-run of cards 3, 6 and 8 (see Deviations) reproduced
  every row of the aborted first attempts exactly in every counted field (wall time
  excluded): 122 of 122 overlapping rows.

## Deviations and caveats

* **Cards 3, 6 and 8 were re-run by this stage.** The executors' first runs hit
  the 120 s foreground limit and their background tasks were gone when assessment
  started (no receipt, no `rm` of the binary). The user asked for a restart; the
  leftover binaries were removed and the unreceipted partial outputs
  (`gls-b36` 37 of 60 rows, `gls-b40-i1` 1 of 20, plus complete-looking
  `torsion-b36-i0-i2`, `torsion-b40-i2`, `gls-b40-i0`, and a missing
  `torsion-b40-i3`) were moved unchanged to [`raw-aborted/`](raw-aborted/) (they
  are also in git history), because the cards' `must_not_exist` rule and lint
  require absent outputs. The three cards then ran unmodified (same card
  sha256, same instrument sha256) and completed with receipts. The card-11 summary
  first written (887 rows) was premature, ran before those cards finished, and is kept
  as `raw-aborted/summary-premature.json`; card 11 was re-run unmodified after
  card 8 (945 rows, 32 files). Only the re-run outputs are cited.
* **Relay check not machine-confirmable.** `task_card.py check-relay` re-lints the
  card first, and after a run every card fails that lint (its outputs and binary
  exist and are not pinned by `binds`), cards 1 and 2 included. The relays were
  checked by hand instead: every receipt's `status` is `completed`, and
  the receipt sha256 each executor quoted matches the file for cards 1, 2, 4, 5, 7, 9,
  10. `raw/relay-N.txt` for those cards holds only the four header lines rebuilt
  from the receipt, not the executors' full text; for cards 3, 6, 8 and 11 it is the
  re-run's tool output. Executors for cards 3, 6 and 8 reported "pending", a
  deviation (backgrounded foreground command), reported not repaired.
* **Unregistered pre-protocol runs** (smoke and timing probes) are disclosed in
  `PROTOCOL.md`; only card outputs are cited.
* **Scale.** Toy sizes, `log₂ #E` 24 to 40, 3 to 6 curves a size, 2 targets a curve
  for index calculus. Gap intervals resample curves, not instruction streams.
  Cofactor 4 is forced on the torsion family (protocol). The instrument is a
  port, not the library: the counted rho's steps over `√(π r / 2A)` lie near 1.0
  to 1.1 on development probes, and the library filter's pass count is a lower
  bound (an implementation gap that favours index calculus).
* **Host.** Shared 4-vCPU, 15 GB container, isolation level L0; two lanes ran at
  once. The metric is GAE (counted operations); wall time is recorded, not used.
* **Extrapolation to 246 bits** is a straight-line extension of fits over
  `log₂ r` 21 to 40; the three-largest-size intervals span parity. It is
  distance remaining, not evidence of a crossover.
* Not done: no independent runner replay, no `isolab` run, no comparison with
  `msolve`-based decomposition or the library's `gaudry_quartic`.

## Files

`raw/` (`*.jsonl`, `summary.json`, `receipt-card-NN.json`, `relay-N.txt`,
`instances.txt`, `smoke-b24.txt`), `raw-aborted/`, `gap_vs_n.svg`, `gap_vs_n.png`,
`plot_gap.py`, `REPORT.typ`, `REPORT.pdf`; copies in
`benchmarks/20261018-blindspots-fp2-gaudry-k2/`. The leaderboard and
`benchmarks/README.md` row are for the integration step.
