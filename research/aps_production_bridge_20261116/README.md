# APS production partition bridge: fixed structural call

Executed source [2a9ae86c](https://github.com/isogeny-labs/crypto/commit/2a9ae86cb2988b66a5598890dcb7630485e4c33a) links the real native APS sampler, equation builder, exact-domain F4 visitor and recovery implementation. The separate ordinary-affine oracle supplies a root-set reference. This report is bounded correctness evidence, not timing or a performance ranking.

Native statement `PCC-20261116-3b5b89` pins the complete literal [report](report.json), source and pre/post-call custody, and validation receipt. Standing is **unverified**; native classification is `not_covered`, exit2. Unsigned artifacts and producer pins are not execution attestations [pcc:unverified]. The full native ledger remains private under the existing publication/reconciliation hold.

## Fixed panel and outcomes

The [protocol](protocol.md) and [tool design](../../tools/aps-partition-bridge/README.md) were fixed before the call. Both sampler streams use the same one known-answer target on F101 with subgroup order107, rather than two independent targets. Each stream tests eight colorings and three routes: cold, shared trace pool and support/degree signature buckets. Every represented cell is searched; root duplicates are removed only after solving.

The native process exited0. Each stream had15 nonempty cells per route; the six routes completed and equaled the separate affine oracle, with two represented root triples per stream, no omitted oracle roots, and scalar37 independently checked. These are producer observations [pcc:unverified].

The shared route accepted11 replay attempts for the first stream and12 for the second. Its signature-bucketed counterpart produced the same diagnostics: every specialized polynomial had the same signature, leaving one bucket. This fixture therefore supplies no evidence that this signature separates reuse families. It also produced no natural pool rejection or fallback; the raw trace damage control is a different public API test [pcc:unverified].

All27 coefficient-grid systems matched literal roots; damaged raw-trace fingerprints were refused. Certification operation counters remain unpriced diagnostics and cannot establish a speedup [pcc:unverified].

| Fixed sampler seed | All quadruples | Valuation safe | Uncertain | Exact sign-safe | Informative group incidents in uncertain tuples |
| --- | ---: | ---: | ---: | ---: | ---: |
| 20261115 | 70 | 48 | 22 | 56 | 3 |
| 20261116 | 70 | 48 | 22 | 56 | 2 |

The sufficient valuation predicate passed every full sign check on its admitted tuples. Exact sign enumeration is stronger on these small profiles. Uncertain tuples contain useful relations and are retained. Neither label safety nor the table asserts that a group relation exists in every tuple. These are tuple-level even-arity observations; quartic polynomial integration is explicitly false [pcc:unverified].

## Validation and limits

[Validation](validation.json) distinguishes the pre-publication6library+2CLI Rust1.98 checks on identical Rust/lock bytes from the pinned-source release build and dedicated call. Strict Clippy and format checks ran on Rust1.98 and1.99; no1.99 runtime suite or root-library suite is claimed. The first compile failure remains in private logs.

The literal report is retained byte-for-byte, without case filtering. Source/build/pre/post manifests are separate; qualifier and source hashes were captured before the call. Detailed build logs and host data remain private. The independent arithmetic is source-separated, not a signed independent execution verifier.

The next discriminating correctness work is to turn the label strata into explicit disjoint quartic cells and compare complete represented sets, preserving uncertain cells. For cache grouping, use varied systems with matched leading-monomial profiles and retain rejection/certification work; support/degree alone did not distinguish this panel. Any performance comparison still requires its prospective checkpoint, complete costs/replay/export and frozen admitted protocol.

Sources checked: existing production APS/F4 APIs; the prior partition study [source review](../aps_partition_structural_20261115/README.md); bridge README derivation and controls. No new novelty claim or change to production assertions. KB unavailable in this runtime; no literature absence claim.
