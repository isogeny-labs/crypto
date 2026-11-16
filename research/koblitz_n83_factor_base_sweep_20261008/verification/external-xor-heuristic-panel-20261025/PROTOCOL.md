# Frozen XOR-SAT heuristic factorial, primary N83 first pattern

The [preceding four-arm panel](../external-xor-20261025/RESULT.md) produced
replayable XCNF for both retained pools and both 5+1/4+2 shapes. All four
CryptoMiniSat calls ended at two million conflicts without a model. This
follow-up changes solver search strategy on the exact two 5+1 formulas, rather
than drawing a relation-yield conclusion from those caps. The 4+2 baseline
remains in the preceding panel; this panel does not test every IC combination.

[`protocol.json`](protocol.json) freezes the input SHA-256 digests, one local
worker binary, CryptoMiniSat 5.14.7 binary, fixed random seed, limits, arm
order and stop rule before timing. The factorial axes are `--branchstr`
(`vmtf+vsids` or `vsids`) and `--autodisablegauss` (1 or 0). Both options
were accepted on a two-variable XOR smoke input before this protocol was
frozen. The baseline combination appears at the start and end for each pool
as an A/A drift control. Hash and Gray alternate within each setting to
reduce order confounding. Every solver process has one thread, one million
conflicts, a 120-second internal cap and a 140-second external watchdog.

The endpoint hierarchy is fixed:

1. A complete `SATISFIABLE` assignment counts only after the existing Rust
   verifier rebuilds the exact XCNF, checks all original CNF and XOR rows,
   re-evaluates field equations, and lifts through exact factor-base domains
   to the original curve-group equation. A rejected group model requires
   blocking and a further search; it is not relation yield.
2. `UNSATISFIABLE` refutes only this first identity pattern on this one public
   trial, if the solver and output are valid. It does not refute a factor base.
3. `INDETERMINATE`, watchdog, OOM and malformed output are censored. Retain
   their exact conflict counts, wall, peak memory, stdout/stderr and exit.

For a matched but censored pair, wall per fixed conflict cap and conflicts
per second are solver-stage diagnostics. A/A spread bounds any statement
about timing; a faster censored run is not a factor-base winner or a measured
relation-yield improvement. S3 GET, worker build, rank, linear algebra,
unseen-target descent and rho are outside these local intervals and remain
in the full objective. `selected_best_total_runtime` stays null.

The runner requires the two existing exported XCNF directories and the three
compressed S3 objects from the [handoff protocol](../external-xor-20261025/PROTOCOL.md).
It verifies the formula and binary hashes before starting, executes arms
sequentially, and writes a process receipt after every arm. A resumed run
must replay completed receipts and skip them byte-identically; it must not
silently rerun or overwrite an arm.

From this worktree, with the prior exports and archive downloads at the stated
paths, run preflight and then the panel:

```sh
PANEL=research/koblitz_n83_factor_base_sweep_20261008/verification/external-xor-heuristic-panel-20261025
FORMULAS=/private/tmp/n83-xcnf-20261025
ARCHIVES=/Volumes/SSD990/llm/tmp/n83-split-replay-40157022080
WORKER=/Volumes/SSD990/crypto/.worktrees/codex-n83-aws-resume-20261024/target/release/examples/koblitz_n83_mixed_orbit_worker
SOLVER=/opt/homebrew/bin/cryptominisat5
OUT=/private/tmp/n83-heuristic-panel-20261025
PYTHONDONTWRITEBYTECODE=1 python3 "$PANEL/run_panel.py" \
  --formulas-dir "$FORMULAS" --archives-dir "$ARCHIVES" \
  --worker "$WORKER" --solver "$SOLVER" --out "$OUT" --preflight-only
PYTHONDONTWRITEBYTECODE=1 python3 "$PANEL/run_panel.py" \
  --formulas-dir "$FORMULAS" --archives-dir "$ARCHIVES" \
  --worker "$WORKER" --solver "$SOLVER" --out "$OUT"
```

If interrupted between completed arms, rerun the second command with
`--resume`. An interrupted arm without a complete receipt is retained for
manual audit, and the runner refuses to overwrite it. Linux/AWS runs require
a separately frozen protocol with that host's binary hashes and resource
envelope; their wall times cannot be combined with this local panel.
