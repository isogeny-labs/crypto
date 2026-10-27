# The `scalar_mul_secret` bit-length timing leak, before and after

2026-10-22. Audit finding 7 of
[`research/generic_dlp_memory_parallelism_20261017/AUDIT_FINDINGS.md`](../generic_dlp_memory_parallelism_20261017/AUDIT_FINDINGS.md).

- Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen and committed before any
  recorded run. Amendment 1 was committed before run 2.
- Report: [`REPORT.pdf`](REPORT.pdf) (source `REPORT.typ`), with the figures
  [`fig_timing_run2.svg`](fig_timing_run2.svg) (the shipped fix) and
  [`fig_timing.svg`](fig_timing.svg) (run 1).
- Numbers: every number here comes from
  [`summary-run2.json`](summary-run2.json) and [`summary.json`](summary.json),
  which `examples/scalar_mul_timing_analyze.rs` writes from `raw-run2/` and
  `raw/`.

## The leak

`ecc::ct::scalar_mul_secret` sends every curve other than secp256k1 and
P-256 to `Point::scalar_mul_ct`, the affine `BigUint` Montgomery ladder. That
covers SM2, the GOST R 34.10-2012 curves and the curve-zoo curves. The SM2,
GOST, EC-KCDSA and EC-ElGamal routines also call the ladder directly with
their secret scalars.

The ladder started at `r0 = O`. Through every leading zero bit of the scalar
it computed `O + P` and `2·O`, which are shortcuts in `Point::add` and
`Point::double`. A scalar with `z` leading zeros therefore ran `z` cheap
iterations, so wall time revealed the bit length of the secret. The bit
lengths of signature nonces are exactly what lattice (hidden-number) attacks
use.

## The fix (`src/ecc/point.rs`)

- **New ladder scalar.** The ladder runs on `k + 2^L` over `L + 1` bits,
  where `L` is the requested length (the order's bit length).
- **No identity in the ladder.** The top bit is always set, so the ladder
  starts from `(P, 2P)`, the state after that bit. The registers hold `[j]P`
  and `[j + 1]P` with `1 ≤ j < 2^(L+1)`. For a point of prime order `n`
  with `L = bits(n)`, they meet the identity only for a few scalars per bit
  position out of `n`.
- **Exact result.** It returns `[k + 2^L]P − [2^L]P`, which is exactly
  `[k mod 2^L]P`, the same as before, for every point:
  - including points outside the order-`n` subgroup;
  - including the singular-curve probe points that
    `cryptanalysis::p192_singular_recovery` feeds `scalar_mul_secret`.
  The usual alternative, `k + n` or `k + 2n` as in OpenSSL, is exact only
  on the subgroup, and would have changed those pinned outputs.
- **The subtrahend.** `[2^L]P` depends only on `P` and `L`, which are
  public, never on `k`. So it comes from the variable-time Jacobian
  `Point::scalar_mul`, with one inversion in all.
  - The first version of the fix (run 1) used `L` affine doublings, which
    cost about a third more group operations.
  - Amendment 1 switched to the Jacobian routine at the user's request.
  - `P` must be public, as it is for every caller: generators, public keys,
    ciphertext points.

## Tests

- **`ladder_work_does_not_depend_on_scalar_length`** (`ecc::point`).
  - A test-only counter records ladder iterations that hold the identity.
  - On SM2's generator it is 0 for scalars of 1 to 256 bits, 0, 1 and
    `n − 1`, and every result equals `[k]G`.
  - The same test **fails on the old ladder**: I checked by reverting the
    ladder temporarily.
- **`ladder_is_exact_for_every_point_of_a_small_curve`** (`ecc::point`).
  - Every point of a curve over `F_97` (identity, 2-torsion and other
    small-order points included), every `k < 160`, with `scalar_bits`
    0, 5 and 8.
  - Each result equals `[k mod 2^scalar_bits]P` from double-and-add.
  - The counter is nonzero here, so it is shown to see identity steps.

## Measurement

**Arms.**
- Before: `main` at `412b60289` plus the driver (`f7f455e7c`).
- A′: the same binary, the A/A control.
- After, in run 2: the shipped fix, `360f28a3d`.
- After, in run 1: the affine-doublings version, `1dce1c654`.

**Method, per run.**
- 5 rotated rounds, 3 repetitions per process.
- Every process ran single-threaded under `tools/isolated_bench.py` on one
  reserved CPU. All 30 processes were uncontended.
- Scalars: 8 classes of leading zero bits, `z ∈ {0, 8, …, 224}`, with
  8 scalars each.
- Point: the generator.
- Pinned output: the result digest of every scalar is identical across all
  runs.

### Run 2: the shipped fix

The table gives the median class time over rounds, in ms. The bracket is
the median over rounds of each round's fraction of its own full-length
class, so it need not equal the ratio of the two ms columns.

| leading zero bits | 0 | 32 | 64 | 128 | 192 | 224 |
|:--|--:|--:|--:|--:|--:|--:|
| SM2, before | 98.4 (1.00) | 90.8 (0.92) | 82.0 (0.83) | 65.0 (0.67) | 48.6 (0.49) | 40.0 (0.40) |
| SM2, after | 98.9 (1.00) | 98.5 (1.00) | 99.7 (1.02) | 99.3 (1.00) | 98.4 (1.00) | 97.4 (0.99) |
| GOST-256 test, before | 80.8 (1.00) | 73.8 (0.89) | 67.0 (0.82) | 54.3 (0.67) | 40.0 (0.48) | 33.1 (0.41) |
| GOST-256 test, after | 77.2 (1.00) | 81.6 (1.00) | 81.6 (1.02) | 78.1 (1.01) | 81.6 (1.09) | 81.5 (1.04) |

**Verdicts** (frozen in `PROTOCOL.md`):
- **Leak present before.** The ratio `R` of the 224-leading-zero class to
  the full-length class was 0.33–0.47 in every round, on both curves, against
  the threshold 0.9.
- **Coarse leak closed after.** The largest deviation of any class from
  the full-length class, `F`, was at most 0.138 (SM2) and 0.165 (GOST).
  The A/A noise band `δ` is 0.296. `R` after was 0.97–1.17.
- **Cost on full-length scalars.** SM2 0.96–1.04× (median 1.01); GOST
  0.86–1.09× (median 0.93). Both are within this host's noise of no change.

### Run 1: the first version, `[2^L]P` by affine doublings

| leading zero bits | 0 | 32 | 64 | 128 | 192 | 224 |
|:--|--:|--:|--:|--:|--:|--:|
| SM2, before | 93.5 (1.00) | 84.9 (0.93) | 74.2 (0.83) | 59.8 (0.64) | 46.9 (0.49) | 38.8 (0.42) |
| SM2, after | 125.6 (1.00) | 123.8 (0.99) | 125.5 (1.00) | 125.4 (1.01) | 126.3 (0.99) | 119.9 (0.96) |
| GOST-256 test, before | 70.2 (1.00) | 69.4 (0.97) | 63.6 (0.94) | 50.4 (0.74) | 38.9 (0.52) | 31.9 (0.43) |
| GOST-256 test, after | 102.2 (1.00) | 102.3 (0.99) | 106.9 (0.98) | 101.3 (1.00) | 101.3 (0.94) | 102.4 (1.00) |

- **Verdicts.** The leak was present before (`R` 0.38–0.48) and closed
  after (`F` at most 0.212, against `δ` 0.229).
- **Cost.** SM2 1.29–1.42× (median 1.36); GOST 1.29–1.55× (median 1.52).

**Reading.**
- Before the fix, a 32-bit scalar on these curves took about 40% of the
  time of a full-length one, in both runs.
- After it, the class medians are within the host's A/A noise of each
  other. The noise is wide, at 23–30% on this 4-vCPU guest.
- The deterministic counter test is the stronger evidence that the ladder's
  work no longer depends on the length.
- With the Jacobian subtrahend the fix costs nothing measurable. The affine
  version cost a third or more.

## Scope

- **Measured:** coarse wall time on this host, for two curves and their
  generators.
- **Not addressed.**
  - The `num-bigint` field arithmetic is still not constant-time at the
    limb level (`SECURITY.md`).
  - Cache, power and EM channels are not touched.
- **Small-order points.** Attacker-chosen off-curve points of small order
  can still make the registers meet the identity mid-ladder. Rejecting them
  is the job of point validation.
- **secp256k1 and P-256** take the projective ladders with complete
  formulas, which never had this shortcut. They are unchanged.

**Follow-up, done in a separate PR (#2436):** EC-KCDSA and EC-ElGamal now
call `scalar_mul_secret` instead of the affine ladder, so on P-256 and
secp256k1 they get the limb-level constant-time projective stack. On the
other curves they still reach the ladder fixed here, through the dispatcher.

## `benchmarks/` entry, outside the results table

The figures and report are copied to
[`benchmarks/20261022-scalar-mul-ct-timing/`](../../benchmarks/20261022-scalar-mul-ct-timing/README.md)
(AGENTS.md, "Performance results also land in `benchmarks/`") and listed in
`benchmarks/README.md` under "Security timing (outside the results table)",
not as a row of the admitted table. The validation gate
(`docs/VALIDATION_GATE.md`, owner rule of 2026-10-19) admits a row only with a
replay certificate, a validator report and a rho reference matched to the
protocol, priced in GAE at 0, 0.1 and 1. This study times a scalar
multiplication: it ranks no ECDLP method, so there is no rho reference to
match, and inventing one would misstate the result. It is timing-leakage
evidence, not a solver-performance improvement.

## Files

- `PROTOCOL.md`;
- `raw-run2/` and `raw/`, each holding `timing.jsonl`, `isolation.jsonl`
  and `manifest.json`;
- `summary-run2.json` and `summary.json`;
- `scripts/run_arms.sh` (orchestration only), `scripts/plot.py` (layout
  only);
- `fig_timing_run2.{svg,png}`, `fig_timing.{svg,png}`, `REPORT.typ`,
  `REPORT.pdf`.

The drivers are `examples/scalar_mul_secret_timing.rs` and
`examples/scalar_mul_timing_analyze.rs`.
