# SAT initial capture and historical replay

This component preserves the original producer budget and deadline through map construction, native initial capture, and its acknowledged checkpoint. Historical replay authenticates original inputs, map, transcript and producer ledger under a separate caller-authorized cumulative replay policy. It checks the committed capture endpoint before and after traversal.

Actual-length input admission retains read buffers and partial failures. Structural JSON scanning checks bounds before materialization; moved JSON trees avoid repeated deep copies while preserving the complete public/private output. The retained worker result uses the existing checkpoint acknowledgement.

The composed source passed 33 exact tests, including real n23/m3/factor-base-size4 fixtures on both twists, persisted historical replay, map/transcript mutation controls, endpoint substitution and ABA, retained partial reads, and bounded structural scanning. The historical replay correctness fixture grants a separate 1e12-work/8e9-byte allowance. It does not establish replay fit under the producer limits.

The previous 27-test batch deferred that historical test. Its prior cap-based deferral rationale did not apply to the test's separate replay policy; the complete 33-test run now executes and passes it.

Full release-library validation is pending. This code supplies internal components; SAT CLI dispatch and causal export are still incomplete. An initial-prefix check is not whole-search completion or an exact fall-degree measurement.

Terminal focused logs, receipts, source manifest and scope correction are archived in the URI recorded in `s3.receipt.json`, with fresh download, byte equality and contained-hash verification. No new scientific cloud job or campaign was launched.

The full requested dataset scope and remaining solver/population gaps are tracked separately in [the coverage audit](../coverage-audit-20261103/REQUIREMENTS.md). This component does not replace those requirements.
