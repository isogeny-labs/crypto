#set page(paper: "a4", margin: (x: 1.6cm, y: 1.6cm), numbering: "1")
#set text(size: 9pt)
#set par(justify: false)
#set table(inset: 3pt, stroke: 0.4pt + luma(150))
#show table: it => { show raw: set text(size: 7pt); set text(size: 7pt); it }
#show raw: set text(size: 8pt)
= Allocator confound study: PR \#2455 against its merge-base, 2026-10-23

Study `research/ecbench_allocator_confound_20261023`, preregistered in `PREREGISTRATION.md` (commit 6276977d5) before any data, amended once in `AMENDMENT_1.md` (exploratory arms, written before their data). Observations only: wall time is descriptive on a shared host and never the headline (AGENTS.md section 6). Every number below is a recorded value or a plain order statistic of recorded values (jq, `analysis/summarise.sh`), or an `ecbench compare`, `band` or `verify` output; Python only drew the graphs.

== Headline

- *Counts are identical (exact comparison).* For 10 base-versus-variant session pairs every count-bearing field of every record matches: 72 of 72 (smoke) and 66 of 66 (claim) records per pair, 0 differences, 0 unmatched. All 24 per-arm `ecbench compare --b-dir` read-outs give ops ratio 1.0, interval [1, 1], `same_seeds` true. A tampered copy is flagged by the same script (negative control).
- *The PR binary reads a much shorter solve wall on the small-set arms (measured, descriptive).* smoke-prime, 8 rho/BSGS/kangaroo arms, n = 12 per arm per binary: pooled median 5.26 to 6.77 ms on the merge-base binary, 0.137 to 0.330 ms on the PR binary, ratio 18.4 to 38.8; in every arm all 12 PR-binary runs lie below the merge-base median. The index-calculus arm `ic-mitm` (about 1.9 ms) does not move (ratio 1.03, 58% of PR runs below).
- *The IC online window on claim-koblitz is not distinguished (measured, descriptive).* n = 40 per binary: median 127.1 us on the merge-base binary against 119.1 us on the PR binary (ratio 1.067, 65% of PR runs below the base median); by order replicate 1.135 and 0.983. The PR text's "about 15% lower" is not reproduced; the preregistered verdict is inconclusive. The whole solve of `rho` and `rho-strong` on claim-koblitz is 23.6 and 38.0 times shorter, as in smoke.
- *Attribution, tested by scratch builds of the PR source (measured, exploratory).* Removing only `malloc_trim(0)` leaves the PR binary just as fast; removing only the new pre-solve `SolveProbe::read` (the `/proc` reads) leaves it just as fast; removing both returns every arm to the merge-base level (ratio 0.75 to 1.05). Either one consolidates the allocator before the clock opens. The trim removes the confound and does so by design; in this build it is not the only thing that does.
- *Audits.* All 14 sessions pass `ecbench verify --replay-all --exit-code` (the PR binary as auditor) and the CI form `--replay 0 --allow-interrupted --exit-code`.
- *Gap to rho is unchanged and is not the point.* On claim-koblitz (Koblitz a=1, n=17) the section 5a band gives IC against the matched `rho-strong` S of 16.85 against 4.00 (w = 0), 17.28 against 4.14 (0.1), 21.16 against 5.36 (1): rho-strong ahead at every price, IC 4.21, 4.17 and 3.95 times it. This study corrects a measuring instrument; it is not an index-calculus improvement.

== Design

#table(columns: 3,
  table.header([*arm*], [*what it is*], [*binary sha256 (prefix)*], ),
  [`base`], [merge-base `3a1ea541d`, `git archive`, own target dir], [a59c4014ff52], 
  [`fix`], [`ecbench/fix-runner` source at `63fede816` (the PR; later branch commits touch no source)], [26f25d9fa47a], 
  [`fix-notrim`], [`fix` with `malloc_trim(0)` replaced by `Some(0)` (`analysis/notrim.patch`)], [in `BINARIES.txt`], 
  [`fix-noprobe`], [`fix` with `SolveProbe::read` reading nothing (`analysis/noprobe.patch`)], [in `BINARIES.txt`], 
  [`fix-notrim2`], [both removed (`analysis/notrim2.patch`)], [in `BINARIES.txt`], 
)


Specs: committed `docs/ecbench/specs/smoke-prime.json` (20-bit prime-order curve, 9 arms including the A/A control `rho-neg-aa`, 2 targets, 3 rounds + 1 warm-up) and the new `specs/claim-koblitz-r10.json` (the committed `claim-koblitz.json` with rounds 1 to 10: Koblitz a=1 n=17, 2 public targets, arms `rho-strong`, `rho`, `ic`). `base` and `fix` were run in ABBA order (`base`, `fix`, `fix`-rev, `base`-rev) to cancel drift and pooled per binary: smoke n = 12, claim n = 40 per arm. The three ablations ran once per spec after the ABBA sessions (smoke n = 6, claim n = 20). Every session: `ecbench run --wait --allow-busy --cpus auto --quiet`, commands in `run_all.sh` and `logs/`.

Host: one shared 4-vCPU Xeon 2.10 GHz VM (`ecbench host` class ECBENV3had995aa2df0f) with other agents building and testing throughout (load average 2.7 to 5.1 at session starts, the cargo lock held every time; `logs/run_all.log`). No run reached L3. Isolation levels recorded per record:

#table(columns: 2,
  table.header([*group*], [*levels (record counts)*], ),
  [`claim-base`], [L1x120], 
  [`claim-fix`], [L1x120], 
  [`claim-noprobe`], [L0x60], 
  [`claim-notrim`], [L1x60], 
  [`claim-notrim2`], [L0x60], 
  [`smoke-base`], [L1x60 L2x48], 
  [`smoke-fix`], [L1x102 L2x6], 
  [`smoke-noprobe`], [L0x54], 
  [`smoke-notrim`], [L1x48 L2x6], 
  [`smoke-notrim2`], [L0x54], 
)


The ablation sessions with `SolveProbe::read` stubbed carry L0 (blocker recorded: "the child did not report its thread count"), and every PR-source record whose solve is under 1 ms carries the blocker "solve too short to admit wall time" (grading v3), which is why the PR binary's smoke levels differ from the merge-base binary's (graded under v2). Their wall values are recorded and reported as descriptive only.

== Results

=== Counts: identical across binaries

#table(columns: 6,
  table.header([*base session*], [*compared session*], [*records matched*], [*differing*], [*unmatched*], [*result*], ),
  [`base-smoke`], [fix-smoke], [72], [0], [0], [identical], 
  [`base-smoke-rev`], [fix-smoke-rev], [72], [0], [0], [identical], 
  [`base-claim`], [fix-claim], [66], [0], [0], [identical], 
  [`base-claim-rev`], [fix-claim-rev], [66], [0], [0], [identical], 
  [`base-smoke`], [fix-notrim-smoke], [72], [0], [0], [identical], 
  [`base-claim`], [fix-notrim-claim], [66], [0], [0], [identical], 
  [`base-smoke`], [fix-notrim2-smoke], [72], [0], [0], [identical], 
  [`base-claim`], [fix-notrim2-claim], [66], [0], [0], [identical], 
  [`base-smoke`], [fix-noprobe-smoke], [72], [0], [0], [identical], 
  [`base-claim`], [fix-noprobe-claim], [66], [0], [0], [identical], 
)


Fields compared per record (`analysis/counts_diff.sh`): status, recovered logarithm, `matches_target`, `algorithm_seed`, `run_id`, workload and method hashes, `total_gae`, S, `lower_bound`, `unpriced`, per-phase adds, doubles, scalar multiplications, GAE and native counters, and all counters. `ecbench compare` per arm (`comparisons/`, `analysis/ops_compare.tsv`): ratio 1.0, interval [1, 1], `same_seeds` true in all 24. `analysis/counts_diff_negative_control.log`: a copy of `fix-smoke` with one `bsgs` record's counters raised by 1 is reported as 1 differing record (exit 1), so the check can fail.

Allocator block (`record.allocator`): every record of a PR-source binary carries `pre_solve_trim: true`; `base` records carry no block.

#table(columns: 4,
  table.header([*group*], [*measured records*], [*with pre_solve_trim true*], [*no allocator block*], ),
  [`claim-base`], [120], [0], [120], 
  [`claim-fix`], [120], [120], [0], 
  [`claim-noprobe`], [60], [60], [0], 
  [`claim-notrim`], [60], [60], [0], 
  [`claim-notrim2`], [60], [60], [0], 
  [`smoke-base`], [108], [0], [108], 
  [`smoke-fix`], [108], [108], [0], 
  [`smoke-noprobe`], [54], [54], [0], 
  [`smoke-notrim`], [54], [54], [0], 
  [`smoke-notrim2`], [54], [54], [0], 
)


=== Smoke: solve wall by arm

#table(columns: 9,
  table.header([*arm*], [*n base*], [*n fix*], [*base median ms*], [*fix median ms*], [*base / fix*], [*share of fix runs below base median*], [*ns per GAE base*], [*ns per GAE fix*], ),
  [`rho-neg`], [12], [12], [5.833], [0.25], [23.37], [100%], [6767], [291], 
  [`rho-plain`], [12], [12], [6.144], [0.277], [22.22], [100%], [5215], [245], 
  [`rho-frozen`], [12], [12], [6.091], [0.33], [18.43], [100%], [3805], [186], 
  [`rho-neg-aa`], [12], [12], [6.767], [0.25], [27.03], [100%], [7976], [267], 
  [`bsgs`], [12], [12], [5.392], [0.186], [28.98], [100%], [4975], [168], 
  [`bsgs-il`], [12], [12], [5.262], [0.162], [32.52], [100%], [10222], [267], 
  [`bsgs-neg`], [12], [12], [5.303], [0.137], [38.76], [100%], [7831], [186], 
  [`kangaroo`], [12], [12], [6.24], [0.196], [31.8], [100%], [4458], [123], 
  [`ic-mitm`], [12], [12], [1.891], [1.83], [1.03], [58%], [247], [221], 
)


Wall values are `time.solve_wall_ns` of verified measured records, pooled over the two orders; ns per GAE is the median of the per-record `solve_wall_ns / total_gae`. The A/A pair `rho-neg` and `rho-neg-aa` (same method) differ by a factor 1.160 on the merge-base binary and 1.003 on the PR binary (table below): the arm-dependent bias the PR describes shows up as an A/A disagreement before and not after, descriptively. The grader of the PR binary (v3) refuses to admit wall time from a solve under 1 ms, so the PR-binary smoke figures are recorded but not admitted wall; no `ecbench` admitted-wall claim is made.

#table(columns: 4,
  table.header([*group*], [*rho-neg median ms*], [*rho-neg-aa median ms*], [*aa / neg*], ),
  [`smoke-base`], [5.833], [6.767], [1.16], 
  [`smoke-fix`], [0.25], [0.25], [1.003], 
  [`smoke-notrim`], [0.236], [0.221], [0.936], 
  [`smoke-noprobe`], [0.277], [0.278], [1.005], 
  [`smoke-notrim2`], [6.57], [6.419], [0.977], 
)


#figure(image("figures/fig1_solve_wall_by_arm.svg", width: 100%), caption: [Recorded solve wall time per arm (dots, log scale), pooled median and quartile bar, n above each bar; merge-base binary, PR binary and three scratch ablations. Sessions in `sessions/`; data in `analysis/summary.json`. Descriptive wall, shared host, L1 or L2.])


=== claim-koblitz: solve and online window

#table(columns: 10,
  table.header([*arm*], [*window*], [*n base*], [*n fix*], [*base median us*], [*fix median us*], [*base / fix pooled*], [*order 1*], [*order 2 (rev)*], [*share of fix runs below base median*], ),
  [`ic`], [solve], [40], [40], [1412.7], [1354.2], [1.043], [1.165], [1.025], [68%], 
  [`ic`], [online], [40], [40], [127.1], [119.1], [1.067], [1.135], [0.983], [65%], 
  [`rho`], [solve], [40], [40], [5886.9], [249.1], [23.637], [25.125], [21.967], [100%], 
  [`rho`], [online], [40], [40], [5885.3], [245.6], [23.965], [25.462], [22.283], [100%], 
  [`rho-strong`], [solve], [40], [40], [5658.4], [148.7], [38.048], [39.385], [35.393], [100%], 
  [`rho-strong`], [online], [40], [40], [34.7], [38], [0.913], [1], [0.901], [43%], 
)


One outlier in the PR binary's `rho-strong` solve (about 3.3 ms against a 0.15 ms median) is in Figure 1 and is kept.

Section 5a band, `ecbench band --dir sessions/fix-claim --ranks` (identical JSON for `base-claim`; the counts do not depend on the binary), S at w = 0, 0.1, 1 GAE per unpriced native operation:

#table(columns: 4,
  table.header([*arm*], [*S at 0*], [*S at 0.1*], [*S at 1*], ),
  [`ic`], [16.852], [17.283], [21.16], 
  [`rho`], [1.541], [1.635], [2.478], 
  [`rho-strong`], [4.004], [4.14], [5.361], 
)


Verdict across the band: `rho-strong` < `ic`, `rho` < `ic`, `rho` < `rho-strong` (order holds at every weight; `uncounted` empty for all three arms). Distance remaining of IC to the matched strong rho on this curve: 4.21, 4.17 and 3.95 times at 0, 0.1 and 1.

=== Attribution: which change removes the cost

#table(columns: 6,
  table.header([*arm (smoke, median ms)*], [*base*], [*fix (trim + probe)*], [*fix-notrim (probe only)*], [*fix-noprobe (trim only)*], [*fix-notrim2 (neither)*], ),
  [`rho-neg`], [5.833], [0.25], [0.236], [0.277], [6.57], 
  [`rho-plain`], [6.144], [0.277], [0.25], [0.275], [6.738], 
  [`rho-frozen`], [6.091], [0.33], [0.281], [0.271], [6.594], 
  [`rho-neg-aa`], [6.767], [0.25], [0.221], [0.278], [6.419], 
  [`bsgs`], [5.392], [0.186], [0.188], [0.212], [6.461], 
  [`bsgs-il`], [5.262], [0.162], [0.14], [0.182], [5.769], 
  [`bsgs-neg`], [5.303], [0.137], [0.123], [0.14], [6.177], 
  [`kangaroo`], [6.24], [0.196], [0.172], [0.215], [8.342], 
  [`ic-mitm`], [1.891], [1.83], [1.866], [1.88], [1.906], 
)


#table(columns: 7,
  table.header([*arm (claim, median us)*], [*window*], [*base*], [*fix*], [*fix-notrim*], [*fix-noprobe*], [*fix-notrim2*], ),
  [`ic`], [solve], [1412.7], [1354.2], [1342.1], [1456.2], [1518.4], 
  [`ic`], [online], [127.1], [119.1], [113.5], [125.2], [120.3], 
  [`rho`], [solve], [5886.9], [249.1], [198], [296.4], [5638.7], 
  [`rho`], [online], [5885.3], [245.6], [194.1], [292], [5635], 
  [`rho-strong`], [solve], [5658.4], [148.7], [147.3], [170], [5631.3], 
  [`rho-strong`], [online], [34.7], [38], [37.7], [42.6], [39.2], 
)


#figure(image("figures/fig2_base_over_variant.svg", width: 100%), caption: [smoke-prime, pooled median solve wall of the merge-base binary divided by each variant's (log scale; 1 = no change). n = 12 per arm for base and fix, 6 for the ablations. Data: `analysis/ratios.json`.])


Read: with the trim or the probe present the PR binary reads 0.12 to 0.33 ms on the eight non-IC smoke arms; with neither it reads 5.8 to 8.3 ms, as the merge-base binary does. The PR's own source comment places the cost in `malloc_consolidate` of fastbins left by set-up; this study measured that removing both consolidation sources restores the cost, and did not trace `malloc_consolidate` itself (no tracer on this VM), so the mechanism named is the PR's and the probe's allocation is a candidate explanation for why the trim looked redundant, not an observation.

== Verdicts against the preregistration

#table(columns: 3,
  table.header([*id*], [*prediction*], [*result*], ),
  [H0 counts identical], [every compared field equal], [*holds*: 10 pairs, 0 differences], 
  [H1 smoke wall], [7 of 8 non-IC arms with base/fix >= 2 and fix median < 1 ms], [*supported*: 8 of 8 (ratio 18.4 to 38.8; fix 0.137 to 0.330 ms)], 
  [H2 claim IC online], [base/fix > 1 in each order and >= 70% of fix runs below base median], [*inconclusive*: pooled 1.067, orders 1.135 and 0.983, 65% below], 
  [H3 audit], [every session passes replay-all and the CI form], [*holds*: 14 of 14], 
  [H4 attribution (exploratory)], [notrim near base if the trim causes the drop], [*not as predicted*: `fix-notrim` is at the `fix` level; AMENDMENT_1: probe also consolidates; with neither the cost returns to base (0.75 to 1.05)], 
  [noise floor], [A/A ratio per binary], [1.160 (base), 1.003 (fix)], 
)


Audits (`ecbench verify --replay-all --exit-code`, receipts in `audits/`, not inside sessions):

#table(columns: 3,
  table.header([*session*], [*result*], [*session id*], ),
  [`base-claim`], [OK: 66 records, 66 verified, 60 replays reproduced, 0 problems], [ECBS1h7c0082ac7265], 
  [`base-claim-rev`], [OK: 66 records, 66 verified, 60 replays reproduced, 0 problems], [ECBS1h4d78060e6148], 
  [`base-smoke`], [OK: 72 records, 72 verified, 54 replays reproduced, 0 problems], [ECBS1hd3725d8a32c3], 
  [`base-smoke-rev`], [OK: 72 records, 72 verified, 54 replays reproduced, 0 problems], [ECBS1h420293eaa9c5], 
  [`fix-claim`], [OK: 66 records, 66 verified, 60 replays reproduced, 0 problems], [ECBS1h157a7553e392], 
  [`fix-claim-rev`], [OK: 66 records, 66 verified, 60 replays reproduced, 0 problems], [ECBS1ha418537656ed], 
  [`fix-noprobe-claim`], [OK: 66 records, 66 verified, 60 replays reproduced, 0 problems], [ECBS1h11617fea463f], 
  [`fix-noprobe-smoke`], [OK: 72 records, 72 verified, 54 replays reproduced, 0 problems], [ECBS1h322f7c2815eb], 
  [`fix-notrim-claim`], [OK: 66 records, 66 verified, 60 replays reproduced, 0 problems], [ECBS1hc3e67b6813c4], 
  [`fix-notrim-smoke`], [OK: 72 records, 72 verified, 54 replays reproduced, 0 problems], [ECBS1h628195e31e14], 
  [`fix-notrim2-claim`], [OK: 66 records, 66 verified, 60 replays reproduced, 0 problems], [ECBS1h9d777ef2e237], 
  [`fix-notrim2-smoke`], [OK: 72 records, 72 verified, 54 replays reproduced, 0 problems], [ECBS1h0aa59508bb08], 
  [`fix-smoke`], [OK: 72 records, 72 verified, 54 replays reproduced, 0 problems], [ECBS1hff17ab9636f0], 
  [`fix-smoke-rev`], [OK: 72 records, 72 verified, 54 replays reproduced, 0 problems], [ECBS1h5d29a35e9eba], 
)


== Scope and limits

- Two sizes (a 20-bit prime-order curve, Koblitz a=1 n=17), one shared 4-vCPU VM below L3, glibc 2.39, one pair of commits. It says nothing about other allocators, larger solves where consolidation is negligible, or L3 hosts.
- `base` and `fix` differ by the whole PR (grading v3, isolation probing, PMU group, session writing); the ablations are scratch builds that do not exist in the repository and change only the lines named in their patches.
- The two binaries ran in different sessions: `ecbench compare` refuses wall time across sessions by design, so the wall comparison is a descriptive summary of recorded values, not an admitted ecbench wall result.
- The ABBA order cancels linear drift only; other agents' builds ran throughout and the cargo lock was held at every session start.
- The PR's "about 15% IC online window" figure is from a session not in this study; this study's claim-koblitz-r10 does not distinguish the two binaries on that window.

== Validation gate status

`docs/VALIDATION_GATE.md` requires an independent replay certificate and an independent validator report. Both were produced on the same host by the same agent that ran the sessions, so `independent: true` cannot honestly be asserted. A candidate `VALIDATION.json` with its replay certificate and validator report is in `validation/` with `independent: false`; `scripts/validation_gate.py evidence validation` refuses it on exactly that. No row is added to `benchmarks/README.md`. A replay of `sessions/` from a host of another class, or by a validator who did not run the study, would complete it; the matched-rho reference and the section 5a band (above) are already in place.

== Files

`PREREGISTRATION.md`, `AMENDMENT_1.md`, `specs/`, `sessions/` (14, unedited), `audits/`, `comparisons/`, `analysis/` (scripts, `summary.json`, `ratios.json`, patches, tables), `figures/`, `reports/run-*/`, `BINARIES.txt`, `logs/`, `validation/`. Benchmarks copy: `benchmarks/20261023-ecbench-allocator-confound/` (graphs and PDFs, no README row).
