# Four APS leads run in parallel

Executed source [98f09378](https://github.com/isogeny-labs/crypto/commit/98f093787ec3fc4c0277cf7ae9f058b0b5b98c07) adds four separate native Rust lead modules. Four delegated workers implemented isolated paths; reciprocal read-only review found a cell-error retention gap, fixed with additional controls before the combined source was pinned. No production solver, root lock or existing workflow assertion changed. This uses the frozen fd2f001a root baseline, not a latest-main integration test.

Native statement `PCC-20261117-908e98` pins all four literal reports, source/pre/post custody, transport control and validation. Standing is **unverified**, native classification `not_covered`, exit2; delegation, checks and producer hashes are not signed verification [pcc:unverified]. Full native ledger publication/reconciliation remains held.

## Cohort, limits and flow

The [fixed plan](protocol.md) predates dispatch. Each worker reserved its own create_new output before a common start barrier was released. Four reserved outputs were observed before release; all four native processes exited0. Flow: planned4, attempted4, completed4, failed/incomplete0, excluded by rule0. This is producer custody, not execution attestation or a timing comparison [pcc:unverified].

Each worker had420seconds including at most30seconds waiting for release, one Rayon thread, native F4 cooperative2000ms/cell, degree24 and8entry/100000word caches where used. Stdout/stderr were separate from worker-owned raw files. The actual-wrapper [transport control](transport-control.json) ran first, deliberately retained all four mock failures and called no solver. No favorable retry, profiler, elapsed-time ranking or performance/EKS run occurred.

All leads use one public F101/order107 known-answer target37G and seeds20261115/20261116. Two streams are not two independent targets. Label-law candidates generate fresh valid points; the other leads preserve the nineteen disjoint cells and all seventy quadruples, including uncertain cells.

## Lead1: constrain the auxiliary coordinate

[Literal output](aux_domains.json) retains exact left/right pair-sum supports, their intersection, the added auxiliary vanishing polynomial, full native witnesses, direct joins and the affine reference. Constrained and full-field Split4 root sets matched both references for all cells. Intersections were empty in12/19 and14/19 cells; each nonempty intersection had one abscissa on this fixture [pcc:unverified].

Empty support explicitly avoided a constrained solver call. This required constructing pair sums; the direct pair join already found the same roots. Construction additions and join matches are recorded but inversions, memory and total arithmetic are unpriced. It is therefore not a demonstrated improvement over direct joining [pcc:unverified].

## Lead2: change the label law

[Literal output](label_design.json) retains fresh valid uG+vQ points, every draw/canonical label, all tuples and all sixteen signed B sums. Both laws share the same raw u stream, but rejection and point coordinates can differ. The comparison does not hold coordinate sets fixed.

| Seed | Absolute weights | Exact sign-safe tuples | Valuation-safe tuples | Informative group incidents | B=0 group incidents |
| --- | --- | ---: | ---: | ---: | ---: |
| 20261115 | 1/2/4/8 | 56 | 48 | 4 | 6 |
| 20261115 | 1/3/9/27 | 64 | 0 | 2 | 6 |
| 20261116 | 1/2/4/8 | 56 | 48 | 4 | 6 |
| 20261116 | 1/3/9/27 | 64 | 0 | 4 | 3 |

All rows retained70 tuples and checked native recovery by separate affine multiplication. Exact label safety improved on this finite profile, while relation yield did not consistently improve. The existing 2-adic sufficient predicate admitted no all-odd candidate tuples; this does not contradict exact sign safety. Uncertain tuples and repeated-label cancellations remain. No polynomial solver was tested for this new law, and no general yield conclusion follows [pcc:unverified].

## Lead3: reduce in the coordinate quotient

[Literal output](quotient.json) retains original/remainder polynomials and every Cartesian input value. The original native equation had439 terms; its scalar-preserving remainder had0to15 terms across these cells, with degree at most1 in each non-singleton coordinate and degree0 in singleton coordinates. Original/reduced F4 and direct evaluation matched the complete affine roots [pcc:unverified].

The domain equations remained. Rewriting work and separate original/remainder verification evaluations are recorded. Tiny cell products also admit direct enumeration, so polynomial shrinkage alone establishes no total-cost or runtime gain. Zero remainders, nonzero constants, invalid domains and expired budgets have explicit controls [pcc:unverified].

## Lead4: choose the pairing per cell

[Literal output](pairings.json) retains all three pairings under cold/shared routes plus a distinct actual selected-cold route. The selector uses only the predeclared domain-size score and fixed tie order. All14 routes completed; selected-route roots matched the independent oracle and cold-control reconstruction, with scalar recovery checked separately [pcc:unverified].

Pairing changed native certification diagnostics. In seed20261116, pairing2 had one rejected replay and one fallback; the completed route still matched the oracle. Its certification counters cannot establish total savings because rebuilding work is not fully priced. Feature validation, comparisons and six score multiplications/three additions per cell are recorded as partial diagnostics [pcc:unverified].

## Requirement status and validation

| Requirement | Status/evidence |
| --- | --- |
| Four leads in parallel | implemented, unverified; four delegated implementations and four native processes behind common barrier |
| Exact auxiliary polynomial/domain | implemented, unverified; complete roots agree with join/reference |
| Fresh valid labels and cancellation controls | implemented, unverified; full draw/sign/tuple records and cap controls |
| Scalar-preserving quotient reduction | implemented, unverified; every Cartesian value equal, both F4 routes checked |
| Input-only per-cell pairing | implemented, unverified; all controls plus distinct selected route |
| Failure retention | implemented, unverified; seed/cell/route errors continue later work; partial/capped controls retained |
| Canonical unified export | blocked; referenced canonical schema bodies unavailable as readable plaintext, native full data/flow retained |
| Timing/scaling/full cost | not attempted; prospective checkpoint, full accounting/replay/export and frozen admission prerequisites open |

[Validation](validation.json): final Rust1.98 package24library+2CLI tests, each CLI guard covering all four binaries; strict Clippy/format1.98/1.99, prose, pinned release build and transport control passed. No1.99 runtime suite, root full-library suite or current-main pass is claimed. Initial prose-lint failure and pre-hardening checks retain their own boundaries in private logs [pcc:unverified].

Every package file matched its Git blob before the call. Source/build/pre/post manifests are distinct and the qualifier was copied readonly and hashed before dispatch. Literal reports retain every field/case without normalization or case removal. Detailed host/dependency/build logs remain private; no protected ledger/history/shared log is published.

Sources checked: APS author manuscript sections3.1/3.4 ([Unitn](https://iris.unitn.it/handle/11572/202653)), existing native builder/visitor and frozen earlier studies; bounded label law relates to [distinct subset sums](https://www.combinatorics.org/volume_5/pdf/v5i1r3.pdf) but repeated-label coefficients require stronger care. No new summation construction/novelty claim. KB tools/catalog unavailable; canonical protocol/schema access remains blocked by encrypted VLT1 bodies. No conforming experiment.export/v1 is invented or claimed. Broader T9/T13/T10 and ledger/checkpoint obligations remain open.
