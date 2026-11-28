# Quotient-polynomial parity instance map

Structures above:CommSemiring R and Polynomial.expand R2 for fourth-linear-power identity;CommRing R,Ideal.span{d},quotient ring and polynomial coefficient/map maps for divisibility bridge. No custom parity syntax or filtered ring is introduced.

Existing laws as stated:Polynomial.expand is an algebra hom;expand_C/expand_X/map_add/map_mul/map_pow compute its action. Generic ring normalization supplies the fourth-degree binomial identity with an explicit fourth-multiple remainder,whose vanishing follows from h4. Polynomial.coeff_expand identifies odd coefficients0. Ideal.Quotient.eq_zero_iff_dvd recovers the divisibility witness directly;do not re-prove its kernel/span argument.

Objects:linear polynomial1+C(s)*X with h4:(4:R)=0,and arbitrary P:R[X] whose quotient image equals expand2 Q. The new composed laws live in owning CryptoLean,first consumer actual integralRationals[X] quotient by C(2^q),q1/2.

Assembly probe:compile named generic module at exact pinned Mathlib/Lean. Fourth identity has explicit even preimage1+C(6*s^2)*X+C(s^4)*X^2. Odd-divisibility proof applies existing quotient theorem to congrArg coeff of supplied image equality,then coeff_expand and actual odd-index nondivisibility. This generic contract does NOT assert the actual study image equality;that is the missing instance obligation.

Missing components only:actual node half-slope divisibility inside integralRationals,actual scaled denominator quotient as even expansion,actual numerator/finite geometric image and subtype map back to rational coefficients. Do not instantiate divisibility in Q alone or assume converse of Transport.reduce_zero_of_bound.

Inheritance table:Polynomial.map from RingHom,coefficients from Polynomial.coeff,even preimage from Polynomial.expand,quotient map from Ideal.Quotient.mk,divisibility from eq_zero_iff_dvd,finite product/power closure from map laws. No private induction,limit raise,compiled trust or algebraic inverse of2/4 in quotient.

Proof chain:expand RHS by hom laws;generic fourth identity splits even part plus4*(sX+s^3X^3);h4 kills remainder. For divisibility,take odd coefficient of map=expand equality,apply coeff_map and coeff_expand,then existing ideal quotient divisibility equivalence. Zero rings/zero d/zero s included;no Nontrivial or unit premise.

Sources checked:Mathlib Expand.lean35--64,91;Ideal/Quotient/Basic.lean75;transport integralRationals Subring and liftPoly/map_liftPoly. KB unavailable as previously diagnosed;exact primary fallback,no novelty claim for standard binomial/ideal algebra. FirstMoment module93b303be stays byte-identical and separately auditable.
