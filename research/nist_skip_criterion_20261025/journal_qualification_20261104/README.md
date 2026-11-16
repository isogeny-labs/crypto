# Random-curve journal qualification

This component continues the requested 10,000 accepted curves in each of six
fields. It implements a separate reader for the October 29 sampler journals
and an interruption inspection mode. It does not collect the baseline or
change the frozen statistic, acceptance rules, nonce derivation, or old results.

Before implementation, the October 29 PROTOCOL.md, sampler.cpp, DEVELOPMENT.md,
and BUILD.json were read, together with the current EKS request/workflow and
recovery contracts. The [PARI manual](https://pari.math.u-bordeaux.fr/dochtml/html/Elliptic_curves.html#ellsea)
documents that ellsea(E,1) can return zero on detecting an order factor, and
uses ellcard on tiny fields regardless of its torsion argument. Consequently
zero is a disposition marker, never an order. The new reader will use ellcard
to count every mapped curve independently of that early-abort path and rerun
primality and point checks. Both programs depend on PARI/GMP/OpenSSL; this is
separate implementation, not an independent library or an execution proof.

The archived external PROTOCOL.md already conditions its Frobenius matrix
model on no eigenvalue 1. Prime-order conditioning is not a new proposed
explanation. That model does not discharge the actual seeded-curve baseline.

The current authenticated portable workflow still builds ecbench; a matching
PARI worker image/route, uploaded checkpoint, finite pilot budget, canonical
execution freeze, independent admission, and production amendment remain launch
requirements. Development here uses frozen records and bounded toy fixtures.

| Requested deliverable | This component's role |
| --- | --- |
| 60,000 accepted curves | Check attempted-prefix and accepted-row integrity; collection remains open |
| Ten published percentiles | No percentiles from development fixtures |
| Cutoff sweep 100–5000 | Preserve existing sweep; journal statistic remains fixed at 691 |
| Pre-data fixed criterion | Preserve October 29 protocol; no data-driven retuning |
| G/b and selection mechanism | No new mechanism asserted by this component |

## Reader and recovery contract

`journal_check.cpp` streams both v1 journals with bounded row memory. It binds
their complete bytes before and after reading, enforces the pinned six-field
cohort, derives each seed and coefficient again, uses full ellcard on every
mapped curve, reruns primality and acceptance predicates, and checks each
accepted point using PARI multiplication (the sampler uses GMP affine
multiplication). Euler exponentiation independently reconstructs the splitting
and ramification columns. An early-abort row must correspond to a composite
full order; its literal NA order is never interpreted as zero.

`verify` requires the exact target and refuses incomplete rows or journals.
`inspect` also admits a valid interrupted prefix: a torn final attempt, or a
final accepted attempt whose accepted row is missing/torn. A missing middle
row, changed value, extra row after the target or reordered counter is an error.
The result names the last safe byte prefixes and next index, bound to nonce,
field, target and both complete source hashes. Completion JSON is not trusted.

`sampler_resume.cpp` and `resume.hpp` retain the old sampler's arithmetic in a
new wrapper. Resume first replays the original journals, then creates an
exclusive fresh output directory, copies only the safe prefixes and replays
the orphan index if necessary. It never edits the originals. Recovery metadata
records the original hashes and retained offsets. File writes handle short
writes; every attempt is fsynced before its accepted row, and directory entries
are fsynced. Errors stop with the partial output preserved. No automatic retry
or replacement nonce exists. Retain the originals with any future export.

These are POSIX file semantics and bounded write-boundary controls. A real
Spot interruption/restore, object-store checkpoint transfer, six-field scheduler
adapter and image/source admission still require separate qualification.
Full replay deliberately recounts rejected curves and can cost substantially
more than the sampler's early-abort path; no replay cost estimate is asserted.

## Build and interface

From this directory, with the recorded installed PARI:

```sh
make BUILD_DIR=/tmp/nist-journal-build test
```

For distribution PARI development packages use `PARI_ROOT=/usr
PARI_LDLIBS=-lpari`. The dedicated main/manual workflow installs the native libraries
and runs only the bounded development controls. It has no cloud credentials.

Interfaces (these lines do not launch anything):

```text
journal_check verify|inspect COHORT BITS NONCE ATTEMPTS ACCEPTED sampler-v1 TARGET
sampler_resume --pilot|--generate COHORT BITS NONCE FRESH_DIR NEW|OLD_DIR ACK_FROZEN_CHECKPOINT
```

Production interfaces accept only the pinned original six fields, lowercase
32-byte nonce hex, and target 10 or 10000. The test harness alone uses p=5003
and target 3, with a fixed public all-zero nonce and a 4096-attempt cap.
The acknowledgement argument cannot replace the external checkpoint gate,
published source/image freeze, original deadline or budget.

Raw development transcripts, toy journals and runtime/build hashes are in
review-evidence/ and MANIFEST.sha256. BUILD.json records their scope, the first
failed fixture assumption, unsupported LeakSanitizer run, and absent Rust tools.
The first failed test's source snapshot was not captured; only the final
successful source/build bindings are claimed. No signed verdict is asserted.

The complete requested study remains on the blocked
[baseline card](../../../backlog/XP-20261104-nist-random-curve-baseline/README.md).

## Development observations and standing

Local controls exited zero: 46 native cases, one CLI empty-prefix inspection
and six expected CLI refusals. The toy stream contains 86 attempts and three
accepted curves in p=5003. All six tested recovery states reproduce the
uninterrupted producer's complete journal bytes; the real-field baseline count
remains zero. Address/undefined sanitizer controls also exited zero with leak
detection disabled because LeakSanitizer failed under this environment's
process tracer. No leak-check coverage is claimed. [pcc:unverified]

PCC record `PCC-20261104-ab1619` pins BUILD.json and MANIFEST.sha256 as unsigned
attestations. `pcc standing` returns `unverified`, and `pcc verify` returns
`not_covered`: these are statement assertions, for which the present PCC tool
has no proof kind. They are not admitted measurement or execution proofs.
PCC's separate sabotage canary returned 56/56 expected classes. Rust library
tests and native backlog validation exited 127 because their commands are
absent locally; CI admission remains pending. [pcc:unverified]

Publication note: the original MANIFEST.sha256 is the immutable receipt for
commit 2432e77f9b4bdc8f5f7565650476a62b5ad733c6. The workflow trigger was then
corrected to main push/manual dispatch under the current repository policy;
CURRENT-MANIFEST.sha256 gives current file bindings. All native code and raw
control bytes are unchanged. See PUBLICATION.md for PR and conflict state.
