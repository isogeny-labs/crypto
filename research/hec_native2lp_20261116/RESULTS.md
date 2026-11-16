# Native signed two-large-prime graph integration passes genera2,3,4controls

The graph combiner processes9000plantedconstraints across three primes and three seeds. All8712emittedsmall-base rows vanish on their known witness, and every vertex affine identity is checked after every input edge. Directed controls exercise balanced cycles, an unbalanced anchor, an anchored cycle, one/zeroLPand nonunit/signedcoefficients; a corrupted relation produces a nonzero witness residual and is rejected. Graph gate exits0/stderrempty; freeze0ef6b25b0942/checkpoint20261116T041713Z precedes one execution.

Integration then replaces only the dense partial-elimination step in the existing bounded native large_prime.rsbaseline. Full catalogue construction, order/cofactorprojection, walk, oracle, decomposition and final scalar verification remain. Each touched potential is checked against projected actual divisor classes and every emitted full row against projected smallplaces/D1/D2. All12fixed curve cases pass, exit0/stderrempty. Freezea9fd49abdc48/checkpoint20261116T042221Z precedes one cohort. No cases dropped/rerun.

| Genus | p | c | ell | Smallbase | Seed | 1LP trials | 2LP trials |
|---|---:|---:|---:|---:|---:|---:|---:|
|2|101|2|4663|23|20260916|103|86|
|2|101|2|4663|23|20260917|157|113|
|3|61|5|124459|16|20260916|421|260|
|3|61|5|124459|16|20260917|311|231|
|4|31|16|239753|8|20260916|866|536|
|4|31|16|239753|8|20260917|1343|761|

Reference and graph candidate have identical stopping trials, row counts and recovered scalar on every case. The table compares allowedLPcounts as observed in both implementations; it is not a wall-time gain. Scalars are the baseline's internally generated known targets ell/3+7, so these are planted controls rather than arbitrary-public-target recovery. No expected scalar is used in the graph combiner. Native group verification checks the returned answer.

## Algebra and counter boundaries

Potential invariant y_v=c_v*y_root+E_v. Substitution into alpha*y_u+beta*y_v+A=0produces a root coefficient deltaand smallrowR. If delta=0the cycle emitsR=0. Otherwise it anchors y_root=-R/deltaand does not yet emit a full relation. Dummyroot0enforceszero; already anchored cycles emitfullrows. This respects the large-prime sign/multiplicity distinction described in the primary draft; not every ordinary graph cycle is a full relation.

Sparse potential storage is incrementally capped20millionentries, including path compression. Potential fill, row arithmetic and catalogue/projection work still need cost accounting. Curve integration still constructs the baseline's dense input vector; this is not an asymptotically optimized implementation. Candidate projected_checks counts touched-potential/full-row equations, unlike the baseline's one-per-input check; those counts are not directly compared as identical work units. Partial-pivot/report fields similarly have graph interpretations. No timing printed or benchmark claimed.

## Correction and next work

The initial source audit missed the already existing native0/1/2LPboundedmodule; AMENDMENT-BASELINE.mdcorrects that before curve execution. New work is a signed graph alternative to its dense elimination, not the first native2LPalgorithm. Initial integration build had misplaced inner-doc comments, fixed before successful build/freeze/run; log retained.

This returns to the original broader native genus2/3/4plan and supplies a sound reference for future reduced-base/large-field comparisons. Next support arbitrary suppliedD2, remove dense partial-input assembly without weakening identities, freeze resource limits/factor-base policy and compare complete costs against existing native controls and matchedrho. Published2LPcomplexity is not reproduced by these controls. No novelty,10x,globalmaporfullcoldclaim; compiled inverse schedule proof remains open and earlier F631cohorts unchanged.
