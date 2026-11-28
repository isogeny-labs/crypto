# RR5 payload source-pin review

2026-11-23 · PR #5099 · unsigned source-byte review [pcc:unverified]

## Decision and scope

Refresh two whole-file SHA-256 expectations in
`research/prime31_four_summand_20261122/native/f4-source/build.rs` to the
reviewed source at main commit
`3dd555c5d9b54c5bc0f3507eb6a917a5cf64e1f6`. Keep the builder's whole-file
equality assertions, unique-anchor checks, ordered nonoverlapping selections,
verbatim copying, and generated-byte readback checks intact.

The review compared all **18 selected byte ranges in nine files** against
producer publication commit `307c514f39a8602d4ba1c5c0821a4c410159615d`
(PR #4974), then checked those producer selections against the historical
`scaling-b677/evidence/build-02/source-manifest.json`. All 18 producer range
hashes agree with that historical manifest. Against current main, 17 selected
ranges are byte-identical. One range contains a 473-byte addition consisting
only of a comment and two `#[cfg(test)]` methods. No existing production
selected function body changed in the inspected diff.

This is a source-provenance review. It supplies no experiment, extraction,
completeness, scalar-recovery, performance, or signed PCC claim. It does not
audit every whole-file dependency imported by the standalone package, and
does not replace its separate compilation and qualification checks.

## Exact source identities

The first failing guard concerned `src/cryptanalysis/ic_boundary.rs`:

| Source state | Commit | Whole-file SHA-256 | Bytes |
|---|---|---|---:|
| Frozen expected source, recovered from Git | `624d3f3112529a80d68590dce9ab32fc463e4a19` | `ee70c40d611bea2a86e782e8513d6e18be49ca575e1159a42512afb18dc346ce` | 999509 |
| Producer publication | `307c514f39a8602d4ba1c5c0821a4c410159615d` | `86963eab84b584b987362c7d9ba183c45d7fefc6207eb792a743670997f21bbd` | 999562 |
| Reviewed current main and refreshed expectation | `3dd555c5d9b54c5bc0f3507eb6a917a5cf64e1f6` | `7a7ff906dd2206b236a31155f0bdaa2d753b890b2edd79a67aaeb5cb52d72311` | 1000016 |

The frozen whole-file expectation already differed from the later producer
publication. Commit `033092b3db51ab255bfa00d030b04caef07e7d81` adds
`y_plus: None` and `y_minus: None` to a binary-group structure outside all
selected ranges. The frozen source and producer publication nevertheless
yield the identical 73745-byte concatenated selection, SHA-256
`5102a5633c36b96097c34d15e72abbfb441fc669053d14b8e8ebaec3fc4af450`.
That selection also agrees with the historical build-02 manifest. The
publication commit is therefore not silently substituted for the earlier
whole-file source identity.

## The one changed selected range

The zero-based range index is 1 in `ic_boundary.rs`. Its start anchor is
`// ── A fast hasher for point keys`; its exclusive end anchor is
`fn mix(v: u64) -> u64 {`.

| Item | Frozen selection / producer publication | Reviewed current main |
|---|---|---|
| Byte interval, start inclusive and end exclusive | `[42828, 77718)` | `[42828, 78191)` |
| Selected bytes | 34890 | 35363 |
| Selected SHA-256 | `cf654b900b55ca692389191c12e761f19bc43722b157d3f23905894a9459d5fc` | `dfddca8d826b072b868153b8e67af24fd645f9b9bb9b951e18b48e3e8b32b5cc` |

The complete change is retained in
[`ic_boundary.rs.range-1.diff`](ic_boundary.rs.range-1.diff). It adds
`OpenCoefMap::iter` and `OpenCoefMap::values`, each guarded by `#[cfg(test)]`,
to expose occupied entries to collision-ledger tests. These methods are
excluded when that module is built without `cfg(test)`. Their addition is a
real selected-byte change and must not be described as source identity.
This textual review does not assert that compiling a particular package
enables or disables every test configuration.

The other five `ic_boundary.rs` selections are unchanged byte for byte.
The reviewed current concatenation has 74218 bytes and SHA-256
`7d08dfae90c782cd3d93c71600420b88ce4d87ff8a5751646328a38e3202f44a`.
The only additional producer-to-current whole-file change is a comment
outside the selected ranges: the factored-subspace description changes from
odd parity to any parity of `n`.

## The second changed whole-file expectation

`src/cryptanalysis/semaev_prime_poly.rs` changed outside the selected `Fp`
implementation:

| Item | SHA-256 |
|---|---|
| Frozen expected whole file and producer publication | `c8249ba825c946dbdb37fa5d6762f8f32d15320344693443e8af0d52f122784a` |
| Reviewed current whole file and refreshed expectation | `e77ebb1576469b24e399cf2cd62059b80607eb65eeaa486847782e6f11714f9b` |
| Identical selected `Fp` bytes in both sources | `c0e28396b7d714b2f0e29a93f2d3c1e1f95090b793541cc930e7f881e01a70e8` |

The selected interval remains `[1618, 3026)`, length 1408 bytes. The old
whole file has 25450 bytes and current main has 35275. No selector anchor
or selected `Fp` byte was changed to accommodate the new source.

## Complete selection inventory

[`selected-ranges.json`](selected-ranges.json) records exact old/current
hashes, sizes, byte intervals, and literal anchors for every range.

| Production file | Selected ranges | Producer-to-current finding |
|---|---:|---|
| `src/cryptanalysis/challenge_ladder.rs` | 1 | Whole file and selection identical |
| `src/cryptanalysis/glv_invariant_base.rs` | 2 | Whole file and selections identical |
| `src/cryptanalysis/groebner_f4.rs` | 1 | Whole file and selection identical |
| `src/cryptanalysis/ic_boundary.rs` | 6 | Five identical; one test-helper addition described above |
| `src/cryptanalysis/pq_groebner_f2.rs` | 1 | Whole file and selection identical |
| `src/cryptanalysis/prime_fast.rs` | 3 | Whole file and selections identical |
| `src/cryptanalysis/residual_walk.rs` | 1 | Whole file and selection identical |
| `src/cryptanalysis/semaev_prime_poly.rs` | 1 | Selected bytes identical; other file bytes changed |
| `src/prime_hyperelliptic/ops.rs` | 2 | Whole file and selections identical in both Git trees |

The audit was captured **before** refreshing the expectations and hydrating
the remaining sparse dependency. Its `pinned_whole_sha256` values deliberately
preserve the frozen expectations. Its workspace builder digest identifies
that pre-refresh snapshot, not the later integration builder. At capture
time `src/prime_hyperelliptic/ops.rs` was absent from the sparse working tree;
the audit read its exact Git blobs and records the local value as `null`.
These fields are retained as historical observations rather than rewritten
to describe the subsequent workspace.

## Evidence integrity and future review

The two supporting files are copied without modification from the read-only
source audit. Their SHA-256 values are:

- `selected-ranges.json`:
  `8c3a89f60585aff08772e1cc6724c6d12f2794bcdcf30ad92a9d0aa8fea74e01`.
- `ic_boundary.rs.range-1.diff`:
  `16b22ba426318698659ceb4a47e7a57b50d284fd9fd00667eb4b7052560b2c41`.

The pin refresh accepts only the two reviewed complete source files. It does
not make the guard conditional, skip a mismatch, replace production code,
or accept an arbitrary current revision. Preserve the historical expected
hashes, manifest, and qualification receipts; later source identity and
qualification belong to new records.

For any future whole-file mismatch, stop at the existing guard. Record the
old expected hash, proposed source commit and hash, then re-extract every
range with the original unique anchors and compare exact bytes. Review
changed selected code and its dependencies, including test-only changes;
do not infer equality from a successful compile. Record a separate review
and new qualification when required before deliberately changing a pin.
A changed or nonunique anchor, reordered/overlapping range, changed
production selected function, or changed imported whole-file dependency
requires an explicit dependency-closure review. Missing sparse files must
be restored from the pinned source, never replaced with local stubs.
