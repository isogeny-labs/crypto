# Integral crossed source operators after the quadratic diamond twist

This is a written interpretation of exact native identities, with geometric
hypotheses exposed. It does not identify the surviving component with a
chosen Koblitz model by an explicit isogeny.

Write f for Frobenius pushforward and t for the actual F2 diamond
pushforward on the proper source Jacobian. They commute. Retain the earlier
modular/Tate/Prym identification and its rational isogeny decomposition.
Let
\[
 Q(f,u)=f^2+fu+2u^2,\quad
 E(t)=(1-t^5)(5-1-t^2-t^4-t^6-t^8),\qquad
 B(f)=f^{32}L_b(-1/f).
\]
Here L_b is the complete, retained degree-32 Euler polynomial of factor
77.2.l.b. B is monic integral and B(1)=3721. The factor E kills the other
diamond components; omitting it would leave part of the genus-61 Jacobian.

The native calculation exports the following actual integer polynomials:
\[
 \Pi_7=E(t)B(f)Q(f,t^7)Q(f,t^6),\qquad
 \Pi_8=E(t)B(f)Q(f,t^4)Q(f,t^3).
\]
Thus, given the proper source morphisms, these define source endomorphisms
without a coefficient denominator. This is not an assertion that they are
primitive idempotents or give saturated, principally polarized embeddings.

The CM coefficient satisfies a^2-a+2=0. Under the direct convention the
twisted roots are -a t^4 and -(1-a)t^7. Under the inverse convention they
are -a t^6 and -(1-a)t^3. The four annihilators are polynomial identities.
JOINT39 also checks the full norms against the retained degree-eight
Frobenius polynomials with X replaced by -X, including the resulting swap.
Pi7 kills one branch and is nonzero on the other under either convention;
Pi8 does the reverse. The convention changes which branch name survives.

The nonzero evaluations occur in Q[a,t]/(a^2-a+2,Phi10(t)). Interpreting
them as isogenies on the simple CM factors uses the retained CM field/type
identification. With that decomposition, the connected image of each Pi
has dimension four. This image construction is conditional on that bridge,
not a new computed normalization, an explicit target isogeny, or a proof
of the full integral Hom lattice.

On rational order-11 points, the twist changes Frobenius from F to t^5 F.
The old untwisted collisions do not stay at rational Frobenius eigenvalue
one. Native enumeration gives the two twisted fixed lines t=7 and t=8
(up to inverse convention), with no common rational joint system.
On them the actions of (Pi7,Pi8) are (9,0) and (0,9), respectively.
Consequently [5]Pi7(R)=R on the observed nonzero order-11 class with tR=7R,
provided the f,t source-morphism interpretation is bound as above.

DIAMOND39 evaluates the full signed divisor action and all eleven scalar
possibilities and finds tR=7R. The prior proposed expectation 6 fails.
That is a corrected expectation, not a failed computation, nor a reason
to alter the sealed untwisted R18 result. Full target sign naming must
follow an explicit convention and model certificate.

Relevant sources: retained R18 CM.md and archived CM20 joint-system JSON;
R33 quadratic diamond twist; R37 actual eleven-class and complementary
Euler polynomial; JOINT-PROTOCOL.md and joint39-01; the universal identities
in lean/TwistOperators39.lean. No new external novelty claim is made.
