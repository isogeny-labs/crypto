# Enforced native event encoding bounds

2026-10-31. Software change; no scientific corpus generation or new cloud job.

The saved diagnostic at source 783f98ce90c2 contains 184 native records. Parent retention refused its 100526-byte canonical event artifact with 758542 original modeled work remaining. The diagnostic and full raw fixtures have verified S3 custody in the parent-diagnostics-failure archive ab7d155212cf133854273ce01fb28ea915ec97ce88990cbb9310f5321ad8f671. Existing limits and workloads remain unchanged.

## What changed

Native construction events now admit a 640-byte whole-envelope ceiling and matrix events a 1408-byte ceiling, within the unchanged allocated 2048-byte buffer. The capture encoder enforces the selected ceiling in both its counting and serialization passes before the precommit callback or event commit. The generic caller preserves its original ceiling. The work formula remains 1024 + 9 * admitted event bytes. Memory and trace reservations are unchanged; previously charged work is never refunded.

This narrows a worst-case modeled reservation. It does not measure fewer instructions or eliminate a serialization pass. Separate native projection custody work investigates actual redundant publication elimination.

## Sources and bound argument

The governing schemas are NativeBoundary, ConstructionBoundaryStage, MatrixBoundaryStage and NativeBoundaryView in src/ml_dataset/koblitz_m3.rs; Reservation and Counters in src/cryptanalysis/koblitz_groebner.rs; and TypedEventRecord plus event_typed_precommit in src/ml_dataset/capture.rs. All variable leaves are unsigned integers, fixed ASCII enum names, booleans or validated 64-character hexadecimal digests. Maximum decimal widths bound every numeric leaf. There are no arbitrary strings or user-controlled maps in this producer payload. Both outer and inner binding fields, all sequence fields, the previous hash, kind wrapper and trailing newline are included.

The maximum-width control enumerates all eight construction stages and all eight matrix stages, including both boolean values of CriterionCandidate. It uses actual production envelope serialization with maximum u64/u32/u8/usize values and checks both size and exact canonical extraction. A separate capture control checks an exact ceiling, a one-byte-short ceiling and zero: refused cases must not reach precommit or advance/write the event stream. Schema growth beyond a bound fails closed under the production writer cap.

## Validation and remaining work

Validation is pending at initial commit. Wider-worker diagnostic failures are preserved, not relabeled as passing. The prior whole-cost analysis shows reservation narrowing alone cannot establish full parent delivery. Runtime/full-library/CLI completion gates remain pending. Dedicated publication changes require separate receipt, accounting and replay review before integration.
