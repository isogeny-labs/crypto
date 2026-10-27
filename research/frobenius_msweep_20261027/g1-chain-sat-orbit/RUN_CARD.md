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
2. Commit this directory and the example; record `git rev-parse HEAD` in `reports/run-<UTC>/commit.txt`. The PR that lands results names that sha.
3. Route: AGENTS.md section 9a / CLAUDE.md make isolab-eks the default for x86 CPU experiments; the director freezes the route in the task card. The commands below also run locally (4 CPUs). A local run is a deviation to state; `wall_s` is then not a deciding metric (PROTOCOL section 6).
4. Build and unit tests (local development check; allowed any time):

       cd /home/user/crypto
       cargo test --release --example frobenius_msweep_g1_chain_sat_orbit      # 14 tests, about 30 s plus build
       cargo build --release --example frobenius_msweep_g1_chain_sat_orbit      # binary: target/release/examples/frobenius_msweep_g1_chain_sat_orbit
       sha256sum target/release/examples/frobenius_msweep_g1_chain_sat_orbit

## 2. Design numbers (solves nothing)

    BIN=target/release/examples/frobenius_msweep_g1_chain_sat_orbit; $BIN plan

Prints n, m, k, fb points, expected yield for the 9 cells (PROTOCOL section 5 table).

## 3. Stages. Every stage is `run_stage.sh STAGE SEED CONFLICT_CAP NATURAL PLANTED COSET "n:m:k ..."` with `RUN` set once for the whole campaign

`run_stage.sh` does: (1) exact oracle per cell (xargs -P4, timeout 7200 s each), (2) one job per (cell, arm in P O R Q, conj 0) plus the A/A job (arm P, conj 1), shuffled with a fixed seed, xargs -P4, timeout 14400 s each, failures recorded in `failures.txt`, (3) `analyze`. Output: `research/frobenius_msweep_20261027/g1-chain-sat-orbit/reports/run-$RUN/<STAGE>/{oracle,rows,analysis}`.

    export RUN=$(date -u +%Y%m%dT%H%M%SZ)
    cd /home/user/crypto/research/frobenius_msweep_20261027/g1-chain-sat-orbit

Stage pilot (excluded from inference; sets the cap; seed 1027 differs from the main seed). Expected wall: 10 to 60 minutes (14 targets, cap 20000, worst case about 14 x 20000 x 5e-3 s = 1400 s per job, 15 jobs on 4 CPUs).

    ./run_stage.sh pilot 1027 20000 6 4 4 "17:4:2 17:5:2 17:6:2"

Cap rule (frozen): from the pilot P-arm rows compute t_conf = median(wall_s / conflicts) over rows with conflicts > 0. Set CAP = the largest value in {20000, 50000, 100000, 200000, 500000, 1000000, 2000000} with CAP x t_conf <= 180 s. If even 20000 exceeds 180 s, n = 17 cells stay at CAP = 20000 and the larger cells are recorded as O-IMPEDIMENT(budget) and not run. The cap is fixed per n before the main stage and never changed after seeing main-stage data.

Stage main-n17 (all 3 cells unconditionally; seed 20261027, N = 40/20/20). Expected wall: oracle under 1 min; solves at most 3 x 5 jobs x 80 x CAP x t_conf (<= 4 h per job, typically minutes to an hour); about 1 to 8 h.

    ./run_stage.sh main-n17 20261027 $CAP17 40 20 20 "17:4:2 17:5:2 17:6:2"

Stage main-n23 (3 cells; same seed). Oracle (23,4,3) under 5 min. Expected 1 to 12 h.

    ./run_stage.sh main-n23 20261027 $CAP23 40 20 20 "23:4:3 23:5:2 23:6:2"

Stage main-n31 (only if the pilot-based cap for n = 31 is at least 20000; the (31,5,3) oracle takes 15 to 40 min). Expected 2 to 20 h; worst case 4 h per job.

    ./run_stage.sh main-n31 20261027 $CAP31 40 20 20 "31:4:8 31:5:3 31:6:2"

Single job by hand (spot check only, not part of the frozen run):

    $BIN oracle --n 17 --m 4 --k 2 --a 0 --seed 1027 --natural 6 --planted 4 --coset 4 --out /tmp/g1-spot/oracle
    $BIN run --n 17 --m 4 --k 2 --a 0 --seed 1027 --natural 6 --planted 4 --coset 4 --arm P --conj 0 --conflicts 20000 --tieout 3 --run-id spot --oracle /tmp/g1-spot/oracle/oracle-a0-n17-m4-k2-s1027.jsonl --out /tmp/g1-spot/rows

(`oracle`, `run` and `analyze` take the same `--natural/--planted/--coset` for the oracle and run steps of a cell.)

## 4. Row fields (one JSONL row per cell, arm, target, conj; `rows/rows-<cell>-<arm>-c<conj>.jsonl`)

schema, run_id, host, cell, a, n, m, k, seed, r, fb_x, fb_points, target_id, class, arm, conj, verdict (found/refuted/censored), censor_reason, oracle, agree, cert_ok, tieout (true/false/null), conflict_cap, conflicts, decisions, propagations, word_ops, patterns_attempted, patterns_refuted, solver_calls, models, group_rejected, invalid_models, peak_vars, peak_clauses, peak_xors, domain_clauses, lift_adds, wall_s, build_s.

`analysis/summary.json` (`g1-chain-sat-orbit/summary/v1`): per cell and stratum: paired ratios with intervals for word_ops, conflicts, propagations, wall_s; the A/A pairing; decision label; `c3_noninferior_at_0.95`; priced GAE at 0, 0.1, 1 with `section_5a_band`; achieved A/A sd and MDE; detector injection table; anytime yield and cost per relation; cost per relation of arm P and the analytic rho placeholder.

## 5. After the run

1. Check `failures.txt` per stage, the host (`host` field), and that every `tieout` is true or null (analysis refuses otherwise).
2. Make the research-visuals report, graph and PDF (skill `research-visuals`). No `benchmarks/` or leaderboard row: no matched rho reference and no VALIDATION.json exist for this cell.
3. Append to `docs/session-logs/`, delete nothing under `reports/`. The shared `target/` is not removed by this cell.
