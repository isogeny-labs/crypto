# The `scalar_mul_secret` bit-length timing leak, before and after

2026-10-22. Audit finding 7 of
[`research/generic_dlp_memory_parallelism_20261017/AUDIT_FINDINGS.md`](../generic_dlp_memory_parallelism_20261017/AUDIT_FINDINGS.md).

- Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen and committed before any
  recorded run.
- Report: [`REPORT.pdf`](REPORT.pdf) (source `REPORT.typ`), with the figure
  [`fig_timing.svg`](fig_timing.svg).
- Numbers: every number here comes from [`summary.json`](summary.json),
  which `examples/scalar_mul_timing_analyze.rs` writes from `raw/`.

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
- **Exact result.** It returns `[k + 2^L]P − [2^L]P`, the subtrahend being
  `L` doublings of the public point. That is exactly `[k mod 2^L]P`, the
  same as before, for every point:
  - including points outside the order-`n` subgroup;
  - including the singular-curve probe points that
    `cryptanalysis::p192_singular_recovery` feeds `scalar_mul_secret`.
  The usual alternative, `k + n` or `k + 2n` as in OpenSSL, is exact only
  on the subgroup, and would have changed those pinned outputs.
- **Cost.** `L` doublings added to `L` additions and `2L` doublings,
  about a third more group operations.

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
- After: the fix, `1dce1c654`.

**Method.**
- 5 rotated rounds, 3 repetitions per process.
- Every process ran single-threaded under `tools/isolated_bench.py` on one
  reserved CPU. All 15 processes were uncontended.
- Scalars: 8 classes of leading zero bits, `z ∈ {0, 8, …, 224}`, with
  8 scalars each.
- Point: the generator.
- Pinned output: the result digest of every scalar is identical across all
  15 runs.

The table gives the median class time over rounds, in ms. The bracket is
the median over rounds of each round's fraction of its own full-length
class, so it need not equal the ratio of the two ms columns.

| leading zero bits | 0 | 32 | 64 | 128 | 192 | 224 |
|:--|--:|--:|--:|--:|--:|--:|
| SM2, before | 93.5 (1.00) | 84.9 (0.93) | 74.2 (0.83) | 59.8 (0.64) | 46.9 (0.49) | 38.8 (0.42) |
| SM2, after | 125.6 (1.00) | 123.8 (0.99) | 125.5 (1.00) | 125.4 (1.01) | 126.3 (0.99) | 119.9 (0.96) |
| GOST-256 test, before | 70.2 (1.00) | 69.4 (0.97) | 63.6 (0.94) | 50.4 (0.74) | 38.9 (0.52) | 31.9 (0.43) |
| GOST-256 test, after | 102.2 (1.00) | 102.3 (0.99) | 106.9 (0.98) | 101.3 (1.00) | 101.3 (0.94) | 102.4 (1.00) |

**Verdicts** (frozen in `PROTOCOL.md`):
- **Leak present before.** The ratio `R` of the 224-leading-zero class to
  the full-length class was 0.38–0.48 in every round, on both curves, against
  the threshold 0.9.
- **Coarse leak closed after.** The largest deviation of any class from
  the full-length class, `F`, was at most 0.212 (SM2) and 0.149 (GOST).
  The A/A noise band `δ` is 0.229. `R` after was 0.86–1.08.
- **Cost on full-length scalars.** SM2 1.29–1.42× (median 1.36); GOST
  1.29–1.55× (median 1.52).

**Reading.**
- Before the fix, a 32-bit scalar on these curves took about 40% of the
  time of a full-length one.
- After it, the class medians are within the host's A/A noise of each
  other. The noise is wide, at 23% on this 4-vCPU guest.
- The deterministic counter test is the stronger evidence that the ladder's
  work no longer depends on the length.

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

**Possible follow-ups, not done here:**
- `[2^L]P` could come from the Jacobian `Point::scalar_mul`, which is
  variable-time but has public inputs, instead of `L` affine doublings.
  That would remove most of the added cost.
- EC-KCDSA and EC-ElGamal on P-256 and secp256k1 could call
  `scalar_mul_secret` instead of the affine ladder, and so get the
  limb-level constant-time projective stack.

## Why there is no `benchmarks/` row

The validation gate (`docs/VALIDATION_GATE.md`, owner rule of 2026-10-19)
admits a `benchmarks/README.md` row only with a replay certificate, a
validator report and a rho reference matched to the protocol, priced in GAE
at 0, 0.1 and 1. This study times a scalar multiplication. It ranks no ECDLP
method, so there is no rho reference to match, and inventing one would
misstate the result. The result therefore stays in this research directory,
which the gate permits.

## Files

- `PROTOCOL.md`;
- `raw/timing.jsonl`, `raw/isolation.jsonl`, `raw/manifest.json`;
- `summary.json`;
- `scripts/run_arms.sh` (orchestration only), `scripts/plot.py` (layout
  only);
- `fig_timing.{svg,png}`, `REPORT.typ`, `REPORT.pdf`.

The drivers are `examples/scalar_mul_secret_timing.rs` and
`examples/scalar_mul_timing_analyze.rs`.
