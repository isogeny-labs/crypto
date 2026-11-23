# Optimized quintic quotient shape and monic scaling

Nine main declarations compile in two modules. FastInterpolationShape proves linear ordinate degree bounds, quadratic support degree and monicity, source numerator/slope shape, and interpolant degree3 with leading coefficient S1/N under N!=0 and S1!=0. FastQuotientShape first proves the general quotient lemma for deg f=5, deg V=3 and monic deg U=4. It then invokes the earlier actual source CRT product-residual theorem to construct Q satisfying f-V^2=(Ua*Ub)*Q, deg Q=2 and LC(Q)=-(S1/N)^2. No quotient identity is assumed by the source endpoint; its witness comes from proved divisibility.

The final two declarations prove (-(N/S1)^2)*(-(S1/N)^2)=1 and that C(-(N/S1)^2)*Q is monic. This is the field-algebra justification of native inverse_lead at three_leads.rs1039, conditional on the native batch inverse decoding to si=1/S1. It is not a word-level inverse, pscale/trim or compiled-Rust proof.

Scope: arbitrary field, both displayed nonzero guards and degree f=5. No f monicity, X4=0 or characteristic-not-two hypothesis is needed for these polynomial statements. Degree f=5 cannot simply be replaced by six: the fixed sextic control f=X6,V=X3 has zero residual. A smooth odd-characteristic curve remains necessary for the separate Jacobian-class interpretation. Intended native correspondence remains prime p<2^31, Field.n=1, diagnostic tables absent, canonical words.

Checks: FastInterpolationShape-attempt-DDZ6uaOm, FastQuotientShape-attempt-HICx2WHf, FastQuotientControls-attempt-ypEhfjET, and FastQuotientReadback-attempt-CWkZqdd6 all exit0, unchanged bindings. Fourteen exact named readbacks show only propext, Classical.choice and Quot.sound. ExpectedFailure-attempt-CUMx3t3f exits1 by rejecting 0=1. Existing pinned compiler/cached dependencies remain trusted; independent review pending.

Five fixed controls prove the fixture degree5, instantiate the complete source quotient-existence theorem with repeated support Ua=X2 and coprime Ub=X2+1 on f=X5-X4-X3+1, verify nonunit inverse-leading scaling N=3,S1=2, reject omission of its minus sign over rationals, and demonstrate sextic cancellation. Fixture smoothness is not claimed.

Retained failure FastQuotientShape-attempt-XlGpc65j needed explicit final ring normalization of square versus product and replacement of unavailable Monic.mul projection with the direct leading-coefficient identity. No theorem hypothesis or conclusion changed. No other attempted module failed in this packet.

Next joints: native batch inverse decoding and coefficient operation order; exact quotient uniqueness and modeled pexact; pscale agrees with pmonic; final negative remainder and stored-array correspondence; exact fast/fallback equality and separate divisor-class addition. No repository edits, scientific/cloud execution, timing or fresh cost claim. All prior packets unchanged and frozen 1,260-target scope preserved.
