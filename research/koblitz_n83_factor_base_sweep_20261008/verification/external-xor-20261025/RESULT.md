# Bounded external-XOR first-pattern panel

The four trial-zero primary-N83 XCNF instances were exported from the retained
S3 base and cross-stream pools and run sequentially with CryptoMiniSat 5.14.7
on one Darwin arm64 host. Each solver invocation used one thread,
`--maxtime 300 --maxconfl 2000000 --verb 1 --verbstat 1 --printsol 1`, and a
330-second process watchdog. This is L0 local evidence with one observation
per arm. The complete [receipt](receipt.json) gives source-file and binary
hashes, inputs, XCNF and raw-output SHA-256 digests, execution order and
resource limits. Raw stdout/stderr are retained as deterministic gzip files.

| Pool | Split | Variables | Original CNF clauses | XOR rows | Export wall (s) | Solver process wall (s) | Conflicts | Result |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| Hash | 5+1 | 120,696 | 634,131 | 6,059 | 3.436 | 162.322 | 2,000,001 | `INDETERMINATE` |
| Gray | 5+1 | 119,690 | 590,873 | 6,059 | 3.803 | 119.208 | 2,000,002 | `INDETERMINATE` |
| Hash | 4+2 | 120,696 | 634,131 | 6,059 | 2.402 | 133.704 | 2,000,001 | `INDETERMINATE` |
| Gray | 4+2 | 118,684 | 547,615 | 6,059 | 2.333 | 192.484 | 2,000,001 | `INDETERMINATE` |

The sum of these four export and solver process intervals is 619.692 seconds.
The archives had already been fetched and the worker had already been built;
those costs are excluded. Neither relation yield nor a complete index-calculus
runtime was measured. The solver returned exit code 15 and a single
`s INDETERMINATE` line in each arm, with no model. The first 20,000-conflict
hash 5+1 startup check is separately retained in `calibration-20k-hash-5-1/`;
it returned the same status after 1.398 seconds.

Run the independent replay with a worker built from this branch and local
copies of the three compressed S3 archives named `s3-<BLAKE3>.jsonl.gz`:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 \
  research/koblitz_n83_factor_base_sweep_20261008/verification/external-xor-20261025/audit.py \
  --worker target/release/examples/koblitz_n83_mixed_orbit_worker \
  --archive-dir /path/to/downloaded-archives
```

The replay in [`replay.json`](replay.json) rebuilt and matched all four
export metadata objects and XCNF SHA-256 digests, checked compressed raw
output hashes, exact capped conflict counters, solver status, exit code and
process bounds. That result validates a bounded solver-stage observation. The
verifier also rejected the actual `INDETERMINATE` stdout without creating a
result artifact, and rejected a one-byte-mutated XCNF before interpreting the
solver output. The independent auditor rejected altered solver status bytes
on the stdout digest check. The four wall times are not a factor-base ranking:
all outcomes are censored, and
the two pools generate different formulas. The earlier native hosted panel
used a different host, source binary and isolation level, so its wall times
cannot be divided into these to claim an improvement.

The next gate is a complete externally produced SAT assignment independently
verified against the original CNF, XOR, field, orbit-domain and group
equations. A group-rejected model requires blocking and continuation. Useful
partial yield, graph closure, full-width rank and log solve, unseen-target
descent, and matched one-target rho remain open. The [problem record](../../../../problems/n83-mixed-orbit-factor-base-selection.md)
tracks the selection obstruction; `selected_best_total_runtime` remains null.
