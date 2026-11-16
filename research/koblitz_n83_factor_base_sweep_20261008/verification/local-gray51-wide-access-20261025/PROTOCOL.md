# Frozen primary N83 Gray 5+1 wide-XOR natural access arm

This is one source-pinned, public trial-zero, all-affine natural-query access
arm on the primary `a=0` curve. It uses the exact K600 sequential base and
disjoint K600 Gray residual pool from the
[11-object S3 bundle](../linux-xor-complete-inputs-20261025/RESULT.md).
The formula is `gray_5_1.xcnf`; no relation is planted into its target.
The previous default-matrix Gray 5+1 call reached 2,000,002 conflicts and
returned `INDETERMINATE` after 119.21 seconds of L0 process wall. The
exploratory 20,000-conflict smoke showed that a 25,000-column matrix limit
admits 332×20,916 XOR matrices excluded by the default 1,000-column limit.
The earlier wide-matrix hash 4+2 arm reached its process watchdog without a
terminal status. This arm tests whether the changed matrix admission obtains
a complete model on the exact Gray 5+1 first pattern within a larger bound.

[`protocol.json`](protocol.json) freezes the formula, source and S3 receipts,
public manifest/corpus, worker and CryptoMiniSat 5.14.7 executable hashes,
one thread, zero solver random seed, branch strategy, forced Gauss, 5,000,000
conflicts, 1,500 solver seconds, a 1,650-second process watchdog and a
300-second verification watchdog. The runner checks all pinned bytes before
creating an output directory. It runs one process, saves raw stdout/stderr,
the exact terminal status/conflict count when present, process wall, Darwin
peak-child-RSS upper bound and host load snapshots. An external watchdog
kills and reaps the solver process group.

A complete SAT assignment is useful only if `verify-cross-xcnf` rebuilds the
exact XCNF from the original archives and metadata, checks every original
CNF/XOR row and field equation, lifts the signed-orbit domains, and verifies
the curve-group sum. A group-rejected model must be blocked and searched
again. `INDETERMINATE`, watchdog, OOM, malformed output and verification
failure are censored for factor-base selection. No solver-stage wall from
this L0 shared host can establish a total-runtime improvement or a Gray/hash
ranking. The one-target cold objective still charges construction, S3
transfer, all relation failures, graph closure, rank, linear algebra,
unseen-target descent, verification, artifact I/O and matched rho.
`selected_best_total_runtime` stays `null`.

After this protocol and runner are committed and pushed, use the fetched
S3 directory from the complete-input handoff:

```sh
PANEL=research/koblitz_n83_factor_base_sweep_20261008/verification/local-gray51-wide-access-20261025
INPUTS=/private/tmp/n83-complete-fetch-cli-20261025-01
WORKER=/Volumes/SSD990/crypto/.worktrees/codex-n83-aws-resume-20261024/target/release/examples/koblitz_n83_mixed_orbit_worker
SOLVER=/opt/homebrew/bin/cryptominisat5
OUT=/Volumes/SSD990/llm/tmp/n83-gray51-wide-access-20261025
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=/private/tmp/n83-handoff-pydeps-20261025 \
  python3 "$PANEL/run.py" --inputs-dir "$INPUTS" --worker "$WORKER" \
  --solver "$SOLVER" --out "$OUT" --preflight-only
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=/private/tmp/n83-handoff-pydeps-20261025 \
  python3 "$PANEL/run.py" --inputs-dir "$INPUTS" --worker "$WORKER" \
  --solver "$SOLVER" --out "$OUT"
```

Run the independent auditor on the raw output before interpreting any
result. The output directory must be new; a stopped run is retained and
must not be silently overwritten.
