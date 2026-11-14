# Coordinate-field and tiny-support contract

This is a bounded oracle experiment for a representation, not a new elliptic
algorithm, lattice-reduction benchmark, global solve, novelty or speed claim.
The two fields are F3[theta]/(theta²+1) and F5[theta]/(theta²+2), with low-degree
coordinate first. Absence of roots in the prime field proves these quadratic
moduli irreducible. Every multiplication is checked against separate direct
pair-formula tables. Field labels are not integers modulo their cardinality.

For five arbitrary field features, the equations t=sum(a_i*x_i),
u=sum(b_i*x_i) define 4 independent prime-coordinate equations in 14 integer
coordinates because the two output blocks contain identity matrices. Their
kernel has index p^4. The supplied primal row basis has ten free feature rows
and four p-output rows and determinant p^4. Dual functionals obey ten coordinate
congruences lambda_i=M(a_i)^T gamma+M(b_i)^T delta; the free lambda coordinates
make these equations onto, so this dual lattice has index p^10. Its ten
p-lambda rows and four free-functional rows have determinant p^10. Row
membership and absolute determinant equal to the proven index certify equality
of lattices. Membership alone is insufficient: doubling one row retains
membership while increasing index. Exact elementary row additions and swaps
preserve the lattice. Positive diagonal scaling followed by exact division
transports the metric representation; this experiment does NOT implement or
benchmark an LLL repair. Both increasing and decreasing weight patterns are
checked. Exactly 128 basis transformations across the two fields are allowed.

For affine constants a0,b0 the linear functional applied to the equations gives
c+sum(lambda_i dot x_i)-gamma dot t-delta dot u=0 mod p, where
c=gamma dot coord(a0)+delta dot coord(b0). The oracle checks all 3^5 feature
tuples in the fixed domain D={0,1,theta} in each field for one declared
functional. These are representation checks, not all guarded curve charts.
An explicit nonsymmetric multiplication matrix detects the wrong transpose.
The cache fingerprint includes field characteristic, modulus, degree, ordered
basis, coefficients, offsets and domain; changed modulus or basis must differ.

The root contract is every ordered triple in D^3 whose monic product equals a
given monic cubic. A candidate repeatedly divides by (X-d) for each d in D in
field arithmetic. The remainder theorem makes each accepted division sound;
division repeats so repeated roots are retained. Unique factorization ensures
every D-supported cubic is completely removed, and any positive residual
degree means no D-supported triple. All distinct permutations give precisely
the ordered triples. This is compared, for EVERY q^3 monic cubic in each field,
to a separate oracle: enumerate the 27 original ordered triples and compute
their coefficients with closed elementary symmetric formulas using table
arithmetic. The oracle does not call candidate division or convolution.
Thus 729+15625=16354 complete support decisions are independently compared,
including repeated, empty, and partially supported cases in characteristic 3.

GF4 with theta²=theta+1 gives encoding(theta²)=3 but 2² mod4=0. This refutes
transporting integer-cubic support to extension coordinates. No binary elliptic
chart is tested. Prime interval bounds and integer Sturm isolation are not used.

All predicate checks are capped at 200000; basis transformations at 128.
The worker has an 80-second internal work cutoff, serialized evidence before
completion acceptance, 84-second final demotion, and 88-second print boundary.
The runner's 90-second process-group deadline covers startup and final stdout;
4GiB and one core are external worker limits. An exception/cap reports
inconclusive and no original exact root count. Root sets are serialized into
exclusive-create raw evidence before the completion flag is accepted. Maximum
raw result size is bounded by 16354 rows and 54 total ordered triples across
the two fields, independent of any hard-instance size.
