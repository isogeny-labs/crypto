# Complete disjoint quartic label cells

Executed source [f6ca72f9](https://github.com/isogeny-labs/crypto/commit/f6ca72f9d11d80d4ea9aaf7cfd124a189bb3094b) connects disjoint label-profile cells to existing native `Expanded4`, exact-domain F4 enumeration and scalar recovery. The ordinary-affine oracle separately checks all unordered quadruples. No production solver/assertion was changed.

Native statement `PCC-20261116-ff48d3` pins the complete [literal report](report.json), source and pre/post-call custody, and [validation](validation.json). Standing is **unverified**: native verification classifies unsigned artifacts as `not_covered`, exit2 [pcc:unverified]. The private native ledger remains unpublished; this PR does not resolve its owner-held publication/reconciliation question.

## Fixed design

The source [plan](../../tools/aps-partition-bridge/QUARTIC.md) and exact [pre-call copy](protocol.md) fix the fixture, seeds, cell construction, controls and limits. There is one public p101/order107 known-answer target37G and two sampler streams, not two independent targets. Each stream contains two points of each absolute B label1/2/4/8. Repeated labels become ordered singleton coordinate slots; unique labels retain both choices.

All label multiplicities0/1/2 summing to4 are retained. Complete coordinate-product coverage is compared with a separate combination traversal. Missing cells, duplicate cells and overlapping coordinate slots are negative controls. Both cold and shared-cache routes visit every cell; incomplete/error outcomes cannot pass. The process limit is420seconds, each cooperative cell budget2000ms, degree24, one Rayon thread. There is no favorable retry.

## Actual observations

The dedicated process exited0. Each stream had19 disjoint cells covering70 quadruples;9 valuation-safe cells cover48 tuples, while10 uncertain cells retain22 tuples. Both native routes returned complete root sets equal to the separate affine oracle and recovered37, checked with separate affine multiplication [pcc:unverified].

| Seed | Distinct root quadruples | Informative incidents in safe cells | Informative incidents in uncertain cells | B=0 group incidents |
| --- | ---: | ---: | ---: | ---: |
| 20261115 | 7 | 3 | 3 | 1 |
| 20261116 | 5 | 3 | 2 | 0 |

The table uses one root set per seed; cold/shared duplicate results are not pooled as additional evidence. Every root occurs in one cell. Group-sign incidents and root counts are different objects. Uncertain cells contain useful relations and are not discarded. The valuation predicate guarantees a nonzero signed B only for the admitted profiles; it does not guarantee that a group relation exists [pcc:unverified].

Shared pools accepted16 and19 replay attempts respectively; no rejection or fallback occurred. Counters include certification work but are not complete costs. These observations do not measure a cache gain, runtime or scaling [pcc:unverified].

## Checks, custody and limits

Rust1.98 package checks passed8library and2CLI tests; each CLI guard exercises both binaries. Strict all-target Clippy and format passed under1.98/1.99. The source-pinned release build and dedicated call exited0. No1.99 runtime suite or root-library suite was run [pcc:unverified].

All package files were matched to their published Git blobs before execution. Source/build/pre/post files are distinct; the separately copied qualifier was readonly and hashed before the call. Literal output is retained intact. The first pinned-build setup failure is retained privately: sparse checkout omitted the new package, then exact published blobs were restored; no solver source changed. Detailed host/dependency/build logs remain private.

The prior triple and tuple-only reports remain frozen under their own source boundaries. This checks `Expanded4`; APS section3.4 used split S3/S3. A matched `Split4` correctness comparison is the next bounded encoding control, followed by broader coefficient profiles. Performance comparisons remain subject to their prospective checkpoint, complete accounting/replay/export and frozen admission prerequisites.

Sources checked (KB): KB tools unavailable and wrapper absent in this sparse runtime. [APS author manuscript](https://iris.unitn.it/handle/11572/202653), sections3.1/3.4, was read through its open author-PDF link after Oxford/ePrint403 errors. It motivates disjoint domains and identifies split4 as a distinct comparison. Semaev ePrint2004/031 remained inaccessible. No literature-absence/novelty claim. The guessed arXiv identifier was unrelated and was rejected as a source. Native references: `aps_single_relation::build_system` and `f4_fp::solve_visit`.
