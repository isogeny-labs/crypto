# Independent Kummer fiber review

Result: validated_scoped written derivation. This is not a universal Lean theorem or verification of all native coordinate formulas.

Published inputs are pinned to125c631e4005785e344167d8f699b4bf23415fe6. Native delta_lift in source/pruned.rs150–163 agrees with the stated coefficient equations. For monic u=X^2+a1X+a0, reduction of v^2 gives b0^2-a0*b1^2 and2*b0*b1-a1*b1^2. Polynomial reduction is valid even when u has repeated roots; squarefreeness is unnecessary here.

Assuming S=b1^2 and a valid true divisor, S!=0 yields b1=t or -t; characteristic!=2 makes2*b1 nonzero, determining b0 uniquely. Negating b1 negates b0. The candidate vector has at most two distinct values. In the S=0 branch, b1=0 and both outer root choices repeat that same branch; global candidates.contains deduplicates before tested=candidates.len(). At most two b0 roots remain. R1=0 is required for validity but is not checked during generation: native j.valid performs this rejection later. Candidate generation can produce two invalid candidates when R1!=0; that does not contradict the bound or the validity-qualified completeness claim.

Characteristic!=2 is necessary for the source's nonzero-b1 division step. The abstract root-count bound can be sharpened in characteristic2, but that does not justify the algorithm's division by2*b1. The producer appropriately assumes odd characteristic throughout the native derivation.

The neighbor uniqueness argument is correct in an abelian group with quotient fibers exactly{P,-P} on the compared neighbor locus and2G!=0. Equality of neighbor images gives2D=0 or2G=0. In the first case D=-D, so there are not two distinct candidates. This is uniqueness only: native projection success, true candidate inclusion, correctness of the coordinate extraction S, and the neighbor fiber property are separately needed for existence. The ordinary local fiber argument cannot silently cover a degree-one or identity neighbor chart.

The distinction between matrix-square arithmetic and geometry is sound. M^2=cI, c!=0 proves invertibility/nonzero preservation. Taking M=I demonstrates that it alone cannot certify translation by a designated nonzero two-torsion point. Correct repair requires the separate translation equation in the correct model and valid domain.

Fresh native replay passed the producer's eleven fixed integer assertions. Seven independent fixed assertions cover S=0 with inconsistent R1, repeated support u=X^2, and the characteristic2 denominator. These fixed checks illustrate boundary algebra; they do not establish the universal chart identity or group quotient theorem. No enumeration, curve computations, timing or scientific execution occurred.

One reviewer build failed because integer literals with rem_euclid lacked explicit i64 type. The exact failed source and compiler stderr remain. Explicit i64 literals corrected it; fail-closed replay.sh rebuilt and reran both controls successfully. An attempted execution after the failed initial compile had no binary and is also retained.

Prior familiarity: reviewer worked on signed graph algebra/source and related polynomial validity, but did not author this Kummer fiber derivation. Native source and written statements were reviewed together after publication; no blind review claim. The prospective review joint is coordinator-PLAN.md. Scope remains written source/math validation, not official scientific status.
