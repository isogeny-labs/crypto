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

The checkpoint gate is a session-side launcher gate: the session runs `scripts/checkpoint.sh gate` (step 2) before it launches or
submits anything, and it is never invoked inside a remote job. Each fenced block below is self-contained and runs in a fresh shell
(it sets its own `cd`, `BIN` and `RUN`). Amendment A2 (PROTOCOL.md 12b) changed the draw rule and the censoring gate; the run must
use a build that contains it.

1. Token available as above.
2. `scripts/checkpoint.sh init`, fill `.resume/RESUME.md` (all sections), `scripts/checkpoint.sh push`, then
   `scripts/checkpoint.sh gate` must exit 0. `run_cells.sh` calls the gate itself and aborts otherwise.
3. Conductor: `conductor check --summary "G4 iota-stable run" --scope path:research/frobenius_msweep_20261027/g4-iota-stable`
   (not done in preparation; the control plane was not reachable).
4. Freeze the protocol: write `protocol.json` from `PROTOCOL.md` revision 2 (same cells, arms, decision rule, estimands in
   section 7) and freeze it with `python3 experiment_protocol.py freeze protocol.json` from the sibling crypto-autoresearcher
   checkout (`tools/`) before the first data point. Put the SHA-256 of `src/analyze.rs` and of `PROTOCOL.md` in it.
5. Unit tests (`cargo test --release`, 24 tests) pass on the exact binary being run; `run_cells.sh` runs them first.
6. Commit the cell (or at least record the dirty state): the driver writes `commit.txt` and `dirty.txt`; a run from a dirty
   tree is allowed only for files outside this directory.

## 2. Run (single entry point)

    cd /home/user/crypto/research/frobenius_msweep_20261027/g4-iota-stable
    SEED=20261027 ./run_cells.sh 3          # 3 parallel cell jobs, 33 cells, largest field first

Expected wall time: 15 to 35 minutes on 3 free cores (single-core 40 to 90 minutes, code reasoning, not measured; budget
60 minutes wall). Memory typically under 300 MB per job, up to about 1.2 GB in a rare tail (keep 4 GB free for 3 jobs).

Per-cell wall budget, single core (PROTOCOL.md section 10; `timing.tsv` in the run directory records the real seconds, compare
the first cells against this and stop, not alter, if one exceeds 3 times):

| field | cells | budget per cell |
|---|---|---|
| n = 11 (a = 1) | 9 | under 1 to 2 min |
| n = 13 (a = 0, 1) | 12 | under 1 to 2 min |
| n = 17 (a = 0, 1) | 10 | 1 to 5 min |
| n = 19 (a = 0) | 5 | 2 to 10 min (the m = 2, l = 10 cell is the longest) |

The script: gate, `cargo test --release`, `cargo build --release`, creates `reports/run-<UTC>/`, writes `commit.txt`,
`dirty.txt`, `toolchain.txt`, `args.txt` (seed, jobs), `sources.sha256`, `binary.sha256`, `host.txt`, `plan.tsv`
(a n m l reps), then one job per cell (`log_*.txt`, `timing.tsv`), then `analyze`, then `MANIFEST.sha256` over the rows and
summaries.

## 3. One cell by hand (one command runs all arms of the cell)

All decision arms of a cell are produced by one invocation and interleaved by replicate: they are exact-matched on the item
count inside each replicate, so splitting the arms of a cell across directories or runs is not possible and not allowed. The
list of arms in a cell is vpoly, vrand, g4poly, g4rand, rset, riota, wrong (columns `arm`, `half`, `rep`; A/A halves are
`half = aa`); the command is identical for every arm:

    cd /home/user/crypto && scripts/checkpoint.sh gate && cd research/frobenius_msweep_20261027/g4-iota-stable \
      && cargo build --release \
      && BIN=$PWD/target/release/g4iota && RUN=$PWD/reports/run-$(date -u +%Y%m%dT%H%M%SZ) && mkdir -p $RUN \
      && $BIN cells | grep -x '0 17 4 6 24' > $RUN/plan.tsv \
      && $BIN run --a 0 --n 17 --m 4 --l 6 --seed 20261027 --out $RUN \
      && $BIN analyze --dir $RUN

(The `gate &&` prefix is required: the hook's launcher pattern does not match `$BIN run`, so without the prefix the hand run
would bypass the gate. Record `--reps`, `--seed` and the git commit by hand if you use a hand run; `run_cells.sh` records them.)

| field | reps per half | bases per cell (5 matched arms x 2 halves x R + 2 singles) |
|---|---|---|
| n = 11 | 100 | 1002 |
| n = 13 | 40 | 402 |
| n = 17 | 24 | 242 |
| n = 19 | 16 | 162 |

`--reps R` overrides (a deviation; record it in PROTOCOL.md section 12 and in plan.tsv, which `analyze` trusts).

## 4. Outputs

All under `research/frobenius_msweep_20261027/g4-iota-stable/reports/run-<UTC>/`: `rows_a*_n*_m*_l*.tsv` (raw rows, one per
base), `log_*.txt` (a `CELL_NOT_RUN` line means the instrument stopped a cell, e.g. no matched draw; the cell is then
INCOMPLETE in the summary; record it, do not rerun with altered parameters), `timing.tsv`, `plan.tsv`, `commit.txt`,
`dirty.txt`, `toolchain.txt`, `args.txt`, `sources.sha256`, `binary.sha256`, `host.txt`, `MANIFEST.sha256`, `summary.md`
(design-level gates, instrument / control / completeness table, decision table, per-m class counts, per-cell arm tables,
what-this-does-not-establish), `summary.tsv`, `fig_cost_ratio.svg`.

## 5. Abort conditions

Gate fails; unit tests fail; a cell reports INSTRUMENT_FAIL or CONTROL_FAIL (record it as the result for that cell; the
instrument, not the base, is suspect: do not rerun with altered parameters); the A/A false-alarm gate fails (design-level
CONTROL_FAIL); more than a quarter of cells NOT_RUN, INCOMPLETE, VOID, CENSORED or UNMATCHED (report the design as
under-resolved). A cell that stops early (`CELL_NOT_RUN`, killed job) is INCOMPLETE and may be rerun only as a documented
deviation of the same unchanged command into a fresh run directory (never into the old one), never with altered parameters.

## 6. After the run (not done in preparation)

1. research-visuals skill: render `summary.md` and `fig_cost_ratio.svg` to a PDF and a PNG in the same directory; keep
   `summary.tsv` beside the SVG.
2. Bind the rows to the frozen protocol: `python3 experiment_protocol.py from-rows <rows file> --protocol protocol.json --out
   export.json --export-id <id>` per rows file (or the concatenation), then `check-export --protocol protocol.json --verify-files`
   (sibling crypto-autoresearcher `tools/`; check `--help` there for the exact `--map`).
3. Delete `target/`; append the session-log entry (`docs/session-logs/YYYY-MM-DD.md`) with the decision table; record the
   realised CV of cpr per arm against the 0.1 to 0.3 assumption of PROTOCOL.md section 6.
4. No `benchmarks/` row and no leaderboard change (not a performance result; no replay certificate, validator report or
   matched rho reference).
