# Actual CM19 C7 curve maps: six automorphisms and elliptic quotients

Source3916f563ce5717f50ee1716ac10a9c979e5f656c; all22 complete frozen
inputs read back before execution. All7 strict phases0 and guards[],
unchanged2GiB/120s/one worker. Full38330B stdout SHA256
343d44f261c12fdacd2469ad8e53f5adcd4641bcd0593679b1af96a3a158e443
at first-B-native/curve-maps.stdout, all38 small raw records retained.
Unsigned native actual-curve correctness qualification. [pcc:unverified]

All57120 distinct ordered images of the fixed projective frame are
classified, with complete2-bit decisions retained:20304 non-general,
36810 point-set rejects,0 coefficient rejects,6 accepted. All accepted
maps have full15 homogeneous coefficients and actual17-point permutations.
Their orders are1,2,3,2,2,3 and the full multiplication/inverse table is
printed. Completeness is for F7 projective/curve automorphisms, using the
canonical embedding; extension-only automorphisms are not enumerated.
[pcc:unverified]

The first order3 map and a deck involution are

```text
R=[[1,5,2],[0,5,0],[2,1,1]], T=diag(1,1,6).
```

Three actual degree2 maps q0 o A to E7:y^2=x^3+2x+6 are obtained
from all deck-conjugate involutions. For q0 and Y!=0,
xE=X/Y, yE=(Z^2+6X^2+3XY+Y^2)/Y^2; Y=0 maps to O.
Each cover has all17 actual images, full coefficient pullback identity,
involution invariance and all11 target fibres checked:8 size2,1 size1,
2 empty. The real two infinity preimages are retained. No scalar image
labels enter these functions. [pcc:unverified]

For the fixed R, the actual eigenbasis columns give

```text
P=[[1,1,1],[5,0,0],[0,1,6]], original coordinates=P*(x,y,z).
Cnormal: x^4+3x^2yz+2xy^3+2xz^3+2y^2z^2=0.
u=yz/x^2, v=y^3/x^3, w=4v+1+3u+2u^2.
Q: w^2=1+6u+6u^2+3u^3+4u^4.
```

Both fixed points at x=0 are explicitly mapped to the two quartic
infinities with signs-2/+2. Full homogeneous degree8 identity, target
smoothness and actual altered-constant/restoration controls pass.
The degree3 cover has5 size3 fibres,2 ramified size1 fibres and4 empty
fibres on E7(F7), all17 actual points and orbit invariance checked.
[pcc:unverified]

The generic quartic conversion produces generalized Weierstrass
(a1,a2,a3,a4)=(5,4,6,2), short model(a,b)=(4,6), and scaling3 to
E7. Its full polynomial identity modulo w^2-f is checked, with an actual
target constant mutation rejected and pristine restoration passing.
All11 quartic points and11 E7 points have actual forward/inverse
roundtrips, including both infinities and finite x=0 exceptions.
The E7 fixture has121 addition/1331 associativity/11 order controls.
No artificial group structure is assigned to quartic points.
[pcc:unverified]

These actual covers give genuine Jacobian pushforward/pullback
correspondences; the standard written composition=[degree] makes each
pullback injective on11-torsion. The current native driver evaluates curve
maps and elliptic laws, not a full genus3 divisor group, generic Mathlib
theorem or CM19 nontrivial S divisor correspondence. Those constructions,
independent replay and complete strongest-reference ECDLP costs remain
open. No exponent gain is inferred from the larger ambient Jacobian.
[pcc:unverified]

The first35999B formatting negative is retained unchanged. Original whole
4799552B ELF remains temporary approved-custody pending. The complete
38KB raw respects the existing artifact threshold; parent5019's separate
82KB raw migration requirement is not waived. Existing owners/paid STOP,
vault lock, formal, CI and merge requirements persist. [pcc:unverified]
