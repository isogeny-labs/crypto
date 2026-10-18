# Blind spot 04: presentation search for the m-sum variety in the binary descent

**Verdict: INCONCLUSIVE** (level: primitive; frozen rule in PROTOCOL.md, applied mechanically by `b04 summarize` in `raw/summary.txt`).
Not a speed or cost result. No cross-method row exists, so there is no gap to matched rho to report and no GAE pricing band applies (0 / 0.1 / 1 GAE per unpriced native op: not applicable, nothing is compared with rho).

## Why INCONCLUSIVE and not ADVANCES

Candidate arm d lowers the closure degree D on 5 of 5 cells (gate holds on all of them), which is the first half of the ADVANCES condition (at least 4 of 5). The second half fails: the A/A control (arm a on draw 1 against arm a on draw 0) must have at least 4 measured cells, and it has 3. Card 02 stopped at command 6: the instrument panicked on cell 4, draw 1 ("no factor base with enough rational elements", examples/blindspots_04_presentation_search.rs:1394), so the control's cell-4 and cell-5 runs never ran. The frozen rule therefore returns INCONCLUSIVE. The A/A control that did run lowers on 0 of 3 cells (the detector does not fire on identical arms).

Arm d does not go below arm e (the known chained route) on any cell: e reaches D = 4 where measured. So even if the control had been valid, the lowering would be against the unchained incumbent only.

## Measured result (median D over 8 random targets, draw 0; from `raw/summary.txt`)

| cell (n,l,m) | a | c | d0 / d | e | b |
|---|---|---|---|---|---|
| 1 (17,4,3) | 7 | 7 | 5 | 4 | 3 (planted only; random unmeasured) |
| 2 (19,4,3) | 7 | 7 | 5 | 4 | 3 (4 of 8 exact, 2 time limit, 2 not attempted) |
| 3 (13,3,4) | 11 (10-11) | 11 (10-11) | 7 (7-8) | 4 | >= 4 (6 not reached, 2 time limit) |
| 4 (15,3,4) | 10 | 10 | 7 | 4 (4 of 8 targets) | >= 4 (5 of 8 attempted) |
| 5 (17,4,4) | >= 13 (certified by degree-12 closure) | >= 13 | 10 | unmeasured | unmeasured |

Per-arm frozen readings: d and d0 LOWER on 5/5 cells; e lowers on 3/5 (2 unmeasured); c MIXED-OR-SAME (0 lowering; 4 gated, 1 unmeasured); b MIXED-OR-SAME with all 5 cells unmeasured. Arms are deterministic in the toy cells, so the spread across targets is nil for most arms (all 8 targets give the same D); the intervals are therefore the observed min-max shown above, not a bootstrap.

Descriptive only, not a claim: median instrument word-ops per refuted random target are much smaller for d than for a (cell 1: 1.8e5 vs 6.0e7; cell 3: 5.4e5 vs 4.9e8; cell 4: 1.2e5 vs 2.6e8), and larger for e (7.8e8 at cell 1) and b (1.9e11 at cell 2). These are counted word operations of a closure instrument on toy cells, not ECDLP cost, and do not enter any rho comparison.

![median D](median_D.png)

## Controls

- A/A (a, draw 1 vs draw 0): 0 of 3 measured cells lower; cell 4 and 5 not run (panic), so the control is invalid by the rule.
- Planted decompositions: every measured arm (a, c, d0, d, e, b where reached) is complete, sound and recovers its planted decomposition on the cells where it ran.
- Arm c (recoding of a in the Edwards frame) equals a on all cells, as expected of a relabelling.
- Card 02's linear/nonlinear recoding controls on cells 1 to 4: not relayed after the stop (commands 7 to 16 not run).

## Run status of the 20 cards

Completed with receipts: 01, 03, 05, 08, 09, 10, 11, 13, 14, 15, 17, 18, 20. Stopped: 02 (command 6 panic). Not completed, no receipt, processes killed before finishing (not rerun, a deviated relay is reported not repaired): 04 (arm e cell 3 random 4-7; its partial log shows the targets were later covered by card 03), 06 (arm e cell 5), 07 (arm b cell 1 random 0-7 and planted), 12 (arm b cell 4 target 0 and planted), 16 (cell 4 target 4), 19 (cell 4 target 7). Note: by the operational stop rule, arm b cell 4 (first card 13 ended not-reached at degree 3 with D_lb 4) and arm e cell 5 were to be recorded BLOCKED(time/memory) rather than retried; arm b cell 4 results come from cards 13, 14, 15, 17, 18 (targets 1, 2, 3, 5, 6).

## Deviations and caveats

- check-relay: the relay copies I could save for completed cards are abridged (header and command lines; stdout/stderr tails elided), so `relay_verbatim` is false for them by construction, receipts' `verify_findings` are empty. Cards lint-fail now only because the removed per-card binary is no longer on disk ("binds does not pin"). I did not repair or rerun any card.
- The six unfinished cards leave arm b cell 1 random targets and cell 5 for e/b unmeasured: the verdict for b is BLOCKED/unmeasured, not evidence that b does not lower D. Arm b at cells 1-2 shows D = 3 (below d) on the targets it did reach, which the frozen rule cannot credit.
- D = 13 or more for a and c at cell 5 is a certified lower bound from a degree-12 closure; the exact value was not attempted.
- Deviations in PROTOCOL.md apply: rustc not cargo; closure degree rather than msolve F4; the same F2-subspace is read in each arm's own frame; identical-solution-set gate defined as completeness on true decompositions plus soundness and planted containment.
- Toy scale (n 13 to 19) and host L0 (shared 4-vCPU container, no isolation): D is a structural degree, so scale and host matter little for it, but nothing here is a timing claim and nothing extrapolates to cryptographic sizes.
- Smoke-test outputs before the measured runs are not data of this item.

## Files

PROTOCOL.md, card-01..20.yaml, raw/ (jsonl, logs, receipts, summary.txt, summary.tsv), median_D.svg, median_D.png, REPORT.pdf. Instrument: examples/blindspots_04_presentation_search.rs (sha256 69456582617f704841172d9a239aaf78003e610d5b3239588ceacde328b40fb5).

To close the verdict: fix the panic at cell 4 draw 1 (or choose a draw that has a factor base), run the A/A control on cells 4 and 5, and finish cards 07, 12, 16, 19, 06. That is a new frozen amendment, not part of this result.
