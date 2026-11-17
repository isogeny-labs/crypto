# First-pattern external XOR solver handoff

The hosted 5+1 primary-N83 Gray and hash pool runs in
[`../hosted-split-panel-20261025/attempt-1/replay-receipt.json`](../hosted-split-panel-20261025/attempt-1/replay-receipt.json)
both reached 20,000 native SAT conflicts without a model. This handoff exports
the same trial-zero all-affine identity pattern to extended DIMACS for an
XOR-aware SAT solver. It changes the solver backend; it is **not** a completed
factor-base comparison. The 4+2 shape is also supported. The exporter never
downloads S3 objects or starts a solver.

The [selection obstruction](../../../../problems/n83-mixed-orbit-factor-base-selection.md)
records the evidence needed to move from this solver probe to a total-runtime
choice.

The inputs are the exact pilot manifest at `pilot-01/manifest.json`, the exact
public corpus at `pilot-01/probe-corpus.json`, and two compressed S3 archives
under `s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/a0/objects/`:

| Role | Compressed BLAKE3 | Compressed SHA-256 |
| --- | --- | --- |
| Base, 600 hash orbits | `3888e2597fd3519807c9d722f49f83329c93ce5cb0929501cf7cb7cbc3cab1a0` | `8463aaf30365d6f6f5d6b189aff4e8afcb6c8305ec27c216700776cbdccba29d` |
| Gray pool | `966a0493c96674f0af6d184615ed05309d08fba5c7342957b4186cd50366dc47` | `df02963d1706fa7a64fd7558d88db79b23ea34801ac58062c4c500113e8c57fe` |
| Hash pool | `b4dda45e6c3edfca673db6f3f82c16cce8c860f5e62c092155258df399a0363e` | `3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf` |

Use a clean build from this branch and retain the source commit, binary hash,
host identity, object GET/HEAD checks, wall time, peak RSS/cgroup memory, solver
version and exit status in each run receipt. The worker rejects changed archive
bytes, manifest, public corpus, same-stream pairing, and out-of-cap domains.
For each pool and each of 5+1 and 4+2, export into a new empty directory:

```sh
cargo build --release --example koblitz_n83_mixed_orbit_worker
WORKER=target/release/examples/koblitz_n83_mixed_orbit_worker
MANIFEST=research/koblitz_n83_factor_base_sweep_20261008/pilot-01/manifest.json
CORPUS=research/koblitz_n83_factor_base_sweep_20261008/pilot-01/probe-corpus.json
BASE_HASH=3888e2597fd3519807c9d722f49f83329c93ce5cb0929501cf7cb7cbc3cab1a0
POOL_HASH=b4dda45e6c3edfca673db6f3f82c16cce8c860f5e62c092155258df399a0363e
"$WORKER" export-cross-xcnf "$BASE_HASH" "$MANIFEST" "$BASE_OBJECT" \
  "$POOL_HASH" "$MANIFEST" "$POOL_OBJECT" "$CORPUS" 5 1 "$EXPORT_DIR"
```

`BASE_OBJECT`, `POOL_OBJECT`, and `EXPORT_DIR` are caller-supplied paths. Fetch
the two S3 objects to the first two paths, check their SHA-256 against the
table, and make `EXPORT_DIR` a nonexistent path. For Gray, change only
`POOL_HASH` and `POOL_OBJECT`. `export.json` binds the complete XCNF digest,
source-file digests, input digests, split, equation, variables, original clauses
and XOR rows. Preserve `instance.xcnf` as an immutable run artifact.

On Linux, a first bounded CryptoMiniSat 5.14.7 probe can use one thread,
`--maxtime 900 --maxconfl 20000 --verb 0 --verbstat 0 --printsol 1`, plus a
1,000-second process watchdog and host memory limit. Capture stdout, stderr,
exit status, elapsed time, and peak memory independently. A process timeout or
solver `UNKNOWN` is censored; an incomplete model is never a relation. Solver
version and backend flags must be pinned before comparing arms. If the solver
prints a complete SAT assignment, replay it from the exact original inputs:

```sh
"$WORKER" verify-cross-xcnf "$BASE_HASH" "$MANIFEST" "$BASE_OBJECT" \
  "$POOL_HASH" "$MANIFEST" "$POOL_OBJECT" "$CORPUS" 5 1 \
  "$EXPORT_DIR" "$SOLVER_STDOUT" "$VERIFY_DIR"
```

`VERIFY_DIR` must be new. Verification rebuilds and byte-compares the XCNF and
metadata; it then checks every original CNF clause, native XOR row, binary
field S3 equation, exact per-position orbit domain, and original curve-group
equation. A SAT assignment rejected by the group check is recorded as
`GROUP_REJECTED_EXTERNAL_MODEL`, not a partial relation. This bridge checks
only the first identity pattern and the first public trial. Other identity
patterns, model blocking after group rejection, relation yield, large-prime
graph closure, rank, linear algebra, target descent, and matched rho remain
separate gates. `selected_best_total_runtime` stays null until those gates and
all charged stages are measured.
