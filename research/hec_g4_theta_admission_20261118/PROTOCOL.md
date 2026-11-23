# Frozen original genus-four theta-input gate [pcc:unverified]

This gate qualifies finite-field roots, coordinate-labelled two-kernels and SOURCE Schottky controls. It does not compute quotient theta constants, a quotient Schottky value, a curve model, target transport or a speed.

Original models only: p31,c16 andp61,c2, F=x9+x3+c*x+1, ell239753/16790591. Reuse qualified prime-order source/simplicity premises from hec_linear_budget DIMENSION-THEOREM and native-kummer-pair-screen; do not claim new independent order verification. Retain both models including p61zero rational-kernel case.

p31: fixed seeded factor splitter (2026112800,SplitMix64/modulus sampling) on degree8 residual;128attempt bound for quarticfactor. Verify bothquartics irreducible by exact Frobenius/gcd. Build F31^4 from firstfactor; split otherquartic using bounded128attempts per recursive node. Bind roots in exactabstractcycleorder0|(1,2,3,4)|(5,6,7,8), eachquartic orbit rotated to minimum fixed-arrayrepresentation. Verify distinctness,F evaluations,fullproduct and Frobeniuspermutation. Build all256reduced2torsion Mumford(u,0)coordinates, verify coefficientFrobenius against abstractmodule. For each of11frozenrationalkernels verify membership/action, fullpointfielddegree in1/2/4 and2816torsioncomposition polynomialidentities. Keep fullpointcoordinates and bothpair/nonpaircases. This is polynomial divisor algebra, not an independent general extension-group implementation or a theta structure.

p61: prove degree9F irreducible by Frobenius/gcd, buildF61^9, take all9Frobeniusconjugates ofgenerator, verifyproduct. The old rationalkernelzero is retained; no newquotientsearch overextensions.

Sourcecontrol: for all126four-subsets of9finitebranchpoints computeThomaeP as productofdifferenceswithinbothparts. Require everyPnonzero, (sumP²)²-16sumP⁴=0 forbothoriginalsources. These are theta8/16 ratios up tocommonpowers, noindividualthetacoordinates/signs inferred. Verify affinecovariance x→2x+3 atp31. For eachsourcecalculation mutateoneP² by+1, adjustP⁴consistently, retain126outcomes andrequire at leastone nonzeroSchottky; invalidthetatuples areNOT validnonJacobianppavcontrols.

Exactpointcountcontrol: p31overFp andFp3 bycompleteenumeration/Legendre withirreduciblecubic founddeterministically. Keeptrace1/trace3evenifzero. p61overFp keeptrace1. These support the conditional odd-trace geometricindecomposability lemma in DERIVATION; no inferred Jacobian status forquotients. Noassertion requiring nonzerotrace ordata-dependentrerun.

CompileRuststandalone withoverflowchecks. Pushsource/protocol/derivation/import/binaryhashes beforeone600CPUprocess; preserveexit/rawstdout/stderr/hashreceipt/no unchangedrerun. GitcustodyduringS3outage, noS3/cloud/merge. NoMagma/Sage/Python scientificexecution. Originalgoalremainsactive.
