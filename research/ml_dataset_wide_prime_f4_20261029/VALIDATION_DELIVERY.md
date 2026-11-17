# Bounded wide-prime F4 dataset delivery

## What was done

The existing generic F4/Fp64 engine now has a bounded dataset entry point for prime m=2, with checked admission and supervised original deadlines, retained boundary events, final basis/result and original-domain root/lift verification. The CLI admits this profile for primes from 2^32 through 2^61-1. Native source and adapter/context underwent independent source review; DESIGN.md, IMPLEMENTATION.md and ADAPTER_CONTEXT.md record implementation and literature provenance.

Validated runtime fe7ccef6eb7f640fc8e95212471b80700d3cf96b passed 271 focused tests (0 failed, 1 ignored) and the full library suite: 5388 passed, 0 failed, 123 ignored. Release CLI SHA256 a4ad63da4d7c1f89835c1aa51af37968ae7582cf60dea2a9acf9e2b51bde914d. Strict Clippy passes at59227cd2220 after an equivalent u64 range predicate rewrite; subsequent changes are research documentation and fixture files. These are exact-source local results, not current-head remote CI.

Frozen v6 at3a935e02529 passed32 CLI commands over six fixed software cells: n47, n53, n61, both twists, m2, factor-base size2, seed42. Each invoked native F4, completed solver/extraction and verified no decomposition in its original finite domain. Each remains status partial and has no independently recovered scalar. The fixed n61 modulus was2305843009213693951. Positive controls are separately executed source tests, not these naturally negative cells.

The S3 receipt records full outputs, binary, protocol, exports, reports and logs. Conditional upload, fresh HEAD/GET, byte/SHA256 comparison and contained hashes passed. All six downloaded cells passed verification with the downloaded binary; six verification outputs matched originals byte for byte. This is producer-native semantic replay, not a separately implemented solver-basis proof.

## Decisions made

V5 stopped after seven successful commands and one cell because a global forbidden-key predicate rejected inputs.field.basis=canonical-residue. Preserve that failed fixture and archive. Independently reviewed v6 permits only that exact public path/value and changes no runtime, case, seed, resource limit or other acceptance check. Its amendment discloses the prior observed case; no outcomes were resampled to obtain success.

Public features retain existing static structural views, not a newly implemented wide polynomial graph. Complete equations and native artifacts remain restricted. Work/storage reservations are admission counters, not executed instructions or measured RSS. First/last fall degrees, full execution/basis derivation, total instruction counts, scalar recovery and model efficacy remain unproved/unknown. Scientific cloud cells remain0.

## Open threads

Current-head CI/review and repository delivery remain pending. The complete original goal retains higher m through15, all requested curve/solver/factor-base axes, production generation, independent consumers, instruction labels and ML workflows. This bounded m2 increment does not redefine completion. Original cloud deadline and total cap remain unchanged; no cloud job was launched for these fixtures.

## Errors

Initial sparse-build fixture omissions and Rust annotation failures, original Clippy style diagnostics and v5 checker failure remain in the saved validation records. V6 and restore succeeded; failed prior evidence was not overwritten. Session-log directory is held by another Conductor lane, so this scoped handoff records the delivery.
