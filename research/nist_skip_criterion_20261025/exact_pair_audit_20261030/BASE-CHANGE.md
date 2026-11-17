# What a public G/b scalar relationship would mean

Consider the same prime-order cyclic curve group of order n, with nonidentity
points Q and G. Suppose a public recipe yields G=[s]Q, where s is known and
nonzero modulo n. If the recipe uses b, its interpretation as a scalar in
Z/nZ must be specified: b is originally a field coefficient in Fp.

Given an ordinary DLP target P=[k]G, compute

    P'=[s^(-1) mod n]P=[k]Q.

A DLP solver for base Q then gives k. Conversely, given R=[k]Q, compute

    R'=[s]R=[k]G,

and a solver for base G gives the same k. The maps are publicly computable
and invertible on this group. They convert between the two DLP formulations;
they do not supply the destination solver. This is an algebraic equivalence,
not a measured attack or a general security proof. If s=0 modulo n, the
recipe produces the identity and fails the nonidentity-generator premise.

Knowing a single scalar relationship does not by itself supply a method for
solving arbitrary target logs. A proposed hidden-witness mechanism still needs
a concrete additional solver/property and a public-versus-secret cost analysis.
This observation does not rule out unrelated implementation or group weaknesses.

The four supplied points were assigned by the first-x/smaller-y rule. Their
coordinate relations follow from the defining equation for a=-3:

    b=y²−x³+3x (mod p).

Thus x=0 gives y²=b, and x=1 gives y²=b−2. These are field identities, not
scalar identities modulo n. A relation among coordinates is not automatically
a known discrete logarithm. Assigned canonical points are not evidence of
historical unpublished generators, which remain unknown.

There is a standard example of a public scalar recipe: RFC5639AppendixA.2,
steps12–13, chooses a smallest-x point and multiplies it by a derived scalar.
The RFC's following note discusses possible small-coordinate side channels.
That implementation concern is separate from the ordinary ECDLP equivalence
above; neither source proves a side-channel weakness for these candidates.
The RFC is a later Brainpool construction, not evidence of the1997NISTcaller.

Primary source checked directly30October2026:
https://www.rfc-editor.org/rfc/rfc5639.txt, AppendixA.2.
Retrieved file SHA256:d19949766009ac82b69e46b09f3a08d712908381300a340d3fa3b1478f4b215e.
The existing conditional Lean equation proofs and the earlier100548trait-cell,
19152envelope and880recipe comparison remain unchanged. No additional DLP
computation, scalar search or machine-checked theorem is claimed here.
