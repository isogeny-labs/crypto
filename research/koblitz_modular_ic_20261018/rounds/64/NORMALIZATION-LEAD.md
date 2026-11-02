# Descending Frobenius lattice for the actual degree-32 affine order

Status: proposed exact construction, with no new normalization computation yet.
Primary source: Singh--Swanson, *An algorithm for computing the integral
closure*, Theorem 1.1 and proof,
https://www.math.purdue.edu/~iswanso/intclSinghSwanson.pdf . Their theorem
starts with a conductor over-lattice and repeatedly retains elements whose
characteristic-2 square remains inside the current lattice. The construction
below specializes that argument to the rank-32 order and uses its trace dual
as a tighter initial over-lattice. This is a mathematical reduction, not a
claim that the existing `normal`-named basis is integrally closed.

Let A=F2[z], K=F2(z), and let O be the finite free rank-32 A-order encoded by
all 528 R61 multiplication columns. First check, as source obligations, that
the coefficients really lie in A, that 1 is in O, that the 528 products close,
that the trace form is nondegenerate over K, and that the generic algebra is
identified with the *actual* degree-32 function field. R61 checks the
product/trace tables, but the last field identification and the F2 twist
coordinate bridge need an explicit audit before interpreting any output as
the normalization of the target source curve.

Write G for the trace Gram matrix and D=det G. The R53 factorization predicts
D=z^76(z+1)^24 f_C^12 with deg f_C=5, hence deg D=160. Recompute this from
the exact R61 G in the new checker. Let V0=O^vee=G^-1 A^32 in the O basis.
Every element integral over A lies in V0: its product with each element of O
is integral, so its field trace lies in A. The adjugate identity gives
D V0 subset O; therefore D is a valid conductor element and V0/O has F2
length 160 if the displayed discriminant check passes.

Iterate V(e+1)={g in V(e) : g^2 in V(e)}. This is an A-submodule in
characteristic 2. The integral closure lies in every V(e). Since every V(e)
contains O and V0/O has finite F2 length, each strict step drops the length
by at least one. At a fixed point, every 2^k-th power of each element remains
in V0, and Theorem 1.1 applied to the conductor over-lattice D^-1 O gives
integrality. Thus equality is an exact stopping certificate for the affine
integral closure, subject to the checked source hypotheses.

For a complete step, represent V(e) by a 32x32 K-basis M(e) in O coordinates.
Compute the polynomial inclusion J(e)=M(e)^-1 for O subset V(e), and a
complete F2 basis q_1,...,q_L of V(e)/O by univariate PID reduction. Here
L<=160, so this is a finite quotient, not a coefficient-box search. The
square map on the quotient is well-defined as an additive F2 map to the
ambient field modulo V(e), since changing g by o in O changes g^2 by o^2 in
O. Use the 528-product table to calculate each q_j^2 in the field, clear all
denominators of its 32 coordinates relative to M(e), reduce modulo that
denominator, and flatten all polynomial remainders to binary rows. The full
F2 kernel is exactly V(e+1)/O. Reconstruct a PID basis of O plus the kernel
lifts and verify both inclusions, every square identity, and the new quotient
length. Recompute the binary rank independently, as in R62. Stop only on
kernel dimension L, with source-bound full-matrix identities and negative
controls; record every unsuccessful step and resource cap.

The first data gate should use only the exact 32-column R61 order and trace
Gram, with the 160-dimensional quotient as the maximum input width. If the
field bridge fails, repair that bridge before claiming target geometry. If
the affine normalization succeeds, compute the z=0, z=1 and f_C local
branches/residue fields and the chart at z=infinity. Their valuations of
b,c,lambda,nu and registered denominators are then the inputs for the
complete Riemann--Roch search and actual level-77 map. Normalizing one
affine chart alone does not prove the proper model or its polarization.
