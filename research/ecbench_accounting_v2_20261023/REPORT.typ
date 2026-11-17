#set document(title: "Accounting v2 results report", author: "ecbench accounting study")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1 / 1")
#set text(size: 9.5pt)
#set par(justify: true)
#set heading(numbering: "1.")
#show heading.where(level: 1): set text(size: 13pt)
#show heading.where(level: 2): set text(size: 11pt)
#show table: set text(size: 7.3pt)
#show table: set par(justify: false)
#show table: it => block(breakable: false, it)
#show raw: set text(size: 8pt)
#set table(stroke: 0.4pt + luma(150), inset: 3.5pt)
#show figure.caption: set text(size: 8pt)

#align(center)[
  #text(size: 16pt, weight: "bold")[Accounting v2: measured check of the new ids and the versioned floor] \
  #v(2pt)
  aburan28/crypto PR \#2465, branch `ecbench/fix-accounting` \
  Study `research/ecbench_accounting_v2_20261023`, run 2026-10-23 \
  Binary built at head `7ed39a752` (sha256 `e265bbc0...54d50b1`)
]

*What this is.* An accounting and instrument correction check. It is not an algorithmic improvement and not a speed result: AGENTS.md's success criterion says relabelling and accounting changes are not improvements, and nothing here is reported as one. Counts are the result; wall time is descriptive only. Every figure is measured (recorded by `ecbench` in a session) or derived (the text says how). Preregistered in `PREREGISTRATION.md` before any data; amendments A1 to A4 are dated in the same file. Run report: `reports/run-20261023T0815Z/`.

= Preregistered outcomes

#table(columns: (auto, 1.6fr, 2.4fr, 1fr),
  [*id*], [*outcome (preregistered)*], [*result*], [*verdict*],
  [O1], [`rho.cubic_v2` at or above 1.0 of its floor on j = 0, interval excluding values below 1.0],
  [S / floor = *1.419*, 95% interval *[1.132, 1.761]* on the registered j = 0 curve (n = 24; the `ecbench table` interval of S divided by the recorded `floor_s` 0.5117, derived). The first run on another j = 0 curve (archived, amendment A4): 1.514 [1.257, 1.800]], [met on both curves],
  [O2], [strong v2 lowers mean S and set-up share against the frozen id at n = 13, 19, 37; ratio interval excluding 1],
  [S ratio v2 / frozen 0.182 [0.171, 0.196], 0.223 [0.201, 0.248], 0.678 [0.499, 0.891]; set-up share 83.3% to 39.9%, 71.3% to 29.3%, 30.2% to 4.3%], [met at all three sizes],
  [O3 (a)-(c)], [v2 minus frozen is exactly the verification phase; other phases equal; same recovered scalar],
  [40 of 40 matched pairs for each generic method, 16 of 16 for `ic.large_prime`; A/A controls identical on every count], [met],
  [O3 (d)], [verification phase `gae = 1.5 log2 r`],
  [fails for every record: the phase is the exact double-and-add count of the planted scalar, a whole number of 19 to 29 GAE here (amendment A2)], [*not met as written*; replacement (d') met 240 of 240, post hoc],
  [O4], [frozen ids' counts equal on the branch and on the main binary],
  [504 of 504 matched records equal on every compared count field; on j = 0 only the floor figures differ, by design], [met],
)

Quoted figures against measured (all within the preregistered 15% band, so none is reported as differing). j = 0, the review's curve (first run, archived): 1.514 quoted, 1.514 measured; `rho.cubic` on the old floor 0.91 quoted, 0.911 measured. On the registered curve used for the primary run (a different curve, so the review's figures are not a prediction for it): 1.419 and 0.720. Strong reference frozen to v2, mean S and set-up share, quoted then measured: n = 13, 14.0 to 2.55 and 84% to 38% against 14.12 to 2.57 and 83% to 40%; n = 19, 2.95 to 0.68 and 77% to 31% against 3.16 to 0.705 and 71% to 29%; n = 37, 0.293 to 0.209 and 31% to 4.5% against 0.291 to 0.197 and 30% to 4.3%. The scratch runs used other seeds and targets; the largest differences are n = 19 frozen S (+7%), its share (-8%) and n = 37 v2 S (-6%).

= Old id to new id

#table(columns: (2.3fr, 2.5fr, 3fr),
  [*frozen id (counts unchanged)*], [*v2 id*], [*what changes in the count*],
  [`rho.cubic`, `rho.quartic`], [`rho.cubic_v2`, `rho.quartic_v2`], [the `[kappa]G` set-up ledger is inside `total_gae` (phase `automorphism_setup`); the walk of `rho.negation_v2`],
  [`rho.negation`, `rho.signed_frobenius`], [`rho.negation_v2`, `rho.signed_frobenius_v2`], [Brent cycle detection beside the 16-point window; lost-walk steps and table queries and inserts counted],
  [`rho.signed_frobenius_strong`], [`rho.signed_frobenius_strong_v2`], [lanes, distinguished-point bits and jump table scaled to r; start multiples charged; verification phase; spent work kept on exhaustion],
  [`bsgs.textbook`, `bsgs.interleaved`, `bsgs.negation`], [`bsgs.*_v2`], [one charged final `[d]G = Q` (phase `verification`)],
  [`kangaroo.vow`], [`kangaroo.vow_v2`], [the final check; restarts in their own phase `restarts`],
  [`claw.pair_table`], [`claw.pair_table_v2`], [the final check; the `recover` phase inside the `max_adds` budget],
  [`ic.large_prime`], [`ic.large_prime_v2`], [the `[r]Q` membership check and the final check counted in `verification`],
  [floor of every record], [`boundaries.floor_rule = "automorphisms/v2"`], [A = 6 on j = 0 (p = 1 mod 3), 4 on j = 1728 (p = 1 mod 4); a record without the field is recomputed under the first rule (A = 2)],
)
`rho.quartic_v2` and the j = 1728 floor were not run: the PR's specs contain no j = 1728 curve.

= Q1: the j = 0 floor

Primary run, on the registered j = 0 curve `icv1-fp25-t9734-bcfe9f51` ($y^2 = x^3 + 1$ over $F_(23736841)$, group order 23 727 108 = 12 times 1 977 259, r = 1 977 259, `registered=true`, A = 6). Session `j0-v2` (the PR's spec `check-prime-j0-floor-v2.json`, which names this curve; 6 targets, 4 rounds, 24 runs per arm, all verified) and `j0-frozen-main` (same curve and seeds, frozen ids, recorded by the main binary). The first run used another j = 0 curve ($y^2 = x^3 + 2$ over $F_4194397$, r = 1 048 627) that is not in `docs/curves/registry.json`, which CI's curve-name check requires; the curve was switched (amendment A4) and the first run kept, unedited, in `archives/fp23-first-run/` as a replication (right panel of figure 1, table below).

#figure(image("figures/fig1_j0_floor.svg", width: 100%), caption: [Mean S / floor on the j = 0 curves (dashed line: floor = 1). Left: sessions `j0-v2` ECBS1h0c64bbd17533 and `j0-frozen-main` ECBS1h7b9bf9250cf7. Right: the archived first run, ECBS1h70fa091addfe and ECBS1h1c109f746e68. 24 runs per arm.])

#table(columns: (2.2fr, 1fr, 0.8fr, 1.2fr, 0.8fr, 0.8fr, 1.2fr, 0.8fr),
  [*arm (binary)*], [*floor_s (A)*], [*mean S*], [*95% interval of S*], [*S / floor*], [*S / floor interval (derived)*], [*first run (archived): mean S, S / floor*], [*n*],
  [`rho.cubic_v2` (branch)], [0.5117 (6)], [0.7262], [[0.579, 0.901]], [*1.419*], [[1.132, 1.761]], [0.7746, 1.514 [1.257, 1.800]], [24],
  [`rho.cubic` (branch)], [0.5117 (6)], [0.6381], [[0.524, 0.768]], [1.247], [[1.024, 1.501]], [0.8074, 1.578], [24],
  [`rho.cubic` (main binary, first rule)], [0.8862 (2)], [0.6381], [[0.524, 0.768]], [*0.720*], [[0.591, 0.867]], [0.8074, 0.911 [0.750, 1.081]], [24],
  [`rho.negation_v2` (reference)], [0.5117 (6)], [1.1349], [[0.877, 1.399]], [2.218], [[1.714, 2.734]], [1.1067, 2.163], [24],
  [`rho.plain`], [0.5117 (6)], [1.2632], [[0.942, 1.597]], [2.469], [[1.841, 3.121]], [1.4909, 2.914], [24],
)
- *O1 is met on both curves:* `rho.cubic_v2` reads 1.419 of its floor with the interval above 1 (1.132 to 1.761), and 1.514 (1.257 to 1.800) on the first curve. The review's 1.514 was this first curve; on the registered curve the same method reads lower, which is a property of the curve and targets, not of the floor rule.
- The frozen `rho.cubic` counts are identical on the two binaries (72 of 72 records, both curves); only the floor differs. Under the first rule it reads 0.720 on the registered curve (interval [0.591, 0.867], excluding 1: a method that uses six automorphisms read below a floor computed with two) and 0.911 on the first curve (interval [0.750, 1.081], including 1); under `automorphisms/v2` it reads 1.247 and 1.578.
- Paired ratio `rho.cubic_v2` over `rho.cubic` of total S: *1.138 [0.899, 1.442]* (24 pairs) on the registered curve and 0.960 [0.826, 1.042] on the first. Both intervals include 1: this study cannot say whether charging the set-up ledger (about 26 GAE, 3% of S at these sizes, per the README) raises the total, since the v2 walk also differs (Brent cycle detection) and the two effects are not separated here.
- Paired `rho.cubic_v2` over `rho.negation_v2` is 0.640 [0.480, 0.853], so negation over cubic is 1.56 [1.17, 2.08] (inverted, derived) on the registered curve and 1.43 [0.96, 1.98] on the first; the prior $sqrt(3) = 1.73$ is inside both intervals.

= Q2: the strong reference, frozen and v2

Koblitz a = 0, public targets, same workloads and the same per-(workload, round) algorithm seed for both ids. Sessions `claim-n13` and `claim-n19` (committed claim specs, unchanged: 8 targets, 3 rounds, 24 runs per arm) and `strong-n37` (new spec, rho arms only: 8 targets, 4 rounds, 32 runs).

#figure(image("figures/fig2_strong_frozen_vs_v2.svg", width: 90%), caption: [Strong reference, frozen against v2: mean S (log scale) and fixed set-up share. Sessions `claim-n13` ECBS1h0417f722e886, `claim-n19` ECBS1h7f68751e5578, `strong-n37` ECBS1h61cd8e1c1709.])

#table(columns: (auto, auto, auto, auto, auto, 1.3fr, auto, auto, auto),
  [*n*], [*log2 r*], [*A*], [*arm*], [*mean S*], [*95% interval*], [*S / floor*], [*set-up share (sum / sum)*], [*(mean of runs)*],
  [13], [10.97], [26], [frozen], [14.118], [[13.871, 14.405]], [57.4], [83.3%], [83.4%],
  [13], [10.97], [26], [v2], [2.571], [[2.446, 2.767]], [10.5], [39.9%], [40.2%],
  [19], [17.00], [38], [frozen], [3.163], [[3.045, 3.306]], [15.6], [71.3%], [71.7%],
  [19], [17.00], [38], [v2], [0.705], [[0.649, 0.767]], [3.47], [29.3%], [30.0%],
  [37], [27.78], [74], [frozen], [0.291], [[0.257, 0.326]], [2.00], [30.2%], [32.0%],
  [37], [27.78], [74], [v2], [0.197], [[0.155, 0.240]], [1.35], [4.3%], [5.2%],
)
Paired ratio of total S, v2 over frozen (`ecbench compare`, cluster interval over workloads): n = 13 *0.182 [0.171, 0.196]*; n = 19 *0.223 [0.201, 0.248]*; n = 37 *0.678 [0.499, 0.891]*. The A/A control (v2 against its repeat) is 1.000 [1.000, 1.000] at all three sizes. Set-up share is computed from the records' `setup` plus `automorphism_setup` phases by one formula for both ids (derived); the frozen id records no `detail.setup_share`.

Reading it: the v2 reference is sized to r, so its fixed set-up is a smaller share and its mean S is lower at the same walk idea. That is the reference instrument being corrected, not a faster algorithm. At n = 13 and 19 the v2 reference is still 10.5 and 3.5 times its floor with 40% and 29% fixed set-up (above the 10% flag the README names), and every row is below the calibrated `log2 r` range for Koblitz curves (39). `ecbench table` on this head prints no flag column, although `methods::row_flags` exists (see limits).

= Q3: one verification more

Same workload, round and algorithm seed for the frozen id and its v2 id. `pairs-prime`: 2^20-class prime curve `icv1-fp20-t727-cd198a38` (log2 r = 19.10); `pairs-koblitz`: Koblitz a = 1, n = 17 (log2 r = 16.00); `pairs-large-prime`: binary n = 11. Each pair has 40 matched records (8 targets, 4 rounds plus 1 warm-up); `ic.large_prime` has 16 (4 targets, 4 rounds).

#figure(image("figures/fig3_verification_exact.svg", width: 55%), caption: [Added total GAE (v2 minus frozen, same workload, round and seed) against the v2 verification phase GAE, every matched record. Sessions `pairs-prime` ECBS1h5c19d0d7e485, `pairs-koblitz` ECBS1h001cebd5ff12, `pairs-large-prime` ECBS1h8ce582cbde15.])

#table(columns: (2.3fr, 0.8fr, 1.1fr, 1.2fr, 1.1fr, 1.1fr, 1.2fr, 0.9fr, 1.6fr),
  [*pair*], [*matched*], [*recovered, seed, status equal*], [*other phases equal*], [*delta total = delta verif.*], [*one scalar mult*], [*(d'): doubles, adds = bitlen - 1, popcount - 1*], [*delta total (GAE)*], [*S ratio v2 / frozen*],
  [`bsgs.textbook`], [40], [40], [40], [40], [40], [40], [20-29], [1.022 [1.021, 1.024]],
  [`bsgs.interleaved`], [40], [40], [40], [40], [40], [40], [20-29], [1.028 [1.024, 1.036]],
  [`bsgs.negation` (prime)], [40], [40], [40], [40], [40], [40], [20-29], [1.033 [1.030, 1.037]],
  [`kangaroo.vow`], [40], [40], [40 (setup = setup + restarts)], [40], [40], [40], [20-29], [1.017 [1.014, 1.024]],
  [`bsgs.negation` (Koblitz)], [40], [40], [40], [40], [40], [40], [19-22], [1.088 [1.073, 1.108]],
  [`claw.pair_table`], [40], [40], [40], [40], [40], [40], [19-22], [1.069 [1.064, 1.075]],
  [`ic.large_prime`], [16], [16], [16], [16], [0 (two scalar mults)], [n/a], [28-29], [1.0003 [1.0003, 1.0003]],
  [A/A `negation_v2`], [40], [40], [40], [40 (delta 0)], [n/a], [n/a], [0], [1.000],
  [A/A `frobenius_v2`], [40], [40], [40], [40 (delta 0)], [n/a], [n/a], [0], [1.000],
)
- (a) to (c) hold for every matched pair. The positive control is the bsgs pairs themselves (the comparison sees a difference of 19 to 29 GAE); the negative control is the A/A rows (it sees none).
- (d) as preregistered fails for every record. The phase is `doubles = bitlength(d) - 1`, `adds = popcount(d) - 1` of the planted scalar d (40 of 40 for each of the six generic pairs), so its size follows d, not $1.5 log_2 r$. Over the 8 targets its mean is 0.899 (prime) and 0.859 (Koblitz) of $1.5 log_2 r$, a small sample of scalars. The README's "about 1.5 log2 r" is right in that sense; "exactly" was an over-reading (amendment A2).
- `ic.large_prime_v2`: the verification phase adds 28 to 29 GAE over the frozen one, two scalar multiplications (`[r]Q` and `[d]G`, about 14 GAE each at r = 991, derived from the 28 to 29 total) where the frozen verification phase held 90 adds of candidate checks and no scalar multiplication.
- The claw budget change (the `recover` phase inside `max_adds`) changed no run here: every matched claw pair differs only by the verification phase.
- The v2 ids cost 1.7% to 8.8% more S on these curves because a fixed 19 to 29 GAE sits on totals of 235 to 1500 GAE. This is a sizing statement for these curves, not for larger r (one scalar multiplication, O(log r), against O(sqrt r)).

#figure(image("figures/fig4_paired_ratios.svg", width: 100%), caption: [Total S of each v2 id over its frozen id on matched (workload, round) pairs, 95% cluster interval from `ecbench compare`, with the A/A controls. Sessions as in figures 2 and 3, `j0-v2` and `strong-n37`.])

= Q4: frozen ids against the main binary

The same frozen-only spec files were run once on the branch binary and once on the main binary (`ecbench-base`, sha256 `a59c4014...f81f`; the coordinator's build of main `3a1ea541d`, whose source tree and build log are not in the repository, and which does not embed its commit: this provenance is the coordinator's word). Compared per (arm, workload, round, warm-up): status, recovered scalar, algorithm seed, `total_gae`, unpriced counters, every phase's adds, doubles, scalar_mults, gae and native counts, all counters, and the run id.

#table(columns: (3fr, auto, auto, auto, 1.6fr),
  [*spec (arms)*], [*matched*], [*all count fields equal*], [*floor figures equal*], [*`compare --b-dir` ratio*],
  [frozen-prime (`rho.negation`, `rho.plain`, `bsgs` x 3, `kangaroo.vow`)], [240], [*240*], [240], [1.000 [1.000, 1.000] each arm],
  [frozen-koblitz (`rho.signed_frobenius`, strong, `claw.pair_table`, `bsgs.negation`)], [160], [*160*], [160], [1.000 each arm],
  [frozen-large-prime (`rho.signed_frobenius`, `ic.large_prime`)], [32], [*32*], [32], [1.000 each arm],
  [frozen-j0 (`rho.negation`, `rho.cubic`, `rho.plain`)], [72], [*72*], [0 (A = 2 on main, A = 6 on branch, by design)], [1.000 each arm],
)
Total *504 of 504*. A second check replays committed sessions whose methods are frozen ids on the branch binary (`audits/committed/`): `ecbench_calibration_20261002` `followup-prime` (`kangaroo.vow`, `rho.negation`) and `followup-koblitz` (`rho.plain`, `rho.signed_frobenius`), 24 replays each; `ecbench_numa_vm_20261002/auto` (`bsgs.negation`, `kangaroo.vow`, `rho.negation`), all 48; `ghs_curve_comparison_20261009` `panel-01` and `non-koblitz-01`, 24 each; `ecbench_wide_gf2m_20261004/smoke-m83` (`claw.pair_table`, strong), all 4; `ecbench_bounds_challenges_20261006/koblitz-toy-reference-e1` (strong), 24; `ecbench_factor_bases_20261013/koblitz17`, all 20; `ecbench_n37_shared_rank_20261004/mac_arm64_l0_01`, all 15. Every replay reproduced; 0 problems.

Two committed sessions fail a replay on the branch binary and fail it identically on the main binary under `--replay-all` (96 and 16 problems, only their `ic.pipeline` root-finding and inherited-F4 arms): `ecbench_rootfind_engine_20261012` `claim-n13` and `smoke-koblitz`. This is a pre-existing condition, not caused by the PR (receipts `audits/committed/PREEXISTING_*`); CI's PR audit uses `--replay 0`, which passes.

= Exploratory results and the pricing band

- *Brent cycle detection changes totals.* `rho.negation_v2` over `rho.negation`: 0.914 [0.809, 1.040] (2^20 prime, 32 pairs). `rho.signed_frobenius_v2` over `rho.signed_frobenius`: 0.983 [0.653, 1.389] (n = 17) and *0.893 [0.804, 0.983]* (n = 37). Runs with a recorded cycle longer than the window: 30 of 32 (`rho-neg-v2`, prime), 13 of 24 (`rho.cubic_v2`, registered j = 0 curve; 12 of 24 on the first curve), 10 of 32 and 5 of 32 (`signed_frobenius_v2`, n = 17 and 37). The README's "mean rose 14%" at n = 37 is the opposite direction to these paired totals (v2 lower); the README cites the review's runs (n = 80, with capped walks), and this study does not resolve the difference. Not a preregistered test.
- *Pricing band (AGENTS.md 5a)*, `analysis/band/`: every v2 arm emits its uncharged counters (the `uncounted` column is empty for every arm), so all arms are ranked across 0, 0.1 and 1 GAE per native operation. At n = 37 strong v2 is below the frozen strong across the band (S 0.197, 0.234, 0.560 against 0.291, 0.331, 0.691); strong v2 and `rho.signed_frobenius` are unresolved there.
- *Distance to the matched rho (success criterion).* The index-calculus arms of the committed claim specs are unchanged by this PR; the reference they are read against is what changed (`analysis/gap_to_rho.tsv`, band weights 0, 0.1, 1). The best IC arm across the band, `ic-rootfind-m2`, is 10.6 to 11.0 times strong v2 at n = 13 (2.0 to 2.3 times the frozen strong) and 34 to 49 times at n = 19 (10 to 11 times the frozen strong). The corrected references make the measured gap larger, not smaller. These are distances remaining, not a gate.

= Deviations, limits, not done

- Amendments (dated in `PREREGISTRATION.md`): A1 added the frozen `ic.large_prime` cross-binary spec; A2 records that outcome (d) was too strict and adds the post hoc (d'); A3 records that the main-binary j = 0 session fails the branch audit by design; A4 records the switch of the j = 0 curve to a registered one, with the first run kept as a replication.
- `j0-frozen-main` (registered curve) is a verbatim archive (`archive/j0-frozen-main.tar.gz`, sha256 beside it; the first curve's is `archives/fp23-first-run/j0-frozen-main.tar.gz`), not a session directory: the branch binary's audit rejects it ("used 6 automorphisms, the curve has 2", 25 problems in both runs), which is the gap the floor rule closes; the main binary accepts it. Committed as a session it would fail CI's re-audit of every `ecbench.session/v1`.
- Sessions record the worktree's `git_commit` at run time (`c5ca4380e`, `2833cfc7a` and, for the registered-curve j = 0 sessions, `3d310548d`, above the binary's source head `7ed39a752`; the commits between add only this study). The binary sha256 is the same in every branch session.
- Isolation: L1 or better only, not the L2 the specs request on most records (a shared 4-vCPU virtual machine with other agents building). Counts are unaffected; no wall-time statement is made, and median solves (0.1 to 1.5 ms) are below the harness's 1 ms admission minimum.
- `ecbench table` prints no flag column on this head (`methods::row_flags` and `setup_share` are referenced only by unit tests); the README of this PR now says the column is a follow-up (the share is in `detail.setup_share`). The flags above are computed by hand from the same thresholds.
- Not run: `rho.quartic_v2` and the j = 1728 floor; sizes between or beyond n = 13, 19, 37 for the strong reference (no scaling claim); a replay of the branch's sessions on the main binary.
- Curve names: the first j = 0 curve is not registered, so its sessions, tables and the PR's original spec text live only under `archives/` (exempt from `scripts/check_curve_names.py`); a run of that script in this sparse checkout (files absent from it read as empty) reports 0 problems, both plain and with `--diff origin/main`.
- Sessions are 16 directories of about 16 MB (the per-record isolation capsule dominates; the whole study directory is 20 MB), more than the few MB asked for; git compresses `records.jsonl` roughly tenfold.
- No `benchmarks/` entry: nothing here measures how fast an algorithm runs or reports a speedup, and no validation-gate row is created. No leaderboard change: no row of `docs/ic/leaderboard.json` is added or moved (accounting changes are not improvements). Canonical graphs checked and left unchanged: `docs/ic-leaderboard.html`, `docs/ic/LEADERBOARD.md`, `docs/ic/leaderboard.json`, `docs/index-calculus-scoreboard.html`, `docs/bounds/*`; they cite frozen ids, whose counts Q4 shows unchanged. A follow-up may add v2-reference columns.

= Files and reproduction

`PREREGISTRATION.md`, `SPEC_SHA256.txt`, `specs/`; `sessions/` (never edited), `archive/`, `archives/fp23-first-run/` (the first j = 0 run: sessions, archive, specs, audits, analysis), `audits/` (replay-all receipts beside the sessions, never inside), `logs/`, `host.txt`; `analysis/` (commands `run.sh`, `run_a1.sh`, `audit.sh`, `replay_committed.sh`; jq and shell scripts that produce every number; `compare/`, `table/`, `band/`, `summary/`, `pair_check.json`, `cross_binary.json`, `gap_to_rho.tsv`, `inventory.tsv`; `plot.py`, which plots recorded values only); `figures/` (SVG, 150 dpi PNG and `*.data.json` with the numbers drawn); `REPORT.typ`, `REPORT.pdf`; `reports/run-20261023T0815Z/`.
