# Algorithm performance, round 2 (2026-10-22)

A second pass over the algorithms behind ecbench and the Koblitz index
calculus. It looks for speedups that leave every counted quantity unchanged.
Report: [`REPORT.pdf`](REPORT.pdf) (source `report.html`, figure
`solve-time.svg` / `solve-time.png`). Benchmarks row:
[`benchmarks/20261022-algorithm-perf-round2`](../../benchmarks/20261022-algorithm-perf-round2/).

## Changes

| Change | Where | Effect |
|---|---|---|
| Reuse per-trial scan buffers (hashes, admitted offsets); add the walk stride mod r with one compare instead of a 128-bit `%` | `koblitz_index_calculus.rs` (`ScanScratch`, `add_mod_u64`) | Koblitz `collect` 0.76–0.90× |
| Build the collector's and solver's shared index map and field structure once, on first use (`OnceLock`) | `koblitz_index_calculus.rs` | `collect_setup` 0.33–0.37× |
| Use Fx-hashed sets for the orbit factor base; step the orbit by k-fold squaring on field words | `koblitz_index_calculus.rs` | `select_projection` 0.67–0.88×, `descent_setup` 0.54× |
| Pre-gather the target's doubled coordinates in aimed scans (`Scan::Gathered`) | `koblitz_index_calculus.rs` | part of `collect` |
| Canonicalise the SIMD tail as one padded block | `koblitz_fast.rs` | part of `collect` |
| Flip the coefficient under the negation map by subtraction (`negmod`), not two word divisions | `ic_boundary.rs` (`rho_canon`) | `rho.negation` 0.90–0.95× |
| Run the extended Euclid inverse on 32-bit words when the modulus fits | `ic_boundary.rs` (`invmod_u32`) | every prime-curve add, small |
| Compute each point's key and hash once per walk step, not four or five times | `ic_boundary.rs` (`TunedWalker::walk`) | `rho.plain` 0.94× |

## Results

**Koblitz `ic price --single-target`** (rule-v3 T01; one thread, `taskset`
pinned; 5 runs × 3 repetitions per binary; medians):

| curve | set-up old → new | ratio | collect | collect_setup |
|:--|--:|--:|--:|--:|
| k0n41 | 57.2 → 45.9 ms | **0.803** | 36.3 → 27.5 ms | 2.11 → 0.69 ms |
| k0n53 | 436.7 → 403.5 ms | **0.924** | 234.8 → 211.5 ms | 11.5 → 4.2 ms |
| k0n61 | 1,697 → 1,520 ms | **0.895** | 1,387 → 1,198 ms | 17.9 → 6.0 ms |

IC online time is unchanged (none of this touches the descent). `build` and
`verify` moved ±10% in both directions on unchanged code: that is noise.

**ecbench sessions** (prime-order curves; old and new binary alternated;
paired geometric mean of `solve_wall_ns` over the same (target, round)):

| spec | arm | paired new/old |
|:--|:--|--:|
| `rho-32` (4 + 4 sessions) | `rho.negation` / A/A copy | **0.931 / 0.896** |
| | `rho.plain` | **0.937** |
| | `rho.frozen_reference` | 1.000 |
| `generic-32` (3 + 3) | `rho.negation` | 0.949 |
| | BSGS ×3, kangaroo | 0.98–1.04 (flat) |
| `prime-ic-28` (3 + 3) | `ic.pipeline` mitm / mitm-prime-batch | 1.01 (flat) |
| | `rho.negation` | 0.903 |

## Outputs unchanged

- 1,776 of 1,776 ecbench records are identical to the old binary's outside timing.
- Every session replays exactly under the *other* binary: `ecbench verify
  --replay-all`, 1,428 replays, 0 mismatches
  ([`replay-certificate.json`](replay-certificate.json), receipts in `verify/`).
- The §5a band of the prime IC sessions is identical between the binaries
  (`band/`).
- None of the 30 `ic price` outputs differs from the previous binary in any
  count, certificate input, IC trial count, recovered logarithm or rho
  reference figure.
- [`validator.py`](validator.py) recomputes every headline ratio from the raw
  files with no shared code ([`validator-report.json`](validator-report.json): pass).

## Not changed, and why

- **Batching inversions across rho walks.** At 32 bits, one field inversion is
  about 110 ns of a ~150 ns rho step. Running 8–16 walks side by side with one
  shared inversion per batch (Montgomery's trick), then committing them in walk
  order, would keep every walk, DP, recovered value and group-operation count.
  It would still change the counted `field_ops`: one inversion per batch
  instead of one per step. `CountedFp` states that the tally is what the code
  performed, and the bounds use `field_invs`. So this is an accounting decision
  for the owner, not an identical-output change. The same holds for batching the
  `mitm` pair-table build.
- **Barrett multiplication** (5.1 vs 4.4 ns for the hardware divide) and a
  **quotient-1 shortcut in Euclid** (slower, from branch mispredictions) both
  measured slower on this host (`microbench/`).
- **Prime IC relation loop**: about 340 ns per trial. That is one walk addition
  plus a guard-set insert and a pair-table probe, and the CPU already overlaps
  them. Linear algebra is about 1 ms of a 60 ms solve, so `IncrementalGauss`
  was left alone.

## Reproduce

```sh
# ecbench, each binary, alternated
ecbench run --spec specs/rho-32.json --out OUT --allow-busy
ecbench verify --dir OUT --replay-all --exit-code
ecbench band --dir OUT...
# Koblitz IC, each binary
koblitz-ic/run_price.sh BIN OUTDIR TAG
python3 summarize.py && python3 validator.py
```

Host: Intel Xeon @ 2.10 GHz (family 6 model 207, AVX-512), 4 vCPU cloud
container; ecbench at `--cpus auto --allow-busy` (below L2). Binaries:
- ecbench old: `03d7abd4e` code (sha256 `65356a79…`).
- ecbench new: `3213bca03` (sha256 `bb282eec…`).
- `ic` old: `1fc837dbf` (sha256 `bdaa213d…`).
- `ic` new: `ac687c2a2` (sha256 `0957239b…`); the later commits change
  nothing on the Koblitz path except formatting.

This is an engineering change: it moves wall time and no cost unit.
