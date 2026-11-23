# Native target calibration continuation

This implements the next target-side steps of the fixed quotient-map lead.
The source case, training frame and 32 challenges remain unchanged.

**Status:** 21 focused tests pass, including the three historical target
fixtures. The target divisor engine and nine-point/32-point pipeline compile
but have not been run on the certified anchor. A required S3 checkpoint is
approved by the user on 2026-10-30 but remains blocked on configured AWS
access. The draft execution record now selects the repository's EKS Spot
backend; a matching target-binary image and command-job route remain to be
qualified. See the
[current review report](../reports/review-20261030T1540Z/REPORT.md) and the
historical [preparation report](../reports/preparation-20261026T1824Z/REPORT.md).

The exact trace-tensor identity has certificate
[`IDC1h368ff5f1ba01b556`](certificates/trace-three-slots.json), checked by the
repository's canonical native certificate code. Its
[integer expansion](certificates/trace-three-slots.expansion.json) agrees on
all six nonzero monomials. No formal prover or independent runner was used.

## Implementation

- Homogeneous coordinate-ring arithmetic over the same explicit F7^18 field;
  exact chart substitutions and a degree-seven smoothness rank check.
- Contact-divisor ideals for characteristics 9,19,26; section-space
  Riemann--Roch and divisor addition relative to the declared B.
- Kummer tensor evaluation through a length-three residue algebra. It
  reproduces all three historical F49 vectors and agrees with direct
  all-six-permutation evaluation. It does not require geometric support
  points to split in the base field.
- Separate calibration, coordinate-only evaluation and validation-only
  reference commands. The latter's O(r) source enumeration is disclosed
  and never supplies labels or targets to the evaluator.
- Fresh-only outputs, input hashes, projective frame/rank checks and explicit
  stops at an unsupported evaluation chart.

The existing [prime-field Pic implementation](../../../src/cryptanalysis/plane_quartic_pic.rs)
was reviewed. It supports odd prime q<2^31 and another fixed class origin;
it does not implement this extension-field theta-origin adapter. Its
section-space approach and Khuri-Makdisi's published algorithms are prior
work, not novelty claims for this continuation.

## Verify fixed components

Use the recorded Rust 1.99.1 toolchain from the repository root:

```sh
cargo test --release --offline --locked \
  --manifest-path research/quotient_calibration_20261025/target-native/Cargo.toml
cargo clippy --release --offline --locked \
  --manifest-path research/quotient_calibration_20261025/target-native/Cargo.toml \
  --all-targets -- -D warnings
cargo test --release --offline --locked \
  --manifest-path research/quotient_calibration_20261025/native/Cargo.toml
```

The target package has eight component tests and nine canonical certificate
tests; the original source package has four tests. These do not execute the
new target frame, challenge oracle or inversion.

The review correction rejects unequal projective vector lengths and validates
all 32 prediction rows before constructing a target divisor. All three output
vectors must contain exactly eight coordinates over the declared field and
must be nonzero as vectors; case IDs, points and successors must match the
frozen inputs.
Regression controls include truncated/extended vectors, zero vectors, invalid
field coefficients, missing/extra rows and duplicate or mismatched cases.

## Run after the gate is satisfied

Complete the ignored resume note and run `scripts/checkpoint.sh gate` before
launching the experiment. Upload approval is recorded; it need not be requested
again. The execution record in `protocol.json` is a valid
draft; record the declared preflight diagnostics and instrument hashes,
freeze it with the canonical sibling tool, and commit it before affected
target data. [PROTOCOL.md](PROTOCOL.md) gives the exact phase boundaries.

Build the native binary, then run `calibrate`, `evaluate` and `validate`
on the qualified EKS command-job image with their declared positional paths.
The standard GitHub Iso Lab workflow currently builds `ecbench`, not this
standalone target executable; do not dispatch its integration fixture as a
substitute. Use fresh paths, keep the generated
matrix immutable and checkpoint it before opening target validation data.
`run-stage.sh RUN_DIRECTORY STAGE ARGUMENTS...` is a thin launcher that
checks the frozen binary, requires the checkpoint gate, enforces a shared
3600-second deadline and 2-GiB address-space cap, preserves raw output/exit
receipts and checkpoints the result. Use the same RUN_DIRECTORY for the
three target phases. It refuses the current draft execution record.
The executable prints its argument requirements on an invalid command;
no default run silently takes place.

The parent 3600-second/2-GiB limit, no-refitting rule and no-replacement rule
remain binding. A chart failure is an implementation feasibility outcome,
not proof that a Jacobian image does not exist. Matrix fitting and even a
32-point pass do not complete inverse fibers, signs, group/Frobenius checks,
Profile B or a global section identity.
