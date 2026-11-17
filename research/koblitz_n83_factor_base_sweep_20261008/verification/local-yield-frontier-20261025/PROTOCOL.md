# Frozen primary-N83 wide-XOR natural access gate

The single arm is the original public trial-zero, all-affine identity pattern
on the primary `a=0` curve, K600 sequential hash small base and disjoint
K600 hash residual pool, with a 4+2 split. The exact XCNF was exported and
independently replayed in [the earlier four-arm panel](../external-xor-20261025/RESULT.md).
The [S3 handoff](../linux-xor-panel-20261025/RESULT.md) fetched the same
XCNF bytes back. The prior 2,000,000-conflict default-CryptoMiniSat arm
returned `INDETERMINATE`. This protocol tests one prospectively chosen
solver change: admit the five 332×20,916 XOR matrices and retain Gaussian
elimination. It does not change the factor base or claim an optimized setting.

An exploratory 20,000-conflict smoke on this exact formula, binary and
flags, except for the lower cap, returned `INDETERMINATE` and printed
`Using 10 matrices recovered from 3486 xors`; its raw streams and receipt
are retained under `smoke/`. [`protocol.json`](protocol.json) freezes the
curve, first-pattern source, public corpus, S3 archive/formula/binary hashes,
single-process cap of five million conflicts, 1,200-second solver timer,
1,350-second external watchdog and 300-second model-verification watchdog
before the full call. The protocol is committed and pushed before timing.
The result is an L0 local *access* observation. Host contention makes its
wall time unsuitable for a factor-base ranking.

The runner refuses changed bytes, identities, versions, host type or flags.
It creates a new output directory, retains raw solver streams and a
hash-bound process receipt, and independently invokes `verify-cross-xcnf`
on any complete SAT assignment. The verifier rebuilds and compares the
original XCNF, then checks every CNF/XOR row, field equation, exact signed
orbit domain and curve-group sum. A group-rejected model requires blocking
and a further search. Caps and watchdogs are censored. The independent
auditor must replay the raw output and any verification receipt before
relation interpretation.

From this worktree, after fetching the three compressed S3 archives named
`s3-<BLAKE3>.jsonl.gz` and restoring the original export directory:

```sh
PANEL=research/koblitz_n83_factor_base_sweep_20261008/verification/local-yield-frontier-20261025
FORMULAS=/private/tmp/n83-xcnf-20261025
ARCHIVES=/Volumes/SSD990/llm/tmp/n83-split-replay-40157022080
WORKER=/Volumes/SSD990/crypto/.worktrees/codex-n83-aws-resume-20261024/target/release/examples/koblitz_n83_mixed_orbit_worker
SOLVER=/opt/homebrew/bin/cryptominisat5
PYTHONDONTWRITEBYTECODE=1 python3 "$PANEL/run_access.py" \
  --formulas-dir "$FORMULAS" --archives-dir "$ARCHIVES" \
  --worker "$WORKER" --solver "$SOLVER" --out /private/tmp/n83-wide4-access-20261025 \
  --preflight-only
PYTHONDONTWRITEBYTECODE=1 python3 "$PANEL/run_access.py" \
  --formulas-dir "$FORMULAS" --archives-dir "$ARCHIVES" \
  --worker "$WORKER" --solver "$SOLVER" --out /private/tmp/n83-wide4-access-20261025
```

The S3 GET, source build, formula export, relation accumulation, graph
closure, rank, linear algebra, unseen-target descent, artifact I/O and
matched rho are outside this solver interval and remain in the fully
charged objective. `selected_best_total_runtime` stays null.
