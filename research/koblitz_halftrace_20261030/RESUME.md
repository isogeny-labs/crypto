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
started, and no speedup is claimed. Auto-merge is paused for the unresolved P1
requirement to retain a matched native comparison. Local checkpoint transport
has no credentials. Checkpoint-only Actions run41082358257 at preparation
source1e6c9a434058f274ddb559c72a81bc279ec3f1cf failed because its approved upload
secret is not configured; all build/experiment jobs were skipped. No checkpoint
was uploaded. protocol.json is prospectively frozen with canonical hash
e8e527aacb96a7e172069353254b47bce33526da7111f913b67bcdc4648c0d07.

## Next steps
Attach an existing approved checkpoint/lab connection or configure the approved
Actions upload secret. Do not repeat the failed preparation until its cause has
changed. A fresh successful source-pinned checkpoint is required before dispatching
the trusted main-branch isolab-experiment.yml with the tested source SHA and this
unchanged spec. Retain original workflow identity, image and binary hashes, L2/A/A
admission, S3 manifests, independent replay, all failures and the canonical export.
Update this PR's report, request resolution of the P1 only with actual evidence,
and verify current-head gate/size/artifacts checks before merge.

## Runs in flight
None. The preparation is terminal failed. No scientific EKS request, Job UID,
data export or result receipt exists. Local development checks have finished;
complete original suite is /tmp/crypto-native-lib-tests-final.log. Later failed
checks are /tmp/crypto-halftrace-followup-*.log. The two raw development archives
are ignored locally pending authorized artifact-store upload, with separate hashes.

## Stop and success conditions
Stop before scientific launch if checkpoint upload fails, source identity is
unverified or the prospective protocol is incomplete. A test pass, image build,
checkpoint receipt or merged PR is not a speedup. Success requires the saved
matched native comparison with cold setup, verified scalars, controls and all
measurement admission gates; report regressions and invalid results as observed.
