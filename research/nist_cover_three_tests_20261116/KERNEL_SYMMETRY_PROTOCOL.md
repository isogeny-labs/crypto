# Cover involution consistency control

2026-11-16. Fixed input: the same P-192 d7 curve and all eight original RREF kernels. Before affected data, test the known cover involution sigma(t,v)=(-t,v).

Because the cover equation is even in t, sigma is defined over Fp. In the odd model it acts by X -> -X/(1+2cX), Y -> Y/(1+2cX)^4, c=1024. Check the exact cleared curve identity over the actual base-field coefficients. The original maps satisfy pi*sigma=pi and rho*sigma=hyperelliptic-involution*rho. Thus sigma acts as+1 on the elliptic pullback and-1 on the Prym pullback. These pullbacks span geometric3-torsion because the splitting isogeny's degree is a power of2, coprime to3.

Apply diag(1,1,-1,-1,-1,-1) to every one of the27 vectors in each archived kernel. Identify its unique matching archived27-element set. Require a bijective involution on all eight kernels, without assuming its orbit sizes or pairing beforehand. Retain the complete permutation and orbits; reject an altered non-involution or mismatched kernel. Do not infer a twist outcome directly from a finite-module match alone: the curve automorphism argument is what gives a polarized Fp quotient isomorphism.

For kernels paired by sigma, any valid same-field Jacobian/Serre-sign classification must agree. The currently running fresh kernel2 construction is retained as a consistency control even if symmetry predicts equivalence. A disagreement would reject the conditional twist method/normalization, not justify changing the recorded kernel data. No novelty claim; this uses the cover's known bielliptic involution.
