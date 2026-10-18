# Item 06: signature-based Boolean Groebner engine and Joux-Vitse trace replay

**Verdict: INCONCLUSIVE** (frozen rule, applied mechanically by `b06 summarize`, `raw/card-14-summarize.stdout.log`).
Level: **constant** (a multiplicative factor on the oracle term of item 02's model; no exponent or primitive claim).
Class: stage diagnostic. No `S`, no rho ratio, no GAE is computed here, so the section 5a 0/0.1/1 GAE band
does not apply to these rows. The **gap to matched rho is unchanged by this item** (distance remaining, not moved).
No improvement over the index-calculus baseline was found; no relabelling is claimed as one.

## What the evidence says, and does not say

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

## Method

Four cells (n, l, m): `c13-3-4`, `c15-3-4`, `c15-5-3`, `c17-4-3`; 17 unsatisfiable targets each (`draws.tsv`, seed
20261020), indices 0-15 measured and 16 held out as the trace. Arms: `f4` (item 05's F4, the incumbent),
`sig` (GVW/F5-style criteria over the Boolean ring), `f4r`/`sigr` (exact-key replay), plus `f4n`, `f4rs`, ablations
`sig0`/`sig4`/`sigc`, and fallback control `f4rx`. Metric: matrix entries touched, paired by target. Frozen in
`PROTOCOL.md` (commit before the instrument; Amendment 1 before any card). Instrument
`examples/blindspots_06_signature_gb.rs` (sha256 d9d7ffc3...), single-threaded, `prlimit --as=4 GiB`.

## Controls

| control | result |
|---|---|
| selftest (card 01): 132 systems, f4/sig/sig0/each criterion identical reduced bases, brute-force count | SELFTEST OK |
| code identity (xcheck): f4 vs item 05's committed `c17-4-3` grevlex rows, 8 targets | 8/8 IDENTICAL, XCHECK OK |
| fallback control `f4rx` (c17-4-3, trace of c13-3-4) | ratio 0.9989, 4 fallbacks of 4: no free lunch, OK |
| ablation `sig4`/`sig0`/`sigc` (card 03: `c17-4-3` t0-1; card 11: `c15-3-4` t0 only, t1 skipped) | all stopped, so bounds only: R <= 0.498 / 0.499 / 0.499 on `c17-4-3` and 0.687 / 0.679 / 1.859 on `c15-3-4` (`sig`: 0.492 / 1.677). No criterion subset reaches F4 within the caps. Zero reductions at the c15-3-4 t0 stop: sig0 592,325, sig4 15,517, sigc 488, so the criteria remove zeros as intended. `c13-3-4` ablation never ran |
| memory sanity | descriptive; `mem(f4/X)` columns in the summary |

## Execution record and deviations (all of them)

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

## Caveats

* Toy scale: n = 13-17, a 4-vCPU shared cloud container (host L0, no isolation level earned, no hardware PMU). The unit is
  matrix entries touched, not wall time; wall times in the logs are descriptive only.
* A stopped `sig` job gives only an upper bound on R, so the `sig` numbers above are bounds (`<=`), never speedups.
* `sigr`/`sign` are BLOCKED in every cell: the recording run of the trace (looser 6x stop) did not refute, so no trace
  exists. The replay-of-signature question is untested, not refuted.
* Countable cells for the replay arms: 2 (c13-3-4, c17-4-3), plus half of c15-5-3. The INCONCLUSIVE rule threshold (fewer
  than 3 countable) is the reason a replay verdict is also unavailable.
* Conclusion scope: on these 4 cells and this engine, signature criteria cut zero reductions by 3 orders of magnitude and
  did not cut touches. This says nothing about other orders, row selection, or a production signature code (msolve, F5).

## Files

* Graph: `figures/ratio-by-cell.svg`, `figures/ratio-by-cell.png`, source `figures/make_graph.py` (lays out the values
  copied from `raw/card-14-summarize.stdout.log`). PDF: `REPORT.pdf`.
* Raw logs and receipts: `raw/`. Protocol: `PROTOCOL.md`. Cards: `card-01.yaml` ... `card-14.yaml`.
* Not a speed result: no `benchmarks/` copy (nothing was shown faster).
