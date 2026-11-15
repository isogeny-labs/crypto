# G6 run card, revision 2 (commands for LATER; nothing here has been run; amended with PROTOCOL.md revision 2, section 14)

## 0. The token (the checkpoint upload token the repository gate needs)

The gate (`scripts/checkpoint.sh gate`, called by the PostToolUse hook) refuses run launchers until a resume checkpoint is uploaded to
S3. It is a **session-side launcher gate**: the SESSION runs it before submitting; it is never invoked inside the remote job and
`run_cells.sh` does not call it. The preparation container has no `CHECKPOINT_UPLOAD_TOKEN`, and the identity it holds was refused
PutObject on `s3://setec-astronomy-artifacts/checkpoints/` (session log 2026-10-27). Mint a token where AWS credentials exist, from
the repository root:

    scripts/checkpoint.sh presign-upload --days 7              # prints the token on stdout
    scripts/checkpoint.sh presign-upload --days 7 --install    # or: writes ~/.config/checkpoint/upload-token on that machine

and in the submitting session `export CHECKPOINT_UPLOAD_TOKEN='<printed token>'`. It is a bearer credential: never commit it, never
paste it into a PR, report or card.

## 1. Route and freeze (to be completed by the experiment director before delegation)

**Route (AGENTS.md 9a; CLAUDE.md "Run authorized x86 CPU experiments ... through isolab-eks"): frozen to Iso Lab on EKS Spot
(`cpu-spot`), ordinary fresh job, not resumable.** Why this class: every metric is an exact integer-derived statistic that is a
deterministic function of the seed and the committed code, so the result does not depend on the host; wall time is not a metric (the
pre-flight timing can only stop a launch). No `cpu-l2-spot`, no L2/A/A admission (those apply to timing claims). A Spot
interruption is harmless: the job is rerun whole. A different backend is used only if the user explicitly requests it; record it here.
Known blockers on 2026-10-27 (session log): Iso Lab's AWS route needs account 104186811093 and the preparation identity is in
another account; the GitHub Actions route is blocked by billing. If the route is unavailable, report that instead of running another
experiment.

| field | value |
|---|---|
| source | the commit containing this directory with a clean tree (`run_cells.sh` refuses a dirty cell directory); record source SHA, binary SHA-256 (`binary.sha256`) and OCI digest separately |
| argv (array, no shell) | `["/bin/bash", "research/frobenius_msweep_20261027/g6-large-m/run_cells.sh", "4"]`, working directory at the checkout root (the script locates itself) |
| env | `G6_MAX_CPU_MIN=600`; `G6_CHECKPOINT_STAMP=<the single line of .resume/.last-push after the session gate passed>` (non-secret; the driver aborts with exit 3 if empty); `ISOLAB_RESULTS_URI` set by Iso Lab |
| resources | 4 vCPU (the `4` is the parallel job count), 2 GiB memory (about 250 MB per job), 2 GiB disk |
| image needs | git checkout with `.git`, Rust stable with cargo and rustc >= 1.78, bash, awk, tar, xz, sha256sum, coreutils, `aws` CLI for the upload; the committed `Cargo.lock`. No checkpoint gate and no credential for it in the job |
| outputs kept | on every exit `driver_exit.txt`, deterministic `run-<UTC>.tar.xz` with `.sha256` in `$ISOLAB_OUTPUT_DIR`, upload of that directory and the unpacked run directory to `$ISOLAB_RESULTS_URI`; upload failure exits 90 |
| deadline | proposed 8 hours from submission (the director freezes it). Expected 1 to 2 CPU hours (unmeasured; 20 to 40 minutes on 4 vCPU); worst case about 23 CPU hours at 250 ns per tuple (PROTOCOL section 11: per-cell budget 2^33 tuples, 2^33.8 where the floor of 8 pairs binds; 5.6 CPU hours at 60 ns), i.e. 6 hours on 4 vCPU. The pre-flight times n = 19, which understates n = 23: the director applies at least x3 when freezing `G6_MAX_CPU_MIN` |
| acceptance | `summary.md` and `verdicts.tsv` exist; `exit_*.txt` are 0; section 0 of `summary.md` lists no missing cell (26 planned); `iota_audit.tsv` has 9 data rows (one header line per call) all `true`; `driver_exit.txt` is 0 |
| retrieval | download the archive and `.sha256`, verify, unpack into `reports/` of a checkout |

## 2. Preconditions (the session passes the gate before submitting; the job never runs it)

1. Token as in section 0; `scripts/checkpoint.sh init`, fill `.resume/RESUME.md` (all sections), `scripts/checkpoint.sh push`, then
   `scripts/checkpoint.sh gate` must exit 0. Then `G6_CHECKPOINT_STAMP="$(cat .resume/.last-push)"` in the shell that submits.
2. Conductor: `conductor check --summary "G6 large-m Frobenius census run" --scope path:research/frobenius_msweep_20261027/g6-large-m`.
3. Freeze the protocol: write `protocol.json` from `PROTOCOL.md` (cells from `g6census cells`, arms, controls, the rule of section 8, delta = 0.10,
   the constants of section 6) and freeze with `tools/experiment_protocol.py freeze` from the sibling crypto-autoresearcher checkout before the first
   data point. **Not done in preparation.** `run_cells.sh` records the SHA-256 of every source, test, doc and script in `inputs.sha256`.
4. Commit the cell directory (clean tree; `target/` is outside the tree).
5. `cargo test --release --locked` (32 tests: 22 in `tests/identities.rs`, 6 in `tests/digits.rs`, 4 in `tests/pipeline.rs`) passes on the exact binary; `run_cells.sh` runs it first.
6. **Open, not done in preparation:** the `experiment.export/v1` step (convert `rows_*.tsv` with the from-rows tool and run check-export before any verdict is read),
   masking of arm labels in the analysis, and a dry run of `analyze` on real-size scratch output (only tiny fixtures were run). `G6_CHECKPOINT_STAMP` is
   checked for non-emptiness only; the real gate is the session hook, which runs before submission.

## 3. Run (single entry point), in order

On Iso Lab the entry point is the argv of section 1. **Locally only on an explicit user request** (it is not the frozen route; the user's own request, not this card, authorises it), from a clean checkout:

    cd /home/user/crypto && G6_CHECKPOINT_STAMP="$(cat .resume/.last-push)" G6_MAX_CPU_MIN=600 \
      bash research/frobenius_msweep_20261027/g6-large-m/run_cells.sh 4

| phase | content | stops when |
|---|---|---|
| 0 | stamp and clean-tree checks (exit 3); `cargo test`; `cargo build --release`; `commit.txt`, `toolchain.txt`, `inputs.sha256`, `binary.sha256`, `host.txt`, `plan.tsv`, `sizes_table.tsv` (Block A); **pre-flight**: one scratch n = 19 cell (first matchable of m = 9..12), ns per tuple, typical and worst-case projection | typical projection above `G6_MAX_CPU_MIN` (exit 4); no matchable pre-flight cell (exit 6) |
| 1 | Block B: nine `iota-audit` runs at n = 13 | any identity fails (exit 5) |
| 2 | the 26 cells (22 original + 4 slot-size control), `xargs -P`, largest n and m first, exit codes in `exit_n*_a*_m*.txt`, logs in `log_*.txt` | never (a failing cell is recorded) |
| 3 | `g6census analyze`: validity, decision rule, size census table, trend, pricing | |
| exit | `finish` trap: archive, sha256, upload | upload failure (exit 90) |

## 4. One block at a time, by hand (**only on an explicit user request**; each block is self-contained; run it whole in a fresh shell)

Block 0, build and test (a local development check; allowed anywhere):

    cd /home/user/crypto/research/frobenius_msweep_20261027/g6-large-m
    export CARGO_TARGET_DIR=${TMPDIR:-/tmp}/g6census-target
    cargo test --release --locked && cargo build --release --locked

Block A, exact formulas (no group, no data; seconds):

    cd /home/user/crypto/research/frobenius_msweep_20261027/g6-large-m
    export CARGO_TARGET_DIR=${TMPDIR:-/tmp}/g6census-target; BIN=$CARGO_TARGET_DIR/release/g6census
    RUN=reports/run-$(date -u +%Y%m%dT%H%M%SZ); mkdir -p $RUN
    $BIN cells > $RUN/plan.tsv; $BIN sizes > $RUN/sizes_table.tsv

Block B, iota identity (exact; seconds each; exit 3 on a failed identity):

    cd /home/user/crypto/research/frobenius_msweep_20261027/g6-large-m
    export CARGO_TARGET_DIR=${TMPDIR:-/tmp}/g6census-target; BIN=$CARGO_TARGET_DIR/release/g6census
    RUN=$(ls -d reports/run-* | tail -1)
    for m in 9 10 11 12; do for k in $m 3; do $BIN iota-audit --n 13 --a 0 --m $m --k $k; done; done > $RUN/iota_audit.tsv
    $BIN iota-audit --n 13 --a 1 --m 9 --k 9 >> $RUN/iota_audit.tsv

Block C, one small cell (replicate curve, minutes) and one target cell (n = 23, the production-shaped one):

    cd /home/user/crypto/research/frobenius_msweep_20261027/g6-large-m
    export CARGO_TARGET_DIR=${TMPDIR:-/tmp}/g6census-target; BIN=$CARGO_TARGET_DIR/release/g6census
    RUN=$(ls -d reports/run-* | tail -1)
    $BIN cell --n 19 --a 0 --m 10 --out $RUN          # defaults: --draws 3000 --k 32 --seed 20261027
    $BIN cell --n 23 --a 0 --m 12 --out $RUN          # n = 23 production-shaped; matchable for some normal bases (PROTOCOL 4.3)

`--draws`, `--k` and `--seed` are frozen at their defaults; a different value is a deviation (record it in PROTOCOL.md section 14).
Exit 2 with `CELL_NOT_RUN` means a refused parameter (m >= n) or an I/O error; `UNMATCHABLE` exits 0 and is a result.

Block D, analysis and figure (after all cells; the figure only plots recorded values):

    cd /home/user/crypto/research/frobenius_msweep_20261027/g6-large-m
    export CARGO_TARGET_DIR=${TMPDIR:-/tmp}/g6census-target; BIN=$CARGO_TARGET_DIR/release/g6census
    RUN=$(ls -d reports/run-* | tail -1)
    $BIN analyze --dir $RUN > /dev/null
    python3 -m venv ${TMPDIR:-/tmp}/venv-g6 && ${TMPDIR:-/tmp}/venv-g6/bin/pip install -q matplotlib typst
    ${TMPDIR:-/tmp}/venv-g6/bin/python plot_g6.py $RUN

## 5. Outputs (under `reports/run-<UTC>/`)

`sizes_n*_a*_m*.tsv` (every census draw: arm, idx, log2 mu, feasible), `rows_n*_a*_m*.tsv` (one row per enumerated instance: arm, idx, log2 mu,
log2 T, p, eff, disp, fano, fano_tamper, planted, conserved, table sizes, build additions, `pred_p` and `pred_disp` of the digit model, NaN unless the instance is a Frobenius chain), `meta_n*_a*_m*.txt` (window, feasible counts, `k_eff`,
`window_center`, skipped items, `status` OK or UNMATCHABLE), `iota_audit.tsv`, `sizes_table.tsv`, `plan.tsv`, `verdicts.tsv`, `summary.md`
(planned versus present, cell verdicts with CHAIN_DERIVED notes, size census table, trend with r(m, s), fixed-m slot-size table, per-covered-target accounting), `fig_g6.svg` / `fig_g6.png`, `log_*`, `exit_*`, provenance files,
`preflight/` (scratch; never read by analyze).

## 6. After the run (research-visuals skill; AGENTS.md 13 and 13a; not done in preparation)

1. Write `REPORT.typ` from `summary.md` and the figure (escape `_` and `#` in Typst markup), render to PDF with `typst compile`, **open and
   inspect the PDF** (every number in it is in `summary.md`; the report states for each cell VOID / UNMATCHABLE / its verdicts, and the
   "cannot reach" list of PROTOCOL.md section 3).
2. Raw TSVs: `sizes_*` hold 12000 rows per cell (about 0.5 MB each, 11 MB in all); pack the raw `sizes_*`, `rows_*`, `log_*`, `exit_*` files as one deterministic
   `raw.tar.xz` with `.sha256`, push with `python3 tools/artifacts/artifacts.py push`, keep `summary.*`, the figure, PDF/PNG, `plan.tsv`, provenance in Git.
3. Delete scratch (`preflight/`, any in-tree build directory).
4. Session-log entry (`docs/session-logs/YYYY-MM-DD.md`) with the cell verdict table. No `benchmarks/` row and no leaderboard change: not a performance result, no
   replay certificate, no matched rho reference (validation gate).
5. Update the lead atlas record for LEAD-ICB-frobenius-search-symmetry with the claims G6-C1 to G6-C5, G6-S6 and G6-SC of PROTOCOL.md section 2 and their verdicts,
   always with the `G6-` prefix: the audit's own C1 to C5 (`coverage.json`) are different claims. For G6-C1/C2 give the chain stratum (derived) and the
   statistical stratum (tested) separately. The atlas record itself has no C-numbering, so quote the claim text with the id.

## 7. Abort conditions (evaluated by `analyze` and the driver)

Unit tests fail; pre-flight projection above budget (exit 4); no timing cell (exit 6); an iota identity fails (exit 5); more than a quarter of
the matchable cells are VOID (instrument untrustworthy; `DIGIT_MODEL_DISAGREES` counts); a cell is missing or `exit_*` is nonzero (INCOMPLETE run, not a result); upload fails (exit 90).
A single VOID cell is that cell's result; it is not rerun with altered parameters.
