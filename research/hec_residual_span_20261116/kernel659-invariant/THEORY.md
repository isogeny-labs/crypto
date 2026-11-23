# A modular scalar and a sufficient arithmetic reduction certificate

[pcc:unverified] Arithmetic/geometric reasoning requires independent review; the present gate computes complex balls and an exact Frobenius rank certificate, NOT the421-adic certificate below.

## Basis-independent objects

Write P=product of36even theta constants, S=sum oftheir8thpowers, and Sigma=sum_i product_j!=i theta_j^8. The classical forms P=chi18 and Sigma=Sigma140 have weights18 and140; S is the standard weight4 theta sum (a rational scalar multiple ofE4). The ratios
J=P²/S^9 and R=P^70/Sigma^9
have weight0. The even powers remove the theta multipliers, so these arefullsymplecticmodularfunctions where denominatorsnonzero. Their rationalFouriernormalizations differonlybyfixedrationalconstants fromnormalizedforms; arithmeticusemustretainthesefactors, particularlytheir421-integrality.

Rawtheta values arebasisdependent. Isometricperioddescriptors shouldgiveoverlappingballs for J andR evenwhenrawvaluesdifferbymultipliers. This tests the implemented scalar/cancellation and supplies anobject forsubsequentalgebraicrecognition; it doesnot make numericalrecognition exact.

## Why R is more useful than a generic E4 denominator

At a smooth hyperelliptic genus3 reduction in odd characteristic, the theta-nullcriterion givesexactlyonezeroevenconstant and35nonzeroones (after appropriatelevelstructure andtrivialization). ConsequentlyP vanishes, whileSigma reduces to theoneproductomittingthezero, whichisnonzero. Underanexplicitintegralnormalization/model at aprimeover421, hyperellipticity thereforeforcesv(R)>0.

Thus a CERTIFIED nonpositive421-adicvaluation oftheEXACTalgebraicR, underthosegoodreduction/normalizationpremises, wouldexcludehyperellipticreduction. No genericE4unitassumptionisneededforthiscriterion. SourcecomplexR-containing0controlisnotanexact421-adiccomputation. Neitheranapproximate rationalfitnorcomplexnonvanishing is a certificateofv(R).

This requirescheckingthetheta-nullcriterion in thefinitecharacteristic setting andanordinarygoodpolarizedmodel. These areexplicitpremises, not consequences oftheCprogram. No pole/denominatornormalizationorCMembedding is silently ignored.

## A manageable algebraic target, with limits

The sourcecurve isdefinedoverQ and sigma overK=Q(zeta7). GaloisoverK preserves eachone-dimensionalsigmaeigenspace onJ[659], hencepermutes themixednonzero slopes within a fixedgammafamily, respectingisotropy. SourceautomorphismsarealsooverK. Thereforethewhole94classset inonefamily isGaloisstableoverK, conditionalonthepriorclassidentification. Forarationalmodularfunction R, the degree94product overthatset hascoefficientsinK. It neednotbeirreducible, andRneednotseparatetheclasses; no exactminimalpolynomialdegree isasserted.

This explainsa possibleclasspolynomial/CMrecognitiontarget. It doesNOT provideheight/denominatorbounds orlicense roundingcomplexnumbersintoexactcoefficients. A secondprecision matchalone doesnotcertifyrational/algebraicrecognition. Recoveringonevaluefromsixembeddingswouldalso requirethecorrectGalois actionandnormalization, notjustrelabelingperiods arbitrarily.

## Absolute simplicity, a separate requirement

The retained Frobeniuspi generatesK. AllitsGaloisconjugateslieinK. If two conjugateshaveequalnthpowers,theirratioisarootofunityinK, hencehasorderdividing14. ShowingQ(pi^14)=Kthereforepreventsanysuchcollisionforanyn. Theprogramcertifiesdegree6 by fullpowerbasisrankmod1009; a nonzeromodularminor provesnonzerorationaldeterminant. TheordinaryHonda-Tatecriterion then givesabsolute simplicityofthesource anditsisogenousquotients, subjecttothepriorFrobeniusidentification.

Absolute simplicity excludesgeometricdecomposition. Evenif R's valuationlaterexcludeshyperellipticity, a nonhyperelliptic principallypolarizedthreefold overF421maystillhaveaSerrequadraticobstruction tobeing aJacobianoverF421. Curveconstruction/descent or anexplicitextensionanditscost remainnecessary. Do not inferanFpplanequarticorcheaptransfer fromthemodularvaluealone.

## Evidence and sources

Primary theta forms/vanishing: Balakrishnan,Ionica,Lauter,Vincent, https://www.uvm.edu/~cvincen1/files/preprints/vincent7.pdf Theorem5.1 andchi18/Sigma140 discussion. CM/module background inpreviousMarseglia/Milne sources. FLINT https://flintlib.org/doc/arb/index.html distinguishes printd (diagnosticapproximationswhoseprintedintervalneednotenclose) fromexactdump/load serialization. Earlierexclusionflagswerecomputedonrigorousinmemoryballs; theirprintddecimalsarenotportableballcertificates. Thisstagecloses thatserializationgapwithoutretroactivelyclaimingtheolddecimalswereexact.

KBsource:9b2081948fb8f20d2ba70d5e97b8a29a is anunreviewedbad-reductionboundpointer, notadenominatorbound forthesenonmaximalorder/planequarticcandidateinvariants. Literature onhyperellipticRosenhain invariantscannotbeappliedwithoutitslocus/orderconditions. There isno421-adicunitcertificateyet, andno10xspeedresult.
