# G5 run card (commands for LATER; nothing here has been run)

## 0. The token (the checkpoint upload token the repository gate needs)

The gate (`scripts/checkpoint.sh gate`, called by the PostToolUse hook and by `run_cells.sh`) refuses run launchers until a resume
checkpoint is uploaded to S3. This container has no AWS profile (`ecc-challenges-new`), no `CHECKPOINT_UPLOAD_TOKEN` and no
`~/.config/checkpoint/upload-token` (the hook says exactly that after every write), so the upload cannot happen here. The token
is a presigned S3 POST policy that can only write keys under `checkpoints/` for up to 7 days. It must be minted where AWS
credentials exist (a machine with the `ecc-challenges-new` profile), from the repository root:

    scripts/checkpoint.sh presign-upload --days 7              # prints the token on stdout
    scripts/checkpoint.sh presign-upload --days 7 --install    # or: writes ~/.config/checkpoint/upload-token on that machine

Hand the printed string to the session that will run, in that session:

    export CHECKPOINT_UPLOAD_TOKEN='<printed token>'           # or put the same string in ~/.config/checkpoint/upload-token

It is a bearer credential: never commit it, never paste it into a PR, report or card. Defaults come from `scripts/checkpoint.sh`
(bucket `setec-astronomy-artifacts`, prefix `checkpoints`, region `us-east-2`).

## 1. Preconditions (the gate is not bypassed anywhere in this cell)

1. Token available as in section 0.
2. `scripts/checkpoint.sh init`, fill `.resume/RESUME.md` (all sections), `scripts/checkpoint.sh push`, then
   `scripts/checkpoint.sh gate` must exit 0. `run_cells.sh` calls the gate itself and aborts otherwise.
3. Conductor: `conductor check --summary "G5 Proposition T / Sigma census run" --scope path:research/frobenius_msweep_20261027/g5-propT-census`
   (not done in preparation).
4. Freeze the protocol: write `protocol.json` from `PROTOCOL.md` (cells, arms, decision rule in section 8) and freeze it with
   `tools/experiment_protocol.py freeze` from the sibling crypto-autoresearcher checkout before the first data point.
5. `cargo test --release` (21 tests: 19 in `tests/identities.rs`, 2 in `tests/pipeline.rs`) passes on the exact binary that runs;
   `run_cells.sh` runs them first. A test failure aborts the run.

## 2. Run (single entry point)

    cd /home/user/crypto/research/frobenius_msweep_20261027/g5-propT-census
    ./run_cells.sh 3          # 3 parallel jobs: 57 overlap cells, 20 rank cells, 28 Proposition T cells

Expected wall time: 8 to 20 minutes on 3 free cores (20 to 50 CPU minutes; plausible range 5 to 60; the per-operation cost has
not been measured). Memory below 300 MB per job. Disk: reports are a few MB; the crate's `target/` is about 100 MB.

The script: gate, `cargo test --release`, `cargo build --release`, creates `reports/run-<UTC>/`, writes `commit.txt`,
`binary.sha256`, `host.txt`, `plan.tsv` (the frozen cell list from `g5census cells`), `jobs.txt`, runs one job per cell
through `xargs -P`, then `g5census analyze`.

## 3. One experiment, one cell, by hand

Each invocation emits all arms of that cell interleaved (the decision rule needs the arms of a cell together; never split a cell's
arms across directories). Arms per kind:

| command | arms emitted (column `arm`) | halves |
|---|---|---|
| `g5census propt` | koblitz, wrongdir, ordinary (+ stable at n = 17) | main, aa (blocks of 500 trials) |
| `g5census overlap` | poly (1 instance), stable (n = 17, l = 8, 2 instances), rand, rset, orbunion, tuned | main, aa |
| `g5census rank` | rand, tuned | main, aa |

    cd /home/user/crypto/research/frobenius_msweep_20261027/g5-propT-census
    cargo build --release
    BIN=$PWD/target/release/g5census; RUN=reports/run-$(date -u +%Y%m%dT%H%M%SZ); mkdir -p $RUN
    $BIN cells > $RUN/plan.tsv                                             # a n m l reps
    $BIN propt   --n 13 --m 5 --trials 20000 --assigns 4 --out $RUN         # ~5 to 30 s
    $BIN overlap --a 0 --n 13 --m 4 --l 7 --reps 200 --out $RUN             # reps from plan.tsv; ~10 to 40 s
    $BIN rank    --a 0 --n 13 --m 3 --l 7 --reps 6 --targets 16 --rowcap 40 --out $RUN
    $BIN analyze --dir $RUN

Planned overlap cells (`a n m l`, from the planner; the full list with reps is `g5census cells`):

| curve (r) | cells (m: l values) |
|---|---|
| (1,11) r = 991 | 2: 7,8; 3: 6,7; 4: 5,6; 5: 5,6; 6: 5,6; 7: 5,6; 8: 6 |
| (0,13) r = 2003 | 2: 9,10; 3: 7,8; 4: 7,8; 5: 7,8; 6: 7; 7: 7; 8: 7 |
| (1,17) r = 65587 | 2: 8,10,11; 3: 8,9; 4: 7,8; 5: 6,7; 6: 6; 7: 6; 8: 6 |
| (0,19) r = 130873 | 2: 12,13; 3: 9,10; 4: 8,9; 5: 7,8; 6: 7; 7: 7; 8: 7 |
| (1,19) r = 262543 (secondary, gate decides) | 2: 11,12; 3: 8,9; 4: 7,8; 5: 7; 6: 6; 7: 6; 8: 6 |

`--reps R` overrides the replicate count (a deviation; record it in PROTOCOL.md section 13). `--seed S` defaults to 20261027.
`--trials`, `--assigns`, `--targets`, `--rowcap` as above are frozen values for the full run.

## 4. Outputs

All under `research/frobenius_msweep_20261027/g5-propT-census/reports/run-<UTC>/`:
`propt_n*_m*.tsv`, `overlap_a*_n*_m*_l*.tsv`, `rank_a*_n*_m*_l*.tsv` (raw rows; `status` other than OK means the base was skipped
and why), `log_*.txt` (a `CELL_NOT_RUN` line means the instrument refused the cell, for example the group gate; record it, do not
rerun with altered parameters), `plan.tsv`, `jobs.txt`, `commit.txt`, `binary.sha256`, `host.txt`, `summary.md` (T table, decision
table with gates, indicator correlation, trend and slopes, stratum table, closure count, rank table, voids, what-this-does-not-
establish), `summary.tsv` (flat statistic rows), `fig_overlap_vs_m.svg`.

After the run (research-visuals skill; not done in preparation): render `summary.md` and `fig_overlap_vs_m.svg` to a PDF and a PNG in
the same directory, keep the SVG's data beside it (`summary.tsv`), delete `target/` if the default target dir was used, and append
the session-log entry (`docs/session-logs/YYYY-MM-DD.md`) with the decision table. No `benchmarks/` row and no leaderboard change:
this is not a performance result, and it has no replay certificate or matched rho reference (validation gate).

## 5. Abort conditions

Gate fails; unit tests fail; a T cell reports `PROP_T_FALSIFIED_OR_INSTRUMENT_FAIL` or `CONTROL_FAIL`, or any cell reports
`INSTRUMENT_FAIL`, in more than a quarter of cells (report the instrument as untrustworthy and stop); more than a quarter of overlap
cells `CENSORED` (report the design as under-resolved). A single failing cell is recorded as that cell's result (the instrument, not
the base, is suspect); it is not rerun with altered parameters.
