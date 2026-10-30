# KUMMER-N14 concrete theta model extension (frozen before execution)

Study ID `KUMMER-N14-CONCRETE-GATE-20261025`. This is a bounded diagnostic,
not a new collector cohort. Inputs are the 18 frozen odd-prime subgroup models
at bits 20, 24, 28, 32, 36, and 40 in the existing theta, coordinate, and
target-gate records. Exactly one existing `target_index=0` row identifies the
generator per model. Its source scalar is not used to choose states.

For each model of order `r`, inspect the two Kummer differences `K(0)` and
`K(G)` and four theta inverse constants. Record every zero coordinate and
constant. Take the unique signed scalars in
`{0,1,2,r-2,r-1} ∪ {floor((2i+1)r/2048): 0<=i<1024}` in ascending order.
For each signed `D=kG`, project `D,D+G,2D,2D+G,2D+2G` with the existing
torsion-repair projection; execute all three existing cached theta xADD calls
and their repair branches, then compare each output projectively with the
independent Jacobian oracle. Execute the existing signed lift on the pair
`K(D),K(D+G)` and compare the returned Mumford divisor with `D`. Count all
field operations for the lift with the existing module counters and charge
Jacobian operations separately. The source's fallback runs as written; any
timeout or resource stop is a recorded incomplete result, not a zero cost.

The per-state CSV retains scalar, projection status, all xADD statuses,
repair roots, lift status and correctness, and operation counters. The JSON
summary retains exact input/source digests, model-level denominator guards,
counts, and arithmetic totals. A native independent validator checks row
uniqueness, sample-set completeness, all totals and digests. A graph and PDF
present the admitted rows without promotion. No elapsed-time or rho claim
follows. Finite sampled states do not establish a concrete all-state bound;
there is no 1,260-target collector or matched rho run in this packet.
