# Faithful quartic discriminant proof passes

`quartic-factored-lean-v6` passes its pinned Lean 4.34.1 build and
transitive-axiom gate on
[orbit-v3/QuarticDiscriminant.lean](../../proof/orbit-v3/QuarticDiscriminant.lean).
The gate audits nine declarations: seven named theorems and two local
associativity/commutativity instance definitions. There are zero failing
and zero trusted-native declarations. The mathematical proofs use only
`propext`, `Classical.choice` and `Quot.sound`; the two instances use no
axioms. [Actual logs](../../results/proof-checks/quartic-factored-v6/)
retain the source hash, build and complete axiom listing.

![Separate algebraic and physical obligations](../../figures/joint-mechanism.svg)

The proof preserves the universal commutative-ring statement: the standard
quartic discriminant evaluated on four elementary symmetric coordinates
equals the product of the six squared root differences. It also proves
that a nonzero value implies all six pairs of roots are distinct. It
does not assume an integral domain or a converse for rings with zero
divisors.

The earlier direct expansion exceeded the unchanged 8 GiB allocation cap.
The new proof first factors addition of a fourth root using cubic
coefficients, proves the cubic discriminant and cubic evaluation identities,
then combines them using small square identities and associativity /
commutativity. This changes proof organization, not its conclusion or
hypotheses. Failed combined/v4 builds and the pre-execution v5 PATH failure
remain preserved. The successful run uses one worker, the busy reservation,
8 GiB and a 900 s watchdog.

This closes the specific polynomial factorization/nonrepetition proof gap.
It does not prove finite-field membership, correspondence of the complete
elimination ideal with physical relations, complete root enumeration,
matrix rank, DLP recovery or a complexity/speed claim. Those bridges remain
explicit in the [proof map](../../proof/PROOF_MAP.md).
