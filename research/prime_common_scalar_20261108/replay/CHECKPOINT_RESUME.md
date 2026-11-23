# Resume: native rational Hermite recovery

## Goal
Native correctness and faithful cost refinement of the actual rational Hermite
recovery primitive; then coordinate/recursive evaluator obligations.

## Lead / hypothesis
The actual constructed finite quotient/recovery is correct at source child
precision. Next test a scaled modular accumulator avoiding full rational sums;
see ACCUMULATOR_NEXT.md for formulas, proof and implementation obligations.

## Spec
protocol-v1.json preserves original freeze. protocol.json v2 changes checkpoint
transport only under explicit owner approval, frozen before data.180positive
cases plus3defective controls each, seed0, exactly2physical invocations.
Source77aed44a5; instrument.json pins unchanged native binaries.

## State
Both native invocations exit0 and frozen analyzer exit0.366 records accepted,
raw TSV files byte-identical, SHA25646a73583d9709c64acbcf02fbadb82734ed48b16cb5c33d85341ca74db512c09.
Unsigned producer evidence; report reports/run-20261109T200742Z/REPORT.md.
Canonical export.json has2physical rows, no valid clocks. Source/markers/raw
outcomes committed/pushed to git origin in place of unavailable S3.

## Next steps
Finish PR3757 evidence/CI review. Continue ACCUMULATOR_NEXT.md: canonical scaled
residue extraction and odd-unit coefficient refinement; charge all construction,
retained state and recomputation; extend coordinate lattice and full recurrence.
Do not invoke run.sh again: this protocol's two invocations are complete.

## Runs in flight
None. Native census and recount are complete, raw evidence is preserved.

## Stop and success conditions
Completed exact finite acceptance:183 unique accepted records each, all controls,
byte-identical replay and exact frozen recount. This2-run budget is exhausted.
No timing, native memory-saving, full recursive runtime or curve solve inferred.

## Open questions
Scaled accumulator native correctness/costs, coordinate lattice, full recurrence.
