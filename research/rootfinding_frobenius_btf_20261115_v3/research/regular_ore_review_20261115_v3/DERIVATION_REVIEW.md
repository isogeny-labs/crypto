# Independent two-term Frobenius derivation

This is an exact conditional characteristic-two norm component. It is not a complete original-root solver, a novelty finding, or a qualified performance claim. The earlier regular-double reduction remains under a!=0,a!=z,G(a)!=0,B!=0 and rational source/target guards.

Independently expanding s=(k/a)ya+a²+a(ha+k) gives s0=(1+h)a²,s1=ya/a+a. The root value gives r0=[ha*ya+a³+z³+s0(a+z)+hz*yT]/(a+z)² and r1=[ya+s1(a+z)+yT]/(a+z)². Target interpolation yields t0=z³+r0z²+s0z+hz*yT,t1=r1z²+s1z+yT. All denominators are fixed nonzero; k is shared globally.

Write F=F0+kF1,G=G0+k. Expanding the literal norm gives N0=F0²+XG0F0+CG0², N1=X(F0+G0F1), N2=F1²+XF1+C. The target/root/derivative identities hold formally over K[k], so every Ni is divisible by D=(X+z)(X+a)². This proves global F2-affinity and the two-term Frobenius structure independently of basis sampling. N1/D and N2/D have degree at most1 in X.

From H e2=r²+hr+k+h²A+zb and Q T=e2+a², alphaT=r1²,betaT=1+hr1,gammaT=r0²+hr0+h²A+zb+a². From H e3=hs+kr+k²+z e2 and Q U=e3+a²b, alphaU=1+r1+zr1²,betaU=r0+hs1+z betaT,gammaU=hs0+z(gammaT+a²)+a²b. These agree with the proposed explicit coefficients.

For Tprime=T+gammaT,Uprime=U+gammaU, cancellation gives alphaU*Tprime+alphaT*Uprime=Delta*k, Delta=alphaU*betaT+alphaT*betaU. If Delta!=0 it recovers k and either original equation with nonzero square coefficient forces the other. Such a coefficient exists because alphaT=0 implies r1=0,alphaU=1. If Delta=0, compatibility and a scalar additive quadratic must be retained; no division or silent branch deletion is valid. A direct83-k binary expansion of both explicit quadratics also preserves Delta=0 without requiring inversion. This direct formulation is a legitimate test of cheaper map construction even if common-square elimination is deferred.

Both formulations must produce the same affine constraint family T in W2,U in W3, with the same fixed b in V. The actual35/52 span dimensions give at most79 constraints on83 k bits; consistent nullity>=4 is a lower bound, not a generic rank79 theorem. An explicit83-k matrix is not automatically sparse or triangular, and an87 support-coordinate formulation is not automatically cheaper. Conversion, exact elimination/provenance, support, factor/order restoration and output serialization costs all count.

Support is exact iff (L_V mod Q)^4=0 for monic cubic Q, because L_V is split squarefree and every cubic root multiplicity is<=3. Every bounded candidate must retain Q,L_V mod Q and the fourth-power residue, including rejected candidates, to permit independent recorded-data verification. Capped nullity/factorization remains UNKNOWN; common-factor k=ha, zero-source and other h/a/ordinate fibers remain separate obligations. Full all-roots recovery includes original ordinate/auxiliary/ghost branches and is not implied by this norm component.

Classical Frobenius/Hermite/linear-algebra identities establish correctness, not novelty or speed. All mathematical statements here were derived symbolically; the reviewer did not execute the candidate algorithm.
