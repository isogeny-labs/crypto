# E26: matched ordinary variable order before solver execution

The 90-workload construction/census from source
`0ac8b4ef653dba62d2dc033bd9cc5d78b5ef1fd5` is preserved. No solver or
candidate-cost outcome has been inspected when adding this control.

Variable permutation itself could account for a Frobenius-arm difference.
Before solver execution, add an ordinary direct-chain control with precisely
the surviving Frobenius order: real addition intermediates before base inputs.
No anti-rational variables or twist computations occur in this control.

The native adapter permutes the original polynomial exponent vectors. It
then sets every anti-rational variable to zero in the Frobenius input, drops
that variable block and identically zero equations, and checks that every
remaining polynomial coefficient equals the ordinary control after monic
normalization. This gives an exact polynomial-level check for all 90 inputs,
not a sampled point comparison. Save every permutation and input hash.

`followup block-match` writes a fresh augmented catalog and copies the
unchanged workload census. It does not regenerate targets, bases, signed
relations or old data. Preserve the original catalog and raw construction.
All four candidates and the ordinary controls remain. The solver panel now
has 7 arms per workload plus one A/A cell: 631 requested cells.
Schedule the A/A control and all arms of each workload together so global
caps cannot systematically defer every matched ordinary control. Set
`F4_FP_GM=1` explicitly, as in the existing native solver control; retain
the same option for all native candidates. No other kernel is changed.

Freeze this amendment and the adapter before running the solver. The V1
30-second native cell, shared 30-second external/checker cell, one-worker,
4-GiB, 1,800-second phase caps and all completeness requirements remain.
Attribution of any lower native count requires this matched ordinary control;
full cost, fastest-external and heldout conclusions remain unresolved.
