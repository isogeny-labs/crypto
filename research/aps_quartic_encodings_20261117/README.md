# Matched Expanded4 and Split4 finite correctness

Executed source [b0919b72](https://github.com/isogeny-labs/crypto/commit/b0919b72b1688dbd9983381470d23d1d81b5fb05) preserves the prior nineteen disjoint label cells and compares existing native resultant and split S3/S3 encodings. The root library and root lock are unchanged from the frozen baseline87158ff6; this is not a latest-main integration test. The earlier PR4336 was externally merged asde8cdfac; this follow-up does not claim its broader research objective is complete.

Native statement `PCC-20261117-f0f239` pins the literal [report](report.json), transport control, source and pre/post custody and [validation receipt](validation.json). Standing is **unverified**, native `not_covered`, exit2. Unsigned artifacts are not execution proof [pcc:unverified]. No private native ledger is published.

## Observations

The process exited0: all8 planned arms were attempted and completed, none failed/incomplete or excluded by rule. Two sampler streams use the same one public known-answer F101/order107 target37G; encodings Expanded4/Split4 and cold/shared routes each visit19 cells and retain all70 tuples including22 uncertain ones. Every complete projected root set matched the separate ordinary-affine oracle and recovered37 with a separate affine multiplication check [pcc:unverified].

Distinct root counts were7 for seed20261115 and5 for seed20261116 in all four matched routes. Full native witness counts were also7/5 in this panel; every Split4 witness retained its fifth unrestricted auxiliary coordinate before projection. No assumption that these multiplicities agree generally is used [pcc:unverified].

| Seed | Encoding | Accepted trace replays | Certification multiplications | Certification pairs | Certification reductions |
| --- | --- | ---: | ---: | ---: | ---: |
| 20261115 | Expanded4 shared | 16 | 1676 | 24 | 81 |
| 20261115 | Split4 shared | 17 | 230 | 42 | 112 |
| 20261116 | Expanded4 shared | 19 | 2223 | 29 | 104 |
| 20261116 | Split4 shared | 14 | 257 | 41 | 111 |

These are the native certification counters, not complete F4 operation counts, wall times or GAE. Split4 uses fewer recorded certification multiplications here but more pairs and reductions; the table establishes no runtime or total-cost ranking. There were no natural cache rejections/fallbacks on this panel [pcc:unverified].

## Requirements and evidence

| Requirement | Status and evidence |
| --- | --- |
| Identical cells/seeds/target | implemented, unverified; source sampler/cell planner and literal arm keys |
| Both encodings/cache modes | implemented, unverified;8/8 attempted and completed, no exclusions |
| Complete roots/recovery | implemented, unverified; per-cell expected/observed plus union and separate multiplication |
| Auxiliary treatment | implemented, unverified; fifth domain full field, full raw witnesses retained |
| Failure retention/output ownership | implemented, unverified; actual-wrapper transport control before scientific call; guards exercise allthree binaries |
| Certification comparison | implemented, unverified; named native counters above, unpriced |
| Canonical unified export validation | blocked; canonical skill/schema files are encrypted VLT1 blobs through available access; connector failed too |
| Runtime/scaling/complete costs | not attempted; prospective checkpoint/accounting/export/protocol admission remain open |

Flow: planned8, attempted8, completed8, failed/incomplete0, excluded by rule0. This is the scientific call only. The separate [transport control](transport-control.json) intentionally reports8 failed mock arms and0 completed solves, retains every later planned arm, and exits0 for transport integrity. It calls no solver and is not counted as scientific evidence. Its output path belongs exclusively to the native worker; stdout/stderr were captured separately [pcc:unverified].

## Reproduction and boundaries

The [fixed plan](protocol.md) was copied from the source design before dispatch. Cooperative per-cell limit2000ms, degree24, pool8entries/100000words, one Rayon thread and420second process limit. No favorable retry. All package bytes matched their Git blobs; source/build/pre/post manifests are separate and the qualifier copy was readonly and hashed before calls. Complete literal output remains intact. No raw dependency/host/build log or protected shared ledger/history is published.

Rust1.98 package tests passed10library+2CLI checks, with each CLI guard testing allthree executables. Strict Clippy/format passed1.98/1.99; pinned-source release build and both native calls exited0. First strict Clippy failed on tuple return complexity; it was replaced with named VisitResult without suppression or assertion changes. No1.99 runtime suite, root-library suite or latest-main validation is claimed [pcc:unverified].

Sources checked: APS author manuscript section3.4 through [Unitn](https://iris.unitn.it/handle/11572/202653), existing native build_system and exact-domain visitor, earlier frozen quartic evidence. This tests the paper's split form against the existing resultant on the same finite cells, not a new summation construction. KB tools/catalog unavailable; no novelty/absence claim. The canonical experiment-protocol pointer was followed, but its referenced skill/schema bodies were encrypted and unreadable. No invented conforming experiment.export/v1 is substituted; native data/flow is preserved for later validated export.

Next bounded discriminating work is varied label profiles and domain sizes while retaining complete affine comparisons, followed by full cost instrumentation before any performance comparison. Ledger publication/reconciliation, checkpoint readback and broader T9/T13/T10 obligations remain open.
