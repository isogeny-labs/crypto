# Independent review: direct branch polynomial multiplicity

The derivative argument matches the exact source identity and proves the stated base-polynomial conclusion under its premises. Rearranging N²-u1u2U=b²F gives P=N²-b²F. At a common root of F and P, N(a)²=0, hence N(a)=0. Differentiation gives P′(a)=-b²F′(a), nonzero because b and F′(a) are nonzero. Thus P and each factor are nonzero; P has a simple root, and factor multiplicities sum to one.

The order of these steps matters: nonzero product is established before applying multiplicity additivity. No division by 2, b, or the residual leading coefficient occurs. The argument works over any field characteristic. Separability is stronger than needed; the local condition F′(a)≠0 suffices. This is the base-polynomial conclusion, without an additional geometric/DVR claim.

The actual coefficient fixture independently gives dq=0, dr=2, ds=dt=0, hence (B,A,C,Q,Rbar)=(-4,0,0,-16,0). Consequently N=0 and U=-16X, with product -16(X⁵-X). Its derivative at zero is16 and factor multiplicities are0,0,1. The stated Bezout combination expands to4, proving separability overQ. The leading residual coefficient is zero, so this fixture also detects an invalid normalization by that coefficient.

All three premise-removal examples retain the exact generic polynomial identity and have the claimed orders: b=0 yields X² (order2); a nonsimple F=X² yields -X² (order2); removing support vanishing yields1-X (order0). These are identity-level controls, without a claim that every tuple is realized by the source coefficientData formulas.

A separate generic characteristic-two witness is F=X, N=0, b=1, u1=X, u2=U=1 at0: the identity holds since -X=X, and the product derivative is1. A branch-root premise control overQ uses N=X+1, F=2X+1, b=1 and P=X²; F(0)≠0 and multiplicity is2. These are written algebraic checks, not executed experiments.

Source correspondence: source_support_base_multiplicity_sum has exactly the three factor-nonzero conclusions and multiplicity sum1. Its two divisor congruences and determinant condition supply source_norm_identity; separability and the branch root supply derivative nonvanishing. The direct proof changes the derivation route, not the conclusion. The source API one_lt_rootMultiplicity_iff_isRoot has no CharZero premise. Product additivity is in RingDivision.lean, included in the reviewer bindings.

Verdict: faithful_scoped at mathematical/source level. Seven producer manifest files and five source bindings match their hashes. The reviewer did not author this derivative derivation, but authored ancestor replay plumbing and proof repair. No compiler or new experiment ran. The coordinator's current branch replay remains separate; this direct derivative route has no compiled Lean artifact.
