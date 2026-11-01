# Native measurements: `btn-prime-scaling`

Rust tool for the prime-field BTN study, depending on the unmodified
`crates/hashed-ecdl-tensor` crate (#2878). It replaces the Python path in the
parent directory (AGENTS.md: no Python for cryptographic research). It does not
modify the crate, whose sources are locked byte-identical across repositories.

```sh
export CARGO_TARGET_DIR=/tmp/btn-target          # keep build artifacts off the repo volume
cargo build --release --manifest-path native/Cargo.toml

B=$CARGO_TARGET_DIR/release/btn-prime-scaling
$B sizes 5 11 17 37 67 131 257 521 1031 2053 4099 8209 16411 32771 65537 > native_runs/sizes.jsonl
$B counts 5 7 11 13                                                       > native_runs/counts.jsonl
$B solve 5 1 2; $B solve 5 2 1; $B solve 7 1 3                            > native_runs/solves.jsonl
$B size-json ../../crates/hashed-ecdl-tensor/examples/eccp131.json        > native_runs/eccp131_size.json
$B report native_runs                                                      > native_runs/report.json
```

| Mode | What it records |
|---|---|
| `sizes P...` | decomposition-network edges, vertices, max degree, interface bits per prime |
| `counts P...` | `Counter::count` vs a brute-force decomposition count; backend; charged steps |
| `solve P SEED XI` | `reduction::solve` with a known scalar, then `reduction::verify` |
| `size-json FILE` | network size for a JSON problem (e.g. the bundled ECCp-131 input) |
| `multtime W...` | time one Fourier integer-multiply network construction per bit width |
| `primetime W...` | time `is_prime` on a w-bit prime (shows the AKS cliff above 64 bits) |
| `report DIR` | derived values from the recorded `*.jsonl`: all read, nothing re-measured |

Each line of output is one JSON object. Curves are the first prime-order
`y^2 = x^3 + ax + b` per prime; brute force enumerates affine points directly.

## Findings (see `../native_runs/report.pdf`)

* **Counts correct.** 16 of 16 counts match brute force, at p = 5, 7, 11, 13,
  full and hash-filtered bases. Backend `Functional` in every count; visited
  assignments equal 2^(4d) exactly. Recursive backend used: never.
* **Solves correct.** p = 5 (two seeds) and p = 7 recover the scalar and pass
  `reduction::verify`.
* **Setup bottleneck (primality).** `is_prime` (called by `Curve::new` and
  `FactorBase::new`) uses a fast u64 path at <= 64 bits and AKS above. At <= 64
  bits certification is <= 17 us; at 65 bits (AKS) it did not finish in 180 s.
  So a curve whose prime exceeds 64 bits (ECCp-131's is 131-bit) cannot even be
  set up, before any contraction. Network construction, by contrast, is fast at
  every width (131-bit multiply builds in 2.5 ms). `multtime`/`primetime` modes.
* **Recursion crossover (headline).** `Counter::count` selects the recursive
  backend only when `lambda / 131072 >= 1` with `lambda = ceil(log2(2·edges))`
  (transcribed from `recursive.rs` `Setup::parameters`), i.e. at least
  2^131070 edges. The largest network built here is 2^17.8 edges. So the
  recursion never engages at any reachable size, matching the manuscript's
  2^(2^17) threshold, and the method shows no sub-exponential behaviour.

## Units and gates

Step counts are the crate's `Meter` charges: deterministic diagnostics, not
calibrated time. **No wall-clock time is reported**, so no timing-isolation
path (`tools/isolated_bench.py`) applies and no `benchmarks/` row is added.
This is a distance-to-target measurement, not a speed result; the validation
gate admits no leaderboard or benchmark row without an independent replay
certificate, validator report and matched rho reference, none of which a
"no speed-up at any reachable size" finding needs.
