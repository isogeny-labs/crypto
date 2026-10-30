# Exact cofactor validation and delivery

Runtime source `40469deb0b4016873b4215b7a33c763f500deda7` preserves a prime
dataset's exact cofactor instead of rejecting values above `u32::MAX` to fit
the legacy ECC context. The dataset-local context checks canonical positive
orders and the exact subgroup-order/cofactor product; reference point
arithmetic and existing field/solver admission remain in force. Legacy
`ecc::CurveParams` is unchanged. This arithmetic context does not itself
replace public cardinality and subgroup verification.

All required local gates completed with Rust 1.98.0 and explicit metadata
matching the clean source before the final build:

| Gate | Observed result |
|---|---|
| Initial focused regressions | 3 passed, 0 failed |
| Final dataset tests | 161 passed, 0 failed, 1 ignored |
| CLI integration | 8 passed, 0 failed |
| Strict lib/bin/CLI Clippy | Passed |
| Strict lib/all-tests Clippy | Passed |
| Full library, immutable final executable | 5,186 passed, 0 failed, 123 ignored; exit 0 |

The frozen large-cofactor fixture is `p=266287972351`, subgroup order `31`,
cofactor `8589934592` and group order `266287972352`. Its exact cardinality
certificate and point order are independently replayed. Bounded BSGS receives
no private witness, recovers scalar `2`, and scalar replay rejects substitution
with `3`. Small-cofactor fixtures compare legacy arithmetic on both twists.
Invalid canonical encodings, zero/overflowing cofactors, mismatched products
and the retained F4/PKM field caps have regression coverage. This is a bounded
adapter fixture, not a full factor-base/export transcript or campaign sample.

The independent static review found no actionable defect in its source-bound
production diff. A later test-only revision pins the initially selected
fixture and strengthens scalar-label checking. Both source snapshots and the
historical review/delta are retained; the earlier pending-test statements are
superseded by the terminal results above.

Immutable final test executable SHA256:
`1d0d9d80b46474d6e9caf10b790b370cb5f2fdc6ab27e10c26c686c120ea9526`.
Copied CLI SHA256:
`ff8e6b3a7af099e6fa14d3a05a7146f0dc530bbeaa593ebbe7acb4407519c283`.

The archive includes source bundle/patch, both final executables, initial
focused executable, all validation logs, review packet and hash manifests:
`s3://setec-astronomy-artifacts/ml-dataset/20261028/software/exact-cofactor/dbd2b01a2de0aa98b51ddf0c0de1323189e83489ea334a4b6c27a1c5660c31cc/validation.tar.gz`.
It contains 85,374,684 bytes; its SHA256 is the hash in the path.
Conditional PUT, fresh ETag-bound GET, byte equality, contained hashes and the
restored Git bundle were verified at `2026-10-28T16:53:24Z`. See
`validation-archive.json` and `validation-40469deb.json`.

This software validation does not establish historical whole-corpus equality,
cloud generation, complete instruction cost, solver superiority or any first-
or last-fall-degree claim. The original $200 total cap, cloud owners, worker
publication authorization and durable quota-admission gates remain unchanged.
