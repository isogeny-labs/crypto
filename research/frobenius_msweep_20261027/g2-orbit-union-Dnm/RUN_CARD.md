# G2 run card (commands for LATER; nothing here has been run)

Preconditions (the gate is not bypassed anywhere in this cell):

1. Upload token for the resume checkpoint. It must be minted where AWS credentials exist (the owner's machine with the
   `ecc-challenges-new` profile); this container has none. On that machine, from the repository root:

       scripts/checkpoint.sh presign-upload --install      # writes ~/.config/checkpoint/upload-token, 7-day lifetime

   or print it without installing and give it to the session as `CHECKPOINT_UPLOAD_TOKEN`:

       scripts/checkpoint.sh presign-upload --days 7

   The token is a bearer credential: never commit it or paste it into a PR, report or card.
2. In this repository: `scripts/checkpoint.sh init`, fill `.resume/RESUME.md` (all seven sections), then
   `scripts/checkpoint.sh push` and `scripts/checkpoint.sh gate` (must exit 0). `run_cells.sh` calls the gate itself.
3. Conductor: `conductor check --summary "G2 orbit-union D(n,m) run" --scope path:research/frobenius_msweep_20261027/g2-orbit-union-Dnm`
   (the control plane at localhost:8080 was down during preparation).

Run (single entry point; arms = frob, geom, rsub, rset, skew, plain, plainbig, all through the same enumerator):

    cd /home/user/crypto/research/frobenius_msweep_20261027/g2-orbit-union-Dnm
    ./run_cells.sh 3          # 3 parallel cell jobs

Expected wall time: 10 to 15 minutes on 3 free cores (single-core upper bound about 28 minutes; allow 30 for contention).
The script runs the unit tests first, builds, writes `plan.tsv`, `commit.txt`, `binary.sha256`, then one job per cell
(32 cells, largest first), then `analyze`.

One cell by hand (all seven arms, interleaved by draw, in one file; the decision rule needs frob, rsub, rset and skew
together, so arms are never split across invocations into one directory):

    BIN=$CARGO_TARGET_DIR/release/g2dnm; RUN=reports/run-$(date -u +%Y%m%dT%H%M%SZ); mkdir -p $RUN
    $BIN run --a 1 --n 13 --m 5 --out $RUN     # default arms: frob,geom,rsub,rset,skew,plain,plainbig; NV=16, NT=48
    $BIN analyze --dir $RUN

Outputs, all under `research/frobenius_msweep_20261027/g2-orbit-union-Dnm/reports/run-<UTC>/`:
`rows_a*_n*_m*.tsv` (raw, one row per cell/arm/draw), `log_*.txt`, `plan.tsv`, `commit.txt`, `binary.sha256`,
`summary.md` (verdict table + per-cell yield tables + A/A), `summary.tsv`, `rho_z.svg` (graph).

After the run (required by the research-visuals skill; not done in preparation): render `summary.md` and `rho_z.svg` to a
PDF and PNG in the same directory following `.claude/skills/research-visuals/SKILL.md`; record host (`nproc`, `uname -a`,
load) in `host.txt`; delete `target/` if the default target dir was used. No `benchmarks/` row and no leaderboard change
(not a performance result). Append the session-log entry (docs/session-logs/YYYY-MM-DD.md) with the verdict table.

Abort conditions: gate fails; unit tests fail; any cell reports INSTRUMENT_FAIL (record, do not rerun with altered
parameters); left-table assertion in `Enumerator::new` (raise nothing: report the cell as not run).
