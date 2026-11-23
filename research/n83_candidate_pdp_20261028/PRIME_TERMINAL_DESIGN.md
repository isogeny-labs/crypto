# Higher-m prime terminal annihilator certificates

Status: implementation contract before affected production. Full panel: all42
retained n83 prime m3..9 original/baseline/spike40 planted/natural systems.
Preserve exact sources, targets, equations and point manifests. This advances
one obligation in the full three-family m2..9 FFD/LFD and elite-base objective.

## Sources and scope
Caminata–Gorla, https://arxiv.org/html/2112.05579, Definition1.3 and Section4:
first fall is a nontrivial highest-form syzygy; it need not bound LFD in either
direction. Read the primary definition before this design. Existing
PRIME_FIRST_FALL_DESIGN.md supplies the explicit selector annihilator;
TOP_DESIGN.md and src/bin/top.rs identify the unchanged higher-m source chain.
No maximal-rank extension or complete lower filtered-space claim is made here.

## Highest-form upper certificate
Normalize the last27 selectors z_i=w_i*v_((m-1)*27+i), with all weights nonzero.
The terminal S3(U,X,t) has highest form F=U^2*X^2, X=sum z_i. In the quotient
by all selector squares, H=product_(j=0..12)(z_(2j)-z_(2j+1)) is nonzero,
H*X=H*z26 and H*X^2=0. Lift H*F to the ideal of selector squares, retaining
the original degree17 identity as an arithmetic circuit: expand H*F and
reduce each repeated selector using z_i^2. Each term reduction has degree17.
The native checker independently multiplies the expanded H by X twice in the
square-zero algebra and checks zero; H has8192 distinct terms of degree13.

Nontriviality is essential. Project all variables except the last selector
block to zero and impose its variable squares. Every original highest-form
generator vanishes: nonterminal relations do not use this final block; terminal
F has U=0; selector squares vanish. H survives. Therefore H is not in the ideal
of other top generators. Every Koszul syzygy has terminal coordinate in that
ideal; H cannot be such a coordinate. Characteristic-p trivial syzygies have
degree greater than17. Thus this construction certifies FFD<=17 for each
validated source, not equality and not a global last-fall bound.

## Separate affine consequence, without strictness
Let A0=H,A1=NF(H*X),A2=NF(H*X^2) under z_i^2=w_i*z_i. Store all three fully
expanded sparse factors. The terminal consequence g=NF(H*S3(U,X,t)) is exactly
A2*(U^2-2tU+t^2)
+A1*(-2tU^2+(-2t^2-2a)U+(-2at-4b))
+A0*(t^2U^2+(-2at-4b)U+(a^2-4bt)).
Store the three U-polynomials as canonical coefficient lists from U^0 to U^2.
No U field equation is introduced. deg A0=13,deg A1,A2<=14, so deg g<=16,
and g has a derivation of degree<=17 from the original affine system.
Other chain equations may already derive g at lower degree. Its nonmembership
in V16 is NOT checked; strict_fall_verified=false and LFD bounds remain null.
A computed affine degree drop alone is not a strict filtered fall certificate.

## Implementation and falsification
Rust performs all polynomial arithmetic. prime_terminal_math.rs extracts the
existing sparse arithmetic and N83PFF1 codec with only visibility/path changes;
its input donor and extraction script are retained. Full-source validation
regenerates every byte including the complete planted witness suffix; it checks
all witness coordinates are canonical,selectors are bits,and all source S3
relations vanish on planted witnesses. Extra equations/comments/redefinitions
are rejected. Source SHA and immutable base manifests are pinned separately.

Controls: small odd selector counts3,5,7,distinct nonzero weights and targets;
independently expand S3(U,X,t) and multiply H using general exponent maps,
compare all coefficients against the compact circuit. Square-zero negative
control H=1 must not annihilate X^2. Check the projection kills all general
chain highest forms for m3..9,whereas retaining an intermediate variable need
not. Reject source changes,wrong base,invalid planted coordinates,zero weights,
truncated/corrupt factors,and a protocol omitting/duplicating cases. Do not label
these development controls production or independent scientific review.

Freeze implementation,42 input identities,protocol and limits before production.
Use one process with bounded memory/time;all failures and partial outputs retained.
Fresh-process and S3-restored replay must reconstruct every factor and certificate.
Document file bytes,variable indices,normalization and circuit. Exact FFD,
strict LFD,solving degree,performance and elite-base advantage are out of scope.
