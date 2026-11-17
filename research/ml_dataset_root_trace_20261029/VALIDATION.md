# Prime root integration validation checkpoint

Validated source: `6542d87e00902c0a4219db7c4a3812ccc9c02bf3`, incorporating main `70fc911961de46ad73947930ef529f6f70d322c9`. This checkpoint records completed software checks on that source. Follow-up replay findings in REPLAY-REVIEW.md require corrections and fresh validation before delivery.

The retained prime root engine is integrated into the dataset root adapter and central certificate replay. The module is unchanged from the donor, SHA-256 `c7afd9b5f9219336d5abcabdd067ca680c1bfefeb10177c6405e787794b8d0ac`. The root adapter retains polynomial, root and bounded event artifacts; central replay rejects inconsistent certificate roles, domains and outcome labels.

## Completed checks

- Release compilation and executable build passed using Rust 1.98 and the locked dependency set.
- Focused root checks: 18 passed, zero failed.
- Unfiltered release library suite: 5,259 passed, zero failed, 123 ignored; exit 0 after 1,554.84 seconds.
- `scripts/clippy-maintained.sh`: exit 0.
- Frozen shell CLI fixture: all 10 commands passed, covering plan/preflight and generate/verify/status/export for both twists.
- Both downloaded fixture datasets passed native verification using the downloaded executable; both verification outputs were byte-identical to their originals.

CLI SHA-256: `73650325124a9519fe395df658af7c56fb7b4b169aebde7e70550b580566d241`.

Archive: `s3://setec-astronomy-artifacts/ml-dataset/20261029/software/root-integration-cli/00be8854f73352162676041b95b2f6c63e5daceebaf729fd858ef61c08619df6/cli.tar.gz` (14,855,245 bytes). Conditional PUT, fresh HEAD and ETag-bound GET, lengths, archive bytes, archive SHA-256 and contained file hashes passed. The controller archive contains the executable, fixture datasets, command logs, manifests, restricted certificates and exports. This is not worker S3 publication evidence.

The two bounded n=7, m=2 fixture cells are software fixtures, not scientific corpus cells. The untwisted cell verified a PDP witness; the twisted cell remained partial without a PDP witness. Both completed their bounded root solver. Neither independently recovered an ECDLP scalar.

## Preserved failures and limits

The first focused run had 17 passes and one fixture-precondition failure: an order-three point doubled to the same x coordinate. The fixture was corrected to n=11. A subsequent focused run had 17 passes and one incorrect test expectation: the final stream record is a solver-return error after the last native event. The corrected assertion verifies both records and absence of a successful certificate/witness. These are preserved failed attempts, not successes. Sparse checkout omitted tracked build fixtures; fixed-commit restoration and hashes are recorded separately.

Source and failed-check checkpoint: `s3://setec-astronomy-artifacts/ml-dataset/20261029/checkpoints/root-integration-corrected/8bc71e07845d090b766f969158e2058337b1fcabf790afe556e01aee87538cde/checkpoint.tar.gz`.

The unfiltered release library suite completed within its original 3,600-second timeout. No duplicate suite was launched during reconciliation. Parent source review completed; peer design review completed, but the peer final combined-source review was interrupted by its usage limit.

Root-set completeness does not certify an entire execution trajectory or total operation count. Both completeness flags remain false. Native arithmetic counters are not retired-instruction measurements. The exporter retains the certificate in its restricted artifact index; the dedicated solution-certificates label hook remains separately pending. Binary root integration, higher-m integration and the full corpus remain unfinished. No first/last-fall theorem, unbounded degree result or speedup follows from these software checks.

## Migration reconciliation

The approved handoff receipt was read using scoped read-only access. Managed Diem remains deadline-expired at 50/52 records. The saved freeze continuation requires 52 durable records, successful generation and no active generator; these prerequisites remain unsatisfied. Its waiter and generator were not restarted.

The historical EKS pilot receipt records BackoffLimitExceeded; a fresh status lookup on October 29 returns NotFound. No replacement job was submitted. Conductor T803 and T805 leases remain active while validation runs. The original USD200 total cap and deadline remain unchanged; no billing reconciliation or additional budget is asserted.
