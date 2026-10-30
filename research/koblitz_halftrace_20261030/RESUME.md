# Resume: native Koblitz halftrace comparison

## Goal
Complete the native Rust integration and its saved baseline/candidate comparison
for PR https://github.com/isogeny-labs/crypto/pull/2955. The comparison includes
cold factor-base setup, relation collection, linear algebra and verified recovery.

## Lead / hypothesis
The existing one-word point lift recomputes a binary-linear odd-degree halftrace
for every coordinate. Lazy field-local byte tables may reduce repeated work while
preserving the exact first root, ordered base, orbit maps and recovered scalar.
Existing orbit folding and joins are prior work; this is generic decomposition.

## Spec
PROTOCOL.md and spec.json in this directory. K_0 over degree 31; eight independent
one-target workloads; divisor indices 1;2; target/order seed 20261030; six measured
alternating rounds and one warmup. Native IC legacy-path baseline, its A/A control,
cached candidate and strong signed-Frobenius rho with A/A. Require L2, exact point
hashes and scalar replay. Intended EKS bound: 900 seconds, four CPUs and four GiB,
no automatic measurement retries. The unmodified-source image comparison remains
separate from the same-binary legacy-path comparison; no transfer claim to m=83
or ECC2K-130 is established.
The scientific executable is pinned to the already-tested feature commit
7c58661935b79153ff74c1208d0592aa56ed04a1; the later preparation checkout contains
only its prospective records and workflow support. The spec bytes are unchanged.

## State
The original full release suite finished: 5447 passed, zero failed, 123 existing
ignored. Final-source cache filter (10), ordered-lift test (1), two native E2E
examples and strict library Clippy passed before the main merges. Current-main
merge preserves the feature code, but its existing ecbench/methods.rs has unclosed
delimiters and prevents the repeated checks from compiling. This failure is
recorded rather than attributed to the cache or hidden. An inherited Cargo.lock
reference mismatch was normalized by Cargo. No scientific run has
started, and no speedup is claimed. Auto-merge is paused for the unresolved P1
requirement to retain a matched native comparison. Local checkpoint transport
has no credentials; this Actions preparation uses only the existing approved
CHECKPOINT_UPLOAD_TOKEN secret and fails closed if it is absent or invalid.

## Next steps
Complete the prospective protocol.json before scientific execution. Collect the
checkpoint-only Actions result; a successful upload is required before dispatching
the trusted main-branch isolab-experiment.yml with an explicit tested source SHA
and this spec. Retain original workflow identity, image and binary hashes, L2/A/A
admission, S3 manifests, independent replay, all failures and the canonical export.
Update this PR's report, request resolution of the P1 only with actual evidence,
and verify current-head gate/size/artifacts checks before merge.

## Runs in flight
This preparation job only: GitHub workflow/run identity is in its Actions URL.
It creates a source-pinned S3 resume checkpoint and does not launch an experiment.
No scientific EKS request, Job UID or result receipt exists yet. Local development
checks, if still active, are under /tmp/crypto-halftrace-followup-*.log in the
authoring workspace. Earlier complete suite: /tmp/crypto-native-lib-tests-final.log.

## Stop and success conditions
Stop before scientific launch if checkpoint upload fails, source identity is
unverified or the prospective protocol is incomplete. A test pass, image build,
checkpoint receipt or merged PR is not a speedup. Success requires the saved
matched native comparison with cold setup, verified scalars, controls and all
measurement admission gates; report regressions and invalid results as observed.
