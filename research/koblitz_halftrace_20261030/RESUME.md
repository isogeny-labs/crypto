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
examples and strict library Clippy passed before the main merges. Upstream
main53f4b531f fixed the earlier methods.rs parser error and lockfile references.
Merged it at4790f74f1 while preserving the cache implementation. The new locked
recheck failed on an unclosed delimiter in binary_semaev_chain_sat.rs, identical
to upstream; ordered/E2E rechecks were not attempted after compilation failed.
Separate receipt and source hashes are in development/recheck-20261030T1641Z/.
No unrelated fix or test suppression was applied. No scientific run has
started, and no speedup is claimed. On 4 November, fast-forwarded to the external
merge507829726fbc731543f4527348b55c5e479242c9. That source contains the upstream
parser repair; fresh cache12/12, ordered-lift1/1 and E2E2/2 rechecks pass.
The full suite has two reproduced resource-publication failures: this host lacks
/proc/self/ns/time and /proc/self/timens_offsets required by the diagnostic
clock. Its source equals current main; no test was disabled or source patched.
The parent suite ended without its summary or exit receipt; the interruption
cause is unknown. The one-pass nested child summary is not a parent result.
Its full partial output is retained in the additive closure-20261104T1707Z
outcome and deterministic archive. No suite was restarted. Original
results remain historical. Source-bound follow-up: development/recheck-20261104T1548Z/.
Auto-merge was found re-enabled externally and paused again for the unresolved P1
requirement to retain a matched native comparison. Local checkpoint transport
has no credentials. Checkpoint-only Actions run41082358257 at preparation
source1e6c9a434058f274ddb559c72a81bc279ec3f1cf failed because its approved upload
secret is not configured; all build/experiment jobs were skipped. No checkpoint
was uploaded. protocol.json is prospectively frozen with canonical hash
e8e527aacb96a7e172069353254b47bce33526da7111f913b67bcdc4648c0d07.

## Next steps
On 8 November, prepared an explicit manual full-library development job in the
existing rust-all-targets workflow. It uses GitHub-hosted Ubuntu with an actual
time-interface preflight, immutable source input and Rust1.99.1. Ordinary compile
jobs remain unchanged; no new credentials, scientific run or benchmark is added.
Publish and lint this change, then dispatch one pinned development validation
attempt and retain its run identity. The old local attempt stays incomplete.
See development/hosted-validation-20261108/ for its bounds and receipt contract.

Attach an existing approved checkpoint/lab connection or configure the approved
Actions upload secret. Do not repeat the failed preparation until its cause has
changed. A fresh successful source-pinned checkpoint is required before dispatching
the trusted main-branch isolab-experiment.yml with the tested source SHA and this
unchanged spec. Retain original workflow identity, image and binary hashes, L2/A/A
admission, S3 manifests, independent replay, all failures and the canonical export.
Update this PR's report, request resolution of the P1 only with actual evidence,
and verify current-head gate/size/artifacts checks before merge.

## Runs in flight
No scientific run. The preparation is terminal failed. No scientific EKS request,
Job UID, data export or result receipt exists. No development checks remain
live. Source507829726f cache/ordered/E2E completed; E2E log reports2passed/0failed.
Old exec73044/34529 handles are unavailable. Full-library cargo10207/test10463
are defunct; /tmp/crypto-halftrace-nov4/full-library-attempt2.exit is absent.
The raw partial log is /tmp/crypto-halftrace-nov4-full-library-attempt2.log,
also retained in development/recheck-20261104T1548Z/closure-20261104T1707Z/runs.tar.xz.
Its incomplete parent result and two observed failures remain open. A first
detached driver likewise had no terminal result and remains separately retained.
Do not automatically repeat these attempts on the unchanged host. The
complete original suite is /tmp/crypto-native-lib-tests-final.log. Later failed
checks are /tmp/crypto-halftrace-followup-*.log. The two raw development archives
are ignored locally pending authorized artifact-store upload, with separate hashes.

## Stop and success conditions
Stop before scientific launch if checkpoint upload fails, source identity is
unverified or the prospective protocol is incomplete. A test pass, image build,
checkpoint receipt or merged PR is not a speedup. Success requires the saved
matched native comparison with cold setup, verified scalars, controls and all
measurement admission gates; report regressions and invalid results as observed.
