# Multi-target IC with a lock-step batched setup collection (frozen evaluation)

Status: [pcc:unverified] producer's own measurement; not independently replayed; same machine, 1 thread, wall clock, toy sizes; the previous-arm reference rows come from an earlier run (run-to-run noise 1.2x to 1.3x per the validator).

Change: `collect_setup_batched` replaces the per-walker inversions of the shared factor-base collection (relations against Q = G) with 256 walkers advanced in lock-step, one batched inversion per round, with the pair-table lookup. Used by `ic_multi_solve_ex(..., table = true, batched = true)`. Setup phase timing at 27 bits (`IC_MULTI_TIMING=1`, `results/pf_setup/explore_27_timing.txt`): before, collection 52 to 81 ms (c = 1: 74 to 81, c = 2: 52 to 64) with pair table 14 to 65 ms and elimination 2 to 14 ms; after, collection 20 to 30 ms (2x to 3x lower), c = 1 setup 35 to 46 ms. Every target of every run verified.

Protocol: frozen in `results/pf_setup/frozen.json` (commit 1eba8dd551, pushed and checked on the remote before the evaluation) with the prediction and abandon rule: new c = 1 / previous batched arm (committed `pf_batched` rows, c = 2.0) <= 0.85 at T = 1000; new c = 1 / tuned BSGS <= 5 at T = 1000; new c = 1 / previous <= 1.0 at T = 10 000; abandon if the T = 1000 ratio >= 1.0. c = 1.0 was chosen from the 27-bit exploratory run; evaluation curves 25, 29, 31 bits were not used for any tuning of this arm. 20 rows per arm (T = 1000 and 10 000), 60 arm-runs, all targets verified, no failed run.

Results (medians of per-row ratios, total time on identical targets):

| T | rows | new c=1 / previous | new c=2 / previous | new c=1 / BSGS | new c=2 / BSGS |
|---|---|---|---|---|---|
| 1000 | 10 | 0.56 (0.44 to 1.29) | 0.79 | 3.7 | 5.9 |
| 10 000 | 10 | 1.47 (1.05 to 1.84) | 0.94 | 5.8 | 3.8 |

By size at T = 1000, new c=1 / previous: 0.98 at 25 bits, 0.50 at 29, 0.54 at 31; new c=1 / BSGS: 2.9, 3.8, 5.0.

Prediction check: at T = 1000, both predictions hold (0.56 <= 0.85 and 3.7 <= 5; no gain at 25 bits). At T = 10 000 the prediction new c=1 / previous <= 1.0 **fails** (1.47): the smaller factor base makes the per-target phase slower, which dominates at T = 10 000; with c = 2 the new setup is a small gain (0.94). The abandon rule was not triggered.

Finding: batching the setup collection is a reproducible gain at T = 1000 for c = 1 (median 0.56 of the previous batched arm; none at 25 bits). The best factor-base constant depends on T: c = 1 for T = 1000 (3.7x behind the tuned BSGS) and c = 2 for T = 10 000 (3.8x behind). The distance to the tuned BSGS is now about 3.7x to 3.8x at the median for both T, down from 6.9x (T = 1000) and 4.9x (T = 10 000) for the previous arm, and still grows with size (c = 1 at T = 1000: 2.9 to 5.0 over 25 to 31 bits).

Limits: the reference rows for the previous arm are from an earlier run (noise 1.2x to 1.3x, up to 1.9x); c is chosen per T from two values only (not frozen separately for T = 10 000, where the result is post hoc); the tuned BSGS arm is the validator-flagged version that keeps stepping found lanes (1.07x to 1.31x slower than a compacted variant), so every ratio here is understated; toy sizes; memory not priced; not independently replayed; no rho arm, §5a band, certificate file, bench record or `VALIDATION.json`.

## Independent validation and required corrections (`results/pf_setup/VALIDATOR_REPORT.md`)

Disposition: validated, scope-limited. Validated: the freeze (commit 1eba8dd551) precedes every row; all deterministic fields reproduce (3 720 of 3 720 on the validator's rebuilds, 1 380 of 1 380 on the producer's binaries); 220 000 returned logs verify in independent Python code with 0 wrong; the setup relations (25 412) and factor-base logs pass; mutating the lane bookkeeping is caught by the relation check and the end-to-end `k G = Q` guard keeps returned logs correct (25 200 guarded targets, 0 wrong; without the guard 5 555 wrong logs, all flagged); T = 1000 new c=1 / previous = 0.56 (0.51 to 0.76) reproduces and the abandon rule is not triggered; at T = 10 000 the c=1 prediction fails (1.38), as reported; the 27-bit collection speed-up of 2x to 3x reproduces.

Corrections that supersede the text above:
- "c=2 small gain (0.94) at T = 10 000" is **not validated**: reruns give 0.98 (0.95 to 1.04), i.e. no gain.
- "distance to the BSGS down from 4.9x to 3.8x at T = 10 000" is **not validated**: in the same session the previous arm and the new arm are both about 4.5x; the committed fall came from the new run's BSGS rows being 1.19x slower than the earlier run's with the same binary.
- "no gain at 25 bits" is **not validated**: reruns give 0.76 (two committed rows were outliers).
- Distance to the tuned BSGS: 3.8x to 4.3x at T = 1000 and 4.3x to 4.5x at T = 10 000; against a compacted BSGS (1.18x to 1.27x faster) 5.0x and 5.5x.
- The 27-bit "before" phase timings are in no committed file (the 2x to 3x collection speed-up reproduces on re-measurement).
- Choosing c by T is post hoc: the freeze lists the c=2 arm without a prediction or a rule for choosing c by T. The direction reproduces (c1/c2 = 0.74 at T = 1000, 1.46 at T = 10 000), but in a 29-bit probe the rule "c = 2 for T >= 10 000" fails at T = 30 000, where c = 3 is 1.37x better; with c retuned per T the ratio to the compacted BSGS stays at 5.2 to 5.9 over T = 1000 to 30 000.
- No rho arm, §5a band, ecbench admission, certificate file, bench record or `VALIDATION.json`; wall-clock only.
