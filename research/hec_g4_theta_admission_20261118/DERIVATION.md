# Genus-four quotient admission: exact inputs and missing construction [pcc:unverified]

## Scope and theorem boundaries

The originalg4p31source has11Frobeniusstable maximalisotropic2kernels (5pair,6other); p61has0. Stability describes a rational subgroup scheme, not pointwise rationality. A quotient by eachp31kernel is a rational principallypolarized fourfold and the2powerisogeny would preserve theoddellsubgroup. No actualtheta quotient or transport is constructed by this gate.

KNRR (arXiv2004.08315v2), Theorem5.8, proves the geometric genus4Schottky-Igusa test in any characteristic!=2 for INDECOMPOSABLE ppav. At31and61 its degree16theta expression can be tested up to a nonzero scalar by S=(sum_even theta8)^2-16sum_even theta16; the displayed normalizing denominator2^6*3²*5*7 is invertible. A zero value still does not provide a rational curve/descent or a nonhyperelliptic model.

Important implementation refusal: KNRR section4/Theorem4.2 requires isogenyparameter ell prime to the even theta level n>=4. Its algorithm6/7 starts from ellipticproducts and odd kernels. It cannot be reused verbatim for THESE2kernels atlevel4; nor can the ellipticpower spanning method be imported into these Fp-simple sources. Need an even-degree compatible theta algorithm with actual source level structure/kernel coordinates.

HPS (arXiv2402.03160v4,2026), Theorem4.2, reconstructs generic genus4curves from suppliedtheta constants overalgebraicallyclosedfields, in characteristic0or sufficientlylargep. Sections4.2/5discuss failures anddescent. This does not certify universal success at31/61 or supply absentquotienttheta data. Code is Magma; it was read as a source pointer, not executed or transplanted.

Naranjo-Pirola (arXiv1705.10154v2), Theorem1.1, is a verygeneral complex-hyperelliptic rigidity result. It is not an instance-exclusion theorem for either finite-field curve. Keep it as prior art against an unrestricted generic route, not a reason to assignnonJacobian to these11quotients.

## Actual 2torsion coordinates

Order9finite roots as r0 fixed, r1..r4 firstquarticFrobeniusorbit, r5..r8secondorbit. Infinityisr9fixed. The oldabstractmodel usesbits0..7, normalizedbit8=0 andinfinitybit=parity. For a vectora thefinitebranchsupport initiallybits(a). Ifitscardinalityexceeds4 replacebyitscomplementamongall9finitepoints. This uses div(y)=sumfiniteWi-9infinity; reducedclass is u=product_(support)(X-ri),v=0,degree<=4.

Frobenius acts oncoefficients and cyclicallypermutesroots. Verifying both representations binds abstractkernel spans toactualcoordinates. For two reducedsupport masksS,T, cancel theirintersectiontwice. If |S xorT|<=4, uS*uT=u_(intersection)^2*u_(S xorT). Otherwise uS*uT*u_result=u_(intersection)^2*F. These are exact principal-divisor/2torsion composition identities; they also coveridentity/cancellation. Fullpointdefinitiondegree of a kernel is theleastd making Frobenius^d fixEVERYvector, not merelythestabilitydegree1. Corresponding theta coordinates can need a largerfield.

## Source Thomae calibration

With infinityfixed, balanced partitions of10branchpoints correspond to choosing4of9finitepoints A; B is its5pointcomplement. Let P_A=product_(i<j inA)(ri-rj)*product_(i<j inB)(ri-rj). Thereare126suchpartitions. Classical Thomae gives theta4 up tosign/commonfactor; squaring removes signs, so theta8 is commonfactor*P_A². The other10even theta constants vanish for thehyperelliptic source. Therefore the sourceSchottky numerator is (sumP_A²)^2-16sumP_A⁴. Its vanishing tests field/root/polynomial instrumentation and sourcecontrols. It doesNOT identify theta signs, provide a levelstructure, or evaluate the quotientform. Affine scaling x→a*x+b multipliesPby a16, sum8by a32andsum16by a64.

## Odd-trace geometric indecomposability lemma

Assume a fourfoldA/Fp is Fp-simple. If a rational principalpolarization geometricallydecomposes, uniqueness of decompositioninto indecomposable polarizedfactors makes Frobeniuspermute thosefactors. Simplicity forces a singletransitiveorbit; otherwise anorbitsumdescends to a properFp-subvariety. The orbitlength e>1 divides4, hence e=2or4. Frobenius^n onH1thenhaszero traceforoddn, becauseitfixesnofactor and itsmatrixhaszerodiagonalblocks. Thus a NONZEROtrace atn1or3 excludes geometricpolarizationdecomposition. EveryFp-isogenousquotient shares thesetraces andFp-simplicity, so thecriterion transfers. This is notabsolute simplicity. Ifbothtraceszero,thegateleavesindecomposabilityunresolved; itdoesnotdeclaredecomposition.

Source traces are p^n+1-#C(Fp^n). The gate counts points exactly and retainszerooutcomes. Written lemma needsindependentmathematicalreview; standarduniqueness input is Benoist-Wittenberg section1.2, https://www.math.ens.psl.eu/~benoist/articles/ch2xk.pdf . No assumption that sourcecanonicalindecomposability automaticallytransfers throughisogeny.

## Sources checked / next constructive requirement

Primarypapers read: https://arxiv.org/pdf/2004.08315 section4andTheorem5.8; https://arxiv.org/pdf/2402.03160 Theorem4.2,sections4.2/5; https://arxiv.org/pdf/1705.10154 Theorem1.1; Poor hyperellipticSchottkyvanishing https://faculty.fordham.edu/poor/files/Schottky%27s%20form94.pdf ; classicalThomae formula sourcevanGeemen, Pointsetsinprojectivespacesandthetafunctions, AST165(1988), NumdamPDF13MBfailedwebfetch; branch-productcontrolisthereforeexplicitlyqualifiedas sourceidentity/notsignedtheta reconstruction. Benoist-Wittenberg primarysection1.2supportsuniqueppavfactorization. ThetAVdocs describehyperellipticconstructor asgenus2; they do not certifygenus4construction. NoSageexecution.

KBreadyrevision9b2119e8fbac482b98b50521c0d604bb,lastrefresh2026-11-17T06:15:25Z,lexicalcurrent/hybridnotbuilt. Queriesgenus4RichelotJacobianSchottky andnonhyperellipticisogenytheta. Hits source:1ec335446c039811fbc8c9dcf510e42c isgenus2infrastructure; source:501ff4648d172f525e91a7e3e3f5620e isDiemheuristicmethodpointer. Neitheradmitsthisquotient. Noabsence/noveltyclaim.

Nextrequiredconstruction: compatibleeven-degree quotienttheta data on labelledkernels, exactSchottkyforEVERYquotient, geometricindecompordifferenthoneststatus, rationalmodeldescent/nonhyperellipticity, explicitoriginalD/Ttransport andchargedcompletecost. Do not substitute sourcezero intoquotienttest orarbitraryfourthrootchoices intoRiemann-validthetas. Genus2/3/4full10xgoalnotfulfilledbythisinputgate.
