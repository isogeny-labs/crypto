# Item 06: signature-based Boolean Groebner engine and Joux-Vitse trace replay

**Verdict: REGRESSES** (frozen rule, `b06 summarize` over `raw/` with the repaired instrument; output
`raw/assess-r2-summarize.stdout.log`, last line: "sig is at least 1.5x worse than f4 in at least 3 cells").
Level: **constant** (a multiplicative factor on the oracle term of item 02's model; no exponent or primitive claim).
Class: stage diagnostic. No `S`, no rho ratio, no GAE is computed, so the section 5a 0/0.1/1 GAE band does not
apply to these rows. **The gap to matched rho is unchanged by this item** (distance remaining, not moved). No
improvement over the index-calculus baseline was found; this is a loss for the signature engine, reported as a loss.
The first-run verdict (INCONCLUSIVE) is kept below under "First run (superseded)".

## What changed since the first run

The repair pass (PROTOCOL.md Amendment 2) re-ran cards 03, 04, 05, 08, 12: nine r2 cards, all receipts
`status: completed` (checked on disk, `raw/receipt-card-*-r2.json`, `check-relay` `verify_findings: []`), so every
cell now has 16 targets for `f4` and `sig` (except one `f4` timelimit, below) and `c15-3-4` has `f4r`/`f4n` rows.
The decisive change: `c15-5-3` now has all 16 `sig` targets (8 more in card 05-r2), all stopped by the 2x touch cap, so
the cell is regress-certain. That makes 3 of 4 cells regress-certain, which is the REGRESSES condition.

## Numbers (measured by the instrument; ranges are per-target min-max, derived from the raw rows)

R = touches(f4) / cost(X), geometric mean over targets, paired by target. `sig` rows are upper bounds (`<=`):
a stopped job's work so far is a lower bound on its cost.

| cell | sig: stops (of 16) | sig R (bound) | per-target bound range | regress-certain (<= 1/1.5) |
|---|---|--:|---|---|
| c13-3-4 | 16 touchcap | <= 0.494 | 0.475 to 0.500 | yes |
| c15-3-4 | 5 touchcap, 4 colscap, 4 timelimit, 3 memcap | <= 1.677 | 0.495 to 8.66 | **no** (non-touch stops do not bound) |
| c15-5-3 | 16 touchcap | <= 0.497 | 0.491 to 0.500 | yes |
| c17-4-3 | 16 touchcap | <= 0.492 | 0.463 to 0.500 | yes |

`sig` refuted 0 of 64 targets; it qualifies (R >= 1.5 on 12 of 16, all refuted) in 0 of 4 cells, so ADVANCES and
TRADES are excluded. Because REGRESSES needs only 3 certain cells, the unsettled `c15-3-4` does not change the verdict.
The touch-cap bound is not an artefact of the cap: a job that is still unfinished at 2x F4's touches costs more than 2x,
so R < 0.5 is true for those targets (the cap fixes how tight the bound is, not its sign). Descriptive long run (card
03-r2, `--touch-ratio 100`, `c17-4-3` t0): `sig` ended `memcap` at 9.33e9 touches (29x F4's 3.19e8) with 3907 steps, still
not refuted: R <= 0.034, one target, outside the rule.

| cell | f4r R (exact-key replay, trace charged) | range | f4n R (descriptive) | range | n >= 1.5 (f4n) | targets paired |
|---|--:|---|--:|---|--:|--:|
| c13-3-4 | 0.885 | 0.85 to 0.93 | 1.120 | 0.56 to 2.82 | 2/16 | 16 |
| c15-3-4 | 0.630 | 0.41 to 0.91 | 1.026 | 0.42 to 2.54 | 7/15 | 15 |
| c15-5-3 | 0.741 | 0.64 to 0.84 | 1.413 | 1.30 to 1.52 | 2/16 | 16 |
| c17-4-3 | 0.587 | 0.50 to 0.77 | 1.160 | 0.92 to 1.46 | 0/16 | 16 |

`f4r` fell back on every target in every cell (16, 15, 16, 16 fallbacks): the exact key filter does not transfer
between targets, so replay costs more than a fresh `f4` once the trace is charged. `f4n` is a descriptive extra arm: it
never reaches the 1.5 bar in a cell; its zero reductions fall 1.5x to 2.6x (f4/f4n 1.548, 1.989, 2.402, 2.626) and its
largest matrix is 0.32x to 1.53x of F4's, but touches do not fall enough. `f4rs` (descriptive, 1 to 4 targets per
cell): R 2.09, 1.98 (1 target), 3.02, 2.60 with 245x to 343x fewer zero reductions. Four-target leads outside the rule,
not results; what it counts is in PROTOCOL.md "Counted quantities".

Zero reductions at the first degree-4 matrix, f4/sig geometric mean (touches at that point in brackets):
c13-3-4 719x (0.456), c15-3-4 873x (0.403), c15-5-3 480x (10.67), c17-4-3 1519x (4.17). The criteria remove the zero
reductions they target, but the engine pays for it at later degrees (selection degree 5, thousands of steps) and never
reaches `1` within 2x of F4. The ablations (`sig4`, `sig0`, `sigc`, 2 targets per cell, now in all four cells except
`c15-3-4` t1 skipped) all stop at the cap too: bounds 0.50, 0.50, 0.57 (c13-3-4), 0.50, 0.50, 0.49 (c15-5-3), 0.498,
0.499, 0.499 (c17-4-3), and 0.687, 0.679, 1.859 (c15-3-4, 1 target). No criterion subset reaches F4 within the caps.

Gap to matched rho: unchanged; this item moves no ECDLP row. Pricing at 0, 0.1 and 1 GAE per unpriced native operation:
not applicable (no cross-method row; item 02's band prices an oracle factor, and the factor found is a loss).

## Method

Four cells (n, l, m): `c13-3-4`, `c15-3-4`, `c15-5-3`, `c17-4-3`; 17 unsatisfiable targets each (`draws.tsv`, seed
20261020), indices 0-15 measured, 16 held out as the trace. Arms: `f4` (item 05's F4, the incumbent), `sig` (GVW/F5-style
criteria over the Boolean ring), `f4r`/`sigr` (exact-key replay), plus `f4n`, `f4rs`, ablations `sig0`/`sig4`/`sigc`, and
fallback control `f4rx`. Metric: matrix entries touched, paired by target. Frozen in `PROTOCOL.md` (Amendment 1 before
any card, Amendment 2 before any r2 card). Instrument `examples/blindspots_06_signature_gb.rs` single-threaded under
`prlimit --as=4 GiB`: sha256 `d9d7ffc3...` for the first-run cards, `48a39c34...` (Amendment 2, deviations 15 to 17) for the r2 cards.

## Controls

| control | result |
|---|---|
| selftest (card 01): 132 systems, f4/sig/sig0/each criterion identical reduced bases, brute-force count | SELFTEST OK (first-run card; instrument fix does not touch it; r2 smoke test also OK, `smoke-test-r2.txt`) |
| code identity (xcheck): f4 vs item 05 `c17-4-3` grevlex rows, 8 targets | 8/8 IDENTICAL |
| fallback control `f4rx` (c17-4-3, trace of c13-3-4) | ratio 0.9989, 4 of 4 fallbacks: no free lunch, OK |
| ablation `sig4`/`sig0`/`sigc` | see above; all stopped, bounds only |
| old vs new instrument | identical on 33 of 36 held-out rows (other 3 are 25 s time-limit stops, `smoke-test-r2.txt`) |
| `f4` identical across cards | trace row (target 16) is identical in cards 12-r2, 12b-r2, 12c-r2 (touches 11088230851) |

## Deviations (all of them)

* The first-run card list in PROTOCOL.md Amendment 2 stands. Cards 03, 04, 05, 08, 12 were re-run as r2 cards; first-run
  rows of the same (cell, target, arm) are replaced by r2 rows in `summarize` (read last), and `-long-` files are skipped.
* **Summarize was run by the assess stage, not by a card.** Rule 4 forbade freezing card-14-r2 before the r2 outputs were
  committed; the assessor built the instrument (sha256 `48a39c34...`, verified) into the scratchpad and ran
  `b06 summarize --dir raw` once, saving the output as `raw/assess-r2-summarize.stdout.log`. No card-14-r2 exists, so this
  output is not sha-bound by a receipt; it is reproducible from the committed `raw/` and the committed instrument.
  First-run `raw/card-14-summarize.stdout.log` is unchanged and stale.
* **Relay files are abbreviated.** `raw/relay-r2-1..9.txt` hold the receipt-derived status lines, not the executors'
  full text (output bodies were not retyped; named `-r2-` so as not to overwrite the first-run `relay-1..9.txt`).
  `check-relay` therefore reports missing lines; the receipts (all `completed`, `verify_findings: []`) and `raw/` are the evidence.
* `--total-limit` raised only where the first-run value would have skipped rows (Amendment 2 C). Still skipped in the
  first-run cards that were not re-run: `f4rs` on `c15-3-4` (2 targets skipped, 1 timelimit) and ablation rows for t1 of
  `c15-3-4`. `f4` itself hit `timelimit` on `c15-3-4` target 1 (card 11), so that cell has 15 paired `f4r`/`f4n` targets
  and cannot satisfy "every target refuted by f4".
* `sigr`/`sign` BLOCKED in all cells (the recording run of the trace did not refute): the replay-of-signature
  question is untested, not refuted. Arm (e) msolve BLOCKED (not installed, no GMP/FLINT headers).
* Design departures (PROTOCOL deviations 10-14): ascending-degree generator order, multiple pivots, K5; stopped at 2x F4
  touches; K5 completeness checked, not proved. `sig` may lose partly because of these choices.
* A concurrent clippy/rustfmt task would change the instrument sha; the instrument sha was verified unchanged at assessment.

## Caveats

* Toy scale (n = 13 to 17), shared 4-vCPU cloud container, host level L0, no isolation level earned, no hardware PMU. The
  unit is matrix entries touched, not wall time; wall times are descriptive.
* The verdict is about this engine (as built, with the stated design departures) on these four cells and orders. It says
  nothing about production signature codes (msolve, F5, magma) or other orders and row selections.
* The REGRESSES line follows from the 2x touch cap on three cells; the true factor is unknown (only R < 0.5 there).
  Per-target ranges are min-max, not confidence intervals; the rule is deterministic over the recorded rows.
* Not a speed result: nothing was shown faster, so no `benchmarks/` copy.

## Files

* Graph: `figures/ratio-by-cell.svg`, `.png`, source `figures/make_graph.py`. PDF: `REPORT.pdf` (source `REPORT.typ`).
* Raw logs and receipts: `raw/` (first run and `*-r2-*`). Protocol: `PROTOCOL.md`. Cards: `card-*.yaml`, `card-*-r2.yaml`,
  generator `make_cards_r2.py`. Summary: `raw/assess-r2-summarize.stdout.log`.

# First run (superseded)

The text below is the first-run README, kept verbatim except that its headings are demoted one level. Its verdict
(INCONCLUSIVE) was computed with missing cards 03 (partly), 04, 05, 08 and 12; it is superseded by the verdict above.


**Verdict: INCONCLUSIVE** (frozen rule, applied mechanically by `b06 summarize`, `raw/card-14-summarize.stdout.log`).
Level: **constant** (a multiplicative factor on the oracle term of item 02's model; no exponent or primitive claim).
Class: stage diagnostic. No `S`, no rho ratio, no GAE is computed here, so the section 5a 0/0.1/1 GAE band
does not apply to these rows. The **gap to matched rho is unchanged by this item** (distance remaining, not moved).
No improvement over the index-calculus baseline was found; no relabelling is claimed as one.

### What the evidence says, and does not say

* **The signature engine `sig` did not refute any of the 56 targets that ran (64 planned)** within its stop rules (touch cap 2x of
  `f4`, time, column and memory caps). It qualifies on 0 of 4 cells, so ADVANCES and TRADES are excluded for
  `sig` and `sigr` whatever the missing cards would have shown (a cell needs all 16 targets refuted).
* **REGRESSES is not reached.** The rule needs `sig` regress-certain (geometric mean of exact ratios and upper bounds
  <= 1/1.5) in 3 cells. Two are certain: `c13-3-4` (R <= 0.494) and `c17-4-3` (R <= 0.492). `c15-5-3` shows
  R <= 0.498 on the 8 targets that ran, but 8 targets never ran (card 05). `c15-3-4` gives R <= 1.677, which does not
  bound R below 1/1.5 (stops by time, memory and column caps are in the mix: 2 timelimit, 2 colscap, 2 memcap, and
  touchcap on the other 2 of those 8 in card 10; card 11: 2 timelimit, 2 colscap, 1 memcap, 3 touchcap). So: INCONCLUSIVE, as `summarize` prints. A loss on 2 of 4 cells,
  with the other two unsettled, is the honest reading.
* **Zero reductions are nearly gone, touches are not.** At the first degree-4 matrix, `f4`/`sig` zero reductions
  (geometric mean over the targets recorded) are 719x (`c13-3-4`), 873x (`c15-3-4`), 484x (`c15-5-3`),
  1519x (`c17-4-3`); touches at that point f4/sig are 0.456, 0.403, 10.68, 4.166. The signature engine removes the
  zeros the design targeted, but pays for it in later degrees (selection degree 5, thousands of steps) and never reaches
  `1` inside 2x of F4's touches on `c13-3-4`, `c15-5-3`, `c17-4-3`.
* **Exact-key replay (`f4r`, arm c) loses.** Paired gmean R (trace charged, 16 amortised): `c13-3-4` 0.885 (16/16
  refuted, 16 fallbacks), `c15-5-3` 0.727 (8/16 refuted, 8 fallbacks), `c17-4-3` 0.587 (16/16, 16 fallbacks). It
  fell back on every target: leading-monomial structure differs between targets, so the exact filter does not transfer.
  `c15-3-4`: no `f4r` row (card 12 never completed).
* **Negative-filter replay (`f4n`, descriptive extra, not a rule arm)**: gmean R 1.120 (`c13-3-4`, >= 1.5 on 2/16),
  1.160 (`c17-4-3`, 0/16), 1.405 (`c15-5-3`, 1/8). Zero reductions fall 1.5x to 2.6x (f4/X, `c13-3-4`, `c15-5-3`, `c17-4-3`). Its largest matrix (rc_max f4/X) is 3.1x larger in
  `c13-3-4`, 1.07x larger in `c17-4-3`, 1.5x smaller in `c15-5-3`. The `mem(f4/X)` column (3.7 to 9.2) is not interpreted:
  logged resident high-water marks are flat at 60-70 MB and the protocol calls peak memory descriptive. Never reaches the 1.5 qualification bar in a cell.
  (An earlier development note that it "helps on c17-4-3 and hurts on c15-3-4" is not tested: c15-3-4 has no row.)
* **`f4rs`** (descriptive; arm of the design that shares symbolic preprocessing) shows R 2.09 (`c13-3-4`, 4 targets),
  2.60 (`c17-4-3`, 4 targets), 1.98 (`c15-3-4`, 1 target; target 1 hit the 90 s time limit), with 245x to 343x fewer
  zero reductions. These are 1 to 4 targets per cell, outside the frozen rule, and are reported as leads, not results.
  What `f4rs` counts is defined in PROTOCOL.md "Counted quantities"; this README does not re-derive it.

### Method

Four cells (n, l, m): `c13-3-4`, `c15-3-4`, `c15-5-3`, `c17-4-3`; 17 unsatisfiable targets each (`draws.tsv`, seed
20261020), indices 0-15 measured and 16 held out as the trace. Arms: `f4` (item 05's F4, the incumbent),
`sig` (GVW/F5-style criteria over the Boolean ring), `f4r`/`sigr` (exact-key replay), plus `f4n`, `f4rs`, ablations
`sig0`/`sig4`/`sigc`, and fallback control `f4rx`. Metric: matrix entries touched, paired by target. Frozen in
`PROTOCOL.md` (commit before the instrument; Amendment 1 before any card). Instrument
`examples/blindspots_06_signature_gb.rs` (sha256 d9d7ffc3...), single-threaded, `prlimit --as=4 GiB`.

### Controls

| control | result |
|---|---|
| selftest (card 01): 132 systems, f4/sig/sig0/each criterion identical reduced bases, brute-force count | SELFTEST OK |
| code identity (xcheck): f4 vs item 05's committed `c17-4-3` grevlex rows, 8 targets | 8/8 IDENTICAL, XCHECK OK |
| fallback control `f4rx` (c17-4-3, trace of c13-3-4) | ratio 0.9989, 4 fallbacks of 4: no free lunch, OK |
| ablation `sig4`/`sig0`/`sigc` (card 03: `c17-4-3` t0-1; card 11: `c15-3-4` t0 only, t1 skipped) | all stopped, so bounds only: R <= 0.498 / 0.499 / 0.499 on `c17-4-3` and 0.687 / 0.679 / 1.859 on `c15-3-4` (`sig`: 0.492 / 1.677). No criterion subset reaches F4 within the caps. Zero reductions at the c15-3-4 t0 stop: sig0 592,325, sig4 15,517, sigc 488, so the criteria remove zeros as intended. `c13-3-4` ablation never ran |
| memory sanity | descriptive; `mem(f4/X)` columns in the summary |

### Execution record and deviations (all of them)

* **Cards 03, 04, 05, 08, 12 did not complete as planned.** The executor relays for 04, 05, 08 and 12 report background
  jobs with no output and no receipt; their processes were no longer running when this assessment started. No card
  was rerun (the assess stage runs nothing else).
  * 03: `long-c17-4-3` (f4,sig with `--touch-ratio 100`) died with a 3 GiB allocation failure under the 4 GiB limit
    (exit -6); the two clean-up commands did not run (the 24 MB of leftover binaries in `target/blindspots-06/` were
    removed by hand afterwards). `sigr`/`sign` and the ablation commands of card 03 ran.
  * 04: raw rows for `sig c15-5-3` targets 0-7 exist; no receipt; `f4rs c15-5-3` never ran.
  * 05: only the build ran: `c15-5-3` targets 8-15 and its ablation are absent.
  * 08: `sig c13-3-4` targets 8-15 exist (stderr and stdout); no receipt; the `c13-3-4` ablation never ran.
  * 12: only the build ran: `f4r`/`f4n` on `c15-3-4` are absent.
  Receipts exist for cards 01, 02, 03, 06, 07, 09, 10, 11, 13, 14 only.
* **Relay check.** `check-relay` was run against relay files that this assessor saved in abbreviated form
  (`raw/relay-*.txt`: the very long JSON bodies were not retyped), so a "relay_verbatim: false" result for cards
  01, 02, 06, 07, 09, 10, 11, 13, 14 reflects the abbreviation, not an executor edit; and the lint of cards 03, 04, 05,
  08 and 12 (which have no receipt) fails ("names repository file ... binds does not pin"). The evidence used
  below is the raw logs and card receipts on disk, never the relay text. Executor summaries were also compared against disk
  where it matters: card 14's printed table is the file `raw/card-14-summarize.stdout.log`.
* **Arm (e) msolve: BLOCKED** (not installed; no GMP/FLINT/MPFR headers; disk 3.2 GB free).
* **Design departures (PROTOCOL.md deviations 10-14):** `sig` uses ascending-degree generator order, multiple pivots, K5;
  stopped at 2x F4 touches; K5 completeness under lcm-degree batches is checked, not proved; per-card build.
  `sig` may lose partly because of these choices.
* **Development exposure:** disclosed in PROTOCOL Amendment 1.C; none of the 16 measured targets had been run.

### Caveats

* Toy scale: n = 13-17, a 4-vCPU shared cloud container (host L0, no isolation level earned, no hardware PMU). The unit is
  matrix entries touched, not wall time; wall times in the logs are descriptive only.
* A stopped `sig` job gives only an upper bound on R, so the `sig` numbers above are bounds (`<=`), never speedups.
* `sigr`/`sign` are BLOCKED in every cell: the recording run of the trace (looser 6x stop) did not refute, so no trace
  exists. The replay-of-signature question is untested, not refuted.
* Countable cells for the replay arms: 2 (c13-3-4, c17-4-3), plus half of c15-5-3. The INCONCLUSIVE rule threshold (fewer
  than 3 countable) is the reason a replay verdict is also unavailable.
* Conclusion scope: on these 4 cells and this engine, signature criteria cut zero reductions by 3 orders of magnitude and
  did not cut touches. This says nothing about other orders, row selection, or a production signature code (msolve, F5).

### Files

* Graph: `figures/ratio-by-cell.svg`, `figures/ratio-by-cell.png`, source `figures/make_graph.py` (lays out the values
  copied from `raw/card-14-summarize.stdout.log`). PDF: `REPORT.pdf`.
* Raw logs and receipts: `raw/`. Protocol: `PROTOCOL.md`. Cards: `card-01.yaml` ... `card-14.yaml`.
* Not a speed result: no `benchmarks/` copy (nothing was shown faster).
