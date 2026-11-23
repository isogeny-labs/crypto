# Deferred signed elimination and shared walk prefixes

## Exact algebra and implementation

PROOF.md derives the complete elimination identity for M=[L A]: rowspace(R)={s:(0,s) in rowspace(M)} and rank(R)=rank(M)-rank(L). It distinguishes emitted equations (which can be zero/dependent) from independent retained constraints. Deferred circuit nodes implement the same signed affine pivots as the eager graph. Forward application evaluates once in topological order; transpose application reverses and accumulates all shared contributions. These are applications of established graph/black-box/circuit-transposition methods, not mathematical novelty claims. SOURCES.md records primary texts and access limits.

The synthetic correctness gate passes36streams over primes3,5,101, seeds1..12, with40prefixes each:1440independent rank/containment checks, exact eager/deferred rows, forward/transpose products, adjoint identities and complete final basis-vector checks. Inputs are arbitrary unplanted relations, so witness residuals alone cannot conceal missing constraints. Directed duplicate endpoints, cancellation, nonunit factors, unbalanced anchoring, anchored cycles, zero retained rows, repeated roots and shared children pass. Exit0, emptystderr. Freezeef4473fcf94a/checkpoint20261116T043823Z precedes its once-only execution.

Constructed fill fixture:512LPvertices,256retained columns,512emitted equations;98431expanded nonzeros versus2560circuitnodes,512leafentries,1535sumnodes. Each implemented forward/transpose call uses at most3582multiplications (=leafentries+2*sums), plus additions, allocations and zeroing. This demonstrates potential on a constructed operator; it is not curve or wall-time evidence and does not establish an optimized explicit baseline comparison.

## Actual curve applicability

Clone the prior supplied-target path and keep catalogue/order projection, exact decomposition, touched-potential and full-row group checks, explicit scalar solver, and final source verification. Instrument ALL emitted roots including zero/dependent equations and constants; verify full operator and transpose on every basis vector. Twelve reference comparisons match scalar, stopping trial, rows, decomposition count and pivot hash. Three additional externally constructed supplied-target controls recover943,24902,47961forg2/3/4and verify in source. Inputs are fixed known-scalar controls, not blinded. Gatebe5e94cd2808/checkpoint20261116T043935Z, once-only, exit0/emptystderr.

The initial deferred representation is unfavorable in every actual case: its multiplication bound exceeds expanded nnz. Cumulative walk coefficients remain dense inside every raw leaf. Preserve that negative applicability evidence.

## Prefix identity and second implementation

At accepted relationi, counts t_i=t_(i-1)+delta_i include every small-base walk step since the preceding accepted relation, including rejected trials. Store W_i=W_(i-1)+leaf(delta_i) and A_i=leaf(decomposition_i,-b_i,-a_i)-W_i. This shares cumulative coefficients without differencing the relation rows. Consequently graph pivots and original solver rows remain identical; this is a representation change, not increased relation rank or yield. Prefix sums/circuit sharing are classical.

The prefix gate repeats the same12reference comparisons and3suppliedtargets, with complete emitted-operator and transpose basis checks: allpass, exit0/emptystderr. Freeze83fa8ce7331f/checkpoint20261116T044149Z precedes one execution. It improves the first circuit's bound on allg2/g3cases, but worsens allg4cases; compared with expanded dot products it is favorable in only4of5g2cases, no g3/g4case. No cases dropped.

## Complete count table

Counts are conservative multiplication units for these concrete operator routines, not wall-time observations. Expanded dot counts one multiplication per nonzero, without coefficient±1specialization. Circuit counts include all dead nodes. Addition, allocation, graph-build, verification, relation search and scalar solve costs are not represented by this table. Row counts include constants and emitted zero/dependent equations, unlike the explicit solve's filtered rows.

|Genus,p|Seed|LP|Expanded nnz|Initial circuit bound|Prefix circuit bound|
|---|---|---|---:|---:|---:|
|2,101|20260916|1|524|911|415|
|2,101|20260916|2|459|904|464|
|2,101|20260917|1|599|1036|455|
|2,101|20260917|2|565|960|461|
|2,101|20260918|2|506|907|457|
|3,61|20260916|1|410|638|472|
|3,61|20260916|2|413|752|490|
|3,61|20260917|1|387|654|435|
|3,61|20260917|2|387|674|434|
|3,61|20260918|2|390|753|484|
|4,31|20260916|1|158|264|333|
|4,31|20260916|2|158|319|365|
|4,31|20260917|1|152|277|380|
|4,31|20260917|2|160|339|426|
|4,31|20260918|2|158|336|378|

## Decision

Do not spend a timing cohort or replace the scalar solver on the strength of the constructed fixture. These actual workloads provide no tenfold operator evidence; prefix sharing alone is insufficient across the requested genera here. A larger-size applicability gate would have to demonstrate expanded fill relative to total stored circuit size before justifying black-box integration. Rectangular/singular solving is unresolved; R^T R is not a sound automatic rank-preserving transformation over finite fields. Full-cost tenfold gains require unchanged work<=10%even if the changed component were eliminated entirely; this is a necessary condition, not measured evidence.

The next mathematical lever must affect relation production/acceptance or substantially change the retained algebra, rather than treating ordinary circuit transposition as a discovery. Existing cold-cost evidence and earlier genus3inversion results remain unchanged. The tenfold objective is open.

## Retained errors and custody

The prefix source-generation marker initially failed before producing its binary source; ensuing build found no binary. BUILD-PREFIX-INITIAL.txt retained. Corrected markers and successful build preceded the freeze; no scientific gate executed during the error. Duke primary PDF403was resolved with the author-hosted NCSU copy. Shared daily log is reserved by another verifying task; SESSION.md records this project's work without editing its file. Raw outputs and three exitstatusfiles are committed; prior S3 checkpoints retain source/binary custody. No timing record was opened because these are exact correctness/representation controls with no reported elapsed-time comparison.
