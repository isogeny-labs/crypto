# Full grounded principal relations: saturation and the remaining scale obstruction

[pcc:unverified]. Author-derived finite-abelian-group/linear-algebra result, not an independent proof verdict, novelty claim, experiment, or efficient construction. This refines GROUNDED-DUAL-SCREEN.md and the rho1-window interpretation before investing in an interpolation finisher. All claims about low-degree search cost remain open.

## 1. Precise relation space

Let J=J(C)(Fp) under the retained smooth model/source premises, with unique ell-primary subgroup of order ell. Let D have order ell and let phi:J->Fell be the normalized homomorphism phi(D)=1. Take a fixed set G of rational point classes P_i-infinity and write H=<P_i-infinity> inside J. Define the integral relation lattice

    L = ker(Z^s -> H),  e_i |-> P_i-infinity.

These are ALL exact principal divisor relations supported on G and infinity, allowing arbitrary integer coefficients and pole degree. They are not merely a bounded interpolation sample. Assume H has a nonzero ell component. Then H/ell H is one-dimensional overFell; the vector lambda=(phi(P_i-infinity)) is nonzero.

Tensoring the right-exact sequence L -> Z^s -> H -> 0 withFell yields

    image(L modell) = ker(Fell^s -> H/ell H).

The last map identifies with r |-> r dotlambda, since phi is nonzero on H's unique orderell component and kills its prime-to-ell part. Therefore

    span_Fell(L modell) = lambda-perp,
    dimension = s-1.

Right-exactness is essential: this statement is about the IMAGE ofL inFell^s, not an unsupported identification ofL/ellL with that image. Equivalently one can use Smith normal form forZ^s/L=H: exactlyone invariant factor has positiveellvaluation underthese premises. The result doesnot assert that a bounded-degree collector spans this space.

If H has noellcomponent, the full relation space instead equalsFell^s and lambda=0. A scale-obstruction classification relying on lambda!=0 cannot silently be applied to that case. H containing an explicitly certified nonzero orderell source combination is one way to certify the premise; knowing abstractgroupmembership isnot a free computationalcertificate.

## 2. Exact criterion for a grounded rho1 line

At the current stage suppose grounded logarithms are parameterized by

    X_G(theta)=alpha+theta*beta,

and the true values are lambda=alpha+theta_true*beta. Any full grounded principal relation r inL yields

    r dotalpha + theta*(r dotbeta)=0.

Under the nonzeroellcomponent premise, every such relation has r dotbeta=0 iff beta belongs tospan(lambda), by the orthogonal-complement result above. Thus:

* If rank[alpha beta]=2, some full grounded principal relation has nonzero signature and fixes theta. Indeed beta cannot be proportional tolambda=alpha+theta_true beta.
* If beta=0, no grounded principal relation changes theta.
* If rank[alpha beta]<=1 and lambda!=0, beta is proportional tolambda, so all full grounded principal relations annihilate the remaining direction. No amount of additional homogeneous principal-relation harvesting on that same G fixes theta.

A target with nonzero theta coefficient is fixed exactly when theta is fixed. This establishes an existence/impossibility criterion for the FULL relation space, not an expected runtime or a constructive low-pole search guarantee. In the lambda=0 case, rank2 cannot occur, but a nonzero beta can still be constrained by the full relation space; keep the subgroup premise explicit.

## 3. One-dimensional availability is not enough

An abstract consistency example: J=C_101, grounded generator P, sourceD=17P, targetT=P. Relative toD, the true targetlog andgroundedlog are6 because17*6=1mod101. A current system containing X_P-k=0 has one free parameter X_P=k=theta. All principal relations supportedonlyonP are multiplesof101P=0 and reduce tozero modulo101, so nonefixestheta. Sourceorder and a finalnativecheck do not create missing linearinformation. An explicit source-normalization relation 17X_P=1 wouldfixit, but obtaining an applicable normalization is a separate task; the scalar17 in this abstract example is not supplied to any current collector.

This is a counterexample to the inference rho1 implies availability of a grounded principal finisher, not a claim that the current22curvewindows exhibit this obstruction. Their alpha/beta vectors have not yetbeenconstructed orclassified. Existing publicsource/targetdecomposition controls remain relevant precisely because normalization can remove a scale obstruction when constructible.

## 4. Source normalization gives a useful sufficient condition

If a source equation d dotX_G=1 is already valid throughout the current solutionline, then d dotalpha=1 and d dotbeta=0. If beta!=0 these imply alpha,beta areindependent: beta=c alpha wouldforcec=0. Therefore a full grounded principal finisher exists bysection2. If beta=0 and targetstillvaries, relations onthis G cannotaffectthetarget; one must use other support or a nonhomogeneous source/targetconstraint. A publicsourceequation touchingungrounded variables cannot be substitutedas a constraint onlyonG without justification.

## 5. Effective zero relations and the missing cost bound

The full lattice statement can also be represented by effective affine zeros with polesonlyatinfinity, but this doesnotmake it cheap. For any r inL, each pointclass has finiteorder n_i. Adding sufficientlylarge nonnegative multiples ofell*n_i*e_i makes all coordinates nonnegative, remainsinL, and preserves r moduloell. The resulting divisor is effective awayfrominfinity and principal, hence has a rationalfunction representative with polesonlyatinfinity. Existence follows from rational principal divisors with a rationalbasepoint; it supplies neither the n_i values nor a low-degree function algorithm.

The resulting poledegree canbe enormous. This argument doesnotjustify a64-trialinterpolator or show thatdegree2g+1functions suffice. The Riemann–Roch residualdegree restriction in GROUNDED-INTERPOLATION-DEGREE.md stillapplies to the boundedconstruction. Distinguish full-lattice saturation, low-degree span, useful rank and completecost.

## 6. Next discriminating evidence

Before a grounded finisher campaign, extract a certified particularsolution and nullvector forkerB at the FIRSTunresolvedrho1prefix, then derive alpha,beta forgroundedpoints from the existingforest. Verify the line against allretainedrawconstraints, keep freeLPcomponent directions explicit, and separatelyprice basisextraction/labelprojection. Classify rank[alpha beta] and record whether a constructiblesource-normalizationcertificate supports the nonzeroellcomponent premise. Zero orscale-obstructed cases mustnot disappear fromthepanel.

Only after rank2 applicability is established should a bounded geometric span gate test whether the chosen low-degree interpolation family actually contains a nonzero-signature relation. A negative boundedspan neednot contradict the full-lattice theorem. A positive span stillneeds coldcost and nativeverification against strongeststaticforest/publicnormalizationcontrols. No new scientificrun here; no S3/cloud/merge. Fullgenus2+10xmathematicalgoalactive/unachieved, independentreviewopen.
