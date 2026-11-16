# Independent branch norm transport source review

Disposition: source fidelity supported; kernel unverified. Exact published candidate 332575b279b07ae75afff81bafa2b96ff8e365f2 was byte-matched to the local source. No compiler, runtime trial or axiom readback was run.

**Actual evaluator.** E uses projectiveCoeffHom and projectiveX. rho is affine localization followed by the inverse branchRingEquiv. Polynomial.ringHom_ext checks constants through the scalar tower and X through its definition.

**Actual curve equation.** curve_eval transports affine_curve, which derives from quotient_affine_relation. No curve-evaluation premise is introduced.

**Norm scalar and sign.** Mapping n^2-u1*u2*U=C(b^2)f gives N^2-product=B^2Y^2. map_pow applies to polynomial C and then E; hc identifies E(C b)=B. Difference of squares gives (N-BY)(N+BY)=product.

**Support valuation.** The norm at a and support root imply n(a)=0. The prior conjugate-order theorem supplies both orders 1 and product order 2. Applying addVal and AddValuation.map_mul gives the three stalk orders summing to 2.

**Source specialization.** source_norm_identity derives the exact polynomial identity from coefficientData, nonzero determinant and the two divisor congruences; separability derives the simple-root condition. Desired stalk norm and order are not wrapper hypotheses.

## Attacks and boundaries

**b=0.** Excluded by hb for order statements; norm identity alone legitimately permits zero b.

**Singular branch.** Excluded by derivative nonzero, obtained from separability in the source wrapper.

**Zero U or zero support polynomial.** Not explicitly excluded. Norm plus simple branch and b nonzero would contradict the finite conjugate product order; additive valuation at zero cannot equal finite 2.

**Characteristic two.** The local algebra and orders do not require distinct conjugates. Do not promote this to a separable double-cover interpretation in characteristic two.

**Wrong factor of two.** Conclusion concerns stalk orders. The base-polynomial multiplicity sum equals selected order 1 in the independent derivation; its formal transport is still absent.

**Abstract or circular bridge.** E and rho bind actual imported maps. No new desired evaluator, norm or valuation equation is assumed. The predecessor does not import this successor.

The five print-axioms commands are requested checks, not evidence that they ran. Source inspection does not resolve elaboration, imported-instance or tactic acceptance. The actual-map construction closes the previously missing source-level stalk norm bridge. Base multiplicity transport remains the next formal component.
