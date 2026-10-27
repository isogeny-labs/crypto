# G1 run card (commands for LATER; nothing here has been run)

## 0. Token and checkpoint gate (not bypassed; no CHECKPOINT_SKIP)

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
       cargo test --release --example frobenius_msweep_g1_chain_sat_orbit      # 19 tests, about 30 s plus build
       cargo build --release --example frobenius_msweep_g1_chain_sat_orbit      # binary: target/release/examples/frobenius_msweep_g1_chain_sat_orbit
       sha256sum target/release/examples/frobenius_msweep_g1_chain_sat_orbit

## 2. Design numbers (solves nothing)

    BIN=target/release/examples/frobenius_msweep_g1_chain_sat_orbit; $BIN plan

Prints n, m, r, k, fb points, expected yield and the exact class count (information only) for the 9 cells (PROTOCOL section 5 table; amended: n = 19, 23, 31).

## 3. Stages. Every stage is `run_stage.sh STAGE SEED DEFAULT_CAP NATURAL PLANTED COSET "n:m:k[:CAP] ..."` with `RUN` set once for the whole campaign

`run_stage.sh` does: (0) writes `provenance.txt` (commit sha, dirty G1 paths, binary sha256, rustc, host, nproc, UTC, arguments), (1) exact oracle per cell (xargs -P4, timeout 7200 s each), (2) one job per (cell, arm in $ARMS, conj 0) plus the A/A job (arm P, conj 1; skipped with `AA=0`), shuffled with a fixed seed, xargs -P4, timeout 14400 s each, failures recorded in `failures.txt`, (3) `analyze` (skipped with `ANALYZE=0`). A cell token `n:m:k:CAP` sets that cell's conflict cap; `n:m:k:IMPEDIMENT` records `O-IMPEDIMENT(budget)` and skips the cell. Output: `research/frobenius_msweep_20261027/g1-chain-sat-orbit/reports/run-$RUN/<STAGE>/{provenance.txt,oracle,rows,analysis,failures.txt}`.

    export RUN=$(date -u +%Y%m%dT%H%M%SZ)
    cd /home/user/crypto/research/frobenius_msweep_20261027/g1-chain-sat-orbit

Stage pilot (excluded from inference; sets the per-cell caps; seed 1027 differs from the main seed; ALL nine cells, arms P and O only, no A/A, no analysis). Expected wall: 14 targets x 18 jobs, cap 20000, worst case about 14 x 20000 x 5e-3 s = 1400 s per job on 4 CPUs, so at most about 2 h, typically minutes.

    RUN=$RUN ARMS="P O" AA=0 ANALYZE=0 ./run_stage.sh pilot 1027 20000 6 4 4 "19:4:2 19:5:2 19:6:2 23:4:2 23:5:2 23:6:2 31:4:2 31:5:2 31:6:2"

Cap rule (frozen, PROTOCOL section 8). Compute the caps with the driver, one line per cell (`n:m:k:CAP` or `n:m:k:IMPEDIMENT`), from the repository root:

    cd /home/user/crypto && $BIN cap --rows research/frobenius_msweep_20261027/g1-chain-sat-orbit/reports/run-$RUN/pilot/rows --budget 180 | tee research/frobenius_msweep_20261027/g1-chain-sat-orbit/reports/run-$RUN/caps.txt

The estimator is per cell: t_conf = max over arms P, O of sum(wall_s - build_s) / sum(conflicts); CAP = the largest value in {20000, 50000, 100000, 200000, 500000, 1000000, 2000000} with max(build_s) + CAP x t_conf <= 180 s; otherwise IMPEDIMENT. The caps are fixed before the main stage and never changed after seeing main-stage data. A main stage takes the caps from `caps.txt` verbatim (no value is chosen by hand):

    CELLS_N19=$(grep '^19:' reports/run-$RUN/caps.txt | tr '\n' ' ')    # likewise ^23: and ^31:
    RUN=$RUN ./run_stage.sh main-n19 20261027 0 40 20 8 "$CELLS_N19"
    RUN=$RUN ./run_stage.sh main-n23 20261027 0 40 20 8 "$CELLS_N23"
    RUN=$RUN ./run_stage.sh main-n31 20261027 0 40 20 8 "$CELLS_N31"

(DEFAULT_CAP 0 is deliberate: `run_stage.sh` exits with an error on any cell token that has no positive cap or IMPEDIMENT, so no cap can be chosen by hand or forgotten.) N = 40 natural, 20 planted, 8 coset per cell. Expected wall: oracle seconds to minutes per cell; solves at most 68 x 180 s = 12240 s per job (under the 14400 s timeout), typically minutes to an hour; each main stage about 1 to 8 h. Check `failures.txt` after every stage: a TIMEOUT-OR-FAIL line makes that cell `invalid_incomplete_rows`.

Single job by hand (spot check only, not part of the frozen run):

    $BIN oracle --n 19 --m 4 --k 2 --a 0 --seed 1027 --natural 6 --planted 4 --coset 4 --out /tmp/g1-spot/oracle
    $BIN run --n 19 --m 4 --k 2 --a 0 --seed 1027 --natural 6 --planted 4 --coset 4 --arm P --conj 0 --conflicts 20000 --tieout 3 --run-id spot --oracle /tmp/g1-spot/oracle/oracle-a0-n19-m4-k2-s1027.jsonl --out /tmp/g1-spot/rows

(`oracle`, `run` and `analyze` take the same `--natural/--planted/--coset` for the oracle and run steps of a cell.)

## 4. Row fields (one JSONL row per cell, arm, target, conj; `rows/rows-<cell>-<arm>-c<conj>.jsonl`)

schema, run_id, host, cell, a, n, m, k, seed, r, fb_x, fb_points, target_id, class, arm, conj, verdict (found/refuted/censored), censor_reason, oracle, agree, cert_ok, tieout (true/false/null), conflict_cap, conflicts, decisions, propagations, word_ops, patterns_attempted, patterns_refuted, solver_calls, models, group_rejected, invalid_models, peak_vars, peak_clauses, peak_xors, domain_clauses, lift_adds, wall_s, build_s.

`analysis/summary.json` (`g1-chain-sat-orbit/summary/v2`): per cell and stratum, per arm O, R, Q against P: the cost-per-relation ratio with 95% interval for word_ops, conflicts, propagations, wall_s (the deciding statistic) and the survivors-only secondary with dropped pairs per direction; the A/A comparison; the `gate` (cell-wide agreement, completeness, targets, censored counts per arm); decision label; `c3_noninferior_at_0.95`; priced GAE at 0, 0.1, 1 with `section_5a_band` and `final_grade_word_ops`; achieved A/A bootstrap sd and MDE; detector injection table; anytime yield and cost per relation; absolute cost per relation per arm and price with the analytic rho placeholder for P. Per cell: `c3_grade_O`, `class_count_info_only`; top level: `c3_cells_supported` (k of 9). The consumer takes the grades as printed and adds no discretion.

## 5. After the run

1. Check `failures.txt` per stage, the host (`host` field), and that every `tieout` is true or null (analysis refuses otherwise).
2. Make the research-visuals report, graph and PDF (skill `research-visuals`). No `benchmarks/` or leaderboard row: no matched rho reference and no VALIDATION.json exist for this cell.
3. Append to `docs/session-logs/`, delete nothing under `reports/`. The shared `target/` is not removed by this cell.
