# R64 exact compact-order source bridge

Fresh `bridge64-04` passes native exit0, its exact marker and the no-error
gate. It maps the retained R53 source into the actual R61 degree-32 field
under `z=1/u`, `V=z²v=v/u²`, and old `lambda=l`. Both old source equations
reduce to zero in the full actual source quotient. A changed quadratic
coordinate is rejected. The coefficient map explicitly sends old `z` to
`1/u`.

The full compact PID matrix is constructed in its source ring before
transport. Its original identity `H*sourceLift=D I` and the transported
identity both pass. All 32 retained basis columns are evaluated as actual
source functions and individually reduce to nonzero elements. Their
canonical 32 by 32 coefficient matrix has rank 32 and a checked inverse
on both sides; an altered matrix entry is rejected. The 89 KB source-bound
bundle saves the actual source equations, mapped old equations, 32
functions, coefficient matrix and inverse. These exact checks identify
the generic rank-32 order with the actual source field, subject to the
retained R51/R53 source-order certificates. They do not compute its
integral closure or proper model.

Three earlier attempts are retained as rejected, even though each printed
a late marker. Attempt01 cast a source-ring module after changing rings,
which Singular could not resolve. Attempt02 mapped the module before
casting and obtained a zero fourth image. Attempt03 imposed the original
matrix identity on that changed representation and rejected it. Attempt04
uses the accepted R53 construction order: cast `basis51` in the compact
ring, then transport the already constructed matrix. `results.json`
checks all four terminal logs and 24 historical source byte pins.

The next exact step is the descending Frobenius normalization in
NORMALIZATION-LEAD.md. The retained trace Gram discriminant predicts a
160-dimensional F2 quotient between the affine order and its trace dual.
Build a complete quotient basis and square map, solve its full binary
kernel, reconstruct the new PID lattice, and iterate to equality. Once
the affine normalization is certified, calculate branches and valuations
on the `z=0`, `z=1`, `f_C` and infinity charts. Those actual divisors feed
the complete prescribed Riemann--Roch space for level-77 maps and
subgroup action.
