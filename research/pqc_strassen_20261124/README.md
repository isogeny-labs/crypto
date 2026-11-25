# PQC dense matrix products: Strassen path timing (2026-11-24)

Timing for PR #5172, which routes the private `mat_mul` of `src/pqc/{mayo,uov,qr_uov,mceliece}.rs`
through the shared `src/pqc/strassen.rs`. Record:
[`benchmarks/records/20261125T0508Z-pqc-strassen.json`](../../benchmarks/records/20261125T0508Z-pqc-strassen.json)
(problem `kernel:pqc_dense_matmul`). Visual copy: [`benchmarks/20261124-pqc-strassen/`](../../benchmarks/20261124-pqc-strassen/).

**Status: measured, unvalidated.** No replay certificate or validator report. The host was not
isolated (shared macOS machine, load average 20 to 43 from other sessions during the run), so treat the
values as indicative. That holds especially for the crossover sweep, where single cells jump by 3x between
neighbouring sizes. These are timings of a toy-parameter library, not a cryptanalytic result.

## Files

| file | what |
|:--|:--|
| `harness/` | release harness; includes `src/pqc/strassen.rs` by path, unchanged, with copies of the four field ops (checked identical to the crate's) and the old per-scheme loops |
| `regression.json`, `regression.log` | raw: old loop and new `strassen::mul` at each scheme's own cutoff, 6 shapes x 4 fields, 5 batches of 3000 calls each |
| `crossover.json`, `crossover.log` | raw: schoolbook and Strassen at cutoffs 16/32/64, square n = 36 to 512, 5 batches per cell |
| `analyze.py` | derives `metrics.json`, the graphs and `REPORT.pdf` from the raw files |
| `regression.svg/png`, `crossover.svg/png`, `REPORT.pdf` | graphs (PNG at 150 dpi from the same source) and the report |

## Reproduce

```sh
CARGO_TARGET_DIR=/tmp/pqc-harness cargo run --release --manifest-path research/pqc_strassen_20261124/harness/Cargo.toml -- regression research/pqc_strassen_20261124/regression.json
CARGO_TARGET_DIR=/tmp/pqc-harness cargo run --release --manifest-path research/pqc_strassen_20261124/harness/Cargo.toml -- crossover research/pqc_strassen_20261124/crossover.json
python3 research/pqc_strassen_20261124/analyze.py research/pqc_strassen_20261124 --commit <sha> --host "<host>"
```

Run the harness under `tools/benchrec.py run <record> -- ...` (docs/benchmark-records.md).

## Results (medians of 5 batches; range = worst/best batch pairing)

Old loop time / new time at the schemes' sizes (higher is better for the new code):

| field (scheme) | 36x36x36 | 36x24x12 |
|:--|--:|--:|
| GF(2) (McEliece) | 4.51 (3.14-6.36) | 2.34 (1.47-3.92) |
| GF(16) (MAYO) | 1.67 (1.24-2.17) | 1.22 (1.03-1.33) |
| GF(256) (UOV) | 1.04 (0.65-1.41) | 0.97 (0.49-1.60) |
| F_31^3 (QR-UOV) | 1.48 (1.24-2.23) | 1.77 (1.23-2.21) |

Schoolbook time / Strassen time at cutoff 32, square n (above 1 means Strassen takes less time):

| field (scheme) | n = 36 | n = 512 |
|:--|--:|--:|
| GF(2) (McEliece) | 0.54 (0.39-0.63) | 0.31 (0.18-0.72) |
| GF(16) (MAYO) | 1.04 (0.91-1.36) | 3.57 (2.92-3.94) |
| GF(256) (UOV) | 1.19 (1.06-1.27) | 3.86 (2.37-13.43) |
| F_31^3 (QR-UOV) | 1.68 (0.77-2.26) | 1.50 (1.26-2.20) |

Reading:
- At the schemes' own sizes, the change comes from the shared row-order loop. For McEliece, it comes from
  leaving its i-j-k loop. It does not come from Strassen recursion, which at cutoff 32 only engages on UOV's
  36x36 products.
- UOV at 36x36x36 is at parity within the batch range.
- Over GF(2) bytes Strassen takes longer at every size measured, which is why McEliece sets
  `CUTOFF = usize::MAX`.
- For the byte fields Strassen pays off only well above toy sizes.
- These values agree in direction with the standalone run quoted in the PR's first version. Single cells
  differ, consistent with the host load.
