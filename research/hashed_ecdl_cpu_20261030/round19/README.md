# CPU arithmetic comparison, round19

Frozen before submission. The single authorized job expired without a node
assignment: zero benchmark executions and no measurement rows or certificates.
The terminal outcome is retained in outcome.json and terminal-status.json.

The protocol keeps the original baseline, all three complete fixtures, one
software thread, all setup and discarded trials, reduction, both independent
replays and serialization. There are six arms, three fixtures, one warmup and
nine measured repetitions per cell: 180 executions if admitted.

| Arm | Effective implementation | Batch |
| --- | --- | --- |
| u | Original baseline 2bd4588fba603ba1787e1d8ad82f084ef461d7cf | 64 |
| x | Identical baseline, A/A duplicate | 64 |
| v | Integrated 5cb7a4072078: cached-affine FixedPair and projective relation checks | 64 |
| w | Same candidate executable | 1 |
| y | Parent fd4b1ec078fa: cached-affine FixedPair and affine relation checks | 64 |
| z | Same parent, with only cached-affine FixedPair branches disabled by the retained patch | 64 |

The z control is explicitly **not** an integrated repository revision. Its
parent, exact patch, effective source hash and executable hash are pinned in
inverse-materialization.json. The historical basename `inverse` is used by the
runner; it identifies z, not an implementation that disables inverse caching.
All other source inputs match y. This prevents the upstream arithmetic changes
between historical revisions from confounding the z/y cached-affine comparison.
The y/v comparison changes relation-sum verification; w/v varies batching.

## Readiness evidence

- Both matched parent variants pass 39 standalone release library tests,
  including all three archived complete-certificate fixtures.
- Candidate software validation (40 tests, strict Clippy, formatting and build)
  is retained in ../projective-relations/; candidate was rebuilt with locked
  dependencies for this bundle.
- The patched control passes strict all-target release Clippy and formatting.
- Six synthetic analyzer tests and three synthetic export tests pass. These
  are packaging checks and do not execute the performance workload.
- Protocol validation reports zero findings. All five bundle executables
  pass SHA-256 readback verification.

Protocol SHA-256:
`3a24606b951d7da114f0d933876253c6625e8cb57dfd977a31a6dbafac2ca8e4`.
The frozen record uses L0 timings as diagnostics; it cannot establish the
10x performance acceptance threshold. No rows from prior panels are pooled.

## Custody and execution

The local bundle is `/home/ec2-user/hashed-ecdl-cpu-round19/bundle`.
Its executables are not committed to Git. build.json and the three
materialization files bind their sources and hashes. Rebuild y from the named
parent using the manifest command; rebuild z from that same source after
applying uncached-fixed-pair.patch. Rebuild v/w from the candidate commit.
Preserve the original baseline executable and its existing provenance.

The runner requires EPHEMERAL_OUT, verifies executable hashes before any cell,
and uses a fixed six-arm rotation with alternating reversal. Each cell has a
60-second timeout; the intended job has a 300-second deadline, 2 vCPUs,
4 GiB memory and one attempt. Do not silently retry or replace a frozen binary.
Check the exact intended job name before any later submission to prevent
duplicate execution. Export only after all cells and returned manifests pass.

The single bounded attempt was submitted at 2026-10-30 21:32:54 UTC as
hashed-ecdl-cpu-round19-20261030, UID e8ae4f11-53d1-4994-b2c0-a858eafc2417.
It reached DeadlineExceeded at 21:37:54 UTC and terminal Failed at 21:37:55.
The pod remained Pending without a node: one current node conflicted with pod
anti-affinity, two with node affinity/selector, and two with untolerated taints.
No retry or deadline extension was submitted. The 180 planned executions did
not start; this panel is invalid under its frozen missing-cell/deadline rule.

The native runner and binary bytes were unchanged. Thin shell orchestration
used the image's AWS CLI and its existing scoped job role to download inputs
and upload outputs, with AWS_MAX_ATTEMPTS=1. submission-request.json retains
the exact argv and envelope. staged-input.sha256 binds every uploaded file.
terminal-s3-list.json records no output objects; terminal-logs.json contains
empty logs. There is no success export, timing comparison or certificate claim.
The separately published report is ../reports/round19/report.pdf.

## References and handoff

The preceding round18 protocol, retained profiles, cached-affine rationale and
projective-relations rationale were reviewed. The arithmetic uses the standard
[EFD Jacobian coordinate identity](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html).
No new group formula is introduced. T-1014 owns this preparation directory;
it does not reserve the shared implementation or the future report directory.
T-1019 owns the subsequent single execution and report. The initial draft
validator rejected a missing freeze block; that schema issue
was corrected before freezing. The committed protocol passes validation.

## Native validation/export migration

PR2980 review identified that the original synthetic tests were implemented in
Python. The active path is now the Rust crate documented in
../native-panel-tools/README.md. Its eleven synthetic checks pass for this panel;
the six original analyzer cases and three original export cases are retained,
with stale-PASS and analyzer-source-mismatch checks added. Original Python files
are preserved under ../native-panel-tools/legacy/round19/ as historical source.
The frozen analyzer, protocol, build/source manifests and binary bytes are
unchanged. Local bundle executable permissions were corrected after the first
native test attempt exposed that copying had stripped them.
