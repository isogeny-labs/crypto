# TR-04 exhaustive bounded relation finding

**State: protocol version3 frozen; primary research execution not yet started.**
Published in [PR #2890](https://github.com/isogeny-labs/crypto/pull/2890).
[Workflow attempt 1](reports/run-20261029T233325Z/README.md) failed its
checkpoint gate before any field execution. No relation/rank/work data exists;
all 27 expected research rows remain missing. The separate artifact-routing
review defect is repaired without retrying the workflow.
The active [protocol](protocol.json) was frozen at 2026-10-29T23:15:13Z against
code commit `f270363df37a17b30210a4671cd31c03d3c20b3a`. Its canonical SHA-256 is
`a142ce49dbc23c78d49b10572aaf6228960f43803472abb9e735536677d2c0ce`.
[Earlier filings](versions/README.md) are retained: v1 is unchanged, and the
failed v2 metadata attempt is labelled invalid. Version3 corrects informational
wording/date metadata and adds full schema validation; the numerical experiment
is identical to v1. See [PREFLIGHT.md](PREFLIGHT.md) for development inputs and
planning estimates, and [PROTOCOL.md](PROTOCOL.md) for the finite contract.

[Development validation](DEVELOPMENT.md) records 40 passing native tests,
strict Clippy/formatting and the preserved incomplete root-library attempts.
The final root-library run on code `445d84bf7` completed with **5,389 passed,
zero failed, 123 ignored, zero filtered and Cargo exit 0**. Its
[collected receipt](root-validation-445d84bf7.json) distinguishes the terminal
full summary from two nested one-test summaries. Pre-data filing commits left
those runtime inputs unchanged. The later `f81b1c6c` main integration changes
root-runtime paths and is not covered by that old receipt; no broad rerun was
started. Native code remains unchanged and its 40 tests, strict Clippy and
formatting pass after integration. See
[post-merge validation](postmerge-validation-20261030.json).
The fixed development preflight and synthetic report tests are separate from
the primary research census, which has not run.

Public-input preparation found 79 eligible models across the three fields;
this does not substitute for their primary model/B/selector records. Planning
uses the explicit 3,219-second conservative serial envelope in PREFLIGHT.md,
with the existing 1,800-second hard field cap and no timing guarantee.

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
The actual first preflight found no checkpoint upload transport and failed
closed. Runner/store readiness remains unverified. No new
runner, credential, IAM policy, capacity or funding state is introduced.

The existing store credential bindings are required before primary execution;
their presence alone does not establish remote authorization. Post-analysis
delivery creates a deterministic raw archive, pushes it through the maintained
artifact tool, then downloads it with a fresh cache and compares bytes. Durable
upload/readback receipts and pointer manifests are required for end-to-end
completion, and remain unverified until an actual run performs them. Raw
GitHub records and all failures remain retained separately.

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
