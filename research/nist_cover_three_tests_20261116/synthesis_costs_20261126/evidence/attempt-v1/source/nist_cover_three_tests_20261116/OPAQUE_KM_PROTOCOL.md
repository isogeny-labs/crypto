# Opaque target class to divisor and reverse map

Frozen 2026-11-17 before new decoder data. User-selected goal remains map
completion for the same P-192 genus3 construction. The preceding map PR4496
has merged. This unit closes the input-representation gap recorded in its
MAPS_STATUS report;it does not rewrite that report or its pins.

## Literature and resources

Primary source read:K.Khuri-Makdisi,"Linear algebra algorithms for divisors
on an algebraic curve",arXiv:math/0105182v3,sections2/5,especially
representation Lemma2.1,division Lemma/Algorithm2.3 and RR Algorithm5.5.
<https://arxiv.org/abs/math/0105182>.
The source PDF/text were downloaded and inspected,not inferred from a snippet.
Existing quartic KM,complete_km.cpp,actual-kernel cubic/residual witness and
moving_transpose.cpp supply the fixed arithmetic. No novelty claim.
KB unavailable:MCP absent and wrapper unavailable. Lean/Cargo/Conductor
unavailable;readable RR arguments and native checks are not compiled or
independent verification. All statements require PCC registration.

Native local development qualifier only:each invocation600seconds/1GiB,
no timing measurement or paid campaign. Remaining paid budget unknown.
Immutable copied sources/protocol/input/binary pre-call hashes,new evidence
directory per attempt,preserve all failures and post-call manifests.

## RR extraction

Let D0=3*H_pole have degree12,Eref degree3 and g0 the pinned cubic with
divisor3*Eref+R (R degree3,squarefree and disjoint). Recheck this complete
degree12 intersection and both incidence/jet conditions.

An opaque input represents alpha=[D-D0] by W_D=L(3D0-D),dimension22.
Validate dimensions and membership in the actual34-dimensional ambient space.
Negate it with unchanged KM arithmetic to obtain W_Dminus for
[Dminus-D0]=-alpha.

Put B=D0+Eref,degree15. Construct L(B),dimension13,as
N/(ell_pole^3*g0),where N is a degree6 plane section vanishing on
2*Eref+R. Construct M=L(3D0-B)=L(2D0-Eref),dimension19.
Use the division algorithm

    L(B-Dminus) = {f in L(B) : f*M is contained in W_Dminus}.

M has degree21 and is base-point free;Lemma2.3 applies. RR guarantees a
nonzero section because deg(B-Dminus)=3. Recover its numerator N from its
evaluation vector and check every reconstructed value. Recover the degree9
numerators of all22 sections of W_Dminus and replay their evaluations too.
The product embedding is checked in the actual ambient space.

The zero divisor of N is Dminus+2*Eref+R+E,where E has degree3 and
[E-Eref]=alpha. Extract it without assuming Dminus known:
move the projective chart Z'=Z+aX with a=0..24 until the complete N/quartic
intersection has affine degree24. Then use X''=X+lambda*Y,Y''=Y in that
chart,lambda=0..308,until the intersection is separated and unramified.
Reconstruct its y-coordinate first on squarefree support via residue gcd/CRT,
then Hensel-lift to the complete multiplicity polynomial. Require both
transformed equations on the full length24 algebra.

The gcd of that support with all22 transformed W_Dminus section numerators
is exactly its degree12 divisor:their residual degree24 line bundle has
no base points (Lemma2.1). Divide by this support and the projected
2*Eref+R support with full multiplicities. The quotient must have degree3.
Recover original x,y by the inverse projective transformation and check
the original quartic. Recheck irreducibility of the actual degree4 infinity
polynomial:then any rational effective degree3 divisor E is affine.
Normalize E into the existing separating affine cycle representation.

Re-encode E-Eref with the completed KM adapter and require equality with
the original opaque input. Then apply the existing line-moving closed
transpose to E-Eref and return a reduced source Mumford class. Constructors
receive only the opaque class and public setup,never a source scalar or source
class witness.

## Deliberate chart failures and fixed cases

For the primary case,first try a_bad=-1/x_ref for the first fixed reference
point (require x_ref nonzero). Since N vanishes there,this chart must drop
degree and reject. After selecting a valid chart,first try the shear that
collides the first two fixed reference points;require distinct transformed
y-coordinates and retain this rejection. Then use the unchanged schedules.
These are extra deliberate bad candidates,not replacement inputs.

Primary qualifier:opaque class of the pinned public pullG image. Require
decoded/re-encoded equality and reverse output exactly3*pullG.
Only after that implementation check passes,run the full fixed panel:
identity,pullG image,its KM double,the pinned generic source-point image,
and the actual target rational point class at retained point1. The first
four reverse results must be0,3*pullG,6*pullG,3*P respectively;the last
must equal the direct transpose of point1 minus the pole-fiber anchor.
Inputs remain opaque KM spaces throughout extraction. No scalar recovery.

Reject an off-ambient input-coordinate change,a changed decoded y,and an
altered reverse result. Retain all input class spaces,RR dimensions,
chosen numerator,intersection/base/Dminus/degree3 supports,chart decisions,
reconstructed cycle,re-encoded class and reverse answers. No universal
performance claim or implicit dropping of exceptional multiplicity.

## Scope

The readable correctness argument uses RR/base-point-freeness,exact divisor
intersection and the previously established correspondence. Projection/chart
schedules have geometric finite-avoidance bounds (24 support points,at most
276 collision directions,24 tangent directions and4 forbidden centers).
The reverse moving/factor routines retain their separately declared execution
caps and universal-certification qualification. No signed independent verdict
or faithful Lean formalization is asserted.
