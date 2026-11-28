# Oriented recovery and the limit of deferring signs

This is an algebraic derivation, not an executed arithmetic gate or a priority claim. It follows the classical projection/differential-neighbor/recovery framework of Chung–Costello–Smith, https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf, sections 2 and 6. The KB lexical query `Kummer signed recovery differential addition index calculus sign relations` at revision9b2119e8fbac482b98b50521c0d604bb returns IDEA-20260807-8027a2 and its proposed EXP-MODEL-4159eb: these discuss sign resolution and do not certify this genus-2 specialization. No novelty follows from the search.

## Translation does not descend by itself

In an odd-order group, suppose translation by Q descends under pi(P)=pi(-P). Then pi(P+Q)=pi(-P+Q). Equality means P+Q=-P+Q, hence 2P=0, or P+Q=P-Q, hence 2Q=0. For nonzero P and Q in an odd-prime-order subgroup both fail. Thus a single quotient coordinate cannot implement the walk translation. This does not preclude differential arithmetic: an oriented pair containing pi(P-Q) provides extra information. Nor does it prove recovery must occur at every nonsmooth step; our tested primitive already recovers only on smooth points.

For R=aD+bT+sum c_j F_j, a signed decomposition supplies sum n_j F_j=R. Replacing only n by -n changes the equation unless the known coefficients are negated with it. An arbitrary lift of pi(R) therefore cannot simply be inserted into the existing linear solver with unchanged a,b,c. A global row negation is harmless only after orientation is consistently resolved. Delaying local sign choices to nonlinear equations is a different solver with a separate cost obligation.

## Rational recovery from a point source

Assume characteristic is odd, y²=f(x) is monic degree five with x4 coefficient zero, Q=(r,s) has s!=0, and R=(u,v) has u=x²+A*x+B, v=b1*x+b0. Let U=u(r)!=0. Assume both neighbors R+Q and R-Q have degree-two monic u polynomials x²+Aplus*x+Bplus and x²+Aminus*x+Bminus. These are geometric conditions, not consequences of split support.

Set dA=Aplus-Aminus, dB=Bplus-Bminus. Then

    b1 = U * ((A+r)*dA-dB) / (4*s)
    v(r) = U²*dA / (4*s)
    b0 = v(r)-r*b1.

Proof: CRT gives the composed interpolant v+tplus*u for adding Q and v+tminus*u for adding -Q, where tplus=(s-v(r))/U and tminus=(-s-v(r))/U. The quotient (f-(v+t*u)²)/((x-r)*u) is monic degree two. Comparing x4 coefficients gives tplus²=r-A-Aplus and tminus²=r-A-Aminus. Thus tplus²-tminus²=-dA and tplus-tminus=2*s/U, whence v(r)=U²*dA/(4*s). Comparing x3 coefficients gives

    Bpm+(A-r)*Apm+B-r*A = -2*tpm*b1-2*tpm²*A.

Subtracting yields dB+(A-r)*dA=-4*s*b1/U+2*A*dA, proving b1 and then b0. Swapping the two neighbors negates v, so orientation is explicit.

The formula replaces a square-root/sign-guess recovery for eligible degree-one sources. Precompute 1/(4s) once and charge it. Neighbor u normalization, current u normalization, projective scaling, exceptional support, identity and degree drops remain real costs. U=0, s=0 or non-degree-two neighbors require a specified fallback; none may be dropped from a cost panel. The general source17*base is not necessarily a point source. Changing a walk to use some factor-base point does not make its discrete logarithm known; its column must remain in the invariant and linear algebra.

## Decision

Do not run a recovery-only performance campaign as a claimed tenfold route. On the retained stream, even eliminating all work outside the current addition kernel yields native-full/kernel REDC at most4.547. This formula is an eligible future component when paired with a substantially cheaper kernel or a different, justified relation-yield mechanism. Its correctness on actual projective chains and its full counter cost remain untested. General genus-3/4 recovery and the full mathematical tenfold objective remain open. [pcc:unverified]
