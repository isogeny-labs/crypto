# Confluent principal relations: algebra, useful rank and capacity

## Algebra established and checked

Implemented homogeneous rational interpolation with local multiplicity conditions, primitive normalization and oriented principal-divisor reconstruction. Gaussian nullspace construction is compared with independent Hermite-CRT/Euclidean reconstruction on nonbranch inputs. Both share field/polynomial and local-series primitives; this is an independent reconstruction algorithm, not an independent curve implementation.

The once-only algebra gate passes600cases:561nonbranchcomparisons,39branchprofiles,47commonfactorcases,49degreedrops,124splitprincipalJacobianidentities,8highernullitiesand3branchfixtures. A corrupt local jet is detected. Nonbranch cancellation leaves max(m-h,0), while branch cancellation leaves max(m-2h,0); directedcontrols distinguish them. Branchpoints use y as uniformizer, not x. Allbranchfixturesareavailable. Exit0/emptystderr, freeze2d6ff1170207/checkpoint20261116T051710Z.

PROOF.md gives exact norm divisibility and residualdegree<=g, including primitive cancellation. Rational/Hermite interpolation and Euclidean reconstruction are classical priorart: D'Andrea/Krick/Szanto, https://arxiv.org/html/1211.6895v3 , sections2/4,theorem4.2. No mathematical originality or speed follows from implementing them.

## Fixed rank screen

Three reused curve/anchor cells: g2p1009c12ell241259,FB478,H8; g3p503c26ell29977319,FB236,H9; g4p127c15ell134781029,FB66,H10. FirstHNONBRANCHcanonicalplaces; exact abscissae in RANK.txt. Distinct versus all-multiset policies share this panel; historical firstHincludingbranches is not silently substituted as a comparator. d0usesM=g+1;d2usesM=g+4. Allmultisetsincludeordinarydistinctprofiles. Samplinghasmultinomialbias as preregistered, not uniformprofile sampling or holdoutselection.

Every constructedfunction passesGaussian/CRT-EEAprimitiveequality andnorm checks. Canonical primitive functions are deduplicated before factoring; normalized rows are separately deduplicated. Fullgroupidentitycheckscovereveryuniqueadmittedprincipalrow. Independent finalGaussianfull/offranksagreeinall20policyrecords. First-completionpositions are recorded by the incremental elimination, not independently replayed prefixrankcertificates. Gate3ec82e6d3468/checkpoint20261116T052001Z, once-only, exit0/emptystderr, all12familyarmscomplete.

## Complete family table

The maximum-arity policy is shown for eachfamily; all20records, including2LPsubsets, remaininRANK.txt. Functioncap4096,profilecap1024anddrawcap100000arefixed. No case dropped. Numbers are deterministic construction/rank diagnostics, not elapsedtimes.

|g,H|d|Profiles|Constructed|Unique primitive functions|First full anchor rank|maxLP|Final rank|Terminal|
|---|---:|---|---:|---:|---:|---:|---:|---|
|2,8|0|distinct|224|224|—|2|0/7|exhausted|
|2,8|0|all|344|338|—|2|0/7|exhausted|
|2,8|2|distinct|896|892|719|2|7/7|exhausted|
|2,8|2|all|4096|4063|749|2|7/7|function cap|
|3,9|0|distinct|1008|989|—|3|0/8|exhausted|
|3,9|0|all|2241|2176|1414|3|8/8|exhausted|
|3,9|2|distinct|2304|2276|1436|3|8/8|exhausted|
|3,9|2|all|4096|4058|1489|3|8/8|function cap|
|4,10|0|distinct|4032|3818|1711|4|9/9|exhausted|
|4,10|0|all|4096|3899|1805|4|9/9|function cap|
|4,10|2|distinct|4096|3798|1350|4|9/9|function cap|
|4,10|2|all|4096|4005|1477|4|9/9|function cap|

Allg3/g4two-LPsubsetsfinishwithzeroretainedanchorrank, evenwhenhigheraritycompletes. Their admissions are respectively11/13/25/53atg3and6/6/14/17atg4in tableorder. More profiles do not remove the need to handle off-anchor support. No noncompleting policy supplies a finite-time speed denominator.

## What the result rules out

Extra repeated support closes one finite family-exhaustion gap (g3,d0), but when both same-dfamilies complete, the mixed all-multiset policy requires MOREconstructedfunctions tofirstcompletion in everyrecorded comparison:749vs719(g2d2),1489vs1436(g3d2),1805vs1711(g4d0),1477vs1350(g4d2). Thed2distinctfamilyalreadycompleteswithsimilarg3constructionwork. These are singlefrozenstreams, not population estimates or a proof that every repeated-support policy is slower.

Most additional primitive functions survive deduplication, so the result is not merely a duplicate-normalization issue. Their useful rank density is insufficient for the proposed tenfold premise. No complete solver or timing campaign is justified by these counts alone. Construction, local jets, Gaussian/Euclidean solves, factoring, sparse elimination, setup and target handling would all need charging in a complete evaluation. Prior strongest complete-control evidence remains unchanged.

## Exact capacity analysis, post-run

For HnonbranchanchorsandtotalmultiplicityM, signed profile capacity is

    sum_r C(H,r) C(M-1,r-1) 2^(r-1),

versus C(H,M)2^(M-1)fordistinctsupport. ExhaustedH8M3gives344vs224;H9M4gives2241vs1008,exactlymatchingthegate. AtfixedM, theirratiois1+M(M-1)/(2H)+O(H^-2). CAPACITY.mdderivesandqualifiesthis: itisnotanindependent-functioncount or a universalalgorithmiclowerbound. Profilecapacityalone supplies no newleadingexponent, and the finite mixed-policy screen supplies no tenfold improvement. A targeted confluent subfamily would require a new constructive density/cost argument; changing the sample until a favorable cell appears is not a successor.

## Decision and custody

Retain the algebra/normalizer and reject promoting indiscriminate multiset expansion as the next tenfold candidate on this evidence. The full mathematical10xobjective remains active; no novelty/exponent/complete-cost assertion. Next research needs a structural useful-relation density mechanism with a cost case against the established complete controls before another prototype, rather than another broad interpolation-family expansion.

Source,lock,nativearithmeticandbinaryhashes committed/checkpoint-gated before eachonce-onlyexecution. Rawoutputs/statusesretained; no build/algebra/scalar/rank/watchdog failure. Importedtablehelperdead-codewarningsretained. Algebra and rank are correctness/count gates; no timingledgerrecord orwalltimeclaim. DailylogreservationT6448preventseditingitsunmergedentry; SESSION.mdcontainsprojecthandoff. Unrelated.resumeedituntouched; PRunmerged.
