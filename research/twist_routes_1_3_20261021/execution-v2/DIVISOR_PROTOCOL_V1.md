# E19 actual degree-three transfers and E24 independent classes, version 1

Freeze date: 2026-10-23. Commit the native Rust/CAS source, index and this
protocol before the panel. Retain every failed chart, assertion, external
error, missing class and resource cap. The original three-route requirements
remain open; this pilot is correctness work with no speed claim.

E19 consumes the four independently checked explicit geometry summaries
listed in GEOMETRY_INPUTS.json. Choose their first retained parameter
presentation, without treating other presentations as distinct curves.
Run actual divisor arithmetic only for the three toy curves p=211,751,10111;
public P-521 remains parameter/geometry only. Use each registered Fp6 tower
and its exact registered flat-field presentation. This does not introduce a
new curve/extension slug or compare methods across extension identities.

Find a simple branch root r of the first degree-three map's cubic pole
polynomial over Fp6 using at most 256 deterministic quadratic-character
factor trials. For the sextic y^2=F(u), let lambda=F'(r), and use
X=lambda/(u-r), Y=lambda^2*y/(u-r)^3. Verify the resulting polynomial
X^6*F(r+lambda/X)/lambda^2 is monic of degree five. Pull a finite elliptic
point back by its cubic fiber and y-map congruence, transport and reduce the
divisor, and subtract the actual origin-fiber class. Handle the old sextic's
infinity fiber explicitly when the cubic loses one degree; higher
ramification outside this adapter is an unresolved assertion, not discarded
evidence. The source origin pulls back to the identity.

Check norm-after-pullback equals multiplication by three for every rational
source point, every inverse, every translation by the source generator,
nonzero image and exact prime subgroup order. On eight fresh E19-domain
public toy targets per curve (labels 0–7 under the frozen E21 hash/rejection
rule), solve the actual destination subgroup logarithm with cold native
baby-step/giant-step receiving only the two divisor classes and subgroup
order. Verify its scalar back in the source group and recover the original
point from its norm using inverse three modulo the source prime. This is a
toy transfer correctness experiment, not an IC speed comparison.

Implement a generic norm on arbitrary reduced degree-at-most-two divisors
by rational-map evaluation in Fp6[X]/U, deleting actual pole factors, summing
the two conjugate images via their slope/trace, and subtracting the degree
times the chart base point's image. Explicitly treat constant-x, opposite,
doubled and degree-one/identity cases. An independent PARI implementation
must replay all emitted reduced-divisor equations, norm compositions and
one deterministic addition partner for every source pullback.

For E24 regenerate the exact original ten selected degree-two covers and
the original 5,855-class population: all 3,551 rational Jacobian classes on
p=59, plus 256 original sampled classes on each other fixture. Native
generic norms must agree with both original specialized norms. Emit all
classes and verify both norms, every inverse, and one deterministic arbitrary
addition partner per class in an independent PARI implementation of Cantor
composition/reduction and quotient-ring norms. This closes an independent
implementation replay obligation only after accepted raw CAS output; it is
not an independent proof of the group-law theorem or host replay.

PARI must independently check the registered field polynomial's
irreducibility and the exact tower-to-flat generator identities. Native
Rust solves the six-dimensional change-of-basis equations and verifies them
before emitting coordinates. Reject any CAS error even with a zero exit
and completion marker. Use the native hash/error adjudicator.

Use one thread, 4-GiB virtual address space and a 3,600-second cap per native
batch or per-curve independent CAS replay. Checkpoint after every curve.
Record high-level unpriced calls only as diagnostic units. Balanced
base-field arithmetic, polarized matching to a particular E18 torsion
graph, formal chart/group maps, complete pricing and full E19/E24 admission
remain open even if every correctness check passes.
