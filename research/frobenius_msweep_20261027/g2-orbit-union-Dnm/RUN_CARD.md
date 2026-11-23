# G2 run card (commands for LATER; nothing here has been run)

## 0. The token command (what was asked for)

The resume-checkpoint upload token must be minted on a machine that has AWS credentials and the `ecc-challenges-new` profile
(this container has no `aws` CLI, no profile, no `CHECKPOINT_UPLOAD_TOKEN` and no `~/.config/checkpoint/upload-token`).
AGENTS.md says a job on the owner's Mac re-mints that file every 15 minutes: look there first and copy it over. Otherwise,
from the repository root on the credentialed machine:

    scripts/checkpoint.sh presign-upload --install        # writes ~/.config/checkpoint/upload-token, default 7 days

or print it (then hand it to the session as the environment variable `CHECKPOINT_UPLOAD_TOKEN`):

    scripts/checkpoint.sh presign-upload --days 7

The token is a bearer credential: never commit it or paste it into a PR, report or card.

## 1. Preconditions (the gate is not bypassed anywhere in this cell; no CHECKPOINT_SKIP)

1. With the token in place, in the repository: `scripts/checkpoint.sh init`, fill `.resume/RESUME.md` (all seven sections),
   `scripts/checkpoint.sh push`, then `scripts/checkpoint.sh gate` (must exit 0). `run_cells.sh` calls the gate itself.
2. Conductor: `conductor check --summary "G2 orbit-union D(n,m) run" --scope path:research/frobenius_msweep_20261027/g2-orbit-union-Dnm`
   (the control plane at localhost:8080 was down during preparation).
3. Commit the cell directory. `run_cells.sh` refuses a dirty tree under this directory (exit 3) and records the frozen sha
   (the last commit that touched the directory). The PR that lands the result names that sha.

Gate scope (amendment 2026-10-27): `scripts/checkpoint.sh gate` is a SESSION-side launcher gate. The session runs it (and `run_cells.sh`, which calls it) before
submitting or launching; it is never invoked inside a remote/isolab job. Each fenced block below is self-contained: it starts with its own `cd` and exports, so it runs in a fresh shell.

## 2. Run

Route (revision 3, to be decided by the experiment director before the freeze): AGENTS.md 9a makes isolab-eks the default for authorized x86 CPU
experiments. This card is written for the shared local container (`./run_cells.sh 3`). The counts and class vectors are exact and cannot depend
on isolation, host load or timing (PROTOCOL section 11), so the route affects wall time only. Either record "local route, counts are
isolation-independent" in the PR, or submit the same command through the isolab-eks skill; do not leave the route unstated. Whichever is
chosen, the `.resume/RESUME.md` checkpoint (section 1) comes first.

Deviation from the "one command per arm" template, stated: arms are NOT run as separate invocations. The decision rule compares
frob against frobperm, rsub, rset and skew on the same targets, interleaved per draw with shuffled arm order, and writes one rows file per
cell; splitting arms across processes would lose the interleaving and collide on file names. The single entry point runs all 8
arms (frob, frobperm, rsub, rset, skew, geom, plain, plainbig) through the same `Enumerator`:

    cd /home/user/crypto/research/frobenius_msweep_20261027/g2-orbit-union-Dnm
    ./run_cells.sh 3          # 3 parallel cell jobs; the 4th CPU stays free

Steps it performs: gate, clean-tree check, `cargo test --release` (30 tests must pass; about 50 s), `cargo build --release`, then writes
`plan.tsv`, `cells.txt`, `run_meta.txt` (frozen sha, HEAD, rustc -V, cargo -V, seed, jobs, nproc, uname, load, start/finish UTC),
`host.txt`, `binary.sha256`, `rows.sha256`; runs the 52 cells (largest estimated first, one process per cell, `xargs -P`), logs each to
`log_a*_n*_m*_l*.txt`, records failed jobs in `run_meta.txt` and continues, then runs `g2dnm analyze` on whatever was written.

Expected wall time: about 18 minutes on 3 free cores (the plan sums to about 3183 s single core, longest cell (17,3,l=6) about
633 s). The 150 ns per lookup behind it is an assumption uncertain by about 3x, so 6 to 54 minutes; allow one hour. See PROTOCOL section 8.

One cell by hand (all 8 arms in one file; set the target dir and build first):

    cd /home/user/crypto/research/frobenius_msweep_20261027/g2-orbit-union-Dnm
    export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$PWD/target}; cargo build --release
    BIN=$CARGO_TARGET_DIR/release/g2dnm; RUN=reports/run-$(date -u +%Y%m%dT%H%M%SZ); mkdir -p $RUN
    $BIN plan > $RUN/plan.tsv; (cd /home/user/crypto && git rev-parse HEAD) > $RUN/commit.txt; sha256sum $BIN > $RUN/binary.sha256
    $BIN run --a 1 --n 13 --m 5 --l 3 --out $RUN      # all arms, NV=16, NT=48 (effective nt per row)
    $BIN analyze --dir $RUN
    # a hand-run cell is only a spot check; it is not the frozen run (no gate, no clean-tree check, no run_meta.txt)

Outputs, all under `research/frobenius_msweep_20261027/g2-orbit-union-Dnm/reports/run-<UTC>/`:
`rows_a*_n*_m*_l*.tsv` (raw, one row per cell/arm/draw, with seed, xs, xodd), `log_*.txt`, `plan.tsv`, `cells.txt`, `run_meta.txt`,
`host.txt`, `binary.sha256`, `summary.md` (verdict table with the "rho resolved" flag, per-cell yield tables, A/A, l ladder with its PERSISTENT / FALLING_OR_ABSENT status, section 12 disclosures),
`summary.tsv`, `verdicts.tsv` (with z_contrast), `rho_z.svg` (graph).

Disk: the whole run is under 25 MB. Rows files over 1 MiB, expected for (13,5,3) and (17,5,3) at about 1.1 and 3 MB per curve,
follow AGENTS.md section 13 (`tools/artifacts`, `.artifacts.json`), not `git add`.

After the run (required by the research-visuals skill; not done in preparation): render `summary.md` and `rho_z.svg` to a PDF and PNG
in the same directory following `.claude/skills/research-visuals/SKILL.md`; delete `target/` if the default target dir was used. No
`benchmarks/` row and no leaderboard change (not a performance result, no validation-gate row). Append the session-log entry
(docs/session-logs/YYYY-MM-DD.md) with the verdict table, naming the frozen sha.

Abort conditions: gate fails; dirty tree; unit tests fail; any cell reports INSTRUMENT_FAIL (record, do not rerun with altered
parameters); a cell process that panics on the left-table assertion (record it as NOT RUN; raise nothing). A cell whose arms have fewer than 16 rows is reported INCOMPLETE (NOT RUN), never re-run with altered parameters.
`run_cells.sh` writes `rows.sha256` (sha256 of every rows file) before the analysis runs (rev 3).
