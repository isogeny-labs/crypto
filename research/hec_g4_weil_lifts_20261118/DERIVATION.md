# Normalized Weil functions and compatible isotropic four-lifts [pcc:unverified]

## Exact functions, not a sign guess

Priorhalfgate givesHi=[Di-4infinity],2Hi=Wi, andF-vDi²=(X-ai)ui². Thusfi=(y-vDi)²/(X-ai)hasdivisor4Di-16infinity. WithuniformizerX4/y at infinity, itsleadingcoefficientis1. Multiplicationalgebra norms evaluateproducts overeffectiveDi,withmultiplicities. Allprimitivefielddata/principalidentities are reused fromthequalifiedgate.

IndependentfunctionconstructiontrackseachCantoraddition. Cancellingcommonverticalfactor d contributesfunctiond(X). A reduction(u,v)→(u2,-v mod u2) contributes(y-v)/u2 because itsdivisor isD(u,v)-D(u2,-v). TheproducthPQhasdivisorDP+DQ-D(P+Q). Hencef4P=hPP²*h(2P,2P)when4P=0. Itsleadingcoefficientiscomputedfromeveryfactor; arbitraryconstantsmustnotbeignored. EachtrackedMumfordoutputmatchesunmodifiedCantoradditionnumerically. Finalnormalizationmakesf4Pmonicat infinity. ThisisclassicalMiller-stylefunctionaccumulation,notanewpairingalgorithm.

## Disjoint moved divisor handles infinity explicitly

For distinctdegree4effectiveD,E withdisjointfinitesupport andn=4, choosebranchpointWboutsidebothsupports andawayfromfunctionfactorpoles. Let h=(X-b)², so div(h)=4Wb-4infinity. MoveE-4infinity toE-4Wb bysubtractingdiv(h), andreplacefEbyfE/h4. Thetwodegree0divisorsnowhavedisjointsupport. TheordinaryWeildefinition gives

e4(D,E) = [fD(E)/fD(Wb)^4] / [(fE(D)/uD(b)^8)]

for infinity-monicfunctions; fE/h4isregularatinfinitywithvalue1. GeneralWeilreciprocityappliedtofDandX-b givesfD(Wb)^4=uD(b)^8, because fDhasdivisor4D-16infinity andthelocalinfinityleadingcoefficientis1. Thereforethisequals fD(E)/fE(D). Thegatedirectandmovedformulasarecomputedseparately,andnormzero/supportexceptionfailsratherthancancellinganundefined0/0.

For theclosedfunctionandb!=ai, theprincipalidentity also givesfi(Wb)=-ui(b)², anexplicitnormalizationcheck. Bilinearity/alternationnondegeneracyarestandardWeilpairingtheorems; finitecomparisonssupplyinstrumentcontrols,nottheirproof. The squaredpairing equals e2(2Hi,2Hj)=-1 fordistinctWeierstrassWi,Wj: bothbranchsubset representativescontainthefixedinfinitypoint.

## Deterministic compatible lifting over Z/4

LetEbeexponentmatrixofe4onthe8Hibasis. Mod2itistheknownoff-diagonal1/diagonal0matrixandisnondegenerate. LetLcontainthe4kernel0liftcolumns. IsotropyofitsdoublesimpliesM=L^TELiseven. ChangeL→L+2X; newGram isM+2(X^TEL+L^TEX)mod4. SixF2equationscancelsixupper-triangleentries. Thispreservestheoriginal2kernelbasis; equationsarecheckedafter solving.

FindBwithL^TEB=Ibyunit-pivotZ4elimination. PutN=B^TEB. ReplacingB→B+LCwithCuppertriangularandCij=Nij(i<j)givesnewcomplementGramN-C+C^T=0. Thus[L B]hasthecanonicalalternatingGram[[0,I],[-I,0]]. ActualMumfordpointsarereconstructedfromthequalifiedHi, andall28freshnativepairingsmustmatchthisGram. Their256first-four-pointcombinationsmustdoubletoexactlytheoriginal16kernelpoints.

Thisprovidesisotropic4lifts/fullsymplectic4basisoverF31⁸, NOTa coherenttheta-nullpointorMumford-to-thetaconversionbyitself. The knownSQISignHDRemarkC.3.4 stillrequiresaffinecompatibility/Riemannrelations. No8torsionrigidification,quotientSchottky,curveDLPtransferorspeedclaimis inferred.

## Sources and limits

Primaryfunction/pairingbackgroundread: Galbraith, Mathematics ofPublicKeyCryptography, Chapter26sections26.1–26.2, https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch26.pdf (generalcurvedivisorevaluation/Weilreciprocity; subsequentpairingpresentationelliptic). Enge, Bilinearpairingsonellipticcurves, sections2–3, https://ems.press/content/serial-article-files/44297 (monicnormalization/signcare; ellipticcontext). Thedisplayedgenus4function/movingargumentaboveisexplicitanddoesnotassumeellipticlineformulasapplytoJacobianaddition. ClassicalJacobianWeilpairingtheoryisalsoinMilneAV andthepriorbranchmodulegate. Independentreviewofwrittenproof/instrumentremainsopen.

KBqueryWeilpairingcommon-supportinfinityleadingcoefficientsreturned unrelatedleadnotes(source:9f2695f25addfa09aeb597d81c86447d,source:34884c18abb08beff2ee71990c9dd502); notusedasproof. Revision9b2119e8fbac482b98b50521c0d604bb. Primary/localexactidentitiesusedinstead. Citeseerhyperellipticsurveyfetchfailed; do notinferabsence. No noveltyclaim.
