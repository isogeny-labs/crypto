# Static prime-chain graph software validation

Runtime source `1565b1fecd726945e2107de6bc461b64281885fc` passed the full release library suite:
**5,243 passed, 0 failed, 123 ignored** with one test thread and
eight Rayon workers, strict maintained Clippy,
scoped formatting and release CLI build. The earlier source `37ccfb20db6` also
passed 20 focused graph/export tests. The full-source suite includes those tests.

The frozen actual-CLI fixture passed five commands using binary
`0d965332c529967b066ddc99819f47cdd57baeccde59a37cbdee94455cbc17b3`: export and immutable retry of each retained n=7, m=3
curve/twist cell, then rejection of rehashed semantic corruption. The Python
consumer reconstructs equations from graph edges and verifies exact field/domain
values, feature hashes, signed points and artifact hashes. Its 27-file archive is
`cad3e711f2d22b2d9e940e87ead34598ea338f3d0ef59def4b5f29dba49bedf8`. Unit tests additionally cover n=61, m=3/4/15,
independent BigUint S3/membership evaluation, malformed graphs, metadata isolation,
and explicit whole-graph resource exclusions.

The S3 manifest and [final fresh download receipt](final-readback.json) retain 47 artifact entries
(43,863,824 bytes), including the binary, source-bound build/test
receipts, complete passing output and both failed CLI attempts. Initial compile
failure (sparse inputs and a test clone), lint failure, Python 3.9 compatibility
failure, staging-directory monitor race and the default-parallel full-suite failures
(two GPU adapter tests and the Hilbert-stop counter-comparison test) remain
recorded. The GPU subset (8 passed) and Hilbert test (1 passed) passed in separate processes;
the final complete-suite check serializes tests to avoid shared process state.
Corrections did not weaken acceptance conditions. The amended fixture was committed before execution.

This is software correctness evidence. Corpus rows added: 0. Verified cloud dataset
cells: 0. No independent scalar recovery, native full-trace replay, instruction-total
verification, first/last-fall theorem, prefix-action training or model efficacy is
established. Earlier partial agent reviews found no production defect, but the
agents hit their usage limit before final review; independent full code review is
not claimed. Exact-head remote CI is a separate PR gate.

The initial report helper selected a nested one-test child summary. The corrected
report uses the last outer summary and checks 5,243 + 0 + 123 = 5,366 against the
outer test count and exit-0 receipt. The incorrect draft and corrective checker
are retained; no runtime test result changed.
