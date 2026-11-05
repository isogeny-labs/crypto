#set page(paper: "a4", margin: (x: 1.6cm, y: 1.6cm), numbering: "1")
#set text(size: 9pt)
#set table(inset: 3pt, stroke: 0.4pt + luma(150))
#show raw: set text(size: 8pt)
= ecbench statistics v2 (PR \#2442): before/after evidence
<ecbench-statistics-v2-pr-2442-beforeafter-evidence>
Date 2026-11-05. Study directory: `research/ecbench_statistics_v2_20261105/`. PDF: #link("REPORT.pdf")[`REPORT.pdf`].
Scope: what PR \#2442 (merge `7edc35a9fb`: linearised t(K-1) ops interval, ns/GAE wall statistic, censored
timeouts/exhausted runs, tail flags, A/A bias correction) changed in the numbers `ecbench compare` and
`ecbench table` print for sessions that were already committed. \#2442 landed without this record; this is it.
Nothing here is a new run, a speed result or an IC leaderboard row.

== Method (all numbers below are measured from the files named; derived ones say how)
<method-all-numbers-below-are-measured-from-the-files-named-derived-ones-say-how>
- Old statistic: `ecbench` built from `7edc35a9fb^1` \= `b58840ef4f` (pre-merge, schema `ecbench.comparison/v1`) in a scratch
  worktree (the build needed five `include_str!` data files added to the sparse checkout; nothing else changed).
  Binary sha256 prefix `d2dc241024a419e7`.
- New statistic: `ecbench` built from this branch’s base `a4dbbd282d` (current main, `ecbench.comparison/v2`), prefix `e5273d184198c976`.
- Both run `compare --json` on the same committed `records.jsonl` (no rerun of any session, default seed 20261002, 4000 resamples):
  #link("extract.sh")[`extract.sh`] -\> `data/raw/{old,new}/*.json`, `data/comparisons.tsv`, `data/tables/{old,new}.*.md`. Re-running
  `extract.sh` reproduced the raw JSON byte-for-byte.
- Sanity check (verified): for every pair, the old ratio equals the new `verified_only_ratio` (e.g. 1.3509 for sub8-da-sat), so the
  two binaries agree on the point estimate over verified pairs; every difference below is the interval, the censoring or the flags.
- Sessions: yield sweep koblitz `ECBS1h0d1ce1d04e54` (16 workloads, 7 and 4 timeouts on the two `da-sat` arms);
  `koblitz-followup-3` `ECBS1hb0cb85a662d8` (16 workloads, 24 of 48 `ic-sub6-mitm` runs exhausted, has an A/A arm);
  ghs `panel-01` `ECBS1h5a490d269b02` (40 workloads incl. 13 exhausted `rho-frob`); sat\_pdp `pilot` `ECBS1h2cae65a6c917`
  (2 workloads, timeouts, K \= 2); yield sweep prime `ECBS1h548d49a71881` (12 pairs, clean).
  Paths in `data/pairs.txt`.

== 1. Ops ratio: interval width and censoring (\[fig 1\])
<ops-ratio-interval-width-and-censoring-fig-1>
Ratio \= sum S\_B / sum S\_A in ecbench.gae; intervals 95%. Old \= percentile cluster bootstrap over verified pairs; new \= linearised
t(K-1) with censored pairs included.

#align(center)[#table(
  columns: 4,
  align: (col, row) => (left,left,left,left,).at(col),
  inset: 6pt,
  [session: B / A], [old ratio \[interval\]], [new ratio \[interval\]], [new status],
  [yield koblitz: sub8-mitm / rho-strong],
  [45.22 \[37.34, 49.17\]],
  [45.22 \[39.74, 51.44\]],
  [complete, K\=16],
  [yield koblitz: sub8-da-f4 / rho-strong],
  [1.796 \[1.406, 2.625\]],
  [1.796 \[1.289, 2.502\]],
  [complete],
  [followup3: ic-sub8-mitm / rho-strong],
  [76.97 \[55.96, 100.0\]],
  [76.97 \[57.01, 103.9\]],
  [complete],
  [ghs: rho-neg / bsgs-neg],
  [4.185 \[3.728, 4.647\]],
  [4.185 \[3.725, 4.702\]],
  [complete, K\=40],
  [prime: abs16-mitm / rho-neg],
  [9.398 \[6.161, 14.15\]],
  [9.398 \[5.702, 15.49\]],
  [K\=12],
  [prime: abs64-subtract / rho-neg],
  [174.1 \[98.3, 300.5\]],
  [174.1 \[89.65, 338.0\]],
  [K\=12],
  [sat pilot: ic-mitm / rho-strong],
  [102.3 \[99.07, 105.8\]],
  [102.3 \[67.33, 155.5\]],
  [K\=2, df 1, `indicative`],
  [yield koblitz: sub8-da-sat / rho-strong],
  [1.351 \[1.202, 1.547\] (9 of 16 pairs)],
  [1.429 \[1.241, 1.646\]],
  [lower bound, 7 censored],
  [yield koblitz: sub6-da-sat / rho-strong],
  [1.892 \[1.259, 3.217\] (12 pairs)],
  [2.486 \[1.442, 4.288\]],
  [lower bound, 4 censored],
  [followup3: ic-sub6-mitm / rho-strong],
  [6.981 \[6.252, 7.730\] (24 of 48)],
  [#strong[648.2 \[419.6, 1001\]]],
  [lower bound, 24 exhausted pairs censored],
  [ghs: rho-frob / bsgs-neg],
  [3.512 \[2.779, 4.319\] (27 of 40)],
  [212.0 \[123.2, 365.1\]],
  [lower bound, 13 exhausted pairs censored],
  [sat pilot: ic-buchberger / rho-strong],
  [no ratio (0 verified pairs)],
  [bound 0 \[0, 0\], 2 censored],
  [uninformative: no verified B run to convert wall time to cost],
)
]

Reading (widths hi - lo derived from the table above): on uncensored rows the new interval is 13.1x wider at K \= 2 (88.17 against 6.73,
the old one being the percentile bootstrap that under-covers there), 1.23x wider at K \= 12 (abs16 9.788 vs 7.989; abs64 248.4 vs 202.2),
1.06x at K \= 16 for followup3 sub8-mitm (46.89 vs 44.04) and 1.00x for yield sub8-mitm (11.70 vs 11.83) and sub8-da-f4 (1.213 vs 1.219),
and 1.06x at K \= 40 (ghs rho-neg 0.977 vs 0.919). Censoring is the large effect: dropping exhausted/timed-out runs understated `ic-sub6-mitm/rho-strong`
by 648.2/6.981 \= 92.9x and `rho-frob/bsgs-neg` by 212.0/3.512 \= 60.4x (point estimates; both are lower bounds, so the true ratio is higher still).
The "648" figure of the PR stands: it is this row, 648.2 \[419.6, 1001\], bound `lower`, from the same records. The yield-sweep
bound in the PR body was withdrawn and is not used here.

== 2. Coverage simulation (\[fig 2\])
<coverage-simulation-fig-2>
The PR’s coverage claims are backed by the Rust tests in `src/cryptanalysis/ecbench/stats.rs` (the `coverage.py` they cite
is not committed, and Python is not allowed here). Reproduced (`data/stats_tests.log`): all 8 `stats::tests` pass,
and the committed test prints linearised coverage 0.9573 / 0.9480 / 0.9470 at K \= 2 / 4 / 8 (ratio scale) and 0.9557 / 0.9460 / 0.9500 (log scale),
matching README section 8 (0.957, 0.948, 0.947 / 0.956, 0.946, 0.950).

This study adds one ignored test, `coverage_table_for_report` (same cluster model, 3 rounds per workload, 2000 replicates per K,
400 bootstrap resamples; Monte-Carlo s.e. about 0.005), comparing the new interval with the old one at K \= 2, 3, 4, 8
(`data/coverage.tsv`):

#align(center)[#table(
  columns: 3,
  align: (col, row) => (right,left,left,).at(col),
  inset: 6pt,
  [K], [linearised t, log scale: coverage / median width], [percentile cluster bootstrap: coverage / median width],
  [2],
  [0.9395 / 16.64],
  [0.8580 / 1.631],
  [3],
  [0.9585 / 2.819],
  [0.9055 / 1.518],
  [4],
  [0.9515 / 1.802],
  [0.9275 / 1.323],
  [8],
  [0.9500 / 0.983],
  [0.9620 / 0.980],
)
]

Differences from the README text, stated as measured: the bootstrap’s K \= 3 coverage here is 0.906 (README says 92-93% "at three or
four"; K \= 4 gives 0.928); and at K \= 2 the linearised log-scale coverage is 0.940 in this run against 0.956 in the committed test (different seed stream and
2000 against 3000 replicates; a 0.016 gap is about 3 Monte-Carlo s.e., so K \= 2 is slightly below nominal on the log scale in this run). The price of honest coverage at small K is width: 10x the bootstrap at K \= 2,
the same at K \= 8. The wall simulation quoted in README section 8 (5 workloads x 3 rounds, "deep dive") is not committed and is
#strong[not reproduced] here; it is not relied on.

== 3. Wall statistic and the A/A gate (\[fig 4\])
<wall-statistic-and-the-aa-gate-fig-4>
v1 headline: median per-pair solve-wall ratio, whose bootstrap interval was often a point (e.g. \[350714.0, 350714.0\] for sub8-da-sat,
or the median at an edge: \[40.28, 55.03\] with median 55.03 for followup3 sub8-mitm). v2 headline: ratio of ns per GAE with a linearised
t interval over pairs. These are different estimands (total wall versus cost per operation) and often disagree in direction: yield koblitz
sub8-mitm/rho-strong is 20.26 (v1 median) but 0.627 \[0.489, 0.804\] ns/GAE (v2), because the arm does 45x the operations at lower cost per operation.
The `da-sat` arms’ ns/GAE (5.1e7 and 8.8e6 ns per GAE) reflect unpriced SAT-solver work, and the arms are flagged as upper bounds.

A/A gate, followup3 (A/A pair `rho-strong`/`rho-strong-aa`, "includes arm A"): bias 1.087 \[0.967, 1.223\]. For `ic-sub8-mitm` the
ns/GAE ratio 1.361 \[1.229, 1.506\] corrects to 1.251 \[1.075, 1.456\] and stays outside noise (true; v1 rule also true). For `ic-sub6-mitm`
1.248 \[1.145, 1.360\] corrects to 1.147 \[0.997, 1.321\]: `outside_noise` false, where the legacy v1 rule says true.
This is the one gating flip in the sample.

== 4. Flags (\[fig 3\])
<flags-fig-3>
Heavy-tail flag (largest run \> 25% of an arm’s total S). Counting `ecbench table` rows of the five sessions (`data/tables/new.*.md`):
with 4 verified runs per row 71 of 71 rows are flagged, with 2 runs 6 of 6, with 8 runs 1 of 12, with 24 runs 1 of 12. With n runs the largest
share is at least 1/n, so at n \<\= 4 the flag fires on every row and carries no information. In `compare`, the arm-level flag fires only in the K \= 2
pilot (top-run share 0.517 with 2 runs); none of the 16-40-workload arms is flagged (top shares 0.03 to 0.22). Recommendation, not
implemented here: make the flag’s threshold depend on n (or require n \>\= 8). `survivors only` flags: 3 rows in ghs panel-01 (those with
unverified runs); the old table printed their means with no mark. `table` per-row intervals are now t(n-1) and marked `indicative` below five
runs (for example rho-frob f2m15: old \[2.605, 7.645\], new \[1.758, 10.635\]).

== Limits and gates
<limits-and-gates>
- Sessions are historical; wall comparisons here are as isolated as the sessions were (levels L0-L2 in the tables). No new
  measurement, no speedup, no matched-rho ranking is claimed. Rows that bound a ratio are bounds, not coverage statements.
- Validation gate (`docs/VALIDATION_GATE.md`): it governs rows in `benchmarks/README.md` and the IC leaderboard. This study is a
  methodology comparison, produces no leaderboard row and no benchmark speed row, so no `VALIDATION.json` is created and `benchmarks/README.md`
  gets a documentary note (outside the admitted table), like the other unadmitted entries. `scripts/validation_gate.py check` is run in the commit.
- Not verified: the PR’s withdrawn yield\_sweep bound; the wall-jitter simulation; the full-session status of every comparison beyond `admissible: true`.
- Conductor was not available in this sandbox (hooks warn and allow edits); the source change is one `#[ignore]` test in `stats.rs`.

Files: `extract.sh`, `plot.py` (layout only), `data/`, `figures/*.{svg,png}`, `REPORT.typ`, `REPORT.pdf`.

= Figures

#figure(image("figures/fig1_ops_intervals.svg", width: 100%), caption: [Fig 1. Ops-ratio intervals, old v1 vs new v2, same sessions. Source data/comparisons.tsv.])
#figure(image("figures/fig2_coverage.svg", width: 100%), caption: [Fig 2. Coverage and median width, 2000 replicates per K. Source data/coverage.tsv.])
#figure(image("figures/fig3_tail_flag_vs_n.svg", width: 70%), caption: [Fig 3. Heavy-tail flag rate against verified runs per table row. Source data/tables.])
#figure(image("figures/fig4_wall_statistic.svg", width: 100%), caption: [Fig 4. Wall statistic, v1 median per-pair ratio vs v2 ns/GAE ratio. Source data/comparisons.tsv.])
