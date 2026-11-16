# One cached exact square decomposition for Cartier and unit saturation

PROPOSED next native experiment; not executed in R61. Reuse the accepted
actual degree32 F2(u) function field, rather than fit square roots on fibers.
This supplies a common exact operator for the two new construction paths.

## A finite matrix with complete certificates
Take the32 canonical monomials e_i=lambda^i*v^j (0<=i<16,0<=j<2), after
checking that they are exactly the standard monomial basis of the actual
source ideal. Form the32x32 matrix S whose columns are the full source
normal forms of e_i squared. Compute an inverse over F2(u) and require BOTH
S*S_inverse=I and S_inverse*S=I. Rebind every column in the full function
field and reject a changed matrix entry. Rank failure is an audit failure
or a separating-coordinate/basis issue to diagnose; never drop columns.
No integral, local-unit or projective-regularity claim follows from this
rational matrix inverse.

For an arbitrary exact source function f, solve a=S_inverse*coords(f).
For each rational coefficient a_i=N_i(u)/D_i(u), split the polynomial
N_i*D_i into its even and odd powers:

    N_i D_i = E_i(u)^2 + u O_i(u)^2.

Over F2 this is literal exponent halving. Require the polynomial identity,
then a_i=(E_i/D_i)^2+u*(O_i/D_i)^2. Set

    g=sum_i (E_i/D_i)*e_i, h=sum_i (O_i/D_i)*e_i.

Check the WHOLE-SOURCE identity f=g^2+u*h^2, including every rational
coefficient and denominator. This is a complete linear solve plus explicit
coefficient splitting; it does not use a root coefficient box. Over F32,
coefficient square roots use c^16 and must be checked in the actual model.
The matrix S itself has F2 coefficients, so the same pinned source matrix
can be base-changed once that field binding is certified.

## Two applications of the same checked decomposition
For differential f du, the proposed Cartier output is h du. Before calling
this a source Cartier matrix, certify that u is separating and that the
completed holomorphic basis, differential-coordinate conversion and output
regularity are correct. If using the trace-dual z=1/u description, input
f dz becomes (f/u^2) du; convert the result back with du=u^2 dz in
characteristic2. Check this conversion explicitly, including infinity.

For a supported unit f, h=0 produces an actual square root g, verified by
g^2=f. Check its COMPLETE divisor and allowed support before adjoining it
to the unit lattice. A nonzero h rules out a square using the certified
unique decomposition. The same cached inverse can process all unit words
in the logarithmic-derivative kernel; odd saturation is a separate test.

## Frozen controls required before execution
Use all32 basis elements as full-source reconstruction controls,1 andu^2
as square controls, u as a nonsquare/separating-coordinate control, and a
changed S entry as a matrix-certificate negative. Retain every inverse,
rational split and field residual. Native Rust/Singular only; same1200s,
4GiB/one-job/one-thread limits; freeze source, coefficients, donors and
control acceptance BEFORE data. No accepted earlier producer is repeated.
The61 holomorphic columns and generated-unit lattice remain subsequent
source-specific inputs, not implied by this proposed matrix construction.
