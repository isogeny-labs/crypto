# G4 run card (commands for LATER; nothing here has been run)

## 0. The token (the checkpoint upload token the repository gate needs)

The gate (`scripts/checkpoint.sh gate`) refuses run launchers until a resume checkpoint is uploaded to S3. This container
has no AWS profile (`ecc-challenges-new`), no `CHECKPOINT_UPLOAD_TOKEN` and no `~/.config/checkpoint/upload-token`, so the
upload cannot happen here. The token is a presigned S3 POST policy that can only write keys under `checkpoints/` for up
to 7 days. It has to be minted where AWS credentials exist (your machine with the `ecc-challenges-new` profile), from the
repository root:

    scripts/checkpoint.sh presign-upload --days 7 --install     # writes ~/.config/checkpoint/upload-token on that machine

or print it and hand it to the session as an environment variable (no `--install`):

    scripts/checkpoint.sh presign-upload --days 7

then, in the session that will run: `export CHECKPOINT_UPLOAD_TOKEN='<printed token>'` (or place the same string in
`~/.config/checkpoint/upload-token`). It is a bearer credential: never commit it, never paste it into a PR, report or card.
Defaults come from `scripts/checkpoint.sh` (bucket `setec-astronomy-artifacts`, prefix `checkpoints`, region `us-east-2`).

## 1. Preconditions (the gate is not bypassed anywhere in this cell)

1. Token available as above.
2. `scripts/checkpoint.sh init`, fill `.resume/RESUME.md` (all sections), `scripts/checkpoint.sh push`, then
   `scripts/checkpoint.sh gate` must exit 0. `run_cells.sh` calls the gate itself and aborts otherwise.
3. Conductor: `conductor check --summary "G4 iota-stable run" --scope path:research/frobenius_msweep_20261027/g4-iota-stable`
   (not done in preparation; the control plane was not reachable).
4. Freeze the protocol: write `protocol.json` from `PROTOCOL.md` (same cells, arms, decision rule) and freeze it with
   `tools/experiment_protocol.py freeze` from the sibling crypto-autoresearcher checkout before the first data point.
5. Unit tests (`cargo test --release`, 13 tests) pass on the exact binary being run; `run_cells.sh` runs them first.

## 2. Run (single entry point)

    cd /home/user/crypto/research/frobenius_msweep_20261027/g4-iota-stable
    ./run_cells.sh 3          # 3 parallel cell jobs, 36 cells, largest field first

Expected wall time: 5 to 15 minutes on 3 free cores (single-core estimate about 12 minutes, plausible range 4 to 40;
allow 30 for contention). Memory below 200 MB per job.

The script: gate, `cargo test --release`, `cargo build --release`, creates `reports/run-<UTC>/`, writes `commit.txt`,
`binary.sha256`, `host.txt`, `plan.tsv`, then one job per cell, then `analyze`.

## 3. One arm or one cell by hand

All seven arms of a cell are produced by one invocation and interleaved by replicate (the decision rule needs vrand,
g4rand, rset, riota and wrong together; never split a cell's arms across directories):

    cargo build --release        # then:
    BIN=$PWD/target/release/g4iota; RUN=reports/run-$(date -u +%Y%m%dT%H%M%SZ); mkdir -p $RUN
    $BIN run --a 0 --n 17 --m 4 --l 6 --out $RUN        # default reps: 24 per half at n=17
    $BIN analyze --dir $RUN

Per arm, for the record: `vpoly`, `vrand`, `g4poly`, `g4rand`, `rset`, `riota`, `wrong` are all written by that one
command (columns `arm`, `half`, `rep`); the A/A halves are `half = aa`. Cell size and replicates:

| field | reps per half | bases per cell (10 R + 2) |
|---|---|---|
| n = 11 | 100 | 1002 |
| n = 13 | 40 | 402 |
| n = 17 | 24 | 242 |
| n = 19 | 16 | 162 |

`--reps R` overrides (a deviation; record it in PROTOCOL.md section 12). `--seed S` defaults to 20261027.

## 4. Outputs

All under `research/frobenius_msweep_20261027/g4-iota-stable/reports/run-<UTC>/`: `rows_a*_n*_m*_l*.tsv` (raw rows, one per
base), `log_*.txt` (a `CELL_NOT_RUN` line means the instrument refused the cell, e.g. enumeration cap; record it, do not
rerun with altered parameters), `plan.tsv`, `commit.txt`, `binary.sha256`, `host.txt`, `summary.md` (instrument/control
table, decision table, per-cell arm tables, what-this-does-not-establish), `summary.tsv`, `fig_cost_ratio.svg`.

After the run (research-visuals skill; not done in preparation): render `summary.md` and `fig_cost_ratio.svg` to a PDF and a
PNG in the same directory, keep the SVG's data beside it (`summary.tsv`), delete `target/` if the default target dir was
used, append the session-log entry (`docs/session-logs/YYYY-MM-DD.md`) with the decision table. No `benchmarks/` row and no
leaderboard change (not a performance result, no replay certificate or rho reference).

## 5. Abort conditions

Gate fails; unit tests fail; a cell reports INSTRUMENT_FAIL or CONTROL_FAIL (record it as the result for that cell; the
instrument, not the base, is suspect: do not rerun with altered parameters); more than a quarter of cells CENSORED or
UNMATCHED (report the design as under-resolved).
