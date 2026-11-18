# Versioned endpoint scheduling and algorithm seeds

The requested ordinary prime-conductor crossing remains at least 80 bits,
with a complete quotient, non-kernel images and repeated matched scalar and
ECDLP measurements. This component fixes planning prerequisites without
claiming a constructed target or measured session.

## Frozen scheduling contract

Add opt-in measurement order `paired_curves_v1`. This is a complete versioned
ordering/seed law; it is not a change to any existing order or solver.

- Require `planted_shared`, an even positive measured-round count, prime-field
  curves with u64 subgroup orders, and at least two distinct workloads per
  subgroup order. This groups same-order curves; map/conductor evidence is
  a separate requirement. No isogeny certificate is inferred from equal r.
- Group executions by `(target_index,r)`, in ascending key order. Inside
  each group retain the listed curve order on even measured rounds and
  reverse it on odd measured rounds. Reverse the arms by the same parity.
  Warm-up rounds alternate separately; measured parity starts at zero.
- Hash canonical JSON with domain
  `ecbench.paired_curves_v1.algorithm_seed` and integer parts
  `[measurement.seed,r,target_seed,target_index,seed_round]`, taking the first
  eight SHA-256 bytes in big-endian order. Warm-ups use the distinct
  `ecbench.paired_curves_v1.warmup_seed` domain. The existing explicit
  algorithm_seed_round option controls absolute versus measured seed indices.
- Shared seeds mean shared RNG inputs, not identical walks under different
  coordinate hashes. No planted scalar is supplied to the solver. Workload
  identities, target construction, method code and operation accounting are
  unchanged. The new spec identity includes the new order value.

Before accepting this component, require: native OpenSSL seed-oracle vectors,
the actual Rust harness's full paired plan for the existing six models/32
targets, balanced method/endpoint order, shared seeds and planted scalars,
negative admissions, tiny solver/replay controls, and byte-identical CLI
legacy output against the preserved previous executable. Build the real
root library and unchanged CLI; no substitute scheduler or solver.

Preserve the previous planner source in source-v2/spec.rs, all earlier
executables in conductor-wide-target-v2, and the existing proof subjects.
Use a new conductor-wide-target-v3 cache for corrected-source qualification.
The full legacy unit suite's separately recorded duplicate-test failure is
not repaired or called a passing suite by this focused target.

These are bounded local build/unit/codec checks, no performance experiment.
No `ecbench run`, benchmark timer, cloud spending or timing claim is allowed
in this component. Tests have a 300-second correctness cap after building;
the seed fixture has a 30-second cap. Every qualifier uses a fresh directory.

Sources checked: existing ecbench spec/runner contracts and canonical seed
implementation; the original MEASUREMENT_PROTOCOL.md; existing wide-field
map packets and prior source/binary receipts; ecbench-extend and
ecbench-measure skills. KB unavailable, wrapper exits 127. This introduces
no cryptographic algorithm or novelty claim. Scalar timing, registration,
strongest wall reference, wide-field IC, large target construction and live
cloud/S3 admission remain separate obligations. No result is established yet.
