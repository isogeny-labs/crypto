# Item 03: polynomial-method solving on descended Semaev systems

## Verdict

**INCONCLUSIVE** (level: not reached; the frozen rule's controls and real runs
did not execute). `b03 summarize` on the recorded outputs prints
"a control did not pass or is missing: INCONCLUSIVE" and
"VERDICT: INCONCLUSIVE (level: n/a)" (`raw/summary-partial.txt`). Arm (e)
msolve remains BLOCKED (not installed). This is not a loss and not a win: only
card 01 ran. Cards 02 to 09 were refused by `task_card.py` before any command
started (see Deviations), so no real FES, real `dinur`/`lptwy`, positive
control 1, negative control cross-check, or F4 run exists.

## What did run (card 01, receipt `raw/receipt-card-01.json`)

* Built `b03` and `b05` (rustc, opt-level 3, target-cpu=native).
* `selftest` SELFTEST OK: 189 counted-model == real-counter checks, 126 parity
  checks equal brute force (48 odd), parity check can fail (29 of 60 wrong at t=1).
* `asym` ASYM OK: Dinur d=2 0.694550 (published 0.6943), d=3 0.811490 (0.8114),
  lptwy d=2 0.876250 (0.8765); all inside the frozen intervals. Positive control 2 passed.
* `sens` SENS OK (E=33 FES ratio 1.888, expected 1.889; E=64 3.610 vs 3.611;
  dinur change 1.4% and 4.0%: exceeds the protocol's 1%, see caveats).
* Counted model for ten cells (targets 0-7 and 8 random systems each).

## Counted-model numbers (descended target 6, log2 bit operations; model only)

| cell | N | E | FES 2^N(2E+2) | dinur | lptwy | rho_dinur | rho_lptwy |
|---|---|---|---|---|---|---|---|
| c13-6-2 | 12 | 13 | 16.81 | 24.93 | 25.44 | 0.0036 | 0.0025 |
| c17-8-2 | 16 | 17 | 21.17 | 28.47 | 29.08 | 0.0063 | 0.0041 |
| c21-10-2 | 20 | 21 | 25.46 | 32.47 | 32.76 | 0.0077 | 0.0063 |
| c25-12-2 | 24 | 25 | 29.70 | 36.11 | 36.91 | 0.0118 | 0.0068 |
| c29-14-2 | 28 | 29 | 33.91 | 39.82 | 40.71 | 0.0166 | 0.0089 |
| c33-16-2 | 32 | 33 | 38.09 | 43.64 | 44.70 | 0.0214 | 0.0102 |

The cost is a function of (N, E, d) only, so the per-cell interval is degenerate
(negative-control identity is seen in the logs but not yet checked against real
random-system runs). Between N=28 and N=32 the local slopes are dinur 0.955,
lptwy 0.998, FES 1.045 per variable: the polynomial-method arms grow more
slowly than the Gray-code FES in this range yet remain about 47x (dinur) and
98x (lptwy) more expensive at N=32. Were real runs to confirm this, the rule's
branch would be TRADES or REGRESSES depending on the jackknifed fit; it is not
applied here because the fit needs `summarize` rows from the real runs
(it printed "fewer than 4 cells with FES rows: no fit"). Crossover N* is not
computed (the crossover block is empty without the run rows).

S_4 (cubic) cells, rho_dinur against the fes-floor: 1.7e-3 (N=13), 2.7e-3 (19),
5.1e-3 (25), 6.2e-3 (31).

## Exponent statement (made whatever the verdict; level exponent)

Published-form asymptotic c: 0.6946 for d=2 and 0.8115 for d=3. Required for a
decomposition win over rho is c < 1/2 - 1/m: 0 for m=2 (distance remaining
0.6946), 1/6 for m=3 (distance remaining 0.6448). Index-calculus exponent
relative to rho, 2/m + 2c: 2.389 (m=2) and 2.290 (m=3); above 1 loses. This is
an asymptotic-formula statement, not a measured improvement.

## Gap to matched rho (AGENTS.md section 5a)

No cross-method row exists: there are no measured group-operation or unpriced
native-operation rows, so pricing at 0, 0.1 and 1 GAE per operation is not
applicable and no ranking is made. The only gap quoted is the asymptotic
distance above.

## Deviations

Cards 02 to 09 were refused by the executor with "card does not lint: ...
names repository file target/blindspots-03/bin/b03 (b05) that binds does not
pin" (tool exit 2, nothing ran). Cause: the cards were authored when the binary
did not exist; once card 01 built it, the lint treats it as an unpinned repo
file. Re-running `task_card.py check-relay` on every card, including card 01,
now reports CARD FAIL for the same reason. Per instructions the relays are
reported, not repaired; the cards were not edited. The user's request to
restart so the builds run was honoured (card 01 built both binaries), but the
rest cannot proceed without a director-side card change (for example
excluding build outputs from the pin rule or pinning them after the build).
Executor relays are in `raw/relay-1.txt` to `raw/relay-9.txt` (card 01's relay
is condensed; its full output is in `raw/card-01-*.log`).

## Caveats

Counted model only; host is shared 4-vCPU at isolation level L0; toy scale
(N <= 32); **the sensitivity control disagrees with the frozen text**: PROTOCOL.md
requires the dinur count to move by less than 1% across E = 17, 33, 64, but the
instrument measured 1.36% (E=33) and 3.99% (E=64) and its code passes at
`rel_d < 0.05`. Read against the frozen 1% the control fails; read against the
code's 5% it passes. This code/protocol mismatch is reported, not resolved; it
is a second reason the verdict is INCONCLUSIVE and needs a director decision.
The built binaries remain in `target/blindspots-03/bin` because card 09, which
removes them, did not run. No speed result, so no `benchmarks/` entry.

![graph](graph.png)
