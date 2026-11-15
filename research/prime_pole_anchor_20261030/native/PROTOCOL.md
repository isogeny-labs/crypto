# Prospective exact development verification

Fixed construction, no parameter search or timing experiment. Before compiling:
L=4,8,16; S=[-L,L-1]; alpha=2L. Build original puncture polynomials and
subtract C(T)/(C'(0)(1-delta)) exactly when delta is even. Check every
coefficient of every integer-node scaled chart for2-adic integrality,
all nodal values, and all odd-node crossing charts for a valuation gain.
At odd nodes use precisions8,12,16 and zeta=-1,0,1,3; independently reconstruct
all equal-parity matrix entries with the two-pole geometric moment expansion,
checking weight b+2 and omitted coefficients at the precision boundary.
An attempted even-denominator modular inverse must be rejected.
Retain all compile/run failures. No complete recursion, ECDLP or speed claim.

Version2 extension, frozen after successful odd-only verification and before
the next execution: reconstruct ALL nonzero active levels v2(t)<log2(L).
Retain weight b+2 at odd levels and b+1 elsewhere; assert coefficient factors
are integral after explicit power-of-two cancellation. Also require nonzero
reconstructed values and entries changed from the original, preventing a
vacuous zero certificate. Preserve the odd-only source/output unchanged.
