# Deterministic operation vectors: pass, no tenfold conclusion

Once-only execution at pushed40e9184bd4e exited0. All108solves (18publicworkloads ×3methods ×2repeats) verified their recovered scalar against the original source/target inside the solver; driver expectedscalar agrees. All54pairedoperationvectors match exactly, including bothmoduli. All108percall REDC identities pass in eachmodulus. Instrumented source reverses byte-for-byte to the original native implementation after deleting taggedcounterlines/moduledeclaration. No arithmetic changes or timingcollected.

The following ranges are across six distinct source/target workloads pergenus, not pooledmeans. Ratio=controlcount/Pencilcount for the stated singlecounter; these are NOT runtime speedups or a weighted totalcost model.

|Genus|Base-field REDC Folded/Pencil|Base-field inverse calls Folded/Pencil|Subgroup REDC Folded/Pencil|
|---|---|---|---|
|2|1.057281–1.267375|0.787356–1.130435|0.900990–2.948980|
|3|1.873001–5.223228|1.459057–4.981818|1.382671–2.498054|
|4|2.100644–4.157634|1.394366–3.639881|1.446328–1.815618|

Expanded and Folded have identical basefieldcounts because their walk/oracle are identical; they differ in subgroupLA. Expanded/Pencil subgroupinverse ratios reach about8.03/11.23/11.60 for genera2/3/4, but selecting that component would hidebasefieldwork and the strongerFoldedcontrol. The fullvector is retained in CONTROL.tsv; SUMMARY.tsv gives perworkload ratios for selected diagnostic dimensions without aggregation. SomePencilgenus2casesuse morebasefieldinversions and subgroupreductions thanFolded. These vectors provide no tenfoldcompletecost evidence.

## What is and is not counted

Counters cover allcalls during the providedvalidpublicinstance algorithm interval, including freshfactorbase/actioncertificate/maps/collection/LA/finalverification. Commonfixturequalification and publictargetconstruction excluded equally. Counts are partitioned byactualbasefield/subgroupmodulus. Everycounter includes nestedcalls: do notsumallcolumns. Rawpolyconvolutionproducts are counted separately because they bypass Fp::mul. REDC=2reduce128+pre+mul_pre is checked for everymodulus. Euclideaninverseiterations countintegerdivisionsteps, notcycles.

This is NOT completehardwareinstruction or primaryoperation qualification. Fpinitialization internalintegerproducts/remainders, allocation/copy/heaptraffic/sorting, branches, RNG, polynomialbookkeeping and otherintegerwork are notfullycounted. Counteroverhead is intrusive, so executionduration mustnotbeused as performance evidence. No conversionweights were fitted. The earlierwallcomparison failednoisegates and remainsunqualified; deterministicvectors do not retroactively validate it.

## Consequence for the next mechanism

The knownsieve reduces basefieldreductionwork modestly in genus2 and more in genera3/4; it cannot be promoted as the requestednewtenfoldbreakthrough. A targetedphasebreakdown can separate invariantfactorbase/characterpreparation from peranchorwork before selecting anothermechanism.

Batch inversion is an available classical optimization: for m nonzero denominators d_i, prefixproducts followed by oneinverse and a backwardpass recover allinverses using atmost3(m−1)fieldmultiplications and oneinverse. This can replace perpoint inversions in ananchor scan; it does not remove evaluation/bucket/anchor/LA/setupwork and is not a mathematicalnovelty claim. A prospective allinput-equivalence/costgate would be needed; do not infer10x from inversioncount alone. Newrelation/anchor-sharing ideas still require independent rank and completecost arguments.

Raw CONTROL/FIXTURES/RUN/EXIT files and OUTPUT.sha256 retain the frozenrun. summarize.rs is a postrunRustanalysishelper; no runtime source or gate changed. Allrawhashes verified. Gitcustody whileS3unavailable; no uploadclaim, unchangedrerun or scientificadmission. Broadobjective remainsopen.
