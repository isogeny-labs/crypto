## 2026-11-23 — guarded coefficient compiler design, pending canonical integration

### What was done (commands, files, features)

Continued the existing in-progress five-coefficient task under draft PR5022;
published trackingdcf0bde3cf as e037a77e. Paused its CI-only checker during source
authoring. Read current main journal/instructions, lessons, frozen numeric
predicate and relevant primary torsion/determinant sources. Recorded prospective
compiler contract and fixed controls before implementation. [pcc:unverified]

### Decisions made (and why)

Use a separate Rust crate and a compact topologically ordered arithmetic
program; retain five unknown coefficient inputs, two forced inverse inputs,
every output/guard and original indexed replay. Investigate the classical
degree5 rational torsion guard only under explicit fixed-field assumptions.
Circuit size and solving degree remain distinct. Existing ACTIVE conflict is
pending with one Copilot request; do not edit it or drop either pending entry.

### Open threads (unfinished, pick up next time)

Implement/qualify the compiler, compare extension/projection against frozen
numeric predicate, preserve failed attempts and then assess a concrete unknown-
coefficient solver. Artifact publication still requires genuine trusted-store
repair; source work can proceed independently. No solver campaign is running.

### Errors (what went wrong, how it was handled)

KB executable and ecdb read credentials unavailable; recorded repository and
primary-source fallback. Several primary-source web fetches failed. One guessed
arXiv identifier resolved to an unrelated paper and was excluded. Sparse source
files were materialized from exact HEAD before use; no frozen input was edited.

## 2026-11-23 — first compiler implementation and build correction

### What was done (commands, files, features)

Added isolated public-data Rust compiler, compact Fp gate kernel, streaming
quadratic rows, two inverse guards, cached determinant, adapted frozen fast
membership recurrence and known-fixture extension/projection controls. Preserved
source-attempt-1.tar.xz and complete test-attempt-1.log before corrections.
The first release build exited101 before tests. [pcc:unverified]

### Decisions made (and why)

Retain all norm forced-factor and all membership-coordinate constraints. Only
public constants/wire identities may simplify gates. Compiler arguments omit
coefficients and raw labels; imported known fixtures enter extension separately.
Keep complete program/witness exports even if publication needs store repair.

### Open threads (unfinished, pick up next time)

Correct integer annotations, finish focused tests and strict lint, then freeze
source and immutable native correctness qualification. No control outcome or
coefficient-root discovery is established by source authoring.

### Errors (what went wrong, how it was handled)

Three BigUint conversions inherited inferred i32 constants; explicitly annotate
the public parameter tuple as u64. Remove the unused import. Review also found
two overlapping RefCell temporary borrows in scalar construction; use one scoped
borrow before any native control. Old source/evidence remains unchanged.

Second source/build attempt also stopped before tests on two inferred i32 loop
indices. Preserved source-attempt-2.tar.xz and test-attempt-2.log; annotate the
constant loop u32 and matrix dimension usize. No native correctness result was
produced by either failed build. [pcc:unverified]

