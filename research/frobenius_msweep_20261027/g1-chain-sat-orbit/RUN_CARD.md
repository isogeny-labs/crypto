# G1 run card (commands for LATER; nothing here has been run)

## 0. Token and checkpoint gate (not bypassed; no CHECKPOINT_SKIP)

This is a session-side launcher gate: the session runs it (the commands below, ending in `scripts/checkpoint.sh gate` exiting 0) before it submits any stage as a job. It is never invoked inside a remote job.

The resume-checkpoint upload token must be minted where AWS credentials exist (the owner's Mac; AGENTS.md says a job there re-mints
`~/.config/checkpoint/upload-token` every 15 minutes, so copy that file first). Otherwise, on a credentialed machine, from the repo root:

    scripts/checkpoint.sh presign-upload --install      # writes ~/.config/checkpoint/upload-token (7 days)
    scripts/checkpoint.sh presign-upload --days 7       # or print it and export CHECKPOINT_UPLOAD_TOKEN

It is a bearer credential: never commit or paste it. Then in the repository: `scripts/checkpoint.sh init`, fill `.resume/RESUME.md`,
`scripts/checkpoint.sh push`, `scripts/checkpoint.sh gate` (must exit 0).

## 1. Preconditions

1. `conductor check --summary "G1 chain-SAT orbit run" --scope path:research/frobenius_msweep_20261027/g1-chain-sat-orbit` (the control plane at localhost:8080 was down during preparation, so this was not obtained).
2. Commit this directory and the example. `run_stage.sh` records the commit sha, dirty G1 paths, binary sha256, rustc version and host in each stage's `provenance.txt`; the PR that lands results names that sha. A dirty G1 path in `provenance.txt` invalidates the run.
3. Route: AGENTS.md section 9a / CLAUDE.md make isolab-eks the default for x86 CPU experiments; the director freezes the route in the task card. The commands below also run locally (4 CPUs). A local run is a deviation to state; `wall_s` is then not a deciding metric (PROTOCOL section 6).
4. Build and unit tests (local development check; allowed any time):

       cd /home/user/crypto
       cargo test --release --example frobenius_msweep_g1_chain_sat_orbit      # 25 tests, about 30 s plus build
       cargo build --release --example frobenius_msweep_g1_chain_sat_orbit      # binary: target/release/examples/frobenius_msweep_g1_chain_sat_orbit
       sha256sum target/release/examples/frobenius_msweep_g1_chain_sat_orbit

## 2. Design numbers (solves nothing)

Every block below starts with `. /home/user/crypto/research/frobenius_msweep_20261027/g1-chain-sat-orbit/env.sh`, which defines `ROOT`, `G1`, `BIN`, `RUN`, `R` in a fresh shell (blocks are independent; no variable carries over). Run the build block of section 1 first so `$BIN` exists.

    . /home/user/crypto/research/frobenius_msweep_20261027/g1-chain-sat-orbit/env.sh && $BIN plan

Prints n, m, r, k, fb points, expected yield and the exact class count (information only) for the 9 cells (PROTOCOL section 5 table; amended: n = 19, 23, 31).

## 3. Stages. Every stage is `run_stage.sh STAGE SEED DEFAULT_CAP NATURAL PLANTED COSET "n:m:k[:CAP] ..."`

The checkpoint gate (section 0) is a session-side launcher gate: the session runs `scripts/checkpoint.sh gate` and it must exit 0 BEFORE it submits any block below as a job. It is never invoked inside a remote job and `run_stage.sh` does not call it.

`run_stage.sh` does: (0) rejects an empty cell list, writes `provenance.txt` (commit sha, dirty G1 paths, binary sha256, rustc, host, nproc, UTC, arguments), (1) exact oracle per cell (xargs -P4, timeout 7200 s each; also writes `manifest-<cell>.json`: field polynomial, curve, generator, r, factor-base points and x-codes, orbits), (2) one job per (cell, arm in $ARMS, conj 0) plus the A/A job (arm P, conj 1; skipped with `AA=0`), shuffled with a fixed seed, xargs -P4, timeout 14400 s each, failures recorded in `failures.txt`, (3a) the native verifier `verify_certs/` (crate `g1-verify-certs`, its own GF(2^n) and curve arithmetic, sharing no code with the driver; re-adds every `found` row's certificate from the rows and the manifest; writes `verify_certs.txt`; a failure is written to `failures.txt`), (3) `analyze` (skipped with `ANALYZE=0`). A cell token `n:m:k:CAP` sets that cell's conflict cap; `n:m:k:IMPEDIMENT` records `O-IMPEDIMENT(budget)` and `n:m:k:NODATA` records `O-IMPEDIMENT(no-pilot-data)`; both skip the cell. Output: `$G1/reports/run-$RUN/<STAGE>/{provenance.txt,oracle,rows,analysis,failures.txt,verify_certs.txt}`.

Block 3.0, mint RUN (once for the campaign; every later block reads it back from `reports/RUN`):

    MINT_RUN=1 . /home/user/crypto/research/frobenius_msweep_20261027/g1-chain-sat-orbit/env.sh && mkdir -p $G1/reports && date -u +%Y%m%dT%H%M%SZ > $G1/reports/RUN && cat $G1/reports/RUN

Block 3.1, stage pilot (excluded from inference; sets the per-cell caps; seed 1027 differs from the main seed; ALL nine cells, arms P and O only, no A/A, no analysis). Expected wall: 14 targets x 18 jobs, cap 20000, worst case about 14 x 20000 x 5e-3 s = 1400 s per job on 4 CPUs, so at most about 2 h, typically minutes.

    . /home/user/crypto/research/frobenius_msweep_20261027/g1-chain-sat-orbit/env.sh && ARMS="P O" AA=0 ANALYZE=0 $G1/run_stage.sh pilot 1027 20000 6 4 4 "19:4:2 19:5:2 19:6:2 23:4:2 23:5:2 23:6:2 31:4:2 31:5:2 31:6:2"

Block 3.2, cap rule (frozen, PROTOCOL section 8). One line per cell (`n:m:k:CAP`, `n:m:k:IMPEDIMENT` or `n:m:k:NODATA`); `caps.txt` must have exactly 9 lines, one per cell, or the block fails:

    . /home/user/crypto/research/frobenius_msweep_20261027/g1-chain-sat-orbit/env.sh && $BIN cap --rows $R/pilot/rows --budget 180 | tee $R/caps.txt && test "$(wc -l < $R/caps.txt)" = 9 && for c in 19:4:2 19:5:2 19:6:2 23:4:2 23:5:2 23:6:2 31:4:2 31:5:2 31:6:2; do grep -q "^$c:" $R/caps.txt || { echo "no caps line for $c" >&2; exit 1; }; done

The estimator is per cell: t_conf = max over arms P, O of sum(wall_s - build_s) / sum(conflicts); CAP = the largest value in {20000, 50000, 100000, 200000, 500000, 1000000, 2000000} with max(build_s) + CAP x t_conf <= 180 s; otherwise IMPEDIMENT; a cell with no pilot solve of conflicts > 0 is NODATA. The caps are fixed before the main stage and never changed after seeing main-stage data. A main stage takes the caps from `caps.txt` verbatim (no value is chosen by hand). DEFAULT_CAP 0 is deliberate: `run_stage.sh` exits with an error on any cell token that has no positive cap, IMPEDIMENT or NODATA, and on an empty cell list.

Blocks 3.3, 3.4, 3.5, main stages (each block is self-contained; N = 40 natural, 20 planted, 8 coset per cell; the cell list is the stage's lines of `caps.txt`):

    . /home/user/crypto/research/frobenius_msweep_20261027/g1-chain-sat-orbit/env.sh && $G1/run_stage.sh main-n19 20261027 0 40 20 8 "$(grep '^19:' $R/caps.txt | tr '\n' ' ')"
    . /home/user/crypto/research/frobenius_msweep_20261027/g1-chain-sat-orbit/env.sh && $G1/run_stage.sh main-n23 20261027 0 40 20 8 "$(grep '^23:' $R/caps.txt | tr '\n' ' ')"
    . /home/user/crypto/research/frobenius_msweep_20261027/g1-chain-sat-orbit/env.sh && $G1/run_stage.sh main-n31 20261027 0 40 20 8 "$(grep '^31:' $R/caps.txt | tr '\n' ' ')"

Expected wall: oracle seconds to minutes per cell; solves at most 68 x 180 s = 12240 s per job (under the 14400 s timeout), typically minutes to an hour; each main stage about 1 to 8 h. Check `failures.txt` and `verify_certs.txt` after every stage: a TIMEOUT-OR-FAIL line makes that cell `invalid_incomplete_rows`; a CERT-VERIFY-FAIL line or a `VERIFY FAIL` means no relation of the stage may be used (insufficient evidence).

Single job by hand (spot check only, not part of the frozen run):

    MINT_RUN=1 . /home/user/crypto/research/frobenius_msweep_20261027/g1-chain-sat-orbit/env.sh && $BIN oracle --n 19 --m 4 --k 2 --a 0 --seed 1027 --natural 6 --planted 4 --coset 4 --out /tmp/g1-spot/oracle
    MINT_RUN=1 . /home/user/crypto/research/frobenius_msweep_20261027/g1-chain-sat-orbit/env.sh && $BIN run --n 19 --m 4 --k 2 --a 0 --seed 1027 --natural 6 --planted 4 --coset 4 --arm P --conj 0 --conflicts 20000 --tieout 3 --run-id spot --oracle /tmp/g1-spot/oracle/oracle-a0-n19-m4-k2-s1027.jsonl --out /tmp/g1-spot/rows

(`oracle`, `run` and `analyze` take the same `--natural/--planted/--coset` for the oracle and run steps of a cell.)

## 4. Row fields (one JSONL row per cell, arm, target, conj; `rows/rows-<cell>-<arm>-c<conj>.jsonl`)

schema, run_id, host, cell, a, n, m, k, seed, r, fb_x, fb_points, target_id, class, arm, conj, target_xy (the solved point, [x,y] codes), cert_points (for found rows: the m certificate points [x,y]; null otherwise), verdict (found/refuted/censored), censor_reason, oracle, agree, cert_ok, tieout (true/false/null), conflict_cap, conflicts, decisions, propagations, word_ops, patterns_attempted, patterns_refuted, solver_calls, models, group_rejected, invalid_models, peak_vars, peak_clauses, peak_xors, domain_clauses, lift_adds, wall_s, build_s.

`analysis/summary.json` (`g1-chain-sat-orbit/summary/v2`): per cell and stratum, per arm O, R, Q against P: the cost-per-relation ratio with 95% interval for word_ops, conflicts, propagations, wall_s (the deciding statistic) and the survivors-only secondary with dropped pairs per direction; the A/A comparison; the `gate` (cell-wide agreement, completeness, targets, censored counts per arm); decision label; `c3_noninferior_at_0.95`; priced GAE at 0, 0.1, 1 with `section_5a_band`, `c3_noninferior_at_positive_prices` (prices 0.1 and 1 only) and `final_grade_word_ops`; `aa_priced_gae` (A/A resolution of each priced leg); achieved A/A bootstrap sd and MDE; detector injection table; anytime yield and cost per relation; absolute cost per relation per arm and price with the analytic rho placeholder for P. Per cell: `c3_grade_O`, `class_count_info_only`; top level: `c3_cells_supported` (k of 9). The consumer takes the grades as printed and adds no discretion.

## 5. After the run

0. Re-verify certificates independently of the run: `cargo run --release --offline --manifest-path $G1/verify_certs/Cargo.toml --target-dir $G1/verify_certs/target -- --rows $R/<STAGE>/rows --manifests $R/<STAGE>/oracle` (exit 0 required; it uses only the rows and `manifest-<cell>.json`, never the solver).
1. Check `failures.txt` per stage, the host (`host` field), and that every `tieout` is true or null (analysis refuses otherwise).
2. Make the research-visuals report, graph and PDF (skill `research-visuals`). No `benchmarks/` or leaderboard row: no matched rho reference and no VALIDATION.json exist for this cell.
3. Append to `docs/session-logs/`, delete nothing under `reports/`. The shared `target/` is not removed by this cell.
