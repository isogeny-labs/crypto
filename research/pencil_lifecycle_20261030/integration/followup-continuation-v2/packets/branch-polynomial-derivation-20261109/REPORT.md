# Direct polynomial derivation of the branch support multiplicity

The exact source identity is N^2 - u1*u2*U = b^2 F. Thus the support product P equals N^2 - b^2 F, with a minus sign before b^2 F. N is ActualNonbranchNorm.numerator; b is the first coefficientData component; U is the unnormalized residual. Neither b nor the residual leading coefficient is divided out.

Let K be the source field. Assume F(a)=0, the support product vanishes at a, b is nonzero, and F is separable. Evaluation of the identity yields N(a)^2=0, hence N(a)=0. Separability at this root supplies F'(a) nonzero.

Differentiate the polynomial identity. P'(a)=2 N(a) N'(a)-b^2 F'(a)=-b^2 F'(a), which is nonzero. Consequently P is nonzero and every one of u1,u2,U is nonzero; these factor conditions are conclusions, not extra hypotheses.

Since P(a)=0 and P'(a) is nonzero, a is a simple root of P. Product additivity of root multiplicities gives mult_a(u1)+mult_a(u2)+mult_a(U)=1, exactly the base-polynomial conclusion of the current branch coefficient theorem.

No division by 2 occurs. This direct argument needs no characteristic-zero or odd-characteristic premise: in characteristic 2, the squared-term derivative is already zero and the surviving b^2 F'(a) remains nonzero. This statement concerns base-polynomial multiplicity; it does not establish the separate geometric branch or DVR statements.

The pinned rational control has F=X^5-X, N=0, b=-4, u1=X^2+1, u2=X^2-1, U=-16X, a=0. Hence P=-16(X^5-X), P'(0)=16, and factor multiplicities are 0,0,1. U has degree one because c=0, illustrating why normalizing by c would be invalid.

Removing premises changes the conclusion: with b=0, F=X and N=X, the product X^2 has multiplicity two; with nonsimple F=X^2, b=1 and N=0, product -X^2 has multiplicity two; without support vanishing, F=X, b=1 and N=1 give product 1-X with multiplicity zero. All examples are over Q at a=0.

Status: source-only mathematical derivation, pending independent review. No Lean file was compiled and no experiment was run. This supplies an alternative review route; the active branch replay and its receipts remain unchanged.

Implementation route: Mathlib Polynomial.one_lt_rootMultiplicity_iff_isRoot relates multiplicity greater than one to joint vanishing of a polynomial and its derivative, without a characteristic-zero premise. rootMultiplicity_pos and product additivity then finish the natural-number statement. Exact API sources are pinned in source-bindings.json.
