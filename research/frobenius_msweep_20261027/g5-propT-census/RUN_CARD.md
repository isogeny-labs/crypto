# G5 run card, revision 2 (commands for LATER; nothing here has been run)

## 0. The token (the checkpoint upload token the repository gate needs)

The gate (`scripts/checkpoint.sh gate`, called by the PostToolUse hook and by `run_cells.sh`) refuses run launchers until a resume
checkpoint is uploaded to S3. The preparation container has no AWS profile (`ecc-challenges-new`), no `CHECKPOINT_UPLOAD_TOKEN`
and no `~/.config/checkpoint/upload-token`, so the upload cannot happen there. The token is a presigned S3 POST policy that can only
write keys under `checkpoints/` for up to 7 days. Mint it where AWS credentials exist, from the repository root:

    scripts/checkpoint.sh presign-upload --days 7              # prints the token on stdout
    scripts/checkpoint.sh presign-upload --days 7 --install    # or: writes ~/.config/checkpoint/upload-token on that machine

Hand the printed string to the session that will run, in that session:

    export CHECKPOINT_UPLOAD_TOKEN='<printed token>'           # or put the same string in ~/.config/checkpoint/upload-token

It is a bearer credential: never commit it, never paste it into a PR, report or card. Defaults come from `scripts/checkpoint.sh`
(bucket `setec-astronomy-artifacts`, prefix `checkpoints`, region `us-east-2`).

## 1. Route and freeze (to be completed by the experiment director before delegation)

**Route (AGENTS.md 9a, CLAUDE.md "Run authorized x86 CPU experiments ... through isolab-eks by default"): frozen to Iso Lab on EKS
Spot (`cpu-spot`), ordinary fresh job, not resumable.** Justification for the choice of this class: the outcome metrics are exact
integer counts that are deterministic functions of the seed and the committed code, so the result does not depend on the host;
wall time is not a metric and no decision reads it (the pre-flight timing can only stop a launch). That makes an isolated
L2 benchmark class unnecessary (no `cpu-l2-spot`, no L2/A/A admission, which apply to timing claims) and a Spot interruption
harmless: every job is rerun whole from the same command. The route is therefore the default, not an exception. A different
backend (for example the preparation container) is used only if the user explicitly requests it, and then it is recorded here.

Frozen request fields to hand to the `isolab-eks` skill (card instructions in its `references/submission.md`):

| field | value |
|---|---|
| source | the commit that contains this directory with a clean tree (`run_cells.sh` refuses a dirty cell directory); image built from that commit; record source SHA, binary SHA-256 (`binary.sha256`) and OCI digest separately |
| argv | `cd research/frobenius_msweep_20261027/g5-propT-census && G5_MAX_CPU_MIN=400 ./run_cells.sh 4` |
| resources | 4 vCPU (the `4` is the number of parallel jobs, equal to the vCPUs), 2 GiB memory (jobs stay below 300 MB), 2 GiB disk (build 100 MB, reports below 50 MB) |
| image needs | Rust stable with cargo, bash, awk, bc, git, coreutils; `scripts/checkpoint.sh gate` must pass inside the job (the driver calls it unconditionally and has no bypass); the committed `Cargo.lock` must be present in the checkout |
| deadline | 3 hours from submission (expected 10 to 25 minutes; the pre-flight aborts above `G5_MAX_CPU_MIN`) |
| acceptance | `reports/run-<UTC>/summary.md` exists; its section 0 shows every job complete or CELL_NOT_RUN; "Abort conditions" says none triggered; `exit_*.txt` are 0 or 2 |
| retrieval | the whole `reports/run-<UTC>/` directory as one archive with its SHA-256 (section 6) |

If the image or build route is unavailable, report that instead of running a different experiment.

## 2. Preconditions (the gate is not bypassed anywhere in this cell)

1. Token available as in section 0; `scripts/checkpoint.sh init`, fill `.resume/RESUME.md` (all sections),
   `scripts/checkpoint.sh push`, then `scripts/checkpoint.sh gate` must exit 0.
2. Conductor: `conductor check --summary "G5 Proposition T / Sigma census run" --scope path:research/frobenius_msweep_20261027/g5-propT-census`
   (not done in preparation).
3. Freeze the protocol: write `protocol.json` from `PROTOCOL.md` (cells from `g5census cells`, arms, controls, the decision rule of
   section 8 with delta = 0.10, the operating-characteristic table of section 6) and freeze it with
   `tools/experiment_protocol.py freeze` from the sibling crypto-autoresearcher checkout before the first data point. **Not done in
   preparation.** The SHA-256 of `PROTOCOL.md`, `RUN_CARD.md`, `run_cells.sh`, `Cargo.lock` and every source and test file is
   recorded by the driver in `inputs.sha256`, so a post-freeze edit is visible.
4. Commit the cell directory (clean tree). `target/` is ignored; the driver builds into `$TMPDIR/g5census-target`.
5. `cargo test --release --locked` (30 tests: 25 in `tests/identities.rs`, 5 in `tests/pipeline.rs`) passes on the exact binary
   that runs; `run_cells.sh` runs it first. A test failure aborts.

## 3. Run (single entry point) and what it does, in order

    cd /home/user/crypto/research/frobenius_msweep_20261027/g5-propT-census
    G5_MAX_CPU_MIN=400 ./run_cells.sh 4

| phase | content | stops when |
|---|---|---|
| 0 | gate; clean-tree check (exit 3); `cargo test`; `cargo build --release --locked`; `commit.txt`, `toolchain.txt`, `inputs.sha256`, `binary.sha256`, `host.txt`, `plan.tsv` (a n m l reps est_ops), `plan_notes.txt`; **pre-flight**: one `--reps 1` call per curve on its most expensive planned cell plus one propt and one rank call, on scratch output in `preflight/`, ns per planning-unit operation and a projected CPU-minute total | projection above `G5_MAX_CPU_MIN` (exit 4) |
| 1 | the 28 Proposition T cells, then `g5census gate` (every T cell must hold and every control must detect) | gate fails (exit 5, `summary.md` written, the expensive cells are NOT launched) |
| 2 | 20 rank cells, then 54 overlap cells (largest first), `xargs -P` with exit codes kept in `exit_<kind>_a.._n.._m.._l...txt` and logs in `log_*.txt` | never (a failing cell is recorded) |
| 3 | `g5census analyze`: reconciliation of planned versus present, decision rule, abort conditions | |

Expected wall time on 4 vCPU: 7 to 20 minutes (25 to 60 CPU minutes; plausible range 8 to 120; per-operation cost not measured).
Memory below 300 MB per job.

## 4. One experiment, one cell, by hand (all flags explicit; `--reps`, `--trials`, `--assigns`, `--targets`, `--rowcap` have no defaults)

Each invocation emits all arms of that cell interleaved (the decision rule needs the arms of a cell together; never split a cell's
arms across directories). Arms per kind:

| command | arms emitted (column `arm`) | halves |
|---|---|---|
| `g5census propt` | koblitz, wrongdir, ordinary (+ stable at n = 17) | main, aa (blocks of 500 trials) |
| `g5census overlap` | poly (1 instance), stable (n = 17, l = 8, 2 instances), rand, rset, orbunion, tuned; every base measured with Frobenius, null, null2 and planted multipliers | main, aa |
| `g5census rank` | rand, tuned | main, aa |

    cd /home/user/crypto/research/frobenius_msweep_20261027/g5-propT-census
    export CARGO_TARGET_DIR=${TMPDIR:-/tmp}/g5census-target; cargo build --release --locked
    BIN=$CARGO_TARGET_DIR/release/g5census; RUN=reports/run-$(date -u +%Y%m%dT%H%M%SZ); mkdir -p $RUN
    $BIN cells > $RUN/plan.tsv 2> $RUN/plan_notes.txt                      # a n m l reps est_ops
    $BIN propt   --n 13 --m 5 --trials 20000 --assigns 4 --out $RUN         # ~5 to 30 s
    $BIN overlap --a 0 --n 13 --m 4 --l 7 --reps 200 --out $RUN             # reps from plan.tsv; ~10 to 60 s
    $BIN rank    --a 0 --n 13 --m 3 --l 8 --reps 6 --targets 16 --rowcap 40 --out $RUN   # l = the largest planned l
    $BIN gate    --dir $RUN                                                 # Proposition T gate (needs the propt files)
    $BIN analyze --dir $RUN

Planned overlap cells (`a n m l`, from the planner; reps in parentheses; `g5census cells` is authoritative):

| curve (r) | cells (m: l values) |
|---|---|
| (1,11) r = 991 | 2: 7,8; 3: 6,7; 4: 6,7; 5: 6; 6: 6; 7: 6; 8: 6 |
| (0,13) r = 2003 | 2: 9,10; 3: 7,8; 4: 7,8; 5: 7,8; 6: 7; 7: 7; 8: 7 |
| (1,17) r = 65587 | 2: 8,10,11; 3: 8,9; 4: 7,8; 5: 6,7; 6: 6; 7: 6; 8: 6 |
| (0,19) r = 130873 | 2: 12,13; 3: 9,10; 4: 8,9; 5: 7,8; 6: 7; 7: 7; 8: 7 |
| (1,19) r = 262543 (secondary, gate decides) | 2: 11,12; 3: 8,9; 4: 7,8; 5: 7; 6: 6; 7: 6; 8: 6 |

`--seed S` defaults to 20261027. A different `--reps` than the plan is a deviation (record it in PROTOCOL.md section 13).

## 5. Outputs

All under `research/frobenius_msweep_20261027/g5-propT-census/reports/run-<UTC>/`:
`propt_n*_m*.tsv`, `overlap_a*_n*_m*_l*.tsv`, `rank_a*_n*_m*_l*.tsv` (raw rows; `status` other than OK means the base was skipped
and why), `log_*.txt`, `exit_*.txt`, `plan.tsv`, `plan_notes.txt`, `jobs.txt`, `jobs_propt.txt`, `jobs_main.txt`, `commit.txt`,
`toolchain.txt`, `inputs.sha256`, `binary.sha256`, `host.txt`, `preflight/` (scratch, never read by analyze), `propt_gate.txt`,
`summary.md` (planned versus present, T table, decision table with gates and the A/A, NN and PLANT columns, gate bookkeeping,
per-m statements by family, indicator correlation, trend and slopes with intervals, low-height stratum B4, orbit strata B5,
closure, rank table with INFORMATIVE flags, abort conditions, voids, what-this-does-not-establish), `summary.tsv` (flat
statistic rows), `fig_overlap_vs_m.svg`.

## 6. After the run (research-visuals skill; AGENTS.md 13 and 13a; not done in preparation)

1. Render `summary.md` and `fig_overlap_vs_m.svg` to a PDF and a PNG in the same directory; keep `summary.tsv` beside the SVG.
2. Raw TSVs: the 200-rep overlap files approach 1 MiB each (about 10 to 15 MB in all), above the artifact threshold. Pack the raw
   `propt_*`, `overlap_*`, `rank_*`, `log_*`, `exit_*` files as one deterministic `raw.tar.xz` with `raw.tar.xz.sha256`
   (AGENTS.md 13a recipe), push it with `python3 tools/artifacts/artifacts.py push` (manifest `.artifacts.json` in the run
   directory), and keep `summary.*`, the figure, the PDF/PNG, `plan.tsv`, `commit.txt`, `toolchain.txt`, `inputs.sha256`,
   `binary.sha256` in Git. Do not `git add` the raw TSVs.
3. Delete scratch (`preflight/`, the build directory if it was inside the tree).
4. Append the session-log entry (`docs/session-logs/YYYY-MM-DD.md`) with the decision table and the abort-condition lines. No
   `benchmarks/` row and no leaderboard change: this is not a performance result, and it has no replay certificate or matched rho
   reference (validation gate).

## 7. Abort conditions (evaluated by `analyze`, section "Abort conditions")

Gate fails; unit tests fail; the pre-flight projection exceeds the budget; a T cell reports `PROP_T_FALSIFIED_OR_INSTRUMENT_FAIL`
or `CONTROL_FAIL` (the driver stops after phase 1); more than a quarter of the evaluated cell-arms VOID (instrument untrustworthy) or
CENSORED (design under-resolved); A/A misses or null-vs-null misses above 12 percent of the evaluated cell-arms (mean 5 percent by
construction); the planted control missing the known excess on more than 12 percent of its informative cell-arms; any job
INCOMPLETE (reported as an incomplete run, not as a result). A single failing cell is recorded as that cell's result (the
instrument, not the base, is suspect); it is not rerun with altered parameters.
