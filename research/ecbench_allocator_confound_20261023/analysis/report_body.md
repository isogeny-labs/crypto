# Allocator confound study: PR #2455 against its merge-base, 2026-10-23

Study `research/ecbench_allocator_confound_20261023`, preregistered in `PREREGISTRATION.md` (commit 6276977d5) before any data, amended once in `AMENDMENT_1.md` (exploratory arms, written before their data). Observations only: wall time is descriptive on a shared host and never the headline (AGENTS.md section 6). Every number below is a recorded value or a plain order statistic of recorded values (jq, `analysis/summarise.sh`), or an `ecbench compare`, `band` or `verify` output; Python only drew the graphs.

## Headline

- **Counts are identical (exact comparison).** For 10 base-versus-variant session pairs every count-bearing field of every record matches: 72 of 72 (smoke) and 66 of 66 (claim) records per pair, 0 differences, 0 unmatched. All 24 per-arm `ecbench compare --b-dir` read-outs give ops ratio 1.0, interval [1, 1], `same_seeds` true. A tampered copy is flagged by the same script (negative control).
- **The PR binary reads a much shorter solve wall on the small-set arms (measured, descriptive).** smoke-prime, 8 rho/BSGS/kangaroo arms, n = 12 per arm per binary: pooled median 5.26 to 6.77 ms on the merge-base binary, 0.137 to 0.330 ms on the PR binary, ratio 18.4 to 38.8; in every arm all 12 PR-binary runs lie below the merge-base median. The index-calculus arm `ic-mitm` (about 1.9 ms) does not move (ratio 1.03, 58% of PR runs below).
- **The IC online window on claim-koblitz is not distinguished (measured, descriptive).** n = 40 per binary: median 127.1 us on the merge-base binary against 119.1 us on the PR binary (ratio 1.067, 65% of PR runs below the base median); by order replicate 1.135 and 0.983. The PR text's "about 15% lower" is not reproduced; the preregistered verdict is inconclusive. The whole solve of `rho` and `rho-strong` on claim-koblitz is 23.6 and 38.0 times shorter, as in smoke.
- **Attribution, tested by scratch builds of the PR source (measured, exploratory).** Removing only `malloc_trim(0)` leaves the PR binary just as fast; removing only the new pre-solve `SolveProbe::read` (the `/proc` reads) leaves it just as fast; removing both returns every arm to the merge-base level (ratio 0.75 to 1.05). Either one consolidates the allocator before the clock opens. The trim removes the confound and does so by design; in this build it is not the only thing that does.
- **Audits.** All 14 sessions pass `ecbench verify --replay-all --exit-code` (the PR binary as auditor) and the CI form `--replay 0 --allow-interrupted --exit-code`.
- **Gap to rho is unchanged and is not the point.** On claim-koblitz (Koblitz a=1, n=17) the section 5a band gives IC against the matched `rho-strong` S of 16.85 against 4.00 (w = 0), 17.28 against 4.14 (0.1), 21.16 against 5.36 (1): rho-strong ahead at every price, IC 4.21, 4.17 and 3.95 times it. This study corrects a measuring instrument; it is not an index-calculus improvement.

## Design

| arm | what it is | binary sha256 (prefix) |
|:--|:--|:--|
| `base` | merge-base `3a1ea541d`, `git archive`, own target dir | a59c4014ff52 |
| `fix` | `ecbench/fix-runner` source at `63fede816` (the PR; later branch commits touch no source) | 26f25d9fa47a |
| `fix-notrim` | `fix` with `malloc_trim(0)` replaced by `Some(0)` (`analysis/notrim.patch`) | in `BINARIES.txt` |
| `fix-noprobe` | `fix` with `SolveProbe::read` reading nothing (`analysis/noprobe.patch`) | in `BINARIES.txt` |
| `fix-notrim2` | both removed (`analysis/notrim2.patch`) | in `BINARIES.txt` |

Specs: committed `docs/ecbench/specs/smoke-prime.json` (20-bit prime-order curve, 9 arms including the A/A control `rho-neg-aa`, 2 targets, 3 rounds + 1 warm-up) and the new `specs/claim-koblitz-r10.json` (the committed `claim-koblitz.json` with rounds 1 to 10: Koblitz a=1 n=17, 2 public targets, arms `rho-strong`, `rho`, `ic`). `base` and `fix` were run in ABBA order (`base`, `fix`, `fix`-rev, `base`-rev) to cancel drift and pooled per binary: smoke n = 12, claim n = 40 per arm. The three ablations ran once per spec after the ABBA sessions (smoke n = 6, claim n = 20). Every session: `ecbench run --wait --allow-busy --cpus auto --quiet`, commands in `run_all.sh` and `logs/`.

Host: one shared 4-vCPU Xeon 2.10 GHz VM (`ecbench host` class ECBENV3had995aa2df0f) with other agents building and testing throughout (load average 2.7 to 5.1 at session starts, the cargo lock held every time; `logs/run_all.log`). No run reached L3. Isolation levels recorded per record:

{{table t6_levels|group|levels (record counts)}}

The ablation sessions with `SolveProbe::read` stubbed carry L0 (blocker recorded: "the child did not report its thread count"), and every PR-source record whose solve is under 1 ms carries the blocker "solve too short to admit wall time" (grading v3), which is why the PR binary's smoke levels differ from the merge-base binary's (graded under v2). Their wall values are recorded and reported as descriptive only.

## Results

### Counts: identical across binaries

{{table t4_counts|base session|compared session|records matched|differing|unmatched|result}}

Fields compared per record (`analysis/counts_diff.sh`): status, recovered logarithm, `matches_target`, `algorithm_seed`, `run_id`, workload and method hashes, `total_gae`, S, `lower_bound`, `unpriced`, per-phase adds, doubles, scalar multiplications, GAE and native counters, and all counters. `ecbench compare` per arm (`comparisons/`, `analysis/ops_compare.tsv`): ratio 1.0, interval [1, 1], `same_seeds` true in all 24. `analysis/counts_diff_negative_control.log`: a copy of `fix-smoke` with one `bsgs` record's counters raised by 1 is reported as 1 differing record (exit 1), so the check can fail.

Allocator block (`record.allocator`): every record of a PR-source binary carries `pre_solve_trim: true`; `base` records carry no block.

{{table t9_alloc|group|measured records|with pre_solve_trim true|no allocator block}}

### Smoke: solve wall by arm

{{table t1_smoke|arm|n base|n fix|base median ms|fix median ms|base / fix|share of fix runs below base median|ns per GAE base|ns per GAE fix}}

Wall values are `time.solve_wall_ns` of verified measured records, pooled over the two orders; ns per GAE is the median of the per-record `solve_wall_ns / total_gae`. The A/A pair `rho-neg` and `rho-neg-aa` (same method) differ by a factor 1.160 on the merge-base binary and 1.003 on the PR binary (table below): the arm-dependent bias the PR describes shows up as an A/A disagreement before and not after, descriptively. The grader of the PR binary (v3) refuses to admit wall time from a solve under 1 ms, so the PR-binary smoke figures are recorded but not admitted wall; no `ecbench` admitted-wall claim is made.

{{table t8_aa|group|rho-neg median ms|rho-neg-aa median ms|aa / neg}}

{{img figures/fig1_solve_wall_by_arm|Recorded solve wall time per arm (dots, log scale), pooled median and quartile bar, n above each bar; merge-base binary, PR binary and three scratch ablations. Sessions in `sessions/`; data in `analysis/summary.json`. Descriptive wall, shared host, L1 or L2.}}

### claim-koblitz: solve and online window

{{table t2_claim|arm|window|n base|n fix|base median us|fix median us|base / fix pooled|order 1|order 2 (rev)|share of fix runs below base median}}

One outlier in the PR binary's `rho-strong` solve (about 3.3 ms against a 0.15 ms median) is in Figure 1 and is kept.

Section 5a band, `ecbench band --dir sessions/fix-claim --ranks` (identical JSON for `base-claim`; the counts do not depend on the binary), S at w = 0, 0.1, 1 GAE per unpriced native operation:

{{table t7_band|arm|S at 0|S at 0.1|S at 1}}

Verdict across the band: `rho-strong` < `ic`, `rho` < `ic`, `rho` < `rho-strong` (order holds at every weight; `uncounted` empty for all three arms). Distance remaining of IC to the matched strong rho on this curve: 4.21, 4.17 and 3.95 times at 0, 0.1 and 1.

### Attribution: which change removes the cost

{{table t3_ablation|arm (smoke, median ms)|base|fix (trim + probe)|fix-notrim (probe only)|fix-noprobe (trim only)|fix-notrim2 (neither)}}

{{table t3b_claim_ablation|arm (claim, median us)|window|base|fix|fix-notrim|fix-noprobe|fix-notrim2}}

{{img figures/fig2_base_over_variant|smoke-prime, pooled median solve wall of the merge-base binary divided by each variant's (log scale; 1 = no change). n = 12 per arm for base and fix, 6 for the ablations. Data: `analysis/ratios.json`.}}

Read: with the trim or the probe present the PR binary reads 0.12 to 0.33 ms on the eight non-IC smoke arms; with neither it reads 5.8 to 8.3 ms, as the merge-base binary does. The PR's own source comment places the cost in `malloc_consolidate` of fastbins left by set-up; this study measured that removing both consolidation sources restores the cost, and did not trace `malloc_consolidate` itself (no tracer on this VM), so the mechanism named is the PR's and the probe's allocation is a candidate explanation for why the trim looked redundant, not an observation.

## Verdicts against the preregistration

| id | prediction | result |
|:--|:--|:--|
| H0 counts identical | every compared field equal | **holds**: 10 pairs, 0 differences |
| H1 smoke wall | 7 of 8 non-IC arms with base/fix >= 2 and fix median < 1 ms | **supported**: 8 of 8 (ratio 18.4 to 38.8; fix 0.137 to 0.330 ms) |
| H2 claim IC online | base/fix > 1 in each order and >= 70% of fix runs below base median | **inconclusive**: pooled 1.067, orders 1.135 and 0.983, 65% below |
| H3 audit | every session passes replay-all and the CI form | **holds**: 14 of 14 |
| H4 attribution (exploratory) | notrim near base if the trim causes the drop | **not as predicted**: `fix-notrim` is at the `fix` level; AMENDMENT_1: probe also consolidates; with neither the cost returns to base (0.75 to 1.05) |
| noise floor | A/A ratio per binary | 1.160 (base), 1.003 (fix) |

Audits (`ecbench verify --replay-all --exit-code`, receipts in `audits/`, not inside sessions):

{{table t5_audits|session|result|session id}}

## Scope and limits

- Two sizes (a 20-bit prime-order curve, Koblitz a=1 n=17), one shared 4-vCPU VM below L3, glibc 2.39, one pair of commits. It says nothing about other allocators, larger solves where consolidation is negligible, or L3 hosts.
- `base` and `fix` differ by the whole PR (grading v3, isolation probing, PMU group, session writing); the ablations are scratch builds that do not exist in the repository and change only the lines named in their patches.
- The two binaries ran in different sessions: `ecbench compare` refuses wall time across sessions by design, so the wall comparison is a descriptive summary of recorded values, not an admitted ecbench wall result.
- The ABBA order cancels linear drift only; other agents' builds ran throughout and the cargo lock was held at every session start.
- The PR's "about 15% IC online window" figure is from a session not in this study; this study's claim-koblitz-r10 does not distinguish the two binaries on that window.

## Validation gate status

`docs/VALIDATION_GATE.md` requires an independent replay certificate and an independent validator report. Both were produced on the same host by the same agent that ran the sessions, so `independent: true` cannot honestly be asserted. A candidate `VALIDATION.json` with its replay certificate and validator report is in `validation/` with `independent: false`; `scripts/validation_gate.py evidence validation` refuses it on exactly that. No row is added to `benchmarks/README.md`. A replay of `sessions/` from a host of another class, or by a validator who did not run the study, would complete it; the matched-rho reference and the section 5a band (above) are already in place.

## Files

`PREREGISTRATION.md`, `AMENDMENT_1.md`, `specs/`, `sessions/` (14, unedited), `audits/`, `comparisons/`, `analysis/` (scripts, `summary.json`, `ratios.json`, patches, tables), `figures/`, `reports/run-*/`, `BINARIES.txt`, `logs/`, `validation/`. Benchmarks copy: `benchmarks/20261023-ecbench-allocator-confound/` (graphs and PDFs, no README row).
