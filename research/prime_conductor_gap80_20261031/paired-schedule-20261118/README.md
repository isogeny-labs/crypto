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
cloud/S3 admission remain separate obligations. The completed qualification is recorded below.


## Corrected-source qualification

The corrected v4 source passes eleven focused integration tests, root-library
clippy and focused-test clippy with warnings denied. The actual CLI's legacy
output is byte-identical to preserved v2, and its paired/full plans are
byte-identical to v3. The CLI qualifies 12 workloads/252 planned executions;
the full panel retains 192 workloads/4032 planned executions, including 32
public target indices per model. Four negative admissions reject with the
expected reasons. No measured session ran. [pcc:unverified]

PCC-20261118-219d4a binds the final summary, actual test stdout, CLI outputs
and current source/executable manifests. It remains an unsigned producer
statement, unverified and not_covered. Earlier v3 subjects and claims retain
their bytes. The independent native seed fixture supplies 680 reference
vectors; the actual events look up the applicable vectors. This does not
claim every fixture row is exercised. [pcc:unverified]

| Requirement | Current evidence |
|---|---|
| Shared scalar/seed identities and reversed endpoint/arm order | Implemented; focused tests and actual CLI codec qualification pass |
| Legacy plan compatibility | Exact v2/v4 byte comparison passes |
| Several fields, dozens of points, repeated rounds | Full native plan checks two fields, six models and 32 indices; no timing session |
| Full legacy root unit suite | Separately recorded pre-existing duplicate-test build failure |
| Endpoint statistics and scalar timing | Not implemented/executed by this component |
| Wide-field index calculus | Explicit admission rejection; no adapter |
| Minimum 80-bit conductor crossing | Quotient, map and non-kernel images still missing |
| Cloud/session admission and S3 publication | Existing blockers retained; PR stays draft |

The full panel uses the previously evaluated degree-2 and degree-5 controls,
not the original prime-degree kernel. Equal subgroup order is not a map or
conductor certificate. Same RNG seeds need not produce identical walks.
Reversal balances the orientation of each listed pair; it does not cycle
all three models through every position or establish joint orthogonality.

### Evidence and reproduction

- [Final summary](final-summary.json), [test stdout](proof/v4-tests.stdout),
  [CLI summary](proof/v4-cli-summary.json), [report PDF](reports/report.pdf).
- `source-v2` preserves the planner before this feature; `source-v3`
  preserves the successful earlier planner/tests before the parity lint fix.
  Current v4 source uses the standard integer parity method.
- The first focused build's nested legacy reexport warning and v3's failed
  root parity lint are preserved with successful corrected outcomes.
- Build/test/CLI pre-call SHA manifests distinguish each source and binary.
  Old manifests referring to overwritten source paths resolve through their
  explicitly preserved version overlays. Release target-v2/v3/v4 directories
  remain separate outside Git.
- `runs.tar.xz` retains the raw development tree, including failed builds and
  qualifiers. Named PCC subjects remain loose and unchanged.

From the repository root, use Rust 1.100.0 and the retained dependency lock:

```sh
cargo test --release --manifest-path research/prime_conductor_gap80_20261031/paired-schedule-20261118/Cargo.toml --locked --offline --lib
cargo clippy --release --lib --locked --offline -- -D warnings
cargo clippy --release --manifest-path research/prime_conductor_gap80_20261031/paired-schedule-20261118/Cargo.toml --locked --offline --tests -- -D warnings
```

The frozen qualifiers require a fresh output directory and the separately
built native executable. `qualify-frozen.sh` checks the exact test list;
`qualify-cli-frozen.sh` invokes only native `ecbench plan`, using Python only
for serialized metadata validation against the native C++ seed fixture.
A test's duration is not an ECDLP measurement. All workloads are unregistered.
[pcc:unverified]
