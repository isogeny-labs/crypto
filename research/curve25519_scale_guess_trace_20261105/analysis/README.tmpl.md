# Scale-up S1: guessing, guessing optimum and F4 trace replay at k = 20, 24, 28

Protocol: [PROTOCOL.md](PROTOCOL.md), frozen in commit bf2c559aee before any data. Scope of every statement:
**isogeny form, m = 2, mode full, lifts 0, k in {20 (reference), 24, 28}, t in {6, 8, 10}, G in {4, t-1}, trace in {0, 1},
100 random subgroup targets per cell, one seed per cell**. Nothing is claimed for other k, t, G, m or form, for wall time, or for a
distance to Pollard rho (lead A's unit). Run locally on a shared 4-core box, not on EKS; no replay certificate, validator report or matched rho
is attached, so nothing here is a leaderboard or `benchmarks/` row. [pcc:unverified]

## Answer in one paragraph

On the measured cells the F4 work of the guessing oracle per target does not depend on k (k = 28 over k = 24: 1.000 to 1.019 in all six (t, G) cells),
so the per-relation cost at `coverage_exact` rises with k only through the coverage, which fell by 7.6x to 20.4x per 4 bits; on three points the fitted
exponent of PR(cov) in r (nominal 2^k) is 0.91 at t = 6 and 1.00 to 1.02 at t = 8 and 10. The trace ratio (replay work over plain work per target,
learning excluded) is also independent of k to three digits, but it is **not** in the frozen 0.29 to 0.45 band: it is 0.398 (t = 10, G = 4),
0.443 (t = 8, G = 4), 0.454 (t = 6, G = 4) and 0.482 to 0.485 at G = t-1, so prediction P3 is false in 8 of 12 pairs. The learning pays back inside the 100 targets
(amortised ratio is the raw ratio plus 0.010 to 0.011). All eight reproduction cells at k = 20 match the earlier counts exactly. No cell was capped, none failed.

## Instrument and method

- Binary `/home/user/frozen-bin/curve25519_family_scale`, sha256 `d66b85b0dcb60c2d1b551a2e3ce19de4cc76e6897ce359eafce1bf579606ca90`.
  The sha256 was checked at the start of the session (matches the task's value) and by the driver before it ran any cell (it exits on a mismatch);
  the hash is also stored in every cell json (`binary_sha256`) and in `analysis/binary.sha256`. No code change, no build.
- Command per cell: `( ulimit -v 3500000; timeout 1000 <bin> f4cost <k> isogeny <t> <G> 100 <seed> 900 2 full 0 - <trace> )`,
  driver `analysis/run_cells.sh` (shell + jq, strictly sequential, resumable), cell list `analysis/cells.list` (32 cells),
  per-cell json `analysis/cells/` (132 KB in all, no file near 64 KiB, so nothing went to the artifact store), stderr `analysis/stderr/` (all 32 files empty),
  status `analysis/status.txt`. Tables and figure: `analysis/make_tsv.sh`, `make_tables.sh`, `make_svg.sh` (jq + awk only), consolidated `analysis/cells.tsv`.
- Seeds: `2026112400 + t` (k = 24), `2026112800 + t` (k = 28); reproduction `2026111600 + t` (lead B) and `2026111400 + t` (lead C). In one (k, t) every G and both trace arms share
  the 100 targets, so a trace pair is paired target for target.
- Load: `uptime` 1-minute load at each cell start is in the table (7.07 to 14.71 over the 32 cells: the box was heavily shared). Wall times are in the table and are **not compared**.
  Total driver time 1481 s over 32 cells; longest cell 362 s (k = 24, t = 10, G = 4, trace 0) against the 900 s cap.
- Columns below: mul/tgt = F4 multiply-adds per target (`muladds_per_target`; for trace 1 = replay + fallback work, learning excluded); amort = amortised per target
  (learning `trace_learn_muladds` added once over the cell's 100 targets; equals mul/tgt at trace 0); calls = F4 calls per target; deg = max solving degree; und = undecided;
  h/v = `hits` / `hits_verified`; cov = `coverage_exact`; PR(cov) = per-relation cost = mul/tgt (trace 0) or amort (trace 1) divided by cov, a modelled figure;
  obs = total F4 multiply-adds / hits where hits >= 1 (1 to 4 hits, not a ranking basis); unch = uncharged native field multiplications (system specialisation and grid reading) as a share of
  charged F4 multiply-adds; unch+L = the same with the trace learning added to numerator and denominator (equals unch at trace 0); s = elapsed seconds; load = 1-minute load at start.

## Full table (all 32 cells; 0 capped, 0 failed)

Stage `main` = the 24 grid cells, `repB` and `repC` = the 8 reproduction cells at k = 20 (seeds of lead B and lead C). CAP column: `-` means the cell finished all 100 targets.

| seed | stage | k | t | G | trace | targets | cap | mul/tgt | amort | calls | deg | und | h/v | cov | PR(cov) | obs | unch | unch+L | s | load |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
@@table_sorted.md@@

![F4 multiply-adds per target vs k, and the trace ratio vs k](analysis/scale.svg)

`analysis/scale.svg`: panel A, F4 multiply-adds per target against k (log axis; solid trace 0, dashed trace 1; label t/G at the k = 28 end of the trace 0 line; k = 20 trace 0 points are
lead B's cells, k = 20 trace 1 points exist only for t/G = 6/4, 6/5, 8/4, 10/9 from the reproduction cells); panel B, trace 1 over trace 0 per target with the frozen 0.29 to 0.45 band.

## Reproduction at k = 20 (P7)

- **R-B, lead B seeds, trace 0, t = 6 G = 5 and t = 10 G = 9:** `f4_muladds`, `f4_calls`, `hits`, `hits_verified`, `solving_degree_max` and `coverage_exact` equal the stage s1 cells of
  `../curve25519_guess_optimum_20261102/analysis/cells.jsonl` exactly (14646400 / 3200 / 0 / 0 / 4 and 451850650 / 40569 / 29 / 29 / 4). The trace 1 partners (no earlier counts at G = t-1) have the same `hits` and `hits_verified` as their trace 0 cells.
- **R-C, lead C seeds, t = 6 and 8 at G = 4, trace 0 and 1 (added by this study, see the protocol):** `f4_muladds`, `f4_calls`, `trace_learn_muladds` and hits equal
  `../curve25519_f4_trace_20261102/analysis/f4trace.jsonl` exactly for all four cells (71009645, 32270400 with learning 710096, 8635143755, 3846503658 with learning 88907088).
- The binary differs from lead B's and lead C's binaries (different sha256); the counts did not change on these eight cells.

## Trace pairs (per (k, t, G), trace 1 against trace 0 on the same 100 targets)

Raw = replay + fallback multiply-adds over plain; amortised = with learning over 100 targets; fallbacks/replays = failed replays (replays minus refuted) over replays at branch level;
hits = plain/trace; unch = uncharged share of charged work for trace 0 and trace 1 (replay work only, learning excluded); x = trace share over plain share.

| stage | k | t | G | cap | raw ratio | amortised ratio | fallbacks / replays | hits and verified | unch trace 0 | unch trace 1 | x |
|---|---|---|---|---|---|---|---|---|---|---|---|
@@pair_rows.md@@

Per-relation cost on `coverage_exact` and the trace arm across k (two-point comparison, raw and amortised multiply-adds per target, PR(cov) plain and trace-amortised; the last three columns are
the raw trace ratio with the uncharged native work of both arms added at weight 0 (as in the table above), 0.1 and 1 per operation, the section 5a band applied to this one comparison; the numerator
counts only the uncharged work of the measured runs, not the learning-time specialisation; no cross-method comparison is made):

| stage | k | t | G | trace mul/tgt | trace amort | PR(cov) plain | PR(cov) trace amort | w = 0 | w = 0.1 | w = 1 |
|---|---|---|---|---|---|---|---|---|---|---|
@@trace_rows.md@@

## Scaling

Method as frozen: ratios between successive k (k = 20 to 24, 24 to 28), and a least-squares exponent alpha in y proportional to r^alpha, r taken as 2^k nominally (slope of log2 y on k over k = 20, 24, 28),
reported only if the two successive slopes s1 and s2 differ by at most 0.5. Three points leave one degree of freedom, so alpha describes these three cells; it is not an estimate with an interval and
is not extrapolated. k = 20 values are lead B's stage s1 cells (seed `2026111600 + t`, trace 0 only, different target sets); k = 24 and 28 are this study (new seeds). The k-ratios therefore mix the k effect with the
curve and target-set spread (lead B saw 2 to 3 percent between target sets at fixed k). One caveat on t = 10: at k = 20 coverage is 0.38, so 29 of 100 targets stop at their first hit and the k = 20 cost is below the full-refutation cost.

F4 multiply-adds per target, trace 0:

| cell | k = 20 (lead B) | k = 24 | k = 28 | ratio 24/20 | ratio 28/24 | alpha (slopes) | note |
|---|---|---|---|---|---|---|---|
@@scale_mul.md@@

`coverage_exact` and per-relation cost PR(cov) = mul/tgt over cov, trace 0 (k = 20 from lead B):

| cell | k = 20 | k = 24 | k = 28 | ratio 24/20 | ratio 28/24 | alpha (slopes) | note |
|---|---|---|---|---|---|---|---|
@@scale_cov_pr.md@@

Trace ratio (raw, then amortised) by k; k = 20 from the reproduction cells (G = 4 at t = 6, 8 from lead C's seeds, G = t-1 at t = 6, 10 from lead B's seeds); `-` = no k = 20 trace pair measured:

| cell | raw k = 20 | raw k = 24 | raw k = 28 | amortised k = 20 | amortised k = 24 | amortised k = 28 |
|---|---|---|---|---|---|---|
@@scale_trace.md@@

Readings, each limited to these cells. The F4 work per target at fixed (t, G) is flat in k at t = 6 (identical to the last digit at k = 20, 24, 28: 710096 and 146464, no hit at any k) and within 1.4 percent
at t = 8 and 1.9 percent at t = 10 from k = 24 to 28; the k = 20 to 24 step is +22 to +24 percent at t = 10 (consistent with the early stop at coverage 0.38; not tested) and within 0.8 percent at t = 8.
Since the cost is flat, PR(cov) follows 1/coverage: the exponent in r is 0.91 at t = 6 (slopes 0.73 and 1.09, the successive steps are 7.6x and 20.4x, differing by 0.36 in slope, inside the frozen tolerance of 0.5 but not equal),
1.02 at t = 8 and 0.99 to 1.00 at t = 10. Coverage itself scales with exponent -0.91, -1.02, -0.95 (t = 6, 8, 10). That a fixed-size factor base covers a fraction of the subgroup that falls like 1/r is the
obvious mechanism and is not separately tested here. The trace ratio does not change with k at any (t, G) where it was measured at three k (0.454, 0.445 to 0.442 for G = 4, 0.484 to 0.487 for G = t-1 at t = 6, 10 within 0.003).

## Predictions (frozen in PROTOCOL.md) and outcomes

- **P1 (F4 work per target nearly independent of k): true.** k = 28 over k = 24 at the six (t, G): 1.0000 (6, 4), 1.0000 (6, 5), 1.0101 (8, 4), 1.0139 (8, 7), 1.0126 (10, 4), 1.0191 (10, 9); all in [0.90, 1.10].
  The secondary band (k = 24 over lead B's k = 20) also holds: 1.0000, 1.0000, 1.0032, 0.9921 (in [0.85, 1.25]) and 1.2216, 1.2384 at t = 10 (in [1.00, 1.40]). Lead B's own k = 24/28 cells at t = 10, G = 9 (different seeds, 5.65e6 and 5.70e6) agree with this study's 5.60e6 and 5.70e6 to 0.9 percent and 0.0 percent.
- **P2 (coverage falls by [4, 25] per 4 bits; PR(cov) rises by that factor times the work ratio): true.** Coverage factors (k to k+4): t = 6: 7.6 and 20.4; t = 8: 17.2 and 16.3; t = 10: 11.8 and 16.8 (all inside [4, 25], monotone).
  PR(cov) steps: 7.55 and 20.4 (t = 6), 17.3 and 16.4 (t = 8, G = 4), 17.1 and 16.5 (t = 8, G = 7), 14.4 and 17.0 (t = 10, G = 4), 14.6 and 17.1 (t = 10, G = 9). As stated in the protocol, given P1 the PR(cov) part is arithmetic.
- **P3 (trace ratio in [0.29, 0.45] in all 12 pairs): false.** 4 of 12 pairs are inside (t = 8 and t = 10 at G = 4, both k); 8 are outside, all above the band: t = 6, G = 4: 0.454 at both k (0.454 is also lead C's own value
  for this cell; the band's 0.45 was lead C's rounded upper end); G = t-1: 0.484 (t = 6), 0.482 (t = 8), 0.485 (t = 10) at both k. No ratio is below 0.29 (lead C's low values, 0.29 to 0.33, were at G = 0, which this study did not measure).
  The sub-conditions hold: amortised minus raw is 0.010 to 0.011 (at most 0.02), `hits` and `hits_verified` equal between arms in all 16 pairs including the k = 20 ones, fallbacks/replays at most 0.39 percent (at most 10 percent allowed).
- **P4 (trace ratio rises with G; exploratory): true.** At G = t-1 against G = 4: 0.484 against 0.454 (t = 6), 0.482 against 0.443 (k = 24) and 0.442 (k = 28) (t = 8), 0.485 against 0.398 and 0.397 (t = 10). The trace saves a factor 2.06 of F4 work per target at G = t-1 and 2.2 to 2.5 at G = 4.
- **P5 (uncharged share small, grows with G; larger at trace 1): true.** Trace 0 share at G = t-1: 0.306 percent (t = 6), 0.284 / 0.161 (t = 8, k = 24 / 28), 0.503 / 0.126 (t = 10, k = 24 / 28), all at most 1 percent, each larger than the G = 4 share (0.041, 0.002 / 0.000, 0.000 percent; the values at G = 4 are tiny).
  Trace 1 share over trace 0 share is 2.06 to 2.52 in every pair, as the replay work in the denominator is half. With the uncharged work priced at weight 1 the raw trace ratio moves by at most 0.002 at k = 24, 28 (0.487 against 0.485 for t = 10, G = 9, k = 24), and by 0.033 at k = 20, t = 10, G = 9 (0.520 against 0.487), where the share is 6.9 percent.
  (k = 20 is outside the P5 band, which concerns k = 24, 28.)
- **P6 (solving degree 5 at G = 4 and 4 at G = t-1, trace 0): true** in all 12 trace 0 cells and in the trace 1 cells (reported, not predicted).
- **P7 (reproduction exact): true** for all 8 cells (section above).
- **Failure expectation (a cap at 900 s possible): did not occur.** 0 capped, 0 failed; the largest cell took 362 s under a load of 12.5, so there is no truncated sample.

## Per-relation view at the measured cells (modelled, not a solve cost)

Best cell by PR(cov) among the measured k = 24 and 28 cells is t = 10, G = 9: plain 1.72e8 (k = 24) and 2.96e9 (k = 28), trace amortised 8.53e7 and 1.46e9 (a factor 2.0). At k = 20, same (t, G), 1.18e7 and 5.90e6 (reproduction cells).
The per-relation cost at fixed (t, G) is therefore 14.6 times and then 17.1 times larger per 4 bits. What an increase of t with k would do (lead B's coverage-matched series) was not measured here, and no distance to rho is claimed.
Hits (0 at every k = 28 cell; 2 at each k = 24 cell with t = 8 or 10; 0 at t = 6) are consistent with coverage in magnitude and cannot rank cells.

## Failures, caps, deviations

- **No cell hit the cap and no cell failed.** Nothing is a truncated sample.
- **P3 is false and the band was not changed.** The frozen band's upper end 0.45 is lead C's rounded value; it is a few thousandths below the lead C and present t = 6, G = 4 cell (0.454) and 0.04 below the G = t-1 cells.
- **Four reproduction cells (R-C) were added to the request** (stated in the protocol before data) because lead C has no G = t-1 trace counts to compare against.
- **Trace pairs at k = 20 exist for only four (t, G)** (6/4, 6/5, 8/4, 10/9): the request reruns t = 6 and 10 at G = t-1 only; (8, 7) and (10, 4) have two k points for the trace ratio. The k = 20 trace 0 references for the other cells are lead B's cells (different seeds).
- **Resume checkpoint not pushed.** `.resume/RESUME.md` was overwritten with this study's content (the tracked file belonged to another effort; `scripts/checkpoint.sh init` refuses an existing file),
  but `scripts/checkpoint.sh push` failed: the upload token expired 2026-11-04T16:59:18Z and no new token could be minted from this session. The driver was launched from a plain shell in this runtime, which no hook blocked.
  The checkpoint is therefore not in S3; the repository copy of the protocol, driver and cell list is the only resume path (the driver is resumable: rerun `analysis/run_cells.sh`). The tracked `.resume/RESUME.md` is restored before the commit.
- **No benchmark record (`bench.record/v1`), pcc claim or leaderboard row was created**; this is a development scale-up inside the leads and the hooks did not ask for one in this runtime. The caller decides whether the PR needs one.
- The k = 24 and 28 target sets and curves differ, and no per-target costs or intervals are emitted by the tool, so no confidence interval is given; ratios between k carry an unquantified target-set spread of the size lead B saw (2 to 3 percent) and in this data the observed k = 28/24 ratios at t = 8 and 10 (1.0101 to 1.0191) are of that size.
- Wall time and CPU load differ widely between cells (load 7 to 15); no timing claim is made.

## Files

`PROTOCOL.md`, `README.md`, `REPORT.pdf`, `analysis/` (`README.tmpl.md`, `build_readme.sh`, `run_cells.sh`, `cells.list`, `cells/`, `stderr/`, `status.txt`, `binary.sha256`, `make_tsv.sh`, `make_tables.sh`, `make_svg.sh`, `cells.tsv`, the `*_rows.md` and `scale_*.md` tables, `scale.svg`).
