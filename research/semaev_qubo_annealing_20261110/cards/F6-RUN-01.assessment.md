# Assessment of CARD-semaev-qubo-annealing-20261110-F6-RUN-01 (facts only)

Assessor: experiment-director (assess mode). Date: 2026-11-10.
Card: research/semaev_qubo_annealing_20261110/cards/F6-RUN-01.json (sha256 2ba610af...9ece, matches receipt card_sha256).
Receipt: cards/receipts/F6-RUN-01.receipt.json: status "completed", commands_total 13.

Not performed: `tools/task_card.py check-relay` (no executor report file was supplied); relay_verbatim is therefore not_assessed.
The card is JSON, not the YAML of the template; assessed as dispatched.

## Classification

Completed with deviations. All 13 commands ran once, in card order, each with exit code 0 and no timeout.
The deviations are (D1) the binary digest differs from the bound digest and (D2) the stdout/header cap label differs from the applied cap. Others are listed below.

## Step results (receipt)

| # | id | exit | seconds | note |
|---|----|------|---------|------|
| 1 | verify-freeze | 0 | 0.003 | 4 lines, each ends OK (PROTOCOL.md, tool/src/main.rs, tool/Cargo.toml, tool/Cargo.lock) |
| 2 | git-head | 0 | 0.008 | 8c3b356ff45ec268ea7d0a21f28050200aa4c515 (equals card.commit) |
| 3 | rustc-version | 0 | 0.061 | rustc 1.97.0 (2d8144b78 2026-07-07) |
| 4 | cargo-version | 0 | 0.042 | cargo 1.97.0 (c980f4866 2026-06-30) |
| 5 | nproc | 0 | 0.007 | 4 |
| 6 | uname | 0 | 0.005 | Linux vm 6.18.44-fc-v338 x86_64 |
| 7 | cpu-model | 0 | 0.009 | Intel(R) Xeon(R) Processor @ 2.10GHz |
| 8 | build | 0 | 7.537 | "Compiling semaev_qubo_annealing v0.1.0 ... Finished release in 7.41s" (a recompile occurred) |
| 9 | test | 0 | 0.171 | line printed: `test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s` |
| 10 | binary-sha256 | 0 | 0.005 | 07371fffa1bb890de18d3d000ee5418987499e01bc78e9526f96a6653a3d9cae |
| 11 | run-full | 0 | 4500.14 | detached; started 2026-11-10T19:40:16Z, finished 20:55:16Z; stderr file empty (0 bytes); argv equals card argv |
| 12 | result-sha256 | 0 | 0.003 | 9bbea2ea5dd017b85461ce9fc619cf99768bcdc2cdfb0a0a9e58586d786b3cdd |
| 13 | result-table | 0 | 0.004 | stdout byte-identical to results/run.stdout.txt (cmp) |

stop_when conditions: none triggered (all exits 0; verify-freeze lines all OK; the expected test line present).

## Checks against files on disk

- verify-freeze: re-run now, 4 lines OK. FREEZE.sha256 lists PROTOCOL.md 7c427397...87eb, tool/src/main.rs 88d7dd34...d61f, tool/Cargo.toml 69fa9a7e...61f, tool/Cargo.lock a1e70b44...94aea. Card binds for these four equal the current files.
- results/run.json sha256 now = 9bbea2ea...3cdd = result-sha256 step output. Match.
- run.json header: protocol_sha256 = 7c427397...87eb (= FREEZE PROTOCOL.md); tool_source_sha256 = 88d7dd34...d61f (= FREEZE main.rs). Match. header.quick = false; ns [12,16,20,24,28]; instances_per_n 20; bootstrap_reps 1000; threads 4.
- 700 rows = 5 n x 20 instances x 7 solvers. verification_failures 0, instrument_errors [], spurious_zero_cost_states total 0, no row with solved=false other than the 6 censored rows.
- stdout table vs run.json: all medians equal run.json baselines_per_n medians after rounding (run.json holds .5 values, e.g. n=16 uniform 13792.5 printed 13792; stdout prints half-to-even). Recomputed medians from rows equal baselines_per_n for all 35 cells.
  - n = 16: uniform 13792 (run.json 13792.5), greedy 35242 (35241.5), sa_B 59672 (59671.5), sa_4B 89760 (89759.5), sa_16B 71861 (71861.0), pt 33464 (33464.0), root_enum 42 (42.5); all [0] censored.
  - n = 28: uniform 40138032 (40138031.5), greedy 66891984 (66891984.5), sa_B 820856178[2] (820856178.5), sa_4B 765670044[2] (765670044.5), sa_16B 367805855[0], pt 395638322[2], root_enum 3324 (3323.5)[0].
- Exponent lines vs run.json fits: all 14 lines (7 solvers x {all, n>=20}) equal run.json slope/ci95_lo/ci95_hi at 3 decimals. Examples: uniform all 1.948 [1.790, 2.115]; root_enum all 0.901 [0.797, 1.038]; root_enum n>=20 0.879 [0.626, 1.135]; pt n>=20 2.179 [1.791, 2.506]; sa_B n>=20 2.095 [1.761, 2.440].
- Summary line: `verification_failures=0 instrument_errors=0 P3_uniform_slope=1.948 verdict=closed` equals run.json summary (verdict closed, P3_ok true, P1_all true, P2_lag1_fail_sizes 0, lead_alive_solver_ci_hi_lt_1_on_n_ge_20 false). (Interpretation not assessed.)
- Landscape: stdout and run.json list n = 12, 16, 20 only (8 lags each); no landscape entry for n = 24 or 28 in run.json. Lag-1 values equal stdout.

## Discrepancies

D1. Binary digest. The card binds tool/target/release/semaev_qubo_annealing at c70c8310f6752cd64f326c6feb8aa625f5b9a556e6b0ea93a535a250c187ba9f. The binary on disk now, and recorded by step binary-sha256, is 07371fff...9cae (mtime 2026-11-10 19:38:57, i.e. produced by card step 8, which printed "Compiling"). The binary that ran in step 11 is the 07371fff build (the card's note says the bound binary is the pre-existing build and binary-sha256 records the card's own build). The bound digest therefore does not match any binary used in this run. Source (main.rs sha256) is unchanged. No evidence is on disk that the c70c8310 binary and the 07371fff binary behave identically.

D2. Cap label. stdout line 1 of the table reads "(censored count in brackets; cap 50 B^2)" (main.rs line 1532, fixed string), and run.json header.eval_cap reads "50*B^2 cost evaluations (censored at the cap)", and summary.baselines_per_n[n=28].cap_50B2 = 13421772800. The cap applied in code is `(50*bb*bb).min(2_000_000_000)` (main.rs line 920), matching PROTOCOL.md section 6 amendment min(50*B^2, 2e9). run.json records for every censored row evals = 2000000000, censored true, solved false: 6 rows, all n = 28 (pt instances 1 and 6; sa_4B instances 9 and 11; sa_B instances 13 and 18). Largest uncensored evals at n = 28 is 1811861384 (< 2e9). At n <= 24, 50*B^2 (204800; 3276800; 52428800; 838860800) is below 2e9, and no n <= 24 row is censored (largest evals: n=12 32765, n=16 984039, n=20 19547648, n=24 233918313). So the label "cap 50 B^2" disagrees with the applied cap at n = 28 only (applied 2e9, label implies 1.34e10). Neither the run.json header nor stdout states the 2e9 value.

D3. Card note vs tool: the card requested --threads 2 in its notes (not in argv); the tool has no such flag and used 4 threads (header.threads = 4). The host was shared with other studies running concurrently (receipt dirty_after lists coord_spectrum, p2_lift_linear_complexity, sat_ladder_scaling outputs). Counts are not timing, so this is recorded only as a condition of the run.

D4. Repository state. `git rev-parse HEAD` now prints 03e1c2027d086d074c4adea6167c34a1727255b5, not the card commit 8c3b356ff45e... printed by step 2 at 19:38 (HEAD advanced after the card was written/run).

D5. Writes outside write_scope. `git status --porcelain` lists ` M .resume/RESUME.md` (the receipt dirty_before and dirty_after both list it, so it was modified before and/or independent of this card; the receipt cannot attribute it), and untracked outputs of other studies (coord_spectrum_20261110, p2_lift_linear_complexity_20261110, sat_ladder_scaling_20261110: cards/receipts/ and results/). Every file newer than the card inside research/semaev_qubo_annealing_20261110/ is under cards/receipts/ or results/ (in scope). No semaev file outside write_scope was found changed. The tool/target directory was rewritten by step 8 (in scope).

D6. Route. Card notes state local host execution and that the section 9a EKS confirmatory run is open; this is local evidence only, not an EKS run (no launcher result, S3 hashes or Job/Pod UIDs exist for this card).

## Not discrepancies (checked)
- No unlisted command appears in the receipt; command order equals card order.
- nproc 4 at compile time and header.threads 4 agree.
- Curve rows in stdout (n=12..28; rung_match Some(true) for 16..28, None for 12) equal run.json curves (n=28 p=268435399 b=3 order=268407199 G=(1,1) B=16384, matches_committed_rung true).

interpretation: not_assessed
