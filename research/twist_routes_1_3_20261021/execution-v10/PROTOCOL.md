# Twist torsion to a source endomorphism — post-v8/v9 diagnostic

Before data: on the same public p827 source, test the candidate alpha=(pi+1)/7.
The already verified full twist7-torsion lies in ker(pi+1); its order833=49*17
suggests a degree17 quotient. This is standard endomorphism/isogeny theory, not
a novel attack. Review before implementation: primary MIT18.783 Fall2025
Lecture5 Theorems5.11/5.15, Lecture13 Theorems13.6/13.8 and surrounding proof;
repository RESEARCH_ISOGENY_LOOP_GLV.md and atlas
LEAD-STR-glv-gls-isogeny-loop-scalar-mult. Known-scalar speed is not DLP speed.
Both primary PDFs were accessed/read; no claim of literature novelty.

Use the unchanged source y²=x³+x+15/F827, order823,G=(2,5),public37G and twist
y²=x³+4x+120, maximal seedH=(2,386),order119. The fixed kernel is generated
by[7]H,order17. Embed it anti-Frobenius, enumerate all17 points/eight paired
x-roots, build normalized Vélu, verify the exact degree17 curve identity,
kernel/poles/infinity and compare with independent GP ellisogeny. Require
the codomain coefficients to be u^4*a,u^6*b for fixed u=7^-1 in F827; rescale
by7 to an actual endomorphism of the source. No choice of sign based on output.

Represent generic points as (X(x),y*S(x)), with y²=x³+x+15. Compute the conjugate
id-alpha by the group law. Require both compositions with alpha equal[17],
and require [7]alpha=pi+id as exact rational-function identities. The latter
uses pi(x,y)=(x^827,y*f(x)^413). Separate field/rational-function controls use
F101. Preserve an error or300s cap without dropping the generic identity.

Check every823 rational source point, addition byG, scalar action353 and
inverse415; reject scalar354 on all822 nonidentity points. Check all833 embedded
twist points: kernel17, image49, and image killed by7. Independently repeat all
generic and finite checks in GP; use the conjugated rational map on the twist,
(X,Y)->(2*R(X/2),Y*S(X/2)), without needing a shared extension implementation.
Record multiplicative order of353 mod823 and verify it independently. No claim
about cheap orbit folding follows just from a finite permutation.

One Rust producer and one independent GP execution, each300s/4GiB/CPU0/one
thread,L0 on existing managed host. The existing documented EKS access/image
fallback applies. No paid resource, unknown key or NIST target. Frozen v8 field,
polynomial and map helpers are copied byte-for-byte as a versioned prefix;
their original source stays untouched. Freeze source/binaries/input/protocol
and Setec gate before main data. Native output, exact scripts/receipts, failures,
reports and raw archive remain separate from v8/v9. A conditional Lean ring
lemma will check the norm-relation implication; concrete curve/endomorphism
bridges remain explicit. Scalar action and endomorphism identities do not
establish a relation-solving or complete IC improvement.
