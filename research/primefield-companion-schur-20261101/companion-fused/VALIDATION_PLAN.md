# Full validation design — not frozen or executed

Preserve all 369 candidate contexts and all 16164 product records from the
sealed companion-blocks-validation archive. Reconstruct full generators from
the locked public constructor archive; principal and inverse generators from
the archived payloads. Associate each vector with its original stage by the
recorded restriction test counts and original full-vector schedule. Verify
that every archived product is consumed exactly once. This is an archival
correctness/count baseline, not a fresh runtime baseline.

Run candidate/list and AA/tuple inputs across those same full contexts:
738 rows and 32328 products. Add 480 carry contexts per arm:
primes {7,13,19,43}, dimensions {1,2,r-1,r,r+1}, widths {1,3,7},
wrap pairs {(0,1),(1,0),(2,1),(r-1,r-2)}, all-maximum/seeded digits.
Both directions: 1920 additional products. Total declared plan:
1698 context rows, 34248 product calls. Freeze the complete expanded seed
schedule and verify its bijection with execution cells before taking data.

For every archived product: compare exact output to its prior dense-verified
reference; verify all fixed counters and native product bit-length bounds;
retain original baseline units separately. For every carry fixture: compare
all packed intermediate values to independently evaluated coefficient lists
in reference.py; check every coefficient against the proof bound. Validate
non-product bit-volume counters using an independent event reconstruction.
Require exact AA equality of outputs, counts and trace fingerprints.

Negative controls: changed output, changed count, altered wrap/sign trace,
changed coefficient trace, equal wraps, and radix not strictly exceeding
the proof bound. The latter rejects an uncertified radix; it does not assert
that every undersized radix causes an actual carry. Include a constructed
primitive overflow certificate to distinguish the two claims.

Before execution: finish checker, independent bit-volume reconstruction,
analyzer and export schema; lock all sources and archives; set one explicit
local cap, no-restart policy and failure retention; freeze protocol and commit.
No numerical trials, selected examples, timing pilot or hidden downsampling
are authorized by this draft alone. The owner has authorized continued
research; this staging requirement preserves the existing experiment rules.

After execution: keep full raw records and original failures, verify locks,
reconcile exact declared counts, render and inspect every report page. Any
future Schur integration is a separate protocol and fresh invocation; the
capped original Schur run remains immutable.
