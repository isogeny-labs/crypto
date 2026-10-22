# Protocol: the `scalar_mul_secret` bit-length timing leak, before and after

Frozen 2026-10-22, before any recorded run. Study:
`research/scalar_mul_ct_timing_20261022/`. Finding 7 of
`research/generic_dlp_memory_parallelism_20261017/AUDIT_FINDINGS.md`.

One feasibility pass of the driver (one repetition, not recorded) was run to
size the repetitions. Its numbers are not data.

## Question

`ecc::ct::scalar_mul_secret` sends every curve other than secp256k1 and P-256
to `Point::scalar_mul_ct`. That affine Montgomery ladder starts at `r0 = O`,
and `O + P` and `2·O` are shortcuts in `Point::add` and `Point::double`, so
every leading zero bit of the secret scalar is cheaper than a one bit.

1. Does wall time fall with the number of leading zero bits on `main`?
2. After the fix, is time flat in the bit length, within the A/A noise?
3. What does the fix cost on full-length scalars?

**The fix.**
- The ladder runs on `k + 2^L` over `L + 1` bits, where `L` is the
  requested length. Its top bit is always set, so the identity appears only
  in the first step, for every scalar.
- It then subtracts `[2^L]P`, which is `L` doublings of the public point.
- The result is exactly `[k mod 2^L]P`, as before, for every point,
  including points outside the order-`n` subgroup. That matters because
  `p192_singular_recovery` relies on it.

## Arms

- *A*: `main` at `412b60289`, with this study's driver
  `examples/scalar_mul_secret_timing.rs` added. The driver uses only API
  present on `main`.
- *A′*: the same binary as A, the A/A control.
- *B*: the fix branch `claude/scalar-mul-secret-timing`.

## Frozen inputs

- **Curves:** SM2 (`CurveParams::sm2()`) and the GOST R 34.10-2012 256-bit
  test curve (`CurveParams::gost_3410_2012_256_test()`). Both take the
  generic ladder. Point: the generator.
- **Scalars:**
  - 8 classes, with leading zero bits `z ∈ {0, 8, 16, 32, 64, 128, 192, 224}`
    below the order's bit length.
  - 8 scalars per class, each with exactly `order_bits − z` bits.
  - Drawn from xorshift64 seeded `0x5ca1ab1e7111e5ed`, in class order.
- **Repetitions:** 3 per process. Each repetition visits all 64 scalars per
  curve, starting one class later than the previous one.

## Procedure

- 5 rounds. Arms are rotated `A B A′` / `B A′ A` / `A′ A B` / …, one process
  per arm per round.
- Each process runs single-threaded under `tools/isolated_bench.py` on one
  reserved CPU.
- A run that the isolation record marks as contended is left out and
  reported.

## Measures

- Per process, curve and class: the median nanoseconds over its
  8 scalars × 3 repetitions.
- **Leak ratio** `R = median(z = 224) / median(z = 0)`, per process and
  curve.
- **Flatness** `F = max over z of |median(z) / median(0) − 1|`, per process
  and curve.
- **A/A band** `δ = max over rounds, curves and z of |median_{A′}(z) / median_A(z) − 1|`.
- **Cost** `median_B(z = 0) / median_A(z = 0)`, per round and curve.
- **Pinned output:** the result digest of every scalar, identical across all
  arms.

Analysis is native, in `examples/scalar_mul_timing_analyze.rs`, which writes
`summary.json`. `scripts/plot.py` only lays it out.

## Verdicts

- **Leak present on A** if `R_A < 0.9` in every round, on both curves.
- **Coarse leak closed on B** if `F_B ≤ δ` in every round, on both curves.
  Otherwise the residual `F_B` is reported against `δ`.
- **Rejection:** reject the fix if any pinned digest differs between arms,
  or if a unit test of the fix fails.
- **Cost:** reported, not gated. The expectation is about `4/3`: `L`
  doublings added to a ladder of `L` additions and `2L` doublings.

## What this cannot establish

- Only coarse wall time, on this host, for these two curves and the
  generator.
- Limb-level timing of the `num-bigint` field arithmetic is not addressed.
  `SECURITY.md` disclaims it, and it is not measured here.
- Cache, power and EM channels are out of scope.
- Points of small order, such as attacker-chosen off-curve points, still
  take shortcuts inside the ladder whenever a partial multiple reaches the
  identity. Rejecting such points is the job of point validation, not of
  this fix.
