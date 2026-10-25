# Independent aggregation of the E26 pilot

This native reader is separate from the experiment driver and reopens each
raw input, native result and accepted Singular transcript. It verifies their
hashes, recomputes branch sums and complete-cell statuses, and checks every
accepted vdim against the construction census. It separately counts accepted
stock msolve bases. Missing/failed branches remain incomplete, never zero-cost
successes. Compare recomputed totals with the primary assessment.

Check that the A/A polynomial inputs are identical and, when certified, that
their native row-multiplication counts agree. Compare the Frobenius arm to
the exact ordinary prefix-order control. Recompute comparisons with the best
certified ordinary control among direct product, factored, Boolean and matched
prefix order. Ratios describe row-reduction multiplications only; all material
remaining costs remain missing. No complete-cost or fastest-tool conclusion
is admitted, and no candidate or heldout selector is chosen.

Hash the accepted original PARI script/outputs and verify that its signed
witness-check count equals the sum of recorded canonical tuples across the
90 workloads. These counts can repeat the same point tuple across overlapping
base sizes; they are not independent experimental samples. Verify all 90
matched-order coefficient checks. Record step-log counter coverage separately;
an inconsistency cannot be hidden by successful basis certificates.

Run only after the solver/checker phases, with fresh output, one worker,
4 GiB and a 600-second cap. Keep this source/protocol in Git before executing.
This is independent aggregation on the same host. It does not replace the
PARI implementation checks, source-group witness checks or an independent host.
