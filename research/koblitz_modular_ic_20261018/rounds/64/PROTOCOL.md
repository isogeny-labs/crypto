# R64 exact coordinate bridge, frozen before data

Test the proposed map from the R53 compact PID order to the actual R61
degree-32 field: `z=1/u`, old `V=v/u²`, and old `lambda=l`. Read the full
R53 old source equations, all 32 R51 compact basis columns and denominator,
and the actual R61 source ideal from their accepted raw files. Require
actual quotient degree32. Map and reduce both old equations against that
actual ideal; change the quadratic coordinate by `1/u` and require failure.

Map every one of 32 compact basis columns, including its full denominator,
to an actual source function. Extract all 32 canonical coefficient columns,
prove full rank32 and both inverse products, and reject an altered column.
Preserve a complete source-bound serialized bundle before the terminal
marker. No subset, coefficient-box, sampled fiber or assumed lambda
identification substitutes for the two source equations and all 32 basis
functions. A failed map is a result requiring diagnosis, not proof that no
other coordinate map exists.

The original local math scope applies: one Singular/FLINT thread, one math
job, 1200 seconds, 4 GiB virtual, deterministic seed64. Exit0, exact marker,
and no whitespace-question-mark error line are jointly necessary.
Preserve full raw output and rejected attempts. Freeze source, protocol,
drivers and hashed donor manifest in Git before data; all copies must bind
to the historical Git revision. Use the existing checkpoint gate. No EKS,
Diem, freezer, new Conductor claim, delegation, merge or retarget.

This experiment proves a generic source-field bridge only if the checks
pass. Affine integral closure, both proper charts, local valuations,
polarization and subgroup transfer remain separate. After a successful
bridge, run the complete descending Frobenius normalization prescribed in
NORMALIZATION-LEAD.md with the R61 trace Gram and multiplication table.

## Version 2 source-ring map repair
`bridge64-01` is rejected in full despite a late marker. Both mapped old
source equations, the changed-quadratic negative and coefficient inversion
were reached without an assertion failure, but Singular reported that
`basis51` was undefined when the source module was matrix-cast *before*
the cross-ring map. The fresh script maps the source module first and then
casts its image to a matrix. All 32 columns, rank, inverse products and
negative controls retain their original acceptance requirements. The first
attempt's partial values are diagnostic only.

## Version 3 basis orientation repair
`bridge64-02` reached the mapped source equations and first three basis
functions but rejected a zero fourth function. The matrix `H` had been
read by columns. The retained compact PID identity `H*sourceLift=D*I`
defines its **rows** as normal-order basis coordinates in the power basis.
The fresh script checks this identity after `z=1/u`, then maps each of all
32 rows. No failed attempt's partial function is accepted; rank, inverse
products, source equations and both negatives remain mandatory.

## Version 4 exact source-matrix transport
`bridge64-03` is rejected at `H*sourceLift=D I`. The earlier inference that
the retained basis should be read by rows was not established: mapping the
module and casting its image did not preserve the original matrix
representation. Reproduce the accepted R53 sequence exactly: cast
`basis51` to a matrix **in the compact source ring**, check its identity
with `sourceLift51` and `D` there, then map those already constructed
matrices and denominator to the actual coefficient field. Check the same
identity after substitution before evaluating any of the 32 retained
columns. Full rank, inverse products, source equations and negatives stay
mandatory; all three rejected raw attempts remain intact.
