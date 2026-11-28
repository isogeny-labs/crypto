# Frozen witness-backed leaf-sum F4 panel

Take the source-recorded S235 verified-hit query rows at indices 3 and 4. For each row, subtract the first point of its recorded factor-base witness: singleton index 27 for query 3 and index 67 for query 4. Preserve exact query, singleton, and residual point coordinates. Assert the selected singleton and its point key match the immutable witness record. The solver receives only the residual x-coordinate.

Build the same 30-variable trace-zero S3 control and invertible `s=u+v` systems as in the five-residual panel, including q-specific trace-relation elimination and 30 Boolean field equations. Reproduce the archived earlier control and leaf-sum inputs byte for byte before generating these two residuals. Run the candidate then control for query 3, and control then candidate for query 4. Pin msolve 0.11.0, FLINT, one thread, flags `-g 2 -t 1 -v 2 -m 256`, 300-second wall cap, 1-GiB observed process-tree RSS gate, and 20-MiB combined output cap. Save every result and raw artifact.

Independently enumerate S3 roots, map leaf-sum roots back to control variables, evaluate both input systems, and check factor-base point lifts and the complete three-point group relation. Local timing is exploratory. Preserve failed, capped, and missing-result statuses.
