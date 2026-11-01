# Certicom 131-bit arithmetic validation protocol

## Scope and hypothesis

The candidate is the native implementation in `certicom131.cuh`.  It supports
the exact published ECCp-131 prime curve and ECC2-131 binary curve.  The
falsifiable hypothesis is that the host and CUDA builds reproduce every frozen
field and group fixture and satisfy the exact curve and subgroup identities
listed below.

This protocol validates arithmetic only.  It does not run a challenge search,
measure a campaign rate, or establish an end-to-end cost.

## Frozen inputs

- `params131.h`, SHA-256
  `a8eb458b6971c4b1c586cd733576bb3f26c2aff1fe6cae4d50e524973b1b1a8f`.
- `vectors131.h`, SHA-256
  `e8ee77658edfc9b91da7a06acbc547b99b0fa0fb2c208689637d68f52f0b890c`.
- Historical generator `oracle131.py`, SHA-256
  `2c1822ac76a592ad8c736c5247ae2fb3dd87ff506e00d1f4c556df5a023171b3`.
  It is preserved for provenance and is not an execution path.
- 256 deterministic multiplication/inversion fixtures for each field, plus
  binary squaring fixtures.
- 16 deterministic scalar-multiplication fixtures per curve, `G+Q`, and `2G`.

## Required checks

Both `make host-test` and `make cuda-test` must report the stable success code
zero after checking:

1. all field fixtures;
2. generator and published target curve membership;
3. `G+Q`, `2G`, identity addition, and inverse-point addition;
4. all scalar-multiplication fixtures; and
5. multiplication of both published points by the published subgroup order.

The CUDA run uses one thread deliberately: it is a correctness replay of the
same native operations, not a throughput measurement.

## Stop and failure conditions

Any nonzero stable verifier code, compiler error, CUDA runtime error, timeout,
or fixture-hash mismatch is a failure.  Preserve its output and do not weaken
the vector count or subgroup checks.  Hardware unavailability is an execution
blocker, not an arithmetic result.

## Cost accounting

The host validation is local.  Any remote CUDA validation records the source
revision, compiler, GPU identity, wall time, runner record, and provider cost
window.  No performance ratio is computed from this correctness run.
