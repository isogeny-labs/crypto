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
$B counts 5 7 11 13                                                       > native_runs/counts.jsonl  # earlier crate; current crate -> counts_pruned.jsonl
$B counts2 5 7 11 13                                                      > native_runs/counts2.jsonl
$B solve 5 1 2; $B solve 5 2 1; $B solve 7 1 3                            > native_runs/solves.jsonl
$B size-json ../../crates/hashed-ecdl-tensor/examples/eccp131-certified-hash.json > native_runs/eccp131_size.json
$B report native_runs                                                      > native_runs/report.json
```

| Mode | What it records |
|---|---|
| `sizes P...` | decomposition-network edges, vertices, max degree, interface bits per prime |
| `counts P...` | `Counter::count` vs a brute-force decomposition count; backend; charged steps |
| `counts2 P...` | original vs derived-last-point variants (A1-only interface, hash-first) at four thresholds: counts, steps, checked witnesses, pinned overhead |
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
* **Setup (primality), resolved with certificates.** Uncertified, `is_prime`
  uses a fast u64 path at <= 64 bits (<= 17 us) and AKS above, which did not
  finish a 65-bit prime in 180 s. With the Pocklington certificates now on
  `main` (`Curve::with_cofactor_and_certificates`, separate change), the bundled
  `eccp131-certified-hash.json` validates in ~13 ms and `size-json` builds the
  real ECCp-131 decomposition network in ~2.8 s: 1,684,796 edges (2^20.7),
  249,620 vertices, max degree 1,032, 526 interface bits (4·131 free + 2 flags).
* **Fallback speed-ups (charged steps).** The upstream functional pruning
  (landed separately; `counts_pruned.jsonl` vs `counts.jsonl`) cuts steps
  1.5–6.4× with identical counts on 16 instances. On top of it, `counts2`
  builds the *derived last point* network with the public `Encoder`: A2 =
  target − A1 is computed instead of searched (same language, free input bits
  4d → 2d), only A1 is marked as interface (so the witness search may stop at
  its first hit, with the same canonical witness), and A1's hash is checked
  before A2 is built. On 32 instances (p = 5, 7, 11, 13; τ = s, s/2, s/4, s/8)
  every variant's count equals the original and brute force, every witness is
  checked, and charged steps fall 1.5–65.3× versus the pruned original, never
  worse. Without hash-first the derived network was slightly worse on 2
  instances (p = 7, τ = 2), where a fixed per-call overhead (~5 charged steps
  per edge) dominates. At ECCp-131 the fallback's worst case falls from 2^524 to
  2^262 assignments per count, still far beyond rho's 2^65.3. [pcc:unverified]
* **Recursion crossover (headline).** `Counter::count` selects the recursive
  backend only when `lambda / 131072 >= 1` with `lambda = ceil(log2(2·edges))`
  (transcribed from `recursive.rs` `Setup::parameters`), i.e. at least
  2^131070 edges. The largest network built here is the real ECCp-131 one, 2^20.7 edges. So the
  recursion never engages at any reachable size, matching the manuscript's
  2^(2^17) threshold, and the method shows no sub-exponential behaviour.
  [pcc:unverified]

## Units and gates

Step counts are the crate's `Meter` charges: deterministic diagnostics, not
calibrated time. The only wall-clock figures (`multtime`, `primetime`) are
unisolated diagnostics of where setup stalls, not performance results, so no
`benchmarks/` row is added. This is a distance-to-target measurement, not a
speed result; the validation gate admits no leaderboard or benchmark row
without an independent replay certificate, validator report and matched rho
reference, none of which a "no speed-up at any reachable size" finding needs.
[pcc:unverified]
