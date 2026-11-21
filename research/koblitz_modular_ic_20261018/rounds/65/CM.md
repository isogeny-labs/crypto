# R65 first descending Frobenius lattice

The accepted `first-frobenius65-01` computes the complete 160-dimensional
`V0/O` quotient for the trace-dual lattice `V0=G^-1 F2[z]^32`. The 32 by 32
trace Gram determinant agrees exactly with
`z^76 (z+1)^24 (z^5+z^3+z^2+z+1)^12`; both Gram inverse products pass.
All 160 quotient classes are squared with the full source multiplication
table. Clearing complete denominators gives 241 binary remainder rows of
rank 80, hence an 80-dimensional square-survivor kernel. Every remainder
is reconstructed from its binary coefficients, all kernel products vanish,
and each survivor is checked again in whole-field coordinates. An altered
kernel is rejected. The exact terminal result line and payload hashes are
in `results.json`.

The accepted `lattice65-01` reconstructs the full PID lattice generated
by the old order and all 80 survivors. It has rank 32, with
`dim_F2(V1/O)=80` and `dim_F2(V0/V1)=80`. Both module inclusions and both
basis inverse products pass. The square of each of its 32 basis columns
lies in `V0`, and a changed inverse is rejected. Thus this is the first
complete descending Frobenius step. It is not a stabilization test or a
proper normalization.

Both attempts finished with exit 0, exact success marker, and no Singular
question-mark error lines. `RECORD_RESULTS.py` binds twelve run-source
copies to the historical Git bytes. The saved quotient, kernel, survivor,
and lattice bundles are the inputs for the next complete step.

Next compute `V2={x in V1 : x^2 in V1}` on a complete basis of `V1/O`.
Iterate the same exact quotient/kernel/lattice calculation until a step
has zero quotient, then identify exceptional branches and actual poles on
the proper level-77 model. The resulting divisor data will determine the
complete Riemann--Roch search space for a map with a checked order-11
action.
