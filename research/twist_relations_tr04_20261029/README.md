# TR-04 exhaustive bounded relation finding

**State: implementation/protocol preparation; research execution not yet
reported.** The machine-readable [protocol](protocol.json) is draft until
source/analysis identities, measured development preflight estimate and its
canonical freeze are completed. [PROTOCOL.md](PROTOCOL.md) defines the exact
finite universe and all four requested parts.

[Development validation](DEVELOPMENT.md) records 40 passing native tests,
strict Clippy/formatting and the preserved incomplete root-library attempt.
The fixed development preflight and synthetic report tests are separate from
the primary research census, which has not run.

The study asks whether a cheap public twist-run selector can improve the
independent original-curve relation rank obtained for its paid work. It
compares random, coordinate progression, collision-avoiding and four fixed
twist-threshold selectors, using the same canonical point conventions and
public relation-query populations. The pair-table finder is **generic
decomposition (bounded by sqrt(r); cannot beat rho)**, not an algebraic
elimination or scalar-recovery result.

| Requested work | Finite scope | Current status |
| --- | --- | --- |
| Bounded twist sampling | L=1,2,3,4; eight candidates; B=2,3 | Implemented and development-tested; primary census unrun |
| Matched controls | Random IID, all affine progressions, collision avoidance | Implemented and development-tested; no comparative outcome reported |
| Identical relation finder | Every nonzero Q for canonical G and all 256 binary G/Q words of length eight | Implemented with independent structural replay; primary coverage unverified |
| Paid independent-rank endpoint | Exact unconditional rank/cost vectors, including selector/rank-budget failures | No result, runtime gain or scalar aggregate claimed |

Fields are p=7,11,19, covering all 531 coefficient-model dispositions. Every
eligible nonsingular prime-order model (n>=7) is included. The exact
subset-state DP exhausts the full finite IID stream distribution Fp^8;
progression inputs exhaust Fp x Fp*. An audit prefix trie exhausts all
fixed-length query words. Neither finite claim covers arbitrary field sizes,
all generator policies, unbounded streams or general ECDLP security.

[workflow.svg](workflow.svg) shows the planned execution and verification
flow. [report.typ](report.typ) is the editable pending-execution report source;
a final PDF and recorded-value graphs must be rendered and inspected after
actual outcomes are collected. This document does not substitute planned
panels, local correctness tests or a successful submission for completed data.

The existing registered native workflow is extended with a checkpoint-gated
`tr04` suite and field-sharded matrix on the documented `crypto-eks-linux`
ARC EKS Spot pool. Actual source/workflow/runner identities, checkpoint
receipts, partial records, control results and export flow must be retained.
Runner/secret readiness is unverified until that preflight succeeds. No new
runner, credential, IAM policy, capacity or funding state is introduced.

The experiment is complete only when all three fields, all declared model
and stream populations, the six actual auxiliary controls and independent
structural verification reconcile. The export has 27 logical rows (nine
arms by three field instances); the 21 selector rows retain 42 B-specific
comparison cells separately. Missing or failed rows remain explicit. The
full finite contract and cost-model gaps are in PROTOCOL.md.

Prior work and immutable evidence remain separate:

- [Native TR01-03 package](../../tools/twist-relations/README.md) and
  [backlog card](../../backlog/XP-20261029-twist-factor-base/README.md).
- [PR #2815](https://github.com/isogeny-labs/crypto/pull/2815) implements the
  earlier native diagnostics. Its later complete library validation on
  source `0044aaf9` is recorded separately; it is not a TR-04 experiment.
- [Quartic-twist image-base study](../../backlog/XP-20261023-quartic-twist-image-base/README.md)
  uses different fields, instances and frozen evidence and is not relabelled.

After execution, add the source-linked run report, exact raw/artifact hashes,
`experiment.export/v1`, independent-verification scope, every failed or
censored outcome, quantitative SVG and inspected PDF. Keep every per-field/B/
threshold tradeoff; do not tune a winner after looking at the data.
