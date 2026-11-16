# Sparse original relation assembly: standalone and cold gates

V2sourcea00660d5e81e, pre-run checkpoint20261116T003417Z. Both registered gates passed once, with no scientific rerun or unexpected failure. V1source, binaries and raw remain unchanged.

The original314relations now store2378nonzero coefficients with maximum support8, rather than314length129vectors. Sorted accumulation exactly cancels coincident positive and negative terms. The fixed canonical-line subtraction is retained. The code checks every original sparse relation after back-substitution and requires every supported point log known.

Standalone run-v2/:232x47reduced core/2413entries, exact nonzero kernel atattempt0,129point logs projectively match the independent old dense collector, all314residuals pass, twelve planted controls and original-matrix/zero corruption tests pass. exit0/stderr empty.

Fullcold cold-run-v2/:all positive stages0; expected missing-loader101. Sparse original rows and reduced core are used by the actual descent, with no dense reference/fallback in candidate. Fresh setup, discovered chart, calibration/alignment, actual inverse solves and conic descent recover183049 with source/destination verification and sourcek+1rejection. Four/two conic descents agree. DRIVER-COLD-V2.txt and cold-run-v2/retain all outputs and exclusions.

This is engineering correctness evidence and a structural representation change. No wall time/RSS measurement, tenfold runtime gain, exponent fit, broader-field or genus2/4claim. The fixed48factor base and F127restrictions remain, as do exhaustive construction/verification controls. The prior failed coldcost result is unchanged.

## Next algebraic candidate: shared canonical-divisor variable

For sparse point-log coordinates x with x(O)=0, let h encode a canonical line and H_i encode each split-line divisor. Current equations are (H_i−h)x=0. Introduce one auxiliary coordinate z and use

    H_i x − z = 0 for every i,
    h x − z = 0 once.

Projection (x,z)↦x and lifting x↦(x,hx) give mutually inverse linear maps between kernels. This is exact over any coefficient field, with no division, randomness or geometric assumption beyond the original relation validity. Rank of the augmented matrix is rank(original)+1, and nullity is preserved. Each quartic line row then has at most5coefficients instead of8, plus one defining row; coincidences may reduce the old rows more, so a net stored-entry or speed gain is not automatic. Keep z on the small factor-base side, not as an eliminable large-prime column, to avoid immediately undoing the transformation.

This is an auxiliary-variable representation, not a claimed novel theorem. It may reduce repeated hub columns/fill or may make the solve worse. Before execution: independent original-residual/projective-log equivalence controls, explicit source/protocol/hash freeze, S3gate, and a separate outputdirectory. Any tenfold runtime claim still requires a full charged same-target comparison, including setup and verification. No experiment for this candidate was executed in V2.
